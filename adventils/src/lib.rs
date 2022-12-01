extern crate argparse;

use argparse::{ArgumentParser, StoreTrue, Store};
use std::fs;

pub struct OptionalArgumentsAndSuchLike {
    pub verbose: bool,
    pub input_file:  String
}

impl OptionalArgumentsAndSuchLike {
    pub fn args() -> Self {
        let mut opts = OptionalArgumentsAndSuchLike { verbose: false, input_file: String::from("") };
        {
            let mut ap = ArgumentParser::new();
            ap.refer(&mut opts.verbose)
                .add_option(&["-v", "--verbose"], StoreTrue, "Enable verbose logging");
            ap.refer(&mut opts.input_file)
                .add_argument("input_file", Store, "File with puzzle input contents.");
            ap.parse_args_or_exit();
        }
        return opts
    }
}

pub fn get_input(path: String) -> String {
    let contents = fs::read_to_string(path.clone())
        .expect(&format!("Could not read input file '{path}'. Did you get the path right?"));
    return contents
}
