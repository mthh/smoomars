extern crate gnuplot;
extern crate smoomars;

use smoomars::utils::*;
use smoomars::{StewartPotentialGrid, stewart, SmoothType, Bbox};
use common::*;
use gnuplot::*;

mod common;

fn example(c: Common) {
    let obs_points = parse_geojson_points("/home/mz/Bureau/input_ra.geojson", "value").unwrap();
    let bbox = Bbox::new(0.8, 4.2, 31.8, 35.2);
    let (reso_lon, reso_lat) = (100, 100);
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
        .set_z_ticks(Some((Fix(1.0), 1)), &[Mirror(false)], &[])
    	.set_z_range(Auto, Auto)
    	.set_palette(HELIX)
    	.set_view(45.0, 175.0);
	c.show(&mut fg, "surface.gnuplot");
}

fn main() {
	Common::new().map(|c| example(c));
}
