use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;

struct AdminCodeMapping {
    provinces: HashMap<String, String>,
    cities: HashMap<String, String>,
}

impl AdminCodeMapping {
    fn new() -> Self {
        AdminCodeMapping {
            provinces: HashMap::new(),
            cities: HashMap::new(),
        }
    }

    fn add_from_line(&mut self, line: &str) {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() < 12 {
            return;
        }

        let feature_class = parts.get(6).map_or("", |v| v);
        let feature_code = parts.get(7).map_or("", |v| v);
        let admin1 = parts.get(10).map_or("", |v| v);
        let admin2 = parts.get(11).map_or("", |v| v);

        let all_names = parts.get(3).map_or("", |v| v);
        let chinese_name = extract_chinese_name(all_names);
        let name = if chinese_name.is_empty() {
            parts.get(1).map_or("", |v| v).to_string()
        } else {
            chinese_name
        };

        if feature_class == "A" && feature_code == "ADM1" {
            if !admin1.is_empty() {
                self.provinces.insert(admin1.to_string(), name);
            }
        }
        else if feature_class == "A" && feature_code == "ADM2" {
            if !admin1.is_empty() && !admin2.is_empty() {
                let key = format!("{}_{}", admin1, admin2);
                self.cities.insert(key, name);
            }
        }
    }

    fn get_province(&self, admin1: &str) -> Option<String> {
        self.provinces.get(admin1).cloned()
    }

    fn get_city(&self, admin1: &str, admin2: &str) -> Option<String> {
        let key = format!("{}_{}", admin1, admin2);
        self.cities.get(&key).cloned()
    }
}

