mod global;
mod tokenizer;

use colored::Colorize;
use std::{
    env,
    process::{Command, ExitCode},
};
use tokenizer::Token;

use global::print_error;

// Lexer

// Parser

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let mut exit_code: u8 = global::EXIT_SUCCES;
    let tokens: Vec<Token>;
    loop {
        if args.len() < 2 {
            exit_code = print_error(
                global::MISSING_ARGUMENT,
                Some(format!(
                    "No input provided:\n{}",
                    "lazuli <input.lzl>".green()
                )),
            );
            break;
            // return ExitCode::from(global::MISSING_ARGUMENT);
        } else {
            let tokenizer_result = tokenizer::tokenize((&args[1]).to_string());
            match tokenizer_result {
                Ok(t) => tokens = t,
                Err(e) => {
                    exit_code = e;
                    break;
                }
            }
            // exit_code = tokenizer::tokenize((&args[1]).to_string()); // this seems convoluted
            // if interupt_exit_code(exit_code) {
            //     break;
            // }
        }

        println!("###TOKENS###");
        for t in tokens.iter() {
            println!(
                "Token type: {}, value: {}",
                t.t_type.to_string(),
                if t.value.is_none() {
                    "## NONE ##"
                } else {
                    t.value.as_ref().unwrap()
                }
            );
        }
        println!("tokens length: {}", tokens.len());

        //BREAK OUT OF LOOP AFTER ALL OPPERATIONS ARE COMPLEATED
        break;
    }

    // println!("Command spawn result {:?}", Command::new("dir").spawn());
    return ExitCode::from(if exit_code == 47 { 0 } else { exit_code });
}

//Exit (true) if code is not 47
fn interupt_exit_code(code: u8) -> bool {
    if code == 47 {
        return false;
    }
    return true;
}
