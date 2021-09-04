pub mod interpreter;
pub mod lexer;
pub mod repl;

fn main() {
    let mut repl = repl::REPL::new();
    repl.run()
}
