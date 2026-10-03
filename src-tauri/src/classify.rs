use image::imageops::FilterType;
use ort::session::builder::GraphOptimizationLevel;
use ort::session::Session;
use ort::value::Tensor;
use std::cell::RefCell;
use std::path::Path;

use crate::fs_util;
use crate::imagenet_labels;
use crate::models::{ClassifyProgress, ClassifyResult, ClassifyRule, ModelDefinition};

// --- 内置模型 ---

static MODEL_NANO_BYTES: &[u8] = include_bytes!("../resources/models/yolov8n-cls.onnx");
static MODEL_SMALL_BYTES: &[u8] = include_bytes!("../resources/models/yolov8s-cls.onnx");

const MODEL_NANO_INPUT_SIZE: u32 = 224;
const MODEL_SMALL_INPUT_SIZE: u32 = 224;

/// 自定义模型默认输入尺寸（图像分类模型常用 224x224）
const DEFAULT_CUSTOM_MODEL_INPUT_SIZE: u32 = 224;

/// 获取所有内置模型信息
pub fn list_builtin_models() -> Vec<ModelDefinition> {
    vec![
        ModelDefinition {
            id: "yolov8n-cls".to_string(),
            name: "YOLOv8n (快速)".to_string(),
            description: "轻量极速版，适合批量处理".to_string(),
            size_mb: 10.4,
            input_size: MODEL_NANO_INPUT_SIZE,
            top1_accuracy: 71.4,
        },
        ModelDefinition {
            id: "yolov8s-cls".to_string(),
            name: "YOLOv8s (精准)".to_string(),
            description: "标准精度版，分类更准确".to_string(),
            size_mb: 24.3,
            input_size: MODEL_SMALL_INPUT_SIZE,
            top1_accuracy: 76.0,
        },
    ]
}

fn load_builtin_model(model_id: &str) -> Result<(&'static [u8], u32), String> {
    match model_id {
        "yolov8n-cls" => Ok((MODEL_NANO_BYTES, MODEL_NANO_INPUT_SIZE)),
        "yolov8s-cls" => Ok((MODEL_SMALL_BYTES, MODEL_SMALL_INPUT_SIZE)),
        _ => Err(format!("未知模型: {model_id}")),
    }
}

// --- 图片分类器 ---

/// 图片分类器
pub struct ImageClassifier {
    session: RefCell<Session>,
    input_size: u32,
}

impl ImageClassifier {
    /// 从模型文件创建分类器（用户自定义模型）
    pub fn new<P: AsRef<Path>>(model_path: P, input_size: u32) -> Result<Self, ort::Error> {
        let session = Session::builder()?
            .with_optimization_level(GraphOptimizationLevel::Level3)?
            .with_intra_threads(2)?
            .commit_from_file(model_path.as_ref())?;
        Ok(Self { session: RefCell::new(session), input_size })
    }

    /// 从内存字节创建分类器（内置模型）
    pub fn from_bytes(model_bytes: &[u8], input_size: u32) -> Result<Self, ort::Error> {
        let session = Session::builder()?
            .with_optimization_level(GraphOptimizationLevel::Level3)?
            .with_intra_threads(2)?
            .commit_from_memory(model_bytes)?;
        Ok(Self { session: RefCell::new(session), input_size })
    }

    /// 对单张图片进行分类
    pub fn classify_image<P: AsRef<Path>>(&self, image_path: P) -> Result<(String, f32), String> {
        // 模型输入仅 input_size×input_size，触发大图缩放解码避免解码整张原图
        let img = fs_util::load_image_limited(image_path.as_ref().to_str().unwrap_or(""), self.input_size)
            .map_err(|e| format!("无法打开图片: {e}"))?;

        let resized = img.resize_exact(self.input_size, self.input_size, FilterType::Triangle);

        let img_h = self.input_size as usize;
        let img_w = self.input_size as usize;
        let plane_size = img_h * img_w;

        let mut chw = vec![0.0_f32; 3 * plane_size];
        for (x, y, p) in resized.to_rgb8().enumerate_pixels() {
            let yi = y as usize;
            let xi = x as usize;
            let base = yi * img_w + xi;
            // YOLOv8-cls 输出已归一化到 [0,1]，无需 ImageNet mean/std
            chw[base] = f32::from(p[0]) / 255.0;
            chw[plane_size + base] = f32::from(p[1]) / 255.0;
            chw[2 * plane_size + base] = f32::from(p[2]) / 255.0;
        }

        let input_array = ndarray::Array4::from_shape_vec((1, 3, img_h, img_w), chw)
            .map_err(|e| format!("创建张量失败: {e}"))?;

        let input_tensor = Tensor::from_array(input_array)
            .map_err(|e| format!("创建输入失败: {e}"))?;

        let mut session = self.session.borrow_mut();
        let outputs = session.run(ort::inputs![input_tensor])
            .map_err(|e| format!("推理失败: {e}"))?;

        let (_shape, scores) = outputs[0]
            .try_extract_tensor::<f32>()
            .map_err(|e| format!("提取输出失败: {e}"))?;

        let scores: Vec<f32> = scores.to_vec();
        if scores.is_empty() {
            return Err("模型输出为空".to_string());
        }
        // YOLOv8-cls 导出的 ONNX 已在分类头内做 softmax，输出即概率；
        // 若再 softmax 一次会把置信度压成 ~1/N 的近似均匀分布
        let sum: f32 = scores.iter().sum();
        let probs: Vec<f32> = if scores.iter().all(|&s| (0.0..=1.0).contains(&s)) && (sum - 1.0).abs() < 1e-2 {
            scores
        } else {
            let max_score = scores.iter().copied().fold(f32::NEG_INFINITY, f32::max);
            let exp_sum: f32 = scores.iter().map(|s| (s - max_score).exp()).sum();
            scores.iter().map(|s| (s - max_score).exp() / exp_sum).collect()
        };

        let (max_idx, max_prob) = probs
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Less))
            .expect("probs non-empty when scores non-empty");

        Ok((imagenet_labels::label(max_idx).to_string(), *max_prob))
    }
}

