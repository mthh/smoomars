#[cfg(test)]
mod test {
    use ::*;
    use std::f64;
    type Pt = utils::CartesianPtValue;
    #[test]
    fn test_idw() {
        let obs_points = utils::parse_json_points::<Pt>("examples/ra.json").unwrap();
        let bbox = bbox::Bbox::new(1.0, 4.0, 32.0, 35.0);
        idw_interpolation(80, 60, &bbox, &obs_points, 2.0).unwrap();
    }

    #[test]
    fn test_stewart_equal_stewart_parallel() {
        let obs_points = utils::parse_json_points::<utils::SphericalPtValue>("examples/ra.json")
            .unwrap();
        let bbox = bbox::Bbox::new(1.0, 4.0, 32.0, 35.0);
        let reso_lat: u32 = 80;
        let reso_lon: u32 = 80;
        let conf1 = StewartPotentialGrid::new(15000.0,
                                              2.0,
                                              SmoothType::Exponential,
                                              &bbox,
                                              reso_lat,
                                              reso_lon,
                                              2);
        let res_par = stewart(&conf1, &obs_points).unwrap();
        let conf2 = StewartPotentialGrid::new(15000.0,
                                              2.0,
                                              SmoothType::Exponential,
                                              &bbox,
                                              reso_lat,
                                              reso_lon,
                                              1);
        let res = stewart(&conf2, &obs_points).unwrap();
        assert_eq!(res_par.len(), res.len());
        for i in 0..res.len() {
            let (res_lat, res_lon, res_value): (f64, f64, f64) = res_par[i].get_triplet();
            let (verif_lat, verif_lon, verif_value): (f64, f64, f64) = res[i].get_triplet();
            assert_eq!(true, utils::almost_equal(res_lat, verif_lat, 0.00001));
            assert_eq!(true, utils::almost_equal(res_lon, verif_lon, 0.00001));
            assert_eq!(true, utils::almost_equal(res_value, verif_value, 0.00001));
        }
    }

    #[test]
    fn test_radial_basis_func_linear() {
        let obs_pts = vec![Pt::new(0.0, 0.0, 0.0),
                           Pt::new(0.0, 100.0, 6.0),
                           Pt::new(75.0, 25.0, 3.1),
                           Pt::new(100.0, 75.0, 7.4)];
        let rbf = Rbf::new(&obs_pts, "linear", None);
        assert_eq!(true,
                   utils::almost_equal(2.843937337, rbf.interp_point((0.0, 50.0)), 0.0000001));
        assert_eq!(true,
                   utils::almost_equal(0.754167644, rbf.interp_point((12.0, 12.0)), 0.0000001));
    }

    #[test]
    fn test_radial_basis_func_cubic() {
        let obs_pts = vec![Pt::new(0.0, 0.0, 0.0),
                           Pt::new(0.0, 100.0, 6.0),
                           Pt::new(75.0, 25.0, 3.1),
                           Pt::new(100.0, 75.0, 7.4)];
        let rbf = Rbf::new(&obs_pts, "cubic", None);
        assert_eq!(true,
                   utils::almost_equal(0.554789362, rbf.interp_point((0.0, 50.0)), 0.0000001));
        assert_eq!(true,
                   utils::almost_equal(-0.181785359, rbf.interp_point((12.0, 12.0)), 0.0000001));
    }

    #[test]
    fn test_radial_basis_func_gaussian() {
        let obs_pts = vec![Pt::new(0.0, 0.0, 0.0),
                           Pt::new(0.0, 100.0, 6.0),
                           Pt::new(75.0, 25.0, 3.1),
                           Pt::new(100.0, 75.0, 7.4)];
        let rbf = Rbf::new(&obs_pts, "gaussian", None);
        assert_eq!(true,
                   utils::almost_equal(3.494929342, rbf.interp_point((0.0, 50.0)), 0.0000001));
        assert_eq!(true,
                   utils::almost_equal(0.777143813, rbf.interp_point((12.0, 12.0)), 0.0000001));
    }
}
