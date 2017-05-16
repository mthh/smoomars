#![crate_name="smoomars"]

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate csv;
extern crate geojson;
extern crate gdal;
extern crate jobsteal;

mod errors;
mod bbox;
mod idw;
mod pot_stewart;

pub mod utils;

pub use self::bbox::Bbox;
pub use self::utils::{ValuesJson, PtValue, SphericalPtValue, CartesianPtValue};
pub use self::idw::idw_interpolation1 as idw_interpolation;
pub use self::pot_stewart::{SmoothType, StewartPotentialGrid, stewart};
pub use errors::*;

#[cfg(test)]
mod test;
