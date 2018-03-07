extern crate clap;
extern crate num_cpus;
extern crate smoomars;
#[macro_use]
extern crate scan_rules;


use smoomars::*;
use clap::{Arg, App};


fn main() {
    let matches = App::new("smoomars").version("0.1.0")
       .about("Compute inverse distance interpolation or population potentials.")
       .arg(Arg::with_name("method")
            .index(1)
            .value_name("METHOD")
            .required(true)
            .possible_values(&["idw", "stewart", "par_stewart"])
            .help("The method to use."))
       .arg(Arg::with_name("input")
            .short("i").long("input")
            .required(true).takes_value(true)
            .value_name("FILE")
            .help("Input file to use (.csv, .json or .geojson). If .geojson, default to spherical distance."))
        .arg(Arg::with_name("power")
             .short("p").long("power")
             .default_value("2")
             .takes_value(true)
             .value_name("POWER")
             .help("Power value for the interpolation function."))
        .arg(Arg::with_name("distance")
             .short("d").long("distance")
             .takes_value(true)
             .default_value("Spherical")
             .value_name("TYPEDISTANCE")
             .help("Cartesian/Spherical regarding to use euclidian distance or spherical distance"))
        .arg(Arg::with_name("scale")
             .short("s").long("scale")
             .required(true).takes_value(true)
             .value_name("SCALE")
             .help("Resolution of the output in number of cells as resoX-resoY."))
        .arg(Arg::with_name("window")
             .short("w").long("window")
             .takes_value(true).require_equals(true)
             .value_name("WINDOW")
             .help("Coordinates of the visualisation window, given as minimum latitude,minimum longitude,maximum latitude,maximum longitude."))
        .arg(Arg::with_name("output")
             .short("o").long("output")
             .required(true).takes_value(true)
             .value_name("FILE")
             .help("Path for output file (according to the outfile extension, .json, .csv, .geojson and .geotiff are accepted)."))
         .arg(Arg::with_name("span")
            .long("span")
            .takes_value(true)
            .value_name("SPAN")
            .help("Span in kilometers"))
        .arg(Arg::with_name("function")
            .long("function")
            .takes_value(true)
            .default_value("pareto")
            .possible_values(&["exponential", "pareto"])
            .help("The name of the smoothing function to use for stewart method."))
         .arg(Arg::with_name("field")
             .short("c").long("field_name")
             .takes_value(true)
             .value_name("FIELD")
             .help("(Required for GeoJSON input) Field name containing the stock values to use."))
        .get_matches();

    let b: f64 = matches.value_of("power").unwrap().parse::<f64>().unwrap();
    let_scan!(matches.value_of("scale").unwrap(); (
        let reso_lat: u32, "-", let reso_lon: u32));
    let method = matches.value_of("method").unwrap();
    let file_path = matches.value_of("input").unwrap();
    let mut dist = matches.value_of("distance").unwrap();
    if file_path.contains("geojson") || file_path.contains("GEOJSON") {
        dist = "Spherical";
    }
    let span = if matches.is_present("span") {
        matches.value_of("span").unwrap().parse::<f64>().unwrap()
    } else {
        0.0
    };
    match dist {
        "Spherical" => {
            let obs_points_spherical;
            if file_path.contains("geojson") || file_path.contains("GEOJSON") {
                let field_name = if matches.is_present("field") {
                    matches.value_of("field")
                } else {
                    None
                };
                if field_name.is_none() {
                    panic!("Error: Field name is required for GeoJSON input (arg. --field=name).");
                }
                obs_points_spherical =
                    utils::parse_geojson_points::<utils::SphericalPtValue>(file_path,
                                                                           field_name.unwrap())
                            .unwrap();
            } else if file_path.contains("json") || file_path.contains("JSON") {
                obs_points_spherical =
                    utils::parse_json_points::<utils::SphericalPtValue>(file_path).unwrap();
            } else {
                obs_points_spherical =
                    utils::parse_csv_points::<utils::SphericalPtValue>(file_path).unwrap();
            }
            let bbox = if matches.is_present("window") {
                let_scan!(matches.value_of("window").unwrap(); (
                    let min_lat: f64, ",", let max_lat: f64, ",", let min_lon: f64, ",", let max_lon: f64));
                Bbox::new(min_lat, max_lat, min_lon, max_lon)
            } else {
                Bbox::from_points(&obs_points_spherical)
            };
            let result = match method {
                "idw" => {
                    println!("IDW");
                    idw_interpolation(reso_lat as u32,
                                      reso_lon as u32,
                                      &bbox,
                                      &obs_points_spherical,
                                      b)
                            .unwrap()
                }
                "stewart" => {
                    if span == 0.0 {
                        panic!("Invalid or missing span value !")
                    }
                    println!("stewart");
                    let conf = StewartPotentialGrid::new(span,
                                                         b as f64,
                                                         SmoothType::Exponential,
                                                         &bbox,
                                                         reso_lat as u32,
                                                         reso_lon as u32,
                                                         1);
                    stewart(&conf, &obs_points_spherical).unwrap()
                }
                "par_stewart" => {
                    if span == 0.0 {
                        panic!("Invalid or missing span value !")
                    }
                    let nb_core = num_cpus::get() as u32;
                    let conf = StewartPotentialGrid::new(span,
                                                         b as f64,
                                                         SmoothType::Exponential,
                                                         &bbox,
                                                         reso_lat as u32,
                                                         reso_lon as u32,
                                                         nb_core);
                    println!("stewart (using {:?} core)", nb_core);
                    stewart(&conf, &obs_points_spherical).unwrap()
                }
                &_ => unreachable!(),
            };
            let output_path = matches.value_of("output").unwrap();
            if output_path.contains("geojson") || output_path.contains("GEOJSON") {
                utils::save_geojson_points(output_path, result).unwrap();
            } else if output_path.contains("geotiff") {
                utils::write_to_raster(result,
                                       &bbox,
                                       (reso_lat as u32, reso_lon as u32),
                                       output_path)
                        .unwrap();
            } else {
                utils::save_json_points(output_path, result).unwrap();
            }
        }
        "Euclidian" => {
            let obs_points = if file_path.contains("json") || file_path.contains("JSON") {
                utils::parse_json_points::<utils::CartesianPtValue>(file_path).unwrap()
            } else {
                utils::parse_csv_points::<utils::CartesianPtValue>(file_path).unwrap()
            };
            let bbox = if matches.is_present("window") {
                let_scan!(matches.value_of("window").unwrap(); (
                    let min_lat: f64, ",", let max_lat: f64, ",", let min_lon: f64, ",", let max_lon: f64));
                Bbox::new(min_lat, max_lat, min_lon, max_lon)
            } else {
                Bbox::from_points(&obs_points)
            };
            let result = match method {
                "idw" => {
                    println!("IDW");
                    idw_interpolation(reso_lat as u32, reso_lon as u32, &bbox, &obs_points, b)
                        .unwrap()
                }
                "stewart" => {
                    if span == 0.0 {
                        panic!("Invalid or missing span value !")
                    }
                    println!("stewart");
                    let conf = StewartPotentialGrid::new(span,
                                                         b,
                                                         SmoothType::Exponential,
                                                         &bbox,
                                                         reso_lat as u32,
                                                         reso_lon as u32,
                                                         1);
                    stewart(&conf, &obs_points).unwrap()
                }
                "par_stewart" => {
                    if span == 0.0 {
                        panic!("Invalid or missing span value !")
                    }
                    let nb_core = num_cpus::get() as u32;
                    let conf = StewartPotentialGrid::new(span,
                                                         b,
                                                         SmoothType::Exponential,
                                                         &bbox,
                                                         reso_lat as u32,
                                                         reso_lon as u32,
                                                         nb_core);
                    println!("stewart (using {:?} core)", nb_core);
                    stewart(&conf, &obs_points).unwrap()
                }
                &_ => unreachable!(),
            };
            let output_path = matches.value_of("output").unwrap();
            if output_path.contains("geotiff") {
                utils::write_to_raster(result,
                                       &bbox,
                                       (reso_lat as u32, reso_lon as u32),
                                       output_path)
                        .unwrap();
            } else {
                utils::save_json_points(output_path, result).unwrap();
            }
        }
        &_ => panic!("Invalid distance type"),
    };
}
