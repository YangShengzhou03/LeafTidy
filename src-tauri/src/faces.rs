//! 人脸聚类：`YuNet` 人脸检测 + `SFace` 特征提取 + 余弦距离 `DBSCAN` 分组。
//! 人物记录持久化到 faces.json，用户改名/合并后（confirmed）下次运行优先匹配。

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use image::imageops::FilterType;
use ort::session::builder::GraphOptimizationLevel;
use ort::session::Session;
use ort::value::Tensor;

use crate::fs_util;
use crate::models::{FacePhoto, Person, PersonGroup, TaskProgress};

// --- 内置模型 ---

static YUNET_BYTES: &[u8] = include_bytes!("../resources/models/face_detection_yunet_2023mar.onnx");
static SFACE_BYTES: &[u8] = include_bytes!("../resources/models/face_recognition_sface_2021dec.onnx");

/// 已确认人物匹配的余弦相似度阈值
const CONFIRMED_MATCH_SIM: f32 = 0.65;
/// DBSCAN 聚类的余弦距离阈值（相似度 ≥ 1 - eps 视为近邻）
const CLUSTER_EPS: f32 = 0.363;
/// `YuNet` 检测分数下限
const DETECT_SCORE_MIN: f32 = 0.6;
/// `YuNet` 输入图像最长边
const YUNET_INPUT_MAX: u32 = 320;
/// `SFace` 输入尺寸
const SFACE_INPUT: u32 = 112;

// --- 人脸引擎 ---

pub struct FaceEngine {
    yunet: RefCell<Session>,
    sface: RefCell<Session>,
}

/// 一张图中检测到的人脸（原图坐标系）
struct DetectedFace {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

impl FaceEngine {
    pub fn new() -> Result<Self, String> {
        let yunet = Session::builder()
            .map_err(|e| format!("创建人脸检测会话失败: {e}"))?
            .with_optimization_level(GraphOptimizationLevel::Level3)
            .map_err(|e| format!("设置优化级别失败: {e}"))?
            .with_intra_threads(2)
            .map_err(|e| format!("设置线程失败: {e}"))?
            .commit_from_memory(YUNET_BYTES)
            .map_err(|e| format!("加载人脸检测模型失败: {e}"))?;

        let sface = Session::builder()
            .map_err(|e| format!("创建特征提取会话失败: {e}"))?
            .with_optimization_level(GraphOptimizationLevel::Level3)
            .map_err(|e| format!("设置优化级别失败: {e}"))?
            .with_intra_threads(2)
            .map_err(|e| format!("设置线程失败: {e}"))?
            .commit_from_memory(SFACE_BYTES)
            .map_err(|e| format!("加载人脸特征模型失败: {e}"))?;

        Ok(Self { yunet: RefCell::new(yunet), sface: RefCell::new(sface) })
    }

