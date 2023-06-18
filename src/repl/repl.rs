use std::io::{BufRead, stdin, stdout, Write};
use colored::Colorize;
use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;

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
        let tokens = lexer.generate_tokens();

        let mut tokens_string = String::new();
        for token in &tokens {
            tokens_string.push_str(&token.to_string());
            tokens_string.push(',');
        }
        tokens_string.pop();

        println!("{}", format!("{}", tokens_string).bright_blue());

        let mut parser = Parser::new(tokens);
        let program = parser.parse();

        if let Ok(program) = program {
            println!("{:#?}", program);
            println!("{}", program);
        } else if let Err(err) = program {
            eprintln!("Error: {err}");
        }
    }
}