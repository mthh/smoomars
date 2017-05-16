#[cfg(test)]
mod test {
    use ::*;
    use std::f64;

    #[test]
    fn test_idw() {
        let obs_points = utils::parse_json_points::<utils::CartesianPtValue>("tests/ra.json").unwrap();
        let bbox = bbox::Bbox::new(1.0, 4.0, 32.0, 35.0);
        idw_interpolation(80, 60, &bbox, &obs_points, 2).unwrap();
    }

    #[test]
    fn test_stewart_equal_stewart_parallel() {
        let obs_points = utils::parse_json_points::<utils::SphericalPtValue>("tests/ra.json").unwrap();
        let bbox = bbox::Bbox::new(1.0, 4.0, 32.0, 35.0);
        let reso_lat: u32 = 80;
        let reso_lon: u32 = 80;
        let conf1 = StewartPotentialGrid::new(15000.0, 2.0, SmoothType::Exponential, &bbox, reso_lat, reso_lon, true);
        let res_par = stewart(&conf1, &obs_points).unwrap();
        let conf2 = StewartPotentialGrid::new(15000.0, 2.0, SmoothType::Exponential, &bbox, reso_lat, reso_lon, false);
        let res = stewart(&conf2, &obs_points).unwrap();
        assert_eq!(res_par.len(), res.len());
        for i in 0..res.len() {
            let (res_lat, res_lon, res_value): (f64, f64, f64) = res_par[i].get_triplet();
            let (verif_lat, verif_lon, verif_value): (f64, f64, f64)  = res[i].get_triplet();
            assert_eq!(true, utils::almost_equal(res_lat, verif_lat, 0.00001));
            assert_eq!(true, utils::almost_equal(res_lon, verif_lon, 0.00001));
            assert_eq!(true, utils::almost_equal(res_value, verif_value, 0.00001));
        }
    }
}
