/// Walk through the directory
extern crate glob;
extern crate log;

use glob::{glob, Paths};
use log::debug;
use std::{error::Error, path::Path};

pub fn iterate_json_files(base_directory: &Path) -> Result<Paths, Box<dyn Error>> {
    let path = base_directory.join("**").join("*.json");
    let path_str = path.to_str().expect("Failed to convert path to string");

    debug!("Search pattern {}", path_str);

    match glob(path_str) {
        Ok(result) => Ok(result),
        Err(e) => Err(Box::new(e)),
    }
}
