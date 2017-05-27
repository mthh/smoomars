#![crate_name="smoomars"]
#![deny(trivial_casts,
        missing_debug_implementations,
        unstable_features,
        unsafe_code,
        unused_import_braces)]
// missing_docs,missing_debug_implementations, missing_copy_implementations, trivial_numeric_casts, unused_qualifications

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate csv;
extern crate gdal;
extern crate geojson;
extern crate jobsteal;
extern crate rulinalg;

mod errors;
mod bbox;
mod idw;
mod pot_stewart;
mod rbf;

pub mod utils;

pub use self::bbox::Bbox;
pub use self::idw::idw_interpolation1 as idw_interpolation;
pub use self::pot_stewart::{SmoothType, StewartPotentialGrid, stewart, stewart_interpolation};
pub use self::rbf::{Rbf, rbf_interpolation};
pub use self::utils::{PtValue, SphericalPtValue, CartesianPtValue};
pub use errors::*;

#[cfg(test)]
mod test;
