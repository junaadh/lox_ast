pub mod environment;
pub mod error;
pub mod expr;
pub mod interpreter;
pub mod macros;
pub mod parser;
pub mod scanner;
pub mod stmt;
pub mod token;
pub mod token_type;
pub mod traits;

use std::{
    env,
    fs::File,
    io::{self, Read, Write},
    process,
    sync::{Arc, Mutex},
};

use interpreter::Interpreter;
use lazy_static::lazy_static;
use parser::Parser;
use scanner::Scanner;

use crate::error::LoxError;

lazy_static! {
    pub static ref ERROR: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}

fn main() {
    let mut args = env::args();
    let mut interpreter = Interpreter::default();
    match args.len() {
        0..=1 => {
            run_prompt(&mut interpreter);
        }
        2 => {
            run_file(args.nth(1).unwrap_or_default(), &mut interpreter);
        }
        _ => {
            println!("Usage: jlox [script]");
            process::exit(64);
        }
    }
}

fn run_prompt(runner: &mut Interpreter) {
    let mut buffer = String::new();
    loop {
        buffer.clear();
        print!("> ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut buffer).unwrap_or_default();
        match run(buffer.as_str(), runner) {
            true => {
                LoxError::clear();
            }
            false => {
                LoxError::clear();
                continue;
            }
        };
    }
}

fn run_file(path: String, runner: &mut Interpreter) {
    if path.is_empty() {
        process::exit(2);
    }

    let mut file = File::open(path).map_err(|err| println!("{err}")).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap_or_default();
    match run(buf.as_str(), runner) {
        true => {
            process::exit(0);
        }
        false => {
            process::exit(65);
        }
    };
}

fn run(buffer: &str, runner: &mut Interpreter) -> bool {
    let mut scanner = Scanner::new(buffer);
    scanner.scan_tokens();
    let mut parser = Parser::new(scanner.tokens);
    match parser.parse() {
        Ok(stmt) => {
            // println!("{:#?}", stmt);
            match runner.interpret(stmt) {
                Ok(_) => true,
                Err(err) => {
                    println!("{}", err);
                    false
                }
            }
        }
        Err(err) => {
            println!("{}", err);
            false
        }
    }
}
