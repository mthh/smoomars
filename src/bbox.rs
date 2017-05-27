use std::f64::{INFINITY, NEG_INFINITY};
use utils::PtValue;

#[derive(Debug, Clone, Copy)]
pub struct Bbox {
    pub min_x: f64,
    pub max_x: f64,
    pub min_y: f64,
    pub max_y: f64,
}

impl Bbox {
    pub fn new(min_x: f64, max_x: f64, min_y: f64, max_y: f64) -> Self {
        Bbox {
            min_x: min_x,
            max_x: max_x,
            min_y: min_y,
            max_y: max_y,
        }
    }

    pub fn from_points<T>(obs_points: &[T]) -> Self
        where T: PtValue
    {
        let (mut min_x, mut max_x, mut min_y, mut max_y) =
            (INFINITY, NEG_INFINITY, INFINITY, NEG_INFINITY);
        for pt in obs_points {
            let (pt_x, pt_y) = pt.get_coordinates();
            if pt_x > max_x {
                max_x = pt_x;
            } else if pt_x < min_x {
                min_x = pt_x;
            }
            if pt_y > max_y {
                max_y = pt_y;
            } else if pt_y < min_y {
                min_y = pt_y;
            }
        }
        Bbox {
            min_x: min_x,
            max_x: max_x,
            min_y: min_y,
            max_y: max_y,
        }
    }
}
