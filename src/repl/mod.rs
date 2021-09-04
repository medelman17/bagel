#![allow(unused_imports)]
use std;
use std::io;
use std::io::Write;
use std::num::ParseIntError;

pub struct REPL {}

impl REPL {
    pub fn new() -> REPL {
        REPL {}
    }

    pub fn run(&mut self) {
        print!("{}[2J", 27 as char);
        println!("🥯 Welcome to Bagel! (v0.0.1)");
        println!("🥯 Bagel is a rust-based Lox interpreter.");
        println!("🥯 For more information re: Lox see Robert Nystrom's");
        println!("🥯 (@munificentbob) excellent book, Crafting Interpreters.");
        println!("🥯 ********************************************************");

        loop {
            let mut buffer = String::new();
            let stdin = io::stdin();

            print!(">>> ");

            io::stdout().flush().expect("Unable to flush stdout.");

            stdin.read_line(&mut buffer).expect("Unable to read line.");

            let buffer = buffer.trim();

            match buffer {
                ".quit" => {
                    println!("🥯 Bye!");
                    break;
                }
                _ => {}
            }
        }
    }
}

mod tests {
    #[test]
    fn test_nothing_for_ci() {
        assert_eq!(1, 1);
    }
}
