static R: f64 = 6372.8;


/// Return haversine distance in meters
/// with R = 6.372.800 meters
/// Expected input (in radians) as lat1, lon1, lat2, lon2
pub fn haversine_distance(th1: f64, ph1: f64, th2: f64, ph2: f64) -> f64 {
    let mut _ph1 = ph1 - ph2;
    _ph1 = _ph1.to_radians();
    let _th1 = th1.to_radians();
    let _th2 = th2.to_radians();
    let dz: f64 = _th1.sin() - _th2.sin();
    let dx: f64 = _ph1.cos() * _th1.cos() - _th2.cos();
    let dy: f64 = _ph1.sin() * _th1.cos();
    ((dx * dx + dy * dy + dz * dz).sqrt() / 2.0).asin() * 2.0 * R
}
// pub fn haversine_distance(th1: f64, ph1: f64, th2: f64, ph2: f64) -> f64 {
//     let _ph1 = ph1 - ph2;
//     let dz: f64 = th1.sin() - th2.sin();
//     let dx: f64 = _ph1.cos() * th1.cos() - th2.cos();
//     let dy: f64 = _ph1.sin() * th1.cos();
//     ((dx * dx + dy * dy + dz * dz).sqrt() / 2.0).asin() * 2.0 * R
// }

pub fn euclidian_distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let (dx, dy) = (x1 - x2, y1 - y2);
    ((dx * dx) + (dy * dy)).sqrt()
}
