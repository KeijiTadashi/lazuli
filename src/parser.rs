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
                    Some(t) => {
                        stmt.var = VarStmt::RET(stmt_ret);
                        return Ok(stmt);
                    }
                    None => {
                        return Err(print_error(
                            Some(WEIRD_ERROR),
                            Some("Expected ';' after 'ret [expr]'".to_owned()),
                        ))
                    }
                }
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
        if let Some(int_lit) = self.try_next(T_INT) {
            let mut term_int_lit = NodeTermInt::new();
            term_int_lit.value = int_lit.value.unwrap();
            term.var = VarTerm::INT_LIT(term_int_lit);
            return Ok(term);
        }
        return Err(print_error(
            Some(WEIRD_ERROR),
            Some("Didn't find term".to_owned()),
        ));
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