fn extract_chinese_name(all_names: &str) -> String {
    let chinese_names: Vec<String> = all_names
        .split(',')
        .filter(|name| name.chars().any(|c| c >= '\u{4e00}' && c <= '\u{9fff}'))
        .map(|name| name.to_string())
        .collect();
    
    if chinese_names.is_empty() {
        return String::new();
    }
    
    for name in &chinese_names {
        if name.ends_with("区") || name.ends_with("县") || name.ends_with("市") 
           || name.ends_with("盟") || name.ends_with("旗") || name.ends_with("自治县")
           || name.ends_with("自治旗") || name.ends_with("自治区") {
            return name.clone();
        }
    }
    
    chinese_names
        .into_iter()
        .min_by_key(|name| name.len())
        .unwrap_or_default()
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct GeoName {
    pub name: String,
    pub chinese_name: String,
    pub latitude: f64,
    pub longitude: f64,
    point: [f64; 3],
    pub country: String,
    admin1: String,
    admin2: String,
    admin3: String,
}

impl GeoName {
    pub fn from_line(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() < 13 {
            return None;
        }

        let name = parts.get(1)?.to_string();
        let all_names = parts.get(3)?.to_string();
        let chinese_name = extract_chinese_name(&all_names);
        let latitude: f64 = parts.get(4)?.parse().ok()?;
        let longitude: f64 = parts.get(5)?.parse().ok()?;
        let country = parts.get(8)?.to_string();
        let admin1 = parts.get(10).unwrap_or(&"").to_string();
        let admin2 = parts.get(11).unwrap_or(&"").to_string();
        let admin3 = parts.get(12).unwrap_or(&"").to_string();

        let point = lat_lon_to_3d(latitude, longitude);

        Some(GeoName {
            name,
            chinese_name,
            latitude,
            longitude,
            point,
            country,
            admin1,
            admin2,
            admin3,
        })
    }

    pub fn from_lat_lon(latitude: f64, longitude: f64) -> Self {
        let point = lat_lon_to_3d(latitude, longitude);
        GeoName {
            name: String::new(),
            chinese_name: String::new(),
            latitude,
            longitude,
            point,
            country: String::new(),
            admin1: String::new(),
            admin2: String::new(),
            admin3: String::new(),
        }
    }

    fn squared_distance(&self, other: &GeoName) -> f64 {
        let dx = self.point[0] - other.point[0];
        let dy = self.point[1] - other.point[1];
        let dz = self.point[2] - other.point[2];
        dx * dx + dy * dy + dz * dz
    }

    fn axis_squared_distance(&self, other: &GeoName, axis: usize) -> f64 {
        let distance = self.point[axis] - other.point[axis];
        distance * distance
    }
}

fn lat_lon_to_3d(lat: f64, lon: f64) -> [f64; 3] {
    let lat_rad = lat.to_radians();
    let lon_rad = lon.to_radians();
    let cos_lat = lat_rad.cos();
    [
        cos_lat * lon_rad.cos(),
        cos_lat * lon_rad.sin(),
        lat_rad.sin(),
    ]
}

struct KDNode {
    left: Option<Box<KDNode>>,
    right: Option<Box<KDNode>>,
    location: GeoName,
}

pub struct KDTree {
    root: Option<Box<KDNode>>,
}

impl KDTree {
    pub fn new(items: Vec<GeoName>) -> Self {
        if items.is_empty() {
            return KDTree { root: None };
        }
        let mut items = items;
        KDTree {
            root: Some(Self::build_tree(&mut items, 0)),
        }
    }

    fn build_tree(items: &mut [GeoName], depth: usize) -> Box<KDNode> {
        if items.is_empty() {
            panic!("build_tree called with empty items");
        }

        let axis = depth % 3;

        // 按当前轴排序
        items.sort_by(|a, b| {
            a.point[axis]
                .partial_cmp(&b.point[axis])
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let mid = items.len() / 2;

        // 分割数组
        let (left_items, rest) = items.split_at_mut(mid);
        let (location, right_items) = rest.split_first_mut().unwrap();

        // 需要克隆 location
        let location_clone = location.clone();

        let left = if !left_items.is_empty() {
            Some(Self::build_tree(left_items, depth + 1))
        } else {
            None
        };

        let right = if !right_items.is_empty() {
            Some(Self::build_tree(right_items, depth + 1))
        } else {
            None
        };

        Box::new(KDNode {
            left,
            right,
            location: location_clone,
        })
    }

    pub fn find_nearest(&self, search: &GeoName) -> Option<&GeoName> {
        self.root
            .as_ref()
            .map(|root| &Self::find_nearest_node(root, search, 0).location)
    }

    fn find_nearest_node<'a>(node: &'a KDNode, search: &GeoName, depth: usize) -> &'a KDNode {
        let axis = depth % 3;

        let go_left = search.point[axis] < node.location.point[axis];
        let (next, other) = if go_left {
            (&node.left, &node.right)
        } else {
            (&node.right, &node.left)
        };

        let mut best = match next {
            Some(next_node) => Self::find_nearest_node(next_node, search, depth + 1),
            None => node,
        };

        if node.location.squared_distance(search) < best.location.squared_distance(search) {
            best = node;
        }

        if let Some(other_node) = other {
            let axis_distance = node.location.axis_squared_distance(search, axis);
            if axis_distance < best.location.squared_distance(search) {
                let possible_best = Self::find_nearest_node(other_node, search, depth + 1);
                if possible_best.location.squared_distance(search)
                    < best.location.squared_distance(search)
                {
                    best = possible_best;
                }
            }
        }

        best
    }
}

pub struct ReverseGeoCode {
    kd_tree_places: KDTree,
    kd_tree_admin: KDTree,
    admin_mapping: AdminCodeMapping,
}

impl ReverseGeoCode {
    pub fn from_reader<R: Read>(reader: R) -> Result<Self, String> {
        let buf_reader = BufReader::new(reader);
        let mut places = Vec::new();
        let mut admin_units = Vec::new();
        let mut admin_mapping = AdminCodeMapping::new();

        for line in buf_reader.lines() {
            let line = line.map_err(|e| format!("读取行失败: {}", e))?;
            
            admin_mapping.add_from_line(&line);
            
            if let Some(geo_name) = GeoName::from_line(&line) {
                let feature_class = line.split('\t').nth(6).map_or("", |v| v);
                let feature_code = line.split('\t').nth(7).map_or("", |v| v);
                
                if feature_class == "P" {
                    places.push(geo_name);
                }
                else if feature_class == "A" && 
                        (feature_code == "ADM1" || feature_code == "ADM2" || feature_code == "ADM3") {
                    admin_units.push(geo_name);
                }
            }
        }

        if places.is_empty() && admin_units.is_empty() {
            return Err("没有找到有效的地理位置数据".to_string());
        }

        println!(
            "加载了 {} 个省份, {} 个城市, {} 个地点, {} 个行政区划",
            admin_mapping.provinces.len(),
            admin_mapping.cities.len(),
            places.len(),
            admin_units.len()
        );

        Ok(ReverseGeoCode {
            kd_tree_places: KDTree::new(places),
            kd_tree_admin: KDTree::new(admin_units),
            admin_mapping,
        })
    }

    pub fn reverse_geocode(&self, latitude: f64, longitude: f64) -> super::models::GpsLocation {
        let search = GeoName::from_lat_lon(latitude, longitude);
        
        let get_display_name = |geo: &GeoName| {
            if !geo.chinese_name.is_empty() {
                geo.chinese_name.clone()
            } else {
                geo.name.clone()
            }
        };
        
        let (province, city, district) = if let Some(admin_unit) = self.kd_tree_admin.find_nearest(&search) {
            let province = self.admin_mapping.get_province(&admin_unit.admin1)
                .or_else(|| Some("未知省份".to_string()));
            
            let city = self.admin_mapping.get_city(&admin_unit.admin1, &admin_unit.admin2)
                .or_else(|| Some("未知城市".to_string()));
            
            let district = if admin_unit.admin3.is_empty() {
                Some("未知区县".to_string())
            } else {
                Some(get_display_name(admin_unit))
            };
            
            (province, city, district)
        } else {
            (Some("未知省份".to_string()), Some("未知城市".to_string()), Some("未知区县".to_string()))
        };
        
        let place = if let Some(place_unit) = self.kd_tree_places.find_nearest(&search) {
            let place_name = get_display_name(place_unit);
            let district_name = district.as_deref().unwrap_or("");
            if !place_name.is_empty() && place_name != district_name {
                Some(place_name)
            } else {
                None
            }
        } else {
            None
        };

        super::models::GpsLocation {
            latitude,
            longitude,
            province,
            city,
            district,
            place,
        }
    }
}

static GEOCODER: OnceLock<ReverseGeoCode> = OnceLock::new();

static GEOCODER_READY: AtomicBool = AtomicBool::new(false);

pub fn init_geocoder(data: &str) -> Result<(), String> {
    let geocoder = ReverseGeoCode::from_reader(data.as_bytes())?;
    let _ = GEOCODER.set(geocoder);
    GEOCODER_READY.store(true, Ordering::SeqCst);
    Ok(())
}

pub fn is_geocoder_ready() -> bool {
    GEOCODER_READY.load(Ordering::SeqCst)
}

pub fn get_geocoder() -> Option<&'static ReverseGeoCode> {
    GEOCODER.get()
}

pub fn reverse_geocode(lat: f64, lng: f64) -> Result<super::models::GpsLocation, String> {
    match get_geocoder() {
        Some(geocoder) => Ok(geocoder.reverse_geocode(lat, lng)),
        None => Err("地理编码器未初始化，请确保 geodata/CN.txt 文件存在".to_string()),
    }
}