use colored::Colorize;
use std::{env, process::ExitCode};

mod lzl_error;
mod parser;
mod tokenizer;
mod tokens;
use crate::lzl_error::*;
use crate::parser::*;
use crate::tokenizer::*;
use crate::tokens::*;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect(); // arg[0] is lazuli
    let exit_code: u8;
    let tokens: Vec<Token>;
    loop {
        if args.len() < 2 {
            exit_code = print_error(
                MISSING_ARGUMENT,
                Some(format!(
                    "No input provided:\n{}",
                    "lazuli <input.lzl>".green()
                )),
            );
            break;
        } else {
            let tokenizer_result = tokenize((&args[1]).to_string());
            match tokenizer_result {
                Ok(t) => tokens = t,
                Err(e) => {
                    exit_code = e;
                    break;
                }
            }
        }

        println!("###TOKENS###");
        for t in tokens.iter() {
            println!("{:?}", t);
        }
        println!("tokens length: {}", tokens.len());

        let mut new_path = String::from(&args[1]);
        new_path.truncate(new_path.len() - 4); //remove last four chars ".lzl"
        let parse_result = parse(tokens, new_path);
        match parse_result {
            Ok(t) => exit_code = t,
            Err(e) => {
                exit_code = e;
                break;
            }
        }

        //BREAK OUT OF LOOP AFTER ALL OPPERATIONS ARE COMPLEATED
        break;
    }
    return ExitCode::from(if exit_code == EXIT_SUCCES {
        0
    } else {
        exit_code
    });
}