    /// 检测人脸，返回原图坐标系的矩形框
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::items_after_statements)]
    fn detect_faces(&self, img: &image::DynamicImage) -> Result<Vec<DetectedFace>, String> {
        let (orig_w, orig_h) = (img.width() as f32, img.height() as f32);

        // 长边缩到 YUNET_INPUT_MAX，并 pad 到 32 的倍数
        let scale = (YUNET_INPUT_MAX as f32) / orig_w.max(orig_h);
        let scaled_w = ((orig_w * scale).round() as u32).max(32);
        let scaled_h = ((orig_h * scale).round() as u32).max(32);
        let resized = img.resize_exact(scaled_w, scaled_h, FilterType::Triangle);
        let pad_w = scaled_w.div_ceil(32) * 32;
        let pad_h = scaled_h.div_ceil(32) * 32;

        // OpenCV Zoo 模型为 BGR 训练，交换通道
        let mut canvas = image::RgbImage::new(pad_w, pad_h);
        for (x, y, p) in resized.to_rgb8().enumerate_pixels() {
            canvas.put_pixel(x, y, image::Rgb([p[2], p[1], p[0]]));
        }

        let plane = (pad_w * pad_h) as usize;
        let mut chw = vec![0.0f32; 3 * plane];
        for (x, y, p) in canvas.enumerate_pixels() {
            let base = (y as usize) * (pad_w as usize) + (x as usize);
            chw[base] = f32::from(p[0]);
            chw[plane + base] = f32::from(p[1]);
            chw[2 * plane + base] = f32::from(p[2]);
        }

        let input = ndarray::Array4::from_shape_vec((1, 3, pad_h as usize, pad_w as usize), chw)
            .map_err(|e| format!("创建检测输入失败: {e}"))?;
        let tensor = Tensor::from_array(input).map_err(|e| format!("创建检测张量失败: {e}"))?;

        let mut session = self.yunet.borrow_mut();
        let outputs = session.run(ort::inputs![tensor]).map_err(|e| format!("人脸检测推理失败: {e}"))?;

        let (_shape, data) = outputs[0]
            .try_extract_tensor::<f32>()
            .map_err(|e| format!("提取检测结果失败: {e}"))?;
        let data = data.to_vec();
        // 2023mar 版本输出 [1, N, 15]，按固定 15 列展开，不依赖 shape API
        const ROW_COLS: usize = 15;
        if data.is_empty() || data.len() % ROW_COLS != 0 {
            return Err(format!("意外的检测输出长度 {}", data.len()));
        }
        let rows = data.len() / ROW_COLS;

        let mut faces = Vec::new();
        for r in 0..rows {
            let row = &data[r * ROW_COLS..(r + 1) * ROW_COLS];
            let score = row[14];
            if score < DETECT_SCORE_MIN {
                continue;
            }
            let cx = row[0] * pad_w as f32 / scale;
            let cy = row[1] * pad_h as f32 / scale;
            let fw = row[2] * pad_w as f32 / scale;
            let fh = row[3] * pad_h as f32 / scale;
            let x = (cx - fw / 2.0).max(0.0) as u32;
            let y = (cy - fh / 2.0).max(0.0) as u32;
            let w = fw.min(orig_w - x as f32) as u32;
            let h = fh.min(orig_h - y as f32) as u32;
            if w > 16 && h > 16 {
                faces.push(DetectedFace { x, y, w, h });
            }
        }
        Ok(faces)
    }

    /// 提取单个人脸的 512 维特征（L2 归一化）
    fn embed_face(&self, img: &image::DynamicImage, face: &DetectedFace) -> Result<Vec<f32>, String> {
        // ponytail: 简单裁剪不做关键点对齐；升级：用 YuNet 眼/嘴关键点做相似变换
        let crop = img.crop_imm(face.x, face.y, face.w, face.h);
        let resized = crop.resize_exact(SFACE_INPUT, SFACE_INPUT, FilterType::Triangle);

        let plane = (SFACE_INPUT * SFACE_INPUT) as usize;
        let mut chw = vec![0.0f32; 3 * plane];
        for (x, y, p) in resized.to_rgb8().enumerate_pixels() {
            let base = (y as usize) * (SFACE_INPUT as usize) + (x as usize);
            // BGR + (v - 0.5) / 0.5
            chw[base] = (f32::from(p[2]) - 127.5) / 127.5;
            chw[plane + base] = (f32::from(p[1]) - 127.5) / 127.5;
            chw[2 * plane + base] = (f32::from(p[0]) - 127.5) / 127.5;
        }

        let input = ndarray::Array4::from_shape_vec((1, 3, 112, 112), chw)
            .map_err(|e| format!("创建特征输入失败: {e}"))?;
        let tensor = Tensor::from_array(input).map_err(|e| format!("创建特征张量失败: {e}"))?;

        let mut session = self.sface.borrow_mut();
        let outputs = session.run(ort::inputs![tensor]).map_err(|e| format!("特征提取推理失败: {e}"))?;

        let (_shape, data) = outputs[0]
            .try_extract_tensor::<f32>()
            .map_err(|e| format!("提取特征失败: {e}"))?;
        let mut vec: Vec<f32> = data.to_vec();
        if vec.is_empty() {
            return Err("模型输出为空".to_string());
        }
        let norm = vec.iter().map(|v| v * v).sum::<f32>().sqrt();
        if norm > 1e-6 {
            for v in &mut vec {
                *v /= norm;
            }
        }
        Ok(vec)
    }
}

// --- 聚类工具 ---

fn cosine_distance(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
    1.0 - dot
}

