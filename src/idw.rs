use ::errors::*;
use ::utils::{almost_equal, PtValue};
use ::bbox::Bbox;
use std::f64;

pub fn idw_interpolation1<T>(reso_x: u32, reso_y: u32, bbox: &Bbox, obs_points: &[T], b: i32) -> Result<Vec<T>>
    where T: PtValue
    {
    let x_step = (bbox.max_x - bbox.min_x) / reso_x as f64;
    let y_step = (bbox.max_y - bbox.min_y) / reso_y as f64;
    let mut plots = Vec::with_capacity((reso_x * reso_y) as usize);
    for i in 0..reso_x {
        for j in 0..reso_y {
            plots.push(
                do_idw1(
                    obs_points,
                    bbox.min_x + x_step * i as f64,
                    bbox.min_y + y_step * j as f64,
                    b)
                );
        }
    }
    Ok(plots)
}

fn do_idw1<T>(obs_points: &[T], x: f64, y: f64, b: i32) -> T
    where T: PtValue
    {
    let (zw, sw): (f64, f64) = obs_points.iter()
        .fold((0.0, 0.0), |mut data, obs_pt| {
            let val = obs_pt.get_value();
            let dist = obs_pt.distance(x, y);
            if almost_equal(dist, 0.0, 1.0e-5) {
                data.0 = val;
            }
            let w = 1.0 / dist.powi(b);
            data.0 += w * val;
            data.1 += w;
            data
        });
    T::new(x, y, zw / sw)
}

// pub fn idw_interpolation2(reso_lat: u32, reso_lon: u32, bbox: &Bbox, obs_points: &[PtValue], b: i32) -> Result<Vec<PtValue>> {
//     let lon_step = (bbox.max_lon - bbox.min_lon) / reso_lon as f64;
//     let lat_step = (bbox.max_lat - bbox.min_lat) / reso_lat as f64;
//     let mut plots = Vec::with_capacity((reso_lat * reso_lon) as usize);
//     for i in 0..reso_lat {
//         for j in 0..reso_lon {
//             plots.push(PtValue{lat: bbox.min_lat + lat_step * i as f64, lon: bbox.min_lon + lon_step * j as f64, value: 0.0})
//         }
//     }
//     do_idw2(obs_points, &mut plots, b);
//     Ok(plots)
// }
//
// fn do_idw2(obs_points: &[PtValue], flat_square_grid: &mut Vec<PtValue>, b: i32){
//     for cell in flat_square_grid {
//         let (zw, sw): (f64, f64) = obs_points.iter()
//             .fold((0.0, 0.0), |mut data, obs_pt| {
//                 let dist = haversine_distance(obs_pt.lon, obs_pt.lat, cell.lon, cell.lat);
//                 if almost_equal(dist, 0.0, 1.0e-5) {
//                     data.0 = obs_pt.value;
//                 }
//                 let w = 1.0 / dist.powi(b);
//                 data.0 += w * obs_pt.value;
//                 data.1 += w;
//                 data
//             });
//         cell.value = zw / sw;
//     }
// }
