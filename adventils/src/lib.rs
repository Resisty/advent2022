extern crate argparse;

use argparse::{ArgumentParser, StoreTrue, Store};
use std::fs;

pub struct Options {
    pub verbose: bool,
    pub input_file:  String
}

impl Options {
    pub fn args() -> Self {
        let mut opts = Options { verbose: false, input_file: String::from("") };
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
    let contents = fs::read_to_string(path)
        .expect("Could not read input file. Did you get the path right?");
    return contents
}
