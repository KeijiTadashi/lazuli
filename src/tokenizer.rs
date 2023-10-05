use std::fs;
use std::rc::Rc;

use colored::Colorize;
use itertools::Itertools;

use crate::lzl_error::*;
use crate::tokens::*;

pub struct LzlFile {
    chars: Vec<char>,
    size: usize,
}

impl LzlFile {
    pub fn new(chars: Vec<char>) -> LzlFile {
        LzlFile {
            size: chars.len(),
            chars: chars,
        }
    }

    pub fn peek(&self) -> Option<char> {
        if self.size <= 0 {
            return None;
        }
        return Some(self.chars[self.size - 1]);
    }

    // pub fn peek_ahead(&self, ahead: usize) -> Option<char> {
    //     if self.size <= ahead {
    //         return None;
    //     }
    //     return Some(self.chars[self.size - ahead - 1]);
    // }

    pub fn next(self: &'_ mut Self) -> Option<char> {
        if let Some(r) = self.chars.pop() {
            self.size -= 1;
            return Some(r);
        }
        return None;
    }
}

pub fn tokenize(path_to_file: Rc<str>) -> Result<Vec<Token>, u8> {
    let mut tokens: Vec<Token> = vec![];

    if !path_to_file.ends_with(".lzl") {
        return Err(print_error(
            Some(INVALID_ARGUMENT),
            Some(format!(
                "Input file doesn't end with .lzl\nFile:\t{}",
                path_to_file,
            )),
        ));
    }

    let contents = fs::read_to_string(path_to_file.to_string());
    let mut cont: LzlFile;
    match contents {
        Ok(c) => {
            cont = LzlFile::new(c.chars().rev().collect_vec());
        }
        Err(e) => {
            return Err(print_error(
                Some(INVALID_ARGUMENT),
                Some(format!("{}\n## {} ##", "Couldn't read file".red(), e)),
            ));
        }
    }

    let mut syntax: String = String::from("");

    while let Some(c) = cont.next() {
        if c.is_whitespace() {
        } else if c.is_alphabetic() {
            syntax.push(c);
            while let Some(ch) = cont.peek() {
                if ch.is_alphanumeric() {
                    syntax.push(ch);
                    cont.next();
                } else {
                    break;
                }
            }
            // All possible syntatic words
            // TODO: Maybe use match or switch case
            if syntax == RETURN.syntax {
                tokens.push(Token {
                    t_type: TokenType::T_RETURN,
                    value: None,
                })
            } else if syntax == INT.syntax {
                tokens.push(Token {
                    t_type: TokenType::T_INT,
                    value: None,
                })
            } else if syntax == IF.syntax {
                tokens.push(Token {
                    t_type: IF.t_type,
                    value: None,
                })
            } else if syntax == WHILE.syntax {
                tokens.push(Token {
                    t_type: WHILE.t_type,
                    value: None,
                })
            } else {
                tokens.push(Token {
                    t_type: TokenType::T_IDENT,
                    value: Some(syntax.clone()),
                })
            }
        } else if c.is_digit(10) {
            syntax.push(c);
            while let Some(ch) = cont.peek() {
                if ch.is_digit(10) {
                    syntax.push(ch);
                    cont.next();
                } else {
                    break;
                }
            }

            tokens.push(Token {
                t_type: TokenType::T_INT_LIT,
                value: Some(syntax.clone()),
            })
        } else {
            syntax.push(c);
            if syntax == LINEEND.syntax {
                tokens.push(Token {
                    t_type: TokenType::T_SEMI,
                    value: None,
                });
            } else if syntax == EQ.syntax {
                tokens.push(Token {
                    t_type: TokenType::T_EQ,
                    value: None,
                })
            } else if syntax == PLUS.syntax {
                tokens.push(Token {
                    t_type: PLUS.t_type,
                    value: None,
                })
            } else if syntax == MINUS.syntax {
                tokens.push(Token {
                    t_type: MINUS.t_type,
                    value: None,
                })
            } else if syntax == STAR.syntax {
                tokens.push(Token {
                    t_type: STAR.t_type,
                    value: None,
                })
            } else if syntax == FSLASH.syntax {
                tokens.push(Token {
                    t_type: FSLASH.t_type,
                    value: None,
                })
            } else if syntax == UNDERSCORE.syntax {
                tokens.push(Token {
                    t_type: UNDERSCORE.t_type,
                    value: None,
                })
            } else if syntax == OPEN_PAR.syntax {
                tokens.push(Token {
                    t_type: OPEN_PAR.t_type,
                    value: None,
                })
            } else if syntax == CLOSE_PAR.syntax {
                tokens.push(Token {
                    t_type: CLOSE_PAR.t_type,
                    value: None,
                })
            } else if syntax == OPEN_CUR.syntax {
                tokens.push(Token {
                    t_type: OPEN_CUR.t_type,
                    value: None,
                })
            } else if syntax == CLOSE_CUR.syntax {
                tokens.push(Token {
                    t_type: CLOSE_CUR.t_type,
                    value: None,
                })
            } else {
                return Err(print_error(
                    Some(WEIRD_ERROR),
                    Some(format!("Undefined char '{}'", c).to_owned()),
                ));
            }
        }
        syntax.clear();
    }
    Ok(tokens)
}
