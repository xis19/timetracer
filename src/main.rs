extern crate argparse;
extern crate env_logger;
extern crate timetracer;

use argparse::{ArgumentParser, Store};
use log::{debug, warn};
use timetracer::file_parser::json_parser;

use std::env::current_dir;
use std::path::PathBuf;

use timetracer::directory_walker::iterate_json_files;

fn main() {
    env_logger::init();

    let mut work_directory_string: String = current_dir().unwrap().to_str().unwrap().to_string();
    {
        let mut argparser = ArgumentParser::new();
        argparser.set_description("Analyze the Clang build time");
        argparser.refer(&mut work_directory_string).add_option(
            &["--work-directory"],
            Store,
            "Set the work directory, or the directory the project is built",
        );
        argparser.parse_args_or_exit();
    }
    debug!("Work directory: {}", work_directory_string);

    let database = PathBuf::from(&work_directory_string).join("tracedb.sqlite");
    let mut connection =
        timetracer::tracedb::get_connection(database.to_str().unwrap().to_string()).unwrap();

    for path_result in iterate_json_files(&PathBuf::from(&work_directory_string)).unwrap() {
        match path_result {
            Err(e) => {
                debug!("Error on {}", e);
            }
            Ok(path) => {
                debug!("JSON file {}", path.display());
                let result = json_parser(&path, &mut connection);
                match result {
                    Ok(()) => {}
                    Err(e) => {
                        warn!("Parse {} error {}", path.display(), e);
                    }
                }
            }
        }
    }
}
