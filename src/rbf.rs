use errors::*;
use utils::PtValue;
use bbox::Bbox;
use std::f64;
use rulinalg::matrix::Matrix;
use rulinalg::vector::Vector;


#[derive(Debug, Clone)]
pub struct Rbf<'a, T: 'a> {
    obs_points: &'a [T],
    weights: Vector<f64>,
    distance_function: fn(f64, f64) -> f64,
    epsilon: f64,
}

impl<'a, T> Rbf<'a, T>
    where T: PtValue
{
    pub fn new(obs_points: &'a [T], distance_function: &str, epsilon: Option<f64>) -> Self {
        let distance_func = match distance_function {
            "linear" => distance_linear,
            "cubic" => distance_cubic,
            "thin_plate" => distance_thin_plate,
            "quintic" => distance_quintic,
            "gaussian" => distance_gaussian,
            "multiquadratic" => distance_multiquadratic,
            "inverse_multiquadratic" => distance_inverse_multiquadratic,
            &_ => panic!("Invalid function name!"),
        };
        let nb_pts = obs_points.len();
        let mut mat = vec![0.0; nb_pts * nb_pts];
        for j in 0..nb_pts {
            for i in 0..nb_pts {
                mat[j * nb_pts + i] = _norm::<T>(&obs_points[i], &obs_points[j]);
            }
        }
        let eps = if epsilon.is_some() {
            epsilon.unwrap()
        } else {
            sum_all(&mat) / ((nb_pts as f64).powi(2) - nb_pts as f64)
        };
        // for j in 0..nb_pts {
        //     for i in 0..nb_pts {
        //         mat[j * nb_pts + i] = distance_func(mat[j * nb_pts + i], eps);
        //     }
        // }
        for ix in 0..(nb_pts * nb_pts) {
            mat[ix] = distance_func(mat[ix], eps);
        }
        let mut values: Vec<f64> = Vec::with_capacity(nb_pts);
        for i in 0..nb_pts {
            values.push(obs_points[i].get_value());
        }
        let mat = Matrix::new(nb_pts, nb_pts, mat);
        let vec = Vector::new(values);
        // let weights = mat.solve(vec).unwrap().into_iter().collect::<Vec<f64>>();
        let weights = mat.solve(vec).unwrap();
        Rbf {
            obs_points: obs_points,
            distance_function: distance_func,
            epsilon: eps,
            weights: weights,
        }
    }

    pub fn interp_point(&self, pt: (f64, f64)) -> f64 {
        let _pt = T::new(pt.0, pt.1, 0.0);
        let mut distances: Vec<f64> = Vec::with_capacity(self.obs_points.len());
        for point in self.obs_points {
            let a = _norm(&_pt, point);
            distances.push((self.distance_function)(a, self.epsilon));
        }
        let dist = Vector::new(distances);
        let r = &dist.elemul(&self.weights);
        r.sum()
    }
}

pub fn rbf_interpolation<T>(reso_x: u32,
                            reso_y: u32,
                            bbox: &Bbox,
                            obs_points: &[T],
                            func_name: &str,
                            epsilon: Option<f64>)
                            -> Result<Vec<T>>
    where T: PtValue
{
    let x_step = (bbox.max_x - bbox.min_x) / reso_x as f64;
    let y_step = (bbox.max_y - bbox.min_y) / reso_y as f64;
    let mut plots = Vec::with_capacity((reso_x * reso_y) as usize);
    let rbf = Rbf::new(obs_points, func_name, epsilon);
    for i in 0..reso_x {
        for j in 0..reso_y {
            let x = bbox.min_x + x_step * i as f64;
            let y = bbox.min_y + y_step * j as f64;
            let value = rbf.interp_point((x, y));
            plots.push(T::new(x, y, value));
        }
    }
    Ok(plots)
}

fn sum_all(mat: &Vec<f64>) -> f64 {
    let mut s: f64 = 0.0;
    for &v in mat {
        s += v;
    }
    s
}


fn _norm<T>(pa: &T, pb: &T) -> f64
    where T: PtValue
{
    let ca = pa.get_coordinates();
    let cb = pb.get_coordinates();
    ((ca.0 - cb.0).powi(2) + (ca.1 - cb.1).powi(2)).sqrt()
}

#[inline(always)]
#[allow(unused_variables)]
fn distance_linear(r: f64, epsilon: f64) -> f64 {
    r
}

#[inline(always)]
#[allow(unused_variables)]
fn distance_cubic(r: f64, epsilon: f64) -> f64 {
    r.powi(3)
}

#[inline(always)]
#[allow(unused_variables)]
fn distance_quintic(r: f64, epsilon: f64) -> f64 {
    r.powi(5)
}

#[inline(always)]
#[allow(unused_variables)]
fn distance_thin_plate(r: f64, epsilon: f64) -> f64 {
    if r == 0.0 { 0.0 } else { r.powi(2) * r.ln() }
}

#[inline(always)]
fn distance_gaussian(r: f64, epsilon: f64) -> f64 {
    1.0 / ((r / epsilon).powi(2) + 1.0).exp()
}

#[inline(always)]
fn distance_inverse_multiquadratic(r: f64, epsilon: f64) -> f64 {
    1.0 / ((r / epsilon).powi(2) + 1.0).sqrt()
}

#[inline(always)]
fn distance_multiquadratic(r: f64, epsilon: f64) -> f64 {
    ((r / epsilon).powi(2) + 1.0).sqrt()
}