/// 简化 `DBSCAN（min_samples=1，每点皆为核心点）：按距离阈值求连通分量`
#[allow(clippy::items_after_statements)]
fn dbscan(vectors: &[Vec<f32>], eps: f32) -> Vec<usize> {
    let n = vectors.len();
    let mut parent: Vec<usize> = (0..n).collect();

    fn find(parent: &mut [usize], mut x: usize) -> usize {
        while parent[x] != x {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }
        x
    }

    for i in 0..n {
        for j in i + 1..n {
            if cosine_distance(&vectors[i], &vectors[j]) <= eps {
                let (ri, rj) = (find(&mut parent, i), find(&mut parent, j));
                if ri != rj {
                    parent[ri] = rj;
                }
            }
        }
    }

    (0..n).map(|i| find(&mut parent, i)).collect()
}

// --- 人物记录持久化 ---

fn faces_path() -> PathBuf {
    let mut path = dirs::data_dir().unwrap_or_else(std::env::temp_dir);
    path.push("leaf-tidy");
    path.push("faces.json");
    path
}

pub fn load_persons() -> Vec<Person> {
    let path = faces_path();
    if !path.exists() {
        return Vec::new();
    }
    let Ok(text) = std::fs::read_to_string(&path) else {
        return Vec::new();
    };
    serde_json::from_str(&text).unwrap_or_default()
}

pub fn save_persons(persons: &[Person]) -> Result<(), String> {
    let path = faces_path();
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| format!("创建数据目录失败: {e}"))?;
    }
    let text = serde_json::to_string_pretty(persons).map_err(|e| format!("序列化人物记录失败: {e}"))?;
    crate::fs_util::atomic_write(&path, text.as_bytes())
}

/// 取下一个空闲的人物编号（人物1、人物2…，允许跳号）
fn next_person_name(persons: &[Person]) -> String {
    let mut used = Vec::new();
    for p in persons {
        if let Some(rest) = p.name.strip_prefix("人物") {
            if let Ok(n) = rest.parse::<u32>() {
                used.push(n);
            }
        }
    }
    let mut n = 1;
    while used.contains(&n) {
        n += 1;
    }
    format!("人物{n}")
}

fn new_person_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

// --- 主流程 ---

