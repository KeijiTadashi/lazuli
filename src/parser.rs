use crate::{
    global::{printd, DebugType},
    lzl_error::{print_error, WEIRD_ERROR},
    nodes::*,
    tokens::{
        Token,
        TokenType::{self, *},
    },
};

pub struct Parser {
    tokens: Vec<Token>,
    // length: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            // length: tokens.len(),
            tokens,
        }
    }

    pub fn parse(self: &'_ mut Self) -> Result<NodeProg, u8> {
        let mut prog: NodeProg = NodeProg::new();
        printd("Started parse".to_owned(), DebugType::MESSAGE);
        while self.peek().is_some() {
            printd(
                format!("Peek in prog: {:?}", self.peek()),
                DebugType::MESSAGE,
            );
            let stmt = self.parse_stmt();
            match stmt {
                Ok(n) => prog.stmts.push(n),
                Err(e) => return Err(e),
            }
        }
        // printd(
        //     format!(
        //         "tokens: {:?}\n peek: {:?}\npeek ahead 1: {:?}\npeek again: {:?}",
        //         self.tokens,
        //         self.peek(),
        //         self.peek_ahead(1),
        //         self.peek()
        //     ),
        //     DebugType::MESSAGE,
        // );
        // let tempnextstuffname = self.next();
        // printd(
        //     format!(
        //         "next: {:?}\npeek: {:?}\ntokens: {:?}",
        //         tempnextstuffname,
        //         self.peek(),
        //         self.tokens
        //     ),
        //     DebugType::MESSAGE,
        // );

        return Ok(prog);
    }

    fn parse_stmt(self: &'_ mut Self) -> Result<NodeStmt, u8> {
        let mut stmt = NodeStmt::new();

        if let Some(peeked) = self.peek() {
            printd(
                format!("Peek in stmt: {:?}", peeked),
                crate::global::DebugType::MESSAGE,
            );
            if peeked.t_type == T_RETURN {
                self.next();
                let mut stmt_ret = NodeStmtRet::new();
                match self.parse_expr() {
                    Ok(n) => stmt_ret.expr = n,
                    Err(e) => return Err(e),
                }
                match self.try_next(T_SEMI) {
                    Some(_) => {
                        stmt.var = VarStmt::RET(stmt_ret);
                    }
                    None => {
                        return Err(print_error(
                            Some(WEIRD_ERROR),
                            Some("Expected ';' after 'ret [expr]'".to_owned()),
                        ))
                    }
                }
            } else if peeked.t_type == T_INT {
                self.next();
                let mut stmt_int = NodeStmtAssign::new();
                stmt_int.var_type = NodeType::N_INT.into();
                match self.try_next(T_IDENT) {
                    Some(t) => {
                        match t.value {
                            Some(ident) => stmt_int.ident = ident,
                            None => return Err(print_error(Some(WEIRD_ERROR), Some(format!("Identifier token should have a value {:?}, should never reach this error.", t)))),
                        };
                    }
                    None => todo!(), //expected identifier
                };
                match self.try_next(T_EQ) {
                    Some(_) => (),
                    None => return Err(print_error(Some(WEIRD_ERROR), Some(format!("Expected '=' after identifier \"{}\", all variables need to be initialized with a value.", stmt_int.ident)))),
                }
                let mut i: usize = 0;
                while let Some(term) = self.peek_ahead(i) {
                    if ![T_INT_LIT, T_IDENT, T_PLUS, T_MINUS, T_FSLASH, T_STAR]
                        .contains(&term.t_type)
                    {
                        if T_SEMI == term.t_type {
                            break;
                        }
                        return Err(print_error(
                            Some(WEIRD_ERROR),
                            Some(format!(
                                "\"{}\" is of type int and \"{:?}\" is not valid here",
                                stmt_int.ident, term
                            )),
                        ));
                    }
                    i += 1;
                }
                match self.parse_expr() {
                    Ok(n) => {
                        stmt_int.expr = n;
                    }
                    Err(e) => return Err(e),
                };
                match self.try_next(T_SEMI) {
                    Some(_) => {
                        stmt.var = VarStmt::ASSIGN(stmt_int);
                    }
                    None => {
                        return Err(print_error(
                            Some(WEIRD_ERROR),
                            Some("Expected ';' after 'int {{name}} = [expr]'".to_owned()),
                        ))
                    }
                }
            } else if peeked.t_type == T_IDENT {
                let mut stmt_ident = NodeStmtAssign::new();
                stmt_ident.ident = self.next().unwrap().value.unwrap();
                match self.try_next(T_EQ) {
                    Some(_) => (),
                    None => return Err(print_error(Some(WEIRD_ERROR), Some(format!("Expected '=' after identifier \"{}\", all variables need to be initialized with a value.", stmt_ident.ident)))),
                }
                // TODO type checking of expression and variable type

                match self.parse_expr() {
                    Ok(n) => {
                        stmt_ident.expr = n;
                    }
                    Err(e) => return Err(e),
                };
                match self.try_next(T_SEMI) {
                    Some(_) => {
                        stmt.var = VarStmt::ASSIGN(stmt_ident);
                    }
                    None => {
                        return Err(print_error(
                            Some(WEIRD_ERROR),
                            Some("Expected ';' after 'int {{name}} = [expr]'".to_owned()),
                        ))
                    }
                }
            } else {
                return Err(print_error(
                    Some(WEIRD_ERROR),
                    Some(format!("Unknown stmt: {:?}", peeked)),
                ));
            }
        } else {
            return Err(print_error(
                Some(WEIRD_ERROR),
                Some("No token at stmt".to_owned()),
            ));
        }

        return Ok(stmt);
    }

    fn parse_expr(self: &'_ mut Self) -> Result<NodeExpr, u8> {
        let mut term_lhs = self.parse_term();
        let mut expr_lhs = NodeExpr::new();

        printd(
            format!("expr: {:?}||term_lhs: {:?}", self.peek(), term_lhs),
            crate::global::DebugType::MESSAGE,
        );
        match term_lhs {
            Err(e) => return Err(e),
            Ok(n) => expr_lhs.var = VarExpr::TERM(n),
        }
        // loop {

        // }

        return Ok(expr_lhs);

        // if let Some(peeked) = self.peek() {
        //     return Ok(self.parse_term());
        // } else {
        //     return Err(print_error(
        //         Some(WEIRD_ERROR),
        //         Some("No token at expr".to_owned()),
        //     ));
        // }
    }

    fn parse_term(self: &'_ mut Self) -> Result<NodeTerm, u8> {
        let mut term = NodeTerm::new();
        // if let Some(peeked) = self.peek() {
        //     if
        // }
        printd(
            format!("Parse Term, peek: {:?}", self.peek()),
            crate::global::DebugType::MESSAGE,
        );
        if let Some(int_lit) = self.try_next(T_INT_LIT) {
            let mut term_int_lit = NodeTermIntLit::new();
            term_int_lit.value = int_lit.value.unwrap();
            term.var = VarTerm::INT_LIT(term_int_lit);
        } else if let Some(ident) = self.try_next(T_IDENT) {
            let mut term_ident = NodeTermIdent::new();
            term_ident.ident = ident.value.unwrap();
            term.var = VarTerm::IDENT(term_ident);
        } else {
            return Err(print_error(
                Some(WEIRD_ERROR),
                Some("Didn't find term".to_owned()),
            ));
        }
        return Ok(term);
    }

    fn peek(&self) -> Option<&Token> {
        if self.tokens.len() <= 0 {
            return None;
        }
        return Some(&self.tokens[self.tokens.len() - 1]);
    }

    fn peek_ahead(&self, ahead: usize) -> Option<&Token> {
        if self.tokens.len() <= ahead {
            return None;
        }
        return Some(&self.tokens[self.tokens.len() - ahead - 1]);
    }

    fn next(self: &'_ mut Self) -> Option<Token> {
        if let Some(t) = self.tokens.pop() {
            return Some(t);
        }
        return None;
    }

    fn try_next(self: &'_ mut Self, token_type: TokenType) -> Option<Token> {
        if let Some(t) = self.peek() {
            if t.t_type == token_type {
                return self.next();
            }
        }
        return None;
    }
}
