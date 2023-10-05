use std::rc::Rc;

use crate::{
    global::{printd, DebugType},
    lzl_error::*,
    nodes::*,
    tokens::{
        bin_prec, Token,
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
        while let Some(peeked) = self.peek() {
            printd(format!("Peek in prog: {:?}", peeked), DebugType::NONE);
            prog.stmts.push(self.parse_stmt()?);
        }
        return Ok(prog);
    }

    fn parse_stmt(self: &'_ mut Self) -> Result<Rc<NodeStmt>, u8> {
        let mut stmt = NodeStmt::new();

        if let Some(peeked) = self.peek() {
            printd(
                format!("Peek in stmt: {:?}", peeked),
                crate::global::DebugType::NONE,
            );
            if peeked.t_type == T_RETURN {
                self.next();
                stmt.var = VarStmt::RET(
                    NodeStmtRet {
                        expr: self.parse_expr(None)?,
                    }
                    .into(),
                );
                if self.try_next(T_SEMI).is_none() {
                    return Err(print_error(
                        Some(WEIRD_ERROR),
                        Some("Expected ';' after 'ret [expr]'".to_owned()),
                    ));
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
                    if ![
                        T_INT_LIT,
                        T_IDENT,
                        T_PLUS,
                        T_MINUS,
                        T_FSLASH,
                        T_STAR,
                        T_UNDERSCORE,
                    ]
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
                stmt_int.expr = self.parse_expr(None)?.into();
                match self.try_next(T_SEMI) {
                    Some(_) => {
                        stmt.var = VarStmt::ASSIGN(stmt_int.into());
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

                stmt_ident.expr = self.parse_expr(None)?.into();

                match self.try_next(T_SEMI) {
                    Some(_) => {
                        stmt.var = VarStmt::ASSIGN(stmt_ident.into());
                    }
                    None => {
                        return Err(print_error(
                            Some(WEIRD_ERROR),
                            Some("Expected ';' after 'int {{name}} = [expr]'".to_owned()),
                        ))
                    }
                }
            } else if peeked.t_type == T_OPEN_CUR {
                stmt.var = VarStmt::SCOPE(self.parse_scope()?.into());
            } else if peeked.t_type == T_IF {
                self.next();
                stmt.var = VarStmt::IF(
                    NodeStmtIf {
                        expr: self.parse_expr(None)?,
                        scope: self.parse_scope()?,
                    }
                    .into(),
                );
            } else if peeked.t_type == T_WHILE {
                self.next();
                stmt.var = VarStmt::WHILE(
                    NodeStmtWhile {
                        expr: self.parse_expr(None)?,
                        scope: self.parse_scope()?,
                    }
                    .into(),
                );
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

        return Ok(stmt.into());
    }

    fn parse_scope(self: &'_ mut Self) -> Result<Rc<NodeScope>, u8> {
        let mut scope = NodeScope::new();
        self.try_next(T_OPEN_CUR);
        while let Some(peeked) = self.peek() {
            printd(format!("Peek in scope: {:?}", self.peek()), DebugType::NONE);
            if peeked.t_type == T_CLOSE_CUR {
                self.next();
                return Ok(scope.into());
            }
            scope.stmts.push(self.parse_stmt()?);
        }
        return Err(print_error(
            Some(WEIRD_ERROR),
            Some("Expected '}'.".to_owned()),
        ));
    }

    fn parse_expr(self: &'_ mut Self, min_precedence: Option<u8>) -> Result<Rc<NodeExpr>, u8> {
        printd(format!("Peek in expr: {:?}", self.peek()), DebugType::NONE);
        let mut expr_lhs = NodeExpr::new();

        let min_prec = min_precedence.unwrap_or(0);

        expr_lhs.var = VarExpr::TERM(self.parse_term()?.into());

        loop {
            let prec: Option<u8>;
            match self.peek() {
                Some(t) => {
                    prec = bin_prec(&t.t_type);
                    if prec.is_none() || prec < Some(min_prec) {
                        break;
                    }
                }
                None => break,
            }
            let operation: Token;
            match self.next() {
                Some(o) => operation = o,
                None => break,
            }
            let next_min_prec: u8 = prec.unwrap() + 1;
            let expr_rhs = self.parse_expr(Some(next_min_prec))?;

            let mut expr = NodeBinExpr::new();
            let mut expr_lhs_new = NodeExpr::new();

            match expr_lhs.var {
                VarExpr::BIN(l) => expr_lhs_new.var = VarExpr::BIN(l.into()),
                VarExpr::TERM(l) => expr_lhs_new.var = VarExpr::TERM(l.into()),
                _ => {
                    return Err(print_error(
                        Some(WEIRD_ERROR),
                        Some(format!("NOT IMPLEMENTED: {:?}", expr_lhs).to_owned()),
                    ))
                }
            }

            expr.lhs = expr_lhs_new.into();
            expr.rhs = expr_rhs.into();

            match operation.t_type {
                T_PLUS => expr.var = VarBinExpr::ADD,
                T_MINUS => expr.var = VarBinExpr::SUB,
                T_FSLASH => expr.var = VarBinExpr::DIV,
                T_STAR => expr.var = VarBinExpr::MUL,
                _ => {
                    return Err(print_error(
                        Some(WEIRD_ERROR),
                        Some(
                            "Shouldn't be able to get here, undifined binary expresion operator"
                                .to_owned(),
                        ),
                    ))
                }
            }

            expr_lhs.var = VarExpr::BIN(expr.into());
        }

        return Ok(expr_lhs.into());
    }

    fn parse_term(self: &'_ mut Self) -> Result<Rc<NodeTerm>, u8> {
        let mut term = NodeTerm::new();
        printd(
            format!(
                "Peek in term: {:?}",
                self.peek().unwrap_or(&Token::default())
            ),
            crate::global::DebugType::NONE,
        );

        if let Some(int_lit) = self.try_next(T_INT_LIT) {
            term.var = VarTerm::INT_LIT(
                NodeTermIntLit {
                    value: int_lit.value.unwrap(),
                }
                .into(),
            );
        } else if let Some(ident) = self.try_next(T_IDENT) {
            term.var = VarTerm::IDENT(
                NodeTermIdent {
                    ident: ident.value.unwrap(),
                }
                .into(),
            );
        } else if self.try_next(T_UNDERSCORE).is_some() {
            term.var = VarTerm::NEG(
                NodeTermNeg {
                    term: self.parse_term()?,
                }
                .into(),
            );
        } else if self.try_next(T_OPEN_PAR).is_some() {
            term.var = VarTerm::PAR(
                NodeTermPar {
                    expr: self.parse_expr(None)?,
                }
                .into(),
            );
            if self.try_next(T_CLOSE_PAR).is_none() {
                return Err(print_error(
                    Some(WEIRD_ERROR),
                    Some(format!("Expexted ')'.").to_owned()),
                ));
            }
        } else {
            return Err(print_error(
                Some(WEIRD_ERROR),
                Some("Didn't find term".to_owned()),
            ));
        }

        return Ok(term.into());
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
