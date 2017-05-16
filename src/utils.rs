use serde_json;
use serde::ser::Serialize;
use std::f64;
use std::fs::File;
use std::io::{Read,Write};
use csv;
use errors::*;
use bbox::Bbox;
use geojson::{Feature, FeatureCollection, GeoJson, Geometry, Value};
use gdal::raster::{Driver, Buffer};


static R: f64 = 6372.8 * 1000.0;

pub enum SetPtValue {
    Spherical(Vec<SphericalPtValue>),
    Cartesian(Vec<CartesianPtValue>)
}

pub trait PtValue {
    fn new(f64, f64, f64) -> Self;
    fn get_coordinates(&self) -> (f64, f64);
    fn get_value(&self) -> f64;
    fn set_value(&mut self, f64);
    fn get_triplet(&self) -> (f64, f64, f64);
    fn distance(&self, f64, f64) -> f64;
}

#[derive(Serialize, Deserialize)]
pub struct ValuesJson<T> where T: PtValue {
    values: Vec<T>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SphericalPtValue {
    pub lat: f64,
    pub lon: f64,
    pub value: f64
}

impl PtValue for SphericalPtValue {
    fn new(lon: f64, lat: f64, value: f64) -> Self {
        SphericalPtValue { lon: lon, lat: lat, value: value }
    }
    fn get_coordinates(&self) -> (f64, f64) {
        (self.lon, self.lat)
    }
    fn get_value(&self) -> f64 {
        self.value
    }
    fn set_value(&mut self, value: f64) {
        self.value = value;
    }
    fn get_triplet(&self) -> (f64, f64, f64) {
        (self.lon, self.lat, self.value)
    }
    fn distance(&self, other_lon: f64, other_lat: f64) -> f64 {
        // let th1 = self.lat;
        // let ph1 = self.lon;
        // let th2 = other.lat;
        // let ph2 = other.lon;
        let mut _ph1 = self.lon - other_lon;
        _ph1 = _ph1.to_radians();
        let _th1 = self.lat.to_radians();
        let _th2 = other_lat.to_radians();
        let dz: f64 = _th1.sin() - _th2.sin();
        let dx: f64 = _ph1.cos() * _th1.cos() - _th2.cos();
        let dy: f64 = _ph1.sin() * _th1.cos();
        ((dx * dx + dy * dy + dz * dz).sqrt() / 2.0).asin() * 2.0 * R
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CartesianPtValue {
    pub x: f64,
    pub y: f64,
    pub value: f64
}

impl PtValue for CartesianPtValue {
    fn new(x: f64, y: f64, value: f64) -> Self {
        CartesianPtValue { x: x, y: y, value: value }
    }
    fn get_coordinates(&self) -> (f64, f64) {
        (self.x, self.y)
    }
    fn get_value(&self) -> f64 {
        self.value
    }
    fn set_value(&mut self, value: f64) {
        self.value = value;
    }
    fn get_triplet(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.value)
    }
    fn distance(&self, other_x: f64, other_y: f64) -> f64 {
        let (dx, dy) = (self.x - other_x, self.y - other_y);
        ((dx * dx) + (dy * dy)).sqrt()
    }
}

pub fn parse_geojson_points<T>(path: &str, field_name: &str) -> Result<Vec<T>> where T: PtValue {
    let mut file = File::open(path)?;
    let mut raw_json = String::new();
    file.read_to_string(&mut raw_json)?;
    let decoded_geojson = raw_json.parse::<GeoJson>()?;
    let features = match decoded_geojson {
        GeoJson::FeatureCollection(collection) => collection.features,
        _ => return Err("Error: expected a FeatureCollection".into())
    };
    let mut res = Vec::with_capacity(features.len());
    for ft in features {
        if let Some(ref geometry) = ft.geometry {
            if let Value::Point(ref positions) = geometry.value {
                let prop = ft.properties.unwrap();
                let value = prop.get(field_name).unwrap();
                let val = if value.is_number() {
                    value.as_f64().unwrap()
                } else {
                    value.to_string().replace("\"", "").parse::<f64>()?
                };
                res.push(T::new(positions[1], positions[0], val));
            }
        } else {
            return Err("Error: empty FeatureCollection".into());
        }
    }
    Ok(res)
}

pub fn parse_json_points<T>(path: &str) -> Result<Vec<T>> where T: PtValue {
    let mut file = File::open(path)?;
    let mut raw_json = String::new();
    file.read_to_string(&mut raw_json)?;
    let decoded: serde_json::Value = serde_json::from_str(&raw_json)?;
    let ref arr = if decoded.is_object() && !decoded.get("values").is_none() && decoded["values"].is_array() {
        decoded["values"].as_array().unwrap()
    } else if decoded.is_array() {
        decoded.as_array().unwrap()
    } else {
        return Err("Invalid datastructure".into());
    };
    let mut res = Vec::with_capacity(arr.len());
    for elem in arr.iter() {
        let value = match elem["value"] {
            serde_json::Value::Number(ref val) => val.as_f64().unwrap(),
            serde_json::Value::String(ref val) => val.to_string().parse::<f64>()?,
            _ => return Err("Invalid datastructure".into())
        };
        let y = match elem["lat"] {
            serde_json::Value::Number(ref val) => val.as_f64().unwrap(),
            serde_json::Value::String(ref val) => val.to_string().parse::<f64>()?,
            _ => return Err("Invalid datastructure".into())
        };
        let x = match elem["lon"] {
            serde_json::Value::Number(ref val) => val.as_f64().unwrap(),
            serde_json::Value::String(ref val) => val.to_string().parse::<f64>()?,
            _ => return Err("Invalid datastructure".into())
        };
        res.push(T::new(x, y, value));
    }
    Ok(res)
}

pub fn save_json_points<T>(path: &str, result_points: Vec<T>) -> Result<()> where T: PtValue + Serialize {
    let encoded = serde_json::to_string(&result_points)?;
    let mut file = File::create(path)?;
    file.write(encoded.as_bytes())?;
    Ok(())
}

pub fn save_geojson_points(path: &str, result_points: Vec<SphericalPtValue>) -> Result<()> {
    let mut features = Vec::with_capacity(result_points.len());
    for res_pt in result_points {
        let (lat, lon, value) = res_pt.get_triplet();
        let geometry = Geometry::new(Value::Point(vec![lon, lat]));
        let mut prop = serde_json::Map::new();
        prop.insert(String::from("value"),  serde_json::to_value(value)?);
        features.push(Feature {
            bbox: None,
            geometry: Some(geometry),
            id: None,
            foreign_members: None,
            properties: Some(prop),
        });
    }
    let feature_collection = FeatureCollection {
        bbox: None,
        features: features,
        foreign_members: None
    };
    let serialized = GeoJson::from(feature_collection).to_string();
    let mut file = File::create(path)?;
    file.write(serialized.as_bytes())?;
    Ok(())
}

pub fn parse_csv_points<T>(path: &str) -> Result<Vec<T>> where T: PtValue{
    let mut rdr = csv::Reader::from_file(path)?;
    let mut res = Vec::new();
    for record in rdr.decode() {
        let (x, y, val): (f64, f64, f64) = record?;
        res.push(T::new(x, y, val));
    }
    Ok(res)
}

pub fn almost_equal(a: f64, b: f64, epsilon: f64) -> bool {
    let diff = (a - b).abs();
    if a == b {
        true
    } else if a == 0.0 || b == 0.0 || diff < f64::MIN_POSITIVE {
        diff < (epsilon * f64::MIN_POSITIVE)
    } else {
        (diff / f64::min(a.abs() + b.abs(), f64::MAX)) < epsilon
    }
}

pub fn write_to_raster<T>(result_points: Vec<T>, bbox: &Bbox, reso: (u32, u32), path: &str) -> Result<()> where T: PtValue {
    let driver = Driver::get("GTiff").unwrap();
    println!("{}", driver.long_name());
    let pixel_size_x = (bbox.max_x - bbox.min_x) / reso.0 as f64;
    let pixel_size_y = (bbox.max_y - bbox.min_y) / reso.1 as f64;
    let dataset = driver.create_with_band_type::<f64>(path, reso.0 as isize, reso.1 as isize, 3).unwrap();
    dataset.set_geo_transform(&[bbox.min_x, pixel_size_x, 0.0, bbox.max_y, 0.0, pixel_size_y]);
    let mut data = Vec::with_capacity(result_points.len());
    for pt in result_points {
        data.push(pt.get_value());
    }
    let buffer = Buffer::new((reso.0 as usize, reso.1 as usize), data);
    dataset.write_raster(3 as isize, (0, 0), (reso.0 as usize, reso.1 as usize), buffer);
    Ok(())
}
