use writing_an_interpreter_in_go::repl::repl::start_repl;

mod lexer;
mod parser;
mod ast;

fn main() {
    start_repl();
}