#[allow(clippy::cast_precision_loss, clippy::too_many_lines, clippy::items_after_statements, clippy::cast_possible_truncation)]
pub fn run_face_cluster<F>(
    paths: &[String],
    mut progress_callback: F,
    token: &crate::cancel::CancelToken,
) -> Result<Vec<PersonGroup>, String>
where
    F: FnMut(&TaskProgress) + Send,
{

    let mut image_paths: Vec<String> = Vec::new();
    for path_str in paths {
        let path = Path::new(path_str);
        if !path.exists() {
            continue;
        }
        for entry in walkdir::WalkDir::new(path)
            .follow_links(false)
            .into_iter()
            .filter_entry(|e| !e.file_name().to_string_lossy().starts_with('.'))
        {
            let Ok(entry) = entry else { continue };
            if !entry.file_type().is_file() {
                continue;
            }
            let p = entry.path().to_string_lossy().to_string();
            if crate::metadata::is_image_file(&p) {
                image_paths.push(p);
            }
        }
    }

    let total = image_paths.len() as u64;
    if total == 0 {
        return Ok(Vec::new());
    }

    let engine = FaceEngine::new()?;
    let mut persons = load_persons();

    struct FaceRecord {
        path: String,
        name: String,
        vector: Vec<f32>,
        thumb: Option<String>,
    }
    let mut records: Vec<FaceRecord> = Vec::new();

    // 阶段一：逐图检测 + 提取特征
    for (idx, path) in image_paths.iter().enumerate() {
        if token.is_cancelled() {
            return Err("[CANCELLED] 用户已取消操作".to_string());
        }
        progress_callback(&TaskProgress {
            total,
            processed: idx as u64,
            current_item: Some(path.clone()),
            percentage: (idx as f64 / total as f64) * 100.0,
        });

        // YuNet 内部缩放到 320px，用 max_dim=320 触发大图缩放解码
        let Ok(img) = fs_util::load_image_limited(path, 320) else { continue };
        let Ok(faces) = engine.detect_faces(&img) else { continue };

        let name = Path::new(path)
            .file_name()
            .map_or_else(|| path.clone(), |n| n.to_string_lossy().to_string());

        for face in faces {
            match engine.embed_face(&img, &face) {
                Ok(vector) => {
                    let crop = img.crop_imm(face.x, face.y, face.w, face.h);
                    let thumb = fs_util::make_thumb_jpeg(crop, 96);
                    records.push(FaceRecord { path: path.clone(), name: name.clone(), vector, thumb });
                }
                Err(e) => log::debug!("特征提取失败 {path}: {e}"),
            }
        }
    }

    if records.is_empty() {
        return Ok(Vec::new());
    }

    // 阶段二：优先匹配已确认人物质心
    let mut assigned: Vec<Option<usize>> = vec![None; records.len()];
    let mut remaining: Vec<usize> = Vec::new();

    for (ri, rec) in records.iter().enumerate() {
        let mut best: Option<(usize, f32)> = None;
        for (pi, person) in persons.iter().enumerate() {
            if !person.confirmed || person.centroid.is_empty() {
                continue;
            }
            let sim = 1.0 - cosine_distance(&rec.vector, &person.centroid);
            if sim >= CONFIRMED_MATCH_SIM && best.is_none_or(|(_, s)| sim > s) {
                best = Some((pi, sim));
            }
        }
        if let Some((pi, _)) = best {
            assigned[ri] = Some(pi);
            // 增量式更新质心（在线均值）
            let person = &mut persons[pi];
            let n = person.photo_count as f32;
            for (c, v) in person.centroid.iter_mut().zip(&rec.vector) {
                *c = (*c * n + v) / (n + 1.0);
            }
            person.photo_count += 1;
        } else {
            remaining.push(ri);
        }
    }

    // 阶段三：剩余特征 DBSCAN 聚成新人物
    let vectors: Vec<Vec<f32>> = remaining.iter().map(|&ri| records[ri].vector.clone()).collect();
    let labels = dbscan(&vectors, CLUSTER_EPS);

    let mut root_order: Vec<usize> = Vec::new();
    let mut root_map: HashMap<usize, usize> = HashMap::new();
    for &label in &labels {
        if let std::collections::hash_map::Entry::Vacant(e) = root_map.entry(label) {
            e.insert(root_order.len());
            root_order.push(label);
        }
    }


    let mut new_person_idx: Vec<Option<usize>> = vec![None; root_order.len()];
    for (cluster, &root) in root_order.iter().enumerate() {
        let members: Vec<usize> = remaining
            .iter()
            .enumerate()
            .filter(|(k, _)| labels[*k] == root)
            .map(|(_, &ri)| ri)
            .collect();

        // 计算簇质心（均值后归一化）
        let dim = records[members[0]].vector.len();
        let mut centroid = vec![0.0f32; dim];
        for &ri in &members {
            for (c, v) in centroid.iter_mut().zip(&records[ri].vector) {
                *c += v;
            }
        }
        let norm = centroid.iter().map(|v| v * v).sum::<f32>().sqrt();
        if norm > 1e-6 {
            for c in &mut centroid {
                *c /= norm;
            }
        }

        let person = Person {
            id: new_person_id(),
            name: next_person_name(&persons),
            confirmed: false,
            centroid,
            photo_count: members.len() as u32,
        };
        persons.push(person);
        new_person_idx[cluster] = Some(persons.len() - 1);

        for ri in members {
            assigned[ri] = Some(persons.len() - 1);
        }
    }

    save_persons(&persons)?;


    let mut groups: Vec<PersonGroup> = persons
        .iter()
        .map(|p| PersonGroup {
            id: p.id.clone(),
            name: p.name.clone(),
            confirmed: p.confirmed,
            photos: Vec::new(),
        })
        .collect();

    for (ri, rec) in records.iter().enumerate() {
        if let Some(pi) = assigned[ri] {
            let sim = 1.0 - cosine_distance(&rec.vector, &persons[pi].centroid);
            groups[pi].photos.push(FacePhoto {
                path: rec.path.clone(),
                name: rec.name.clone(),
                thumb: rec.thumb.clone(),
                similarity: sim,
            });
        }
    }

    groups.retain(|g| !g.photos.is_empty());
    // 确认人物在前，同组按相似度降序
    groups.sort_by(|a, b| {
        b.confirmed
            .cmp(&a.confirmed)
            .then_with(|| b.photos.len().cmp(&a.photos.len()))
    });
    for g in &mut groups {
        g.photos.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap_or(std::cmp::Ordering::Equal));
    }

    progress_callback(&TaskProgress {
        total,
        processed: total,
        current_item: None,
        percentage: 100.0,
    });

    Ok(groups)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_distance() {
        assert!((cosine_distance(&[1.0, 0.0], &[1.0, 0.0])).abs() < 1e-6, "同向量距离为 0");
        assert!((cosine_distance(&[1.0, 0.0], &[0.0, 1.0]) - 1.0).abs() < 1e-6, "正交向量距离为 1");
        assert!((cosine_distance(&[1.0, 0.0], &[-1.0, 0.0]) - 2.0).abs() < 1e-6, "反向向量距离为 2");
    }

    #[test]
    fn test_dbscan_two_clusters_and_outlier() {
        // 两个紧簇 + 1 个离群点
        let vectors = vec![
            vec![1.0, 0.0],
            vec![0.99, 0.14],
            vec![0.98, 0.2],
            vec![0.0, 1.0],
            vec![0.14, 0.99],
            vec![0.2, 0.98],
            vec![-1.0, 0.0],
        ];
        let labels = dbscan(&vectors, 0.363);
        assert_eq!(labels[0], labels[1]);
        assert_eq!(labels[1], labels[2]);
        assert_eq!(labels[3], labels[4]);
        assert_eq!(labels[4], labels[5]);
        assert_ne!(labels[0], labels[3]);
        assert_ne!(labels[0], labels[6]);
    }

    #[test]
    fn test_centroid_merge_math() {
        // (1,0) 权重 2 与 (0,1) 权重 3 合并 → (0.4, 0.6)，与 merge_persons 同公式
        let mut c = [1.0f32, 0.0];
        let (nt, ns) = (2.0f32, 3.0f32);
        let v = vec![0.0f32, 1.0];
        let total = nt + ns;
        for (c, v) in c.iter_mut().zip(&v) {
            *c = (*c * nt + v * ns) / total;
        }
        assert!((c[0] - 0.4).abs() < 1e-6 && (c[1] - 0.6).abs() < 1e-6);
    }

    #[test]
    fn test_next_person_name_skips_gaps() {
        let persons = vec![
            Person { id: "a".into(), name: "人物1".into(), confirmed: false, centroid: vec![], photo_count: 0 },
            Person { id: "b".into(), name: "人物2".into(), confirmed: false, centroid: vec![], photo_count: 0 },
        ];
        assert_eq!(next_person_name(&persons), "人物3");

        // 用户把人物2改名为「爸爸」→ 下一个空闲号仍是 3
        let persons = vec![
            Person { id: "a".into(), name: "人物1".into(), confirmed: true, centroid: vec![], photo_count: 0 },
            Person { id: "b".into(), name: "爸爸".into(), confirmed: true, centroid: vec![], photo_count: 0 },
        ];
        assert_eq!(next_person_name(&persons), "人物2");

        // 编号空洞（人物1、人物3）→ 补 2
        let persons = vec![
            Person { id: "a".into(), name: "人物1".into(), confirmed: false, centroid: vec![], photo_count: 0 },
            Person { id: "b".into(), name: "人物3".into(), confirmed: false, centroid: vec![], photo_count: 0 },
        ];
        assert_eq!(next_person_name(&persons), "人物2");
    }

    #[test]
    fn test_person_record_roundtrip() {
        let persons = vec![Person {
            id: "x".into(),
            name: "人物1".into(),
            confirmed: false,
            centroid: vec![0.1, 0.2, 0.3],
            photo_count: 5,
        }];
        save_persons(&persons).unwrap();
        let loaded = load_persons();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].name, "人物1");
        assert_eq!(loaded[0].photo_count, 5);
        // 清理测试数据
        let _ = std::fs::remove_file(faces_path());
    }
}