/// 逐张分类并复制图片（识别一张 → 复制一张）
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::too_many_lines, clippy::too_many_arguments)]
pub fn classify_images_batch(
    model_source: &str,
    is_builtin: bool,
    image_paths: &[String],
    rule: &ClassifyRule,
    output_dir: &str,
    log_callback: impl Fn(&str),
    progress_callback: impl Fn(&ClassifyProgress),
    token: &crate::cancel::CancelToken,
) -> Result<Vec<ClassifyResult>, String> {
    if image_paths.is_empty() {
        return Ok(Vec::new());
    }

    let total = image_paths.len() as u64;
    let model_desc = if is_builtin { model_source } else { "自定义模型" };
    let msg = format!("[AI分类] 开始加载模型 {model_desc} ({total} 张图片)");
    log::info!("{msg}");
    log_callback(&msg);
    let load_start = std::time::Instant::now();

    let classifier = if is_builtin {
        let (bytes, input_size) = load_builtin_model(model_source)?;
        ImageClassifier::from_bytes(bytes, input_size)
            .map_err(|e| {
                let msg = format!("[AI分类] 模型加载失败: {e}");
                log::error!("{msg}");
                log_callback(&msg);
                msg
            })?
    } else {
        ImageClassifier::new(model_source, DEFAULT_CUSTOM_MODEL_INPUT_SIZE)
            .map_err(|e| {
                let msg = format!("[AI分类] 模型加载失败: {e}");
                log::error!("{msg}");
                log_callback(&msg);
                msg
            })?
    };

    let load_elapsed = load_start.elapsed().as_millis();
    let msg = format!("[AI分类] 模型加载完成，耗时 {load_elapsed}ms");
    log::info!("{msg}");
    log_callback(&msg);

    let mut results = Vec::with_capacity(image_paths.len());
    let batch_start = std::time::Instant::now();
    let mut last_logged_count = 0u64;
    let mut failed_count = 0u64;

    for (idx, path) in image_paths.iter().enumerate() {
        if token.is_cancelled() {
            let msg = format!("[AI分类] 用户取消，已处理 {idx}/{total} 张");
            log::warn!("{msg}");
            log_callback(&msg);
            return Err("[CANCELLED] 用户已取消操作".to_string());
        }

        progress_callback(&ClassifyProgress {
            total,
            processed: idx as u64,
            current_item: Some(path.clone()),
            percentage: (idx as f64 / total as f64) * 100.0,
            current_category: None,
            result: None,
        });

        let start = std::time::Instant::now();

        match classifier.classify_image(path) {
            Ok((category, confidence)) => {
                let cat = if confidence >= rule.confidence_threshold {
                    category
                } else {
                    "other".to_string()
                };
                let elapsed = start.elapsed().as_millis() as u64;

                // 诊断日志：首张图片耗时反映冷启动性能，>1s/张提示性能回退
                if idx == 0 || elapsed > 1000 {
                    let msg = format!(
                        "[AI分类] 第 {}/{} 张完成，耗时 {elapsed}ms，类别 {cat} ({:.2}%)",
                        idx + 1,
                        total,
                        confidence * 100.0,
                    );
                    log::info!("{msg}");
                    log_callback(&msg);
                }

                // 逐张复制到目标目录
                let move_result = copy_single_file(path, &cat, output_dir, f64::from(confidence), &log_callback);

                results.push(ClassifyResult {
                    source_path: path.clone(),
                    success: move_result.is_ok(),
                    error: move_result.as_ref().err().cloned(),
                    category: Some(cat),
                    confidence: Some(confidence),
                    process_time_ms: Some(elapsed),
                });
            }
            Err(e) => {
                let msg = format!("[AI分类] 处理失败 {path}: {e}");
                log::warn!("{msg}");
                log_callback(&msg);
                failed_count += 1;
                results.push(ClassifyResult {
                    source_path: path.clone(),
                    success: false,
                    error: Some(e),
                    category: None,
                    confidence: None,
                    process_time_ms: None,
                });
            }
        }

        let processed = (idx + 1) as u64;

        // 每 50 张输出一次进度
        if processed - last_logged_count >= 50 || processed == total {
            let pct = (processed as f64 / total as f64) * 100.0;
            let elapsed_secs = batch_start.elapsed().as_secs();
            let rate = if elapsed_secs > 0 {
                processed as f64 / elapsed_secs as f64
            } else {
                0.0
            };
            let msg = format!(
                "[AI分类] 进度: {processed}/{total} ({pct:.1}%), 已用 {elapsed_secs}s, 速率 {rate:.1} 张/秒",
            );
            log::info!("{msg}");
            log_callback(&msg);
            last_logged_count = processed;
        }

        progress_callback(&ClassifyProgress {
            total,
            processed,
            current_item: Some(path.clone()),
            percentage: (processed as f64 / total as f64) * 100.0,
            current_category: results.last().and_then(|r| r.category.clone()),
            result: results.last().cloned(),
        });
    }

    let batch_elapsed = batch_start.elapsed().as_secs();
    let success_count = total - failed_count;
    let msg = format!(
        "[AI分类] 完成: 成功 {success_count} 张, 失败 {failed_count} 张, 总耗时 {batch_elapsed}s",
    );
    log::info!("{msg}");
    log_callback(&msg);
    Ok(results)
}

