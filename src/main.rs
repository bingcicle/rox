pub mod ast;
pub mod error;
pub mod interpreter;
pub mod parser;
pub mod scanner;
pub mod token;

use clap::{arg, command};
use error::RoxError;
use parser::Parser;
use scanner::Scanner;
use std::fs::File;
use std::io::prelude::*;

fn run(contents: String) {
    let mut scanner = Scanner::new(contents);
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    let expr = match parser.parse() {
        Ok(expr) => expr,
        Err(e) => panic!("Problem parsing {:?}", e),
    };
}

fn run_file(file_path: &str) {
    let mut f = File::open(file_path).expect("Path does not exist");
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("Unable to read file");
    run(contents);
}

fn run_prompt() {
    unimplemented!()
}

fn main() {
    let matches = command!().arg(arg!([script])).get_matches();

    if let Some(script) = matches.value_of("script") {
        println!("Value for script: {}", script);
        run_file(script);
    } else {
        println!("Usage: rox [script]");
    }
}
