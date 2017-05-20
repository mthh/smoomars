extern crate gnuplot;
extern crate smoomars;

use smoomars::utils::*;
use smoomars::{StewartPotentialGrid, stewart, SmoothType, idw_interpolation, Bbox, rbf_interpolation};
use gnuplot::*;


fn example(c: Common) {
    let obs_points = parse_geojson_points("/home/mz/Bureau/input_ra.geojson", "value").unwrap();
    let bbox = Bbox::new(0.8, 4.2, 31.8, 35.2);
    let (reso_lon, reso_lat) = (80, 80);
    let conf1 = StewartPotentialGrid::new(30000.0, 3.0, SmoothType::Exponential, &bbox, reso_lat, reso_lon, true);
    let res_stew: Vec<SphericalPtValue> = stewart(&conf1, &obs_points).unwrap();
	let mut z1 = Vec::with_capacity(res_stew.len());
	for pt in res_stew {
        z1.push(pt.get_value());
	}
	let mut fg = Figure::new();
	c.set_term(&mut fg);
	fg.axes3d()
    	.set_title("Population potentials", &[])
    	.surface(z1.iter(), reso_lon as usize, reso_lat as usize, Some((0.8, 31.8, 4.2, 35.2)), &[])
    	.set_x_label("X", &[])
    	.set_y_label("Y", &[])
    	.set_z_label("Z", &[])
    	.set_z_range(Auto, Auto)
    	.set_palette(HELIX)
    	.set_view(45.0, 175.0);
	c.show(&mut fg, None);

    let conf2 = StewartPotentialGrid::new(30000.0, 3.0, SmoothType::Pareto, &bbox, reso_lat, reso_lon, true);
    let res_stew_pareto: Vec<SphericalPtValue> = stewart(&conf2, &obs_points).unwrap();
	let mut z1 = Vec::with_capacity(res_stew_pareto.len());
	for pt in res_stew_pareto {
        z1.push(pt.get_value());
	}
	let mut fg = Figure::new();
	c.set_term(&mut fg);
	fg.axes3d()
    	.set_title("Population potentials", &[])
    	.surface(z1.iter(), reso_lon as usize, reso_lat as usize, Some((0.8, 31.8, 4.2, 35.2)), &[])
    	.set_x_label("X", &[])
    	.set_y_label("Y", &[])
    	.set_z_label("Z", &[])
    	.set_z_range(Auto, Auto)
    	.set_palette(HELIX)
    	.set_view(45.0, 175.0);
	c.show(&mut fg, None);

    let result_idw: Vec<SphericalPtValue> = idw_interpolation(reso_lon, reso_lat, &bbox, &obs_points, 2).unwrap();
    let mut z1 = Vec::with_capacity(result_idw.len());
	for pt in result_idw {
        z1.push(pt.get_value());
	}
    let mut fg = Figure::new();
    c.set_term(&mut fg);
    fg.axes3d()
        .set_title("Idw (power 2)", &[])
        .surface(z1.iter(), reso_lon as usize, reso_lat as usize, Some((0.8, 31.8, 4.2, 35.2)), &[])
    	.set_x_label("X", &[])
    	.set_y_label("Y", &[])
    	.set_z_label("Z", &[])
    	.set_z_range(Auto, Auto)
    	.set_palette(HELIX)
    	.set_view(45.0, 175.0);
	c.show(&mut fg, None);

    let obs_points_two_stocks = parse_csv_points::<CartesianPtValue>("examples/two_stocks.csv").unwrap();
    let bbox = Bbox::new(0.0, 10.0, 0.0, 10.0);
    let (reso_lon, reso_lat) = (100, 100);
    let conf1 = StewartPotentialGrid::new(2.5, 2.0, SmoothType::Exponential, &bbox, reso_lat, reso_lon, false);
    let res_stew: Vec<CartesianPtValue> = stewart(&conf1, &obs_points_two_stocks).unwrap();
	let mut z1 = Vec::with_capacity(res_stew.len());
	for pt in res_stew {
        z1.push(pt.get_value());
	}

    let mut fg = Figure::new();
    c.set_term(&mut fg);
    fg.axes3d()
        .set_title("Two stocks. Stewart Exponential (beta=2)", &[])
        .surface(z1.iter(), reso_lon as usize, reso_lat as usize, Some((0.0, 0.0, 10.0, 10.0)), &[])
    	.set_x_label("X", &[])
    	.set_y_label("Y", &[])
    	.set_z_label("Z", &[])
    	.set_z_range(Auto, Auto)
    	.set_palette(HELIX)
    	.set_view(45.0, 175.0);
	c.show(&mut fg, None);


    let bbox = Bbox::new(0.0, 10.0, 0.0, 10.0);
    let (reso_lon, reso_lat) = (100, 100);
    let conf1 = StewartPotentialGrid::new(2.5, 2.0, SmoothType::Pareto, &bbox, reso_lat, reso_lon, false);
    let res_stew: Vec<CartesianPtValue> = stewart(&conf1, &obs_points_two_stocks).unwrap();
	let mut z1 = Vec::with_capacity(res_stew.len());
	for pt in res_stew {
        z1.push(pt.get_value());
	}

    let mut fg = Figure::new();
    c.set_term(&mut fg);
    fg.axes3d()
        .set_title("Two stocks. Stewart Pareto (beta=2)", &[])
        .surface(z1.iter(), reso_lon as usize, reso_lat as usize, Some((0.0, 0.0, 10.0, 10.0)), &[])
    	.set_x_label("X", &[])
    	.set_y_label("Y", &[])
    	.set_z_label("Z", &[])
    	.set_z_range(Auto, Auto)
    	.set_palette(HELIX)
    	.set_view(45.0, 175.0);
	c.show(&mut fg, None);


    let bbox = Bbox::new(0.0, 10.0, 0.0, 10.0);
    let (reso_lon, reso_lat) = (40, 40);
    let res_rbf: Vec<CartesianPtValue> = rbf_interpolation(reso_lon, reso_lat, &bbox, &obs_points_two_stocks, "inverse_multiquadratic", Some(1.66)).unwrap();
	let mut z1 = Vec::with_capacity(res_rbf.len());
	for pt in res_rbf {
        z1.push(pt.get_value());
	}

    let mut fg = Figure::new();
    c.set_term(&mut fg);
    fg.axes3d()
        .set_title("Inverse multiquadratic RBF interpolation (epsilon: 1.66).", &[])
        .surface(z1.iter(), reso_lon as usize, reso_lat as usize, Some((0.0, 0.0, 10.0, 10.0)), &[])
    	.set_x_label("X", &[])
    	.set_y_label("Y", &[])
    	.set_z_label("Z", &[])
    	.set_z_range(Auto, Auto)
    	.set_palette(HELIX)
    	.set_view(45.0, 175.0);
	c.show(&mut fg, None);

}

pub struct Common
{
	pub no_show: bool,
	pub term: Option<String>,
}

impl Common
{
	pub fn new() -> Option<Common> {
		Some(Common { no_show: false, term: None })
	}

	pub fn show(&self, fg: &mut Figure, filename: Option<&str>) {
		if !self.no_show {
			fg.show();
		}
		if filename.is_some() {
			fg.echo_to_file(filename.unwrap());
		}
	}

	pub fn set_term(&self, fg: &mut Figure) {
		self.term.as_ref().map(|t| { fg.set_terminal(&t[..], ""); });
	}
}


fn main() {
	Common::new().map(|c| example(c));
}
