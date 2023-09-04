use core::fmt;
use std::fs;

use colored::Colorize;

use crate::global::{self, print_error};

pub fn tokenize(path_to_file: String) -> Result<Vec<Token>, u8> {
    let mut tokens: Vec<Token> = vec![];

    if !path_to_file.ends_with(".lzl") {
        // println!(
        //     "{}: Invalid argument: file doesn't end with .lzl",
        //     global::INVALID_AGUMENT
        // );
        // eprintln!(
        //     "E{}:\tInvalid argument: file doesn't end with .lzl\n\tFile:\t{}",
        //     global::INVALID_AGUMENT,
        //     path_to_file
        // );
        return Err(print_error(
            global::INVALID_ARGUMENT,
            Some(format!(
                "Input file doesn't end with .lzl\nFile:\t{}",
                path_to_file,
            )),
        ));
    }

    let contents = fs::read_to_string(path_to_file);
    match contents {
        Ok(cont) => {
            let mut syntax: String = String::from("");
            // for i in 0..c.len() {
            //     if (c.chars().nth(i))
            // }
            let mut chars = cont.chars();
            let mut c = chars.next();
            let mut ch: char;

            while c.is_some() {
                ch = c.unwrap();
                println!("CHAR: {}", ch);
                // Skip whitespace (that isn't part of a string)
                if ch.is_whitespace() {
                    c = chars.next();
                }
                // Start of name
                else if ch.is_alphabetic() {
                    syntax.push(ch);
                    c = chars.next();
                    if c != None {
                        ch = c.unwrap();
                    }
                    while ch.is_alphanumeric() {
                        syntax.push(ch);
                        c = chars.next();
                        if c != None {
                            ch = c.unwrap();
                        } else {
                            break;
                        }
                    }

                    // All possible syntatic words
                    // TODO: Maybe use match or switch case
                    if syntax == "ret" {
                        tokens.push(Token {
                            t_type: TokenType::RETURN,
                            value: None,
                        })
                    } else {
                        return Err(print_error(
                            global::WEIRD_ERROR,
                            Some(format!(
                                "undefined syntax (which should have been read as an identifier): {}",
                                syntax
                            )),
                        ));
                    }
                }
                // Digits (only ints now)
                else if ch.is_digit(10) {
                    syntax.push(ch);
                    c = chars.next();
                    if c != None {
                        ch = c.unwrap();
                    }
                    while ch.is_digit(10) {
                        syntax.push(ch);
                        c = chars.next();
                        if c != None {
                            ch = c.unwrap();
                        } else {
                            break;
                        }
                    }

                    tokens.push(Token {
                        t_type: TokenType::INT,
                        value: Some(syntax.clone()),
                    })
                }
                //Symbols
                else {
                    if ch == ';' {
                        tokens.push(Token {
                            t_type: TokenType::SEMI,
                            value: None,
                        });
                    }
                    c = chars.next();
                }

                syntax.clear();
            }
        }
        Err(e) => {
            eprintln!("{}\n## {} ##", "Couldn't read file".red(), e);
            return Err(print_error(global::INVALID_ARGUMENT, None));
        }
    }

    Ok(tokens)
}

// struct SyntaxType
// {
//     syntax: String,
//     t_type: TokenType
// }

pub struct Token {
    pub t_type: TokenType,
    pub value: Option<String>,
}

pub enum TokenType {
    RETURN,
    INT,
    SEMI,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::RETURN => write!(f, "RETURN"),
            TokenType::INT => write!(f, "INT"),
            TokenType::SEMI => write!(f, "SEMI"),
        }
    }
}

// const RETURN: SyntaxType = SyntaxType {syntax: String::from("ret"), t_type: TokenType::RETURN};
// const INT: SyntaxType = SyntaxType {syntax: String::from("ret"), t_type: TokenType::RETURN};
// const LINEEND: SyntaxType = SyntaxType {syntax: String::from(";"), t_type: TokenType::SEMI};