/// 将单张图片复制到目标目录（处理文件名冲突）
fn copy_single_file(source: &str, category: &str, output_dir: &str, confidence: f64, log_callback: &dyn Fn(&str)) -> Result<(), String> {
    let source_path = Path::new(source);
    if !source_path.exists() {
        return Err("源文件不存在".to_string());
    }

    let target_dir = format!("{output_dir}/{category}");
    std::fs::create_dir_all(&target_dir)
        .map_err(|e| format!("创建目录失败 {target_dir}: {e}"))?;

    let file_name = source_path
        .file_name()
        .map_or_else(|| "unknown".to_string(), |n| n.to_string_lossy().to_string());

    let final_dest = crate::fs_util::resolve_duplicate_filename(
        Path::new(&target_dir),
        &file_name,
    )
    .1;

    let dest_str = final_dest.to_string_lossy().to_string();

    std::fs::copy(source_path, &final_dest)
        .map_err(|e| format!("复制文件失败: {e}"))?;

    let msg = format!("[AI分类] 复制: {source} -> {dest_str} ({category}, {:.1}%)", confidence * 100.0);
    log::info!("{msg}");
    log_callback(&msg);
    Ok(())
}

/// 检查自定义模型文件是否有效
pub fn validate_model<P: AsRef<Path>>(model_path: P) -> bool {
    ImageClassifier::new(model_path, DEFAULT_CUSTOM_MODEL_INPUT_SIZE).is_ok()
}

#[cfg(test)]
mod tests {
    // 与 classify_image 中的判定逻辑保持一致
    fn to_probs(scores: Vec<f32>) -> Vec<f32> {
        let sum: f32 = scores.iter().sum();
        if scores.iter().all(|&s| (0.0..=1.0).contains(&s)) && (sum - 1.0).abs() < 1e-2 {
            scores
        } else {
            let max_score = scores.iter().copied().fold(f32::NEG_INFINITY, f32::max);
            let exp_sum: f32 = scores.iter().map(|s| (s - max_score).exp()).sum();
            scores.iter().map(|s| (s - max_score).exp() / exp_sum).collect()
        }
    }

    #[test]
    fn passthrough_when_already_softmaxed() {
        let probs = to_probs(vec![0.1, 0.7, 0.2]);
        assert_eq!(probs, vec![0.1, 0.7, 0.2], "已是概率的输出不应再次 softmax");
    }

    #[test]
    fn softmax_when_logits() {
        let probs = to_probs(vec![0.0, 5.0, 0.0]);
        assert!((probs[1] - 0.986).abs() < 1e-2, "logits 应正确 softmax，实际 {probs:?}");
        assert!((probs.iter().sum::<f32>() - 1.0).abs() < 1e-5);
    }
}
