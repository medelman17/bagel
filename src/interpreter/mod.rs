#![allow(unused_imports)]
use std::fs::File;

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    pub fn run(&mut self) {}

    pub fn run_file(&mut self, _: &str) {
        // let mut file = match File::open(path) {
        //     Err(why) => panic!("Couldn't open {}: {}", path, why),
        //     Ok(file) => file,
        // };
    }
}
