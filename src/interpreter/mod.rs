#![allow(unused_imports)]
use colored::*;
use std::fs::File;

pub struct Interpreter {
    is_interactive: bool,
    had_error: bool,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            is_interactive: false,
            had_error: false,
        }
    }

    pub fn run(&mut self) {
        if self.had_error && !self.is_interactive {
            panic!("Had Error");
        }
    }

    pub fn run_file(&mut self, _: &str) {
        // let mut file = match File::open(path) {
        //     Err(why) => panic!("Couldn't open {}: {}", path, why),
        //     Ok(file) => file,
        // };
    }

    pub fn error(line: i16, message: &str) {
        Interpreter::report(line, " ", message);
    }

    fn report(line: i16, loc: &str, message: &str) {
        println!(
            "[line {}] {} {}: {}",
            line,
            "Error".red().bold(),
            loc,
            message
        );
    }
}
