use errors::*;
use utils::{almost_equal, PtValue};
use bbox::Bbox;
use std::f64;


pub fn idw_interpolation1<T>(reso_x: u32,
                             reso_y: u32,
                             bbox: &Bbox,
                             obs_points: &[T],
                             b: f64)
                             -> Result<Vec<T>>
    where T: PtValue
{
    let x_step = (bbox.max_x - bbox.min_x) / reso_x as f64;
    let y_step = (bbox.max_y - bbox.min_y) / reso_y as f64;
    let mut plots = Vec::with_capacity((reso_x * reso_y) as usize);
    let idw = Idw::new(obs_points, b);
    for i in 0..reso_x {
        for j in 0..reso_y {
            let x = bbox.min_x + x_step * i as f64;
            let y = bbox.min_y + y_step * j as f64;
            let val = idw.interp_point((x, y));
            plots.push(T::new(x, y, val));
        }
    }
    Ok(plots)
}

pub struct Idw<'a, T: 'a> {
    obs_points: &'a [T],
    power: f64,
}

impl<'a, T> Idw<'a, T>
    where T: PtValue
{
    pub fn new(obs_points: &'a [T], power: f64) -> Self {
        Idw {
            obs_points: obs_points,
            power: power,
        }
    }

    pub fn interp_point(&self, pt: (f64, f64)) -> f64 {
        let (zw, sw): (f64, f64) = self.obs_points
            .iter()
            .fold((0.0, 0.0), |mut data, obs_pt| {
                let val = obs_pt.get_value();
                let dist = obs_pt.distance(pt.0, pt.1);
                if almost_equal(dist, 0.0, 1.0e-5) {
                    data.0 = val;
                }
                let w = 1.0 / dist.powf(self.power);
                data.0 += w * val;
                data.1 += w;
                data
            });
        zw / sw
    }
}
