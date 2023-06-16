use std::io::{BufRead, Read, stdin, stdout, Write};
use colored::Colorize;
use crate::lexer::lexer::Lexer;

const PROMPT: &str = ">> ";

/// Start the REPL application for the user to enter single line commands
pub fn start_repl() {
    let mut input_buffer = String::new();

    ctrlc::set_handler(move || {
        print!("\r{}\n{PROMPT}", "Please enter 'exit' to leave the application!".red());
        stdout().lock().flush().expect("Error while flushing stdout!");
    }).expect("Failed to set CTRL + C handler!");

    println!("{}", "Welcome to ...! Just type in your commands:".green());
    loop {
        input_buffer.clear();
        print!("{PROMPT}");
        stdout().lock().flush().expect("Error while flushing stdout!");
        stdin().lock().read_line(&mut input_buffer).expect("Error while reading from stdin!");
        if input_buffer.trim() == "exit" { break }
        let mut lexer = Lexer::new(&mut input_buffer);
        println!("{}", format!("{:?}", lexer.generate_tokens()).bright_blue());
    }
}