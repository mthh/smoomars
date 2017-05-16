#[derive(Debug)]
pub struct Bbox {
    pub min_x: f64,
    pub max_x: f64,
    pub min_y: f64,
    pub max_y: f64
}

impl Bbox {
    pub fn new(min_x:f64, max_x:f64, min_y:f64, max_y:f64) -> Bbox {
        Bbox {min_x: min_x, max_x: max_x, min_y: min_y, max_y: max_y}
    }
}
