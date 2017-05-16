use ::errors::*;
use ::utils::*;
use ::bbox::Bbox;
use std::f64;
use std::marker::{Send, Sync};
use jobsteal::{make_pool, BorrowSpliteratorMut, Spliterator};

pub enum SmoothType {
    Exponential = 0,
    Pareto
}

pub struct StewartPotentialGrid<'a> {
    smooth_func: fn(f64, f64, f64) -> f64,
    reso_x: u32,
    reso_y: u32,
    beta: f64,
    alpha: f64,
    bbox: &'a Bbox,
    n_thread: u32
}

impl<'a> StewartPotentialGrid<'a> {
    pub fn new(span: f64, beta: f64, interaction_type: SmoothType, bbox: &'a Bbox, reso_x: u32, reso_y: u32, parallel: bool) -> Self {
        let n_thread = match parallel { true => 3, false => 1 };
        match interaction_type {
            SmoothType::Exponential => {
                StewartPotentialGrid {
                    bbox: bbox,
                    reso_x: reso_x, reso_y: reso_y,
                    beta: beta,
                    alpha: 0.69314718055994529 / (span).powf(beta),
                    smooth_func: exponential,
                    n_thread: n_thread

                }
            },
            SmoothType::Pareto => {
                StewartPotentialGrid {
                    reso_x: reso_x, reso_y: reso_y,
                    bbox: bbox,
                    beta: beta,
                    alpha: ((2.0 as f64).powf(1.0 / beta) - 1.0) / span,
                    smooth_func: pareto,
                    n_thread: n_thread
                }
            }
        }
    }
}

#[inline(always)]
fn exponential(alpha: f64, beta: f64, dist: f64) -> f64 {
    (-alpha * dist.powf(beta)).exp()
}

#[inline(always)]
fn pareto(alpha: f64, beta: f64, dist: f64) -> f64 {
    (1.0 + alpha * dist).powf(-beta)
}


pub fn stewart<T>(stewart_config: &StewartPotentialGrid, obs_points: &[T]) -> Result<Vec<T>> where T: PtValue + Send + Sync {
    let (bbox, reso_x, reso_y) = (stewart_config.bbox, stewart_config.reso_x, stewart_config.reso_y);
    let x_step = (bbox.max_x - bbox.min_x) / reso_x as f64;
    let y_step = (bbox.max_y - bbox.min_y) / reso_y as f64;
    let mut plots = Vec::with_capacity((reso_x * reso_y) as usize);
    for i in 0..reso_x {
        for j in 0..reso_y {
            plots.push(T::new(bbox.min_x + x_step * i as f64, bbox.min_y + y_step * j as f64, 0.0));
        }
    }
    if stewart_config.n_thread < 2 {
        do_pot(&mut plots, obs_points, stewart_config)
    } else {
        do_pot_par(&mut plots, obs_points, stewart_config)
    }
    Ok(plots)
}

fn do_pot<T>(flat_grid:  &mut Vec<T>, obs_points: &[T], stewart_config: &StewartPotentialGrid) where T: PtValue {
    let (beta, alpha) = (stewart_config.beta, stewart_config.alpha);
    let func = stewart_config.smooth_func;
    for cell in flat_grid.iter_mut() {
        // let (cell_x, cell_y) = cell.get_coordinates();
        let value = obs_points.iter()
            .fold(0.0, |mut sum, obs_pt| {
                let (x, y, val) = obs_pt.get_triplet();
                sum += val * func(alpha, beta, cell.distance(x, y));
                sum
            });
        cell.set_value(value);
    }
}

fn do_pot_par<T>(flat_grid:  &mut Vec<T>, obs_points: &[T], stewart_config: &StewartPotentialGrid) where T: PtValue + Send + Sync {
    let (beta, alpha) = (stewart_config.beta, stewart_config.alpha);
    let func = stewart_config.smooth_func;
    let mut pool = make_pool(stewart_config.n_thread as usize).unwrap();
    flat_grid.split_iter_mut().for_each(&pool.spawner(), |cell|{
        let value = obs_points.iter()
            .fold(0.0, |mut sum, obs_pt| {
                let (x, y, val) = obs_pt.get_triplet();
                sum += val * func(alpha, beta, cell.distance(x, y));
                sum
            });
        cell.set_value(value);
    });
}

//
// More straightforward implementation, without using/creating a StewartPotentialGrid instance, etc :
//
//
// pub fn par_stewart(reso_lat: u32, reso_lon: u32, bbox: Bbox, obs_points: &[PtValue], b: f64) -> Result<Vec<PtValue>> {
//     let lon_step = (bbox.max_lon - bbox.min_lon) / reso_lon as f64;
//     let lat_step = (bbox.max_lat - bbox.min_lat) / reso_lat as f64;
//     let mut plots = Vec::with_capacity((reso_lat * reso_lon) as usize);
//     for i in 0..reso_lat {
//         for j in 0..reso_lon {
//             // dens_mat.push(vec![0.0; n_obs_pts]);
//             plots.push(PtValue{lat: bbox.min_lat + lat_step * i as f64, lon: bbox.min_lon + lon_step * j as f64, value: 0.0})
//         }
//     }
//     let span = 15.0;
//     do_pot_par(&mut plots, obs_points, span, b);
//     Ok(plots)
// }
//
// fn do_pot_par(flat_grid:  &mut Vec<PtValue>, obs_points: &[PtValue], span: f64, beta: f64){
//     let alpha = 0.69314718055994529 / (span).powf(beta);
//     let mut pool = make_pool(3).unwrap();
//     flat_grid.split_iter_mut().for_each(&pool.spawner(), |cell|{
//         cell.value = obs_points.iter()
//             .fold(0.0, |mut sum, obs_pt| {
//                 sum += obs_pt.value * (-alpha * (haversine_distance(obs_pt.lon, obs_pt.lat, cell.lon, cell.lat)).powf(beta)).exp();
//                 sum
//             });
//     });
// }
//
// pub fn stewart(reso_lat: u32, reso_lon: u32, bbox: Bbox, obs_points: &[PtValue], b: f64) -> Result<Vec<PtValue>> {
//     let lon_step = (bbox.max_lon - bbox.min_lon) / reso_lon as f64;
//     let lat_step = (bbox.max_lat - bbox.min_lat) / reso_lat as f64;
//     let mut plots = Vec::with_capacity((reso_lat * reso_lon) as usize);
//     // let mut dens_mat = Vec::with_capacity((reso_lat * reso_lon) as usize);
//     for i in 0..reso_lat {
//         for j in 0..reso_lon {
//             plots.push(PtValue{lat: bbox.min_lat + lat_step * i as f64, lon: bbox.min_lon + lon_step * j as f64, value: 0.0})
//         }
//     }
//     let span = 15000.0;
//     do_pot(&mut plots, obs_points, span, b);
//     Ok(plots)
// }
//
// fn do_pot(flat_grid:  &mut Vec<PtValue>, obs_points: &[PtValue], span: f64, beta: f64){
//     let alpha = 0.69314718055994529 / (span).powf(beta);
//     for cell in flat_grid.iter_mut() {
//         cell.value = obs_points.iter()
//             .fold(0.0, |mut sum, obs_pt| {
//                 sum += obs_pt.value * (-alpha * (haversine_distance(obs_pt.lon, obs_pt.lat, cell.lon, cell.lat) * 1000.0).powf(beta)).exp();
//                 sum
//             });
//     }
// }
