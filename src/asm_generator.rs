use std::{fs::File, io::Write, rc::Rc};

use crate::{
    global::*,
    lzl_error::{print_error, EXIT_SUCCES, WEIRD_ERROR},
    nodes::*,
};

/// location in bytes (smallest typing is 1 byte long) INCORRECT NOW 8 BYTES
// struct Var<'a> {
//     var_type: &'a NodeType,
//     ident: &'a String,
#[derive(Debug)]
struct Var {
    var_type: Rc<NodeType>,
    ident: String,
    stack_loc: u16, // 65535 Bytes in max stack size NO LONGER CORRECT
}

pub struct AsmGenerator {
    out: String,
    vars: Vec<Var>,
    // vars: Vec<Var<'a>>,
    exit_code: u8,
    stack_size: u16,
}

impl AsmGenerator {
    pub fn new() -> AsmGenerator {
        return AsmGenerator {
            out: String::new(),
            vars: vec![],
            exit_code: EXIT_SUCCES,
            stack_size: 0,
        };
    }
    // 6 space tabs, t3 for 3 letter commands "mov...", t4 for 4 letter "call.."
    // const TAB: &str = "      ";
    // const S3: &str = "   ";
    // const S4: &str = "  ";

    pub fn generate_asm(&mut self, filename: String, prog: NodeProg) -> Result<(), u8> {
        printd(
            "Started generating assembly".to_string(),
            DebugType::MESSAGE,
        );
        self.gen_prog(prog);
        // let mut i: usize = 0;

        // write string to file

        let mut file =
            File::create(format!("{}.asm", filename)).expect("Failed to create .asm file");
        file.write_all(self.out.as_bytes())
            .expect("Couldn't write to .asm file");
        file.flush().expect("Couldn't flush .asm file"); // can probably not use flush from what I found, but for now let's keep it here so stuff doesn't break TODO

        printd(format!("Created {}.asm", filename), DebugType::CREATE);

        if self.exit_code == EXIT_SUCCES {
            return Ok(());
        }
        return Err(self.exit_code);
    }

    fn gen_prog(&mut self, prog: NodeProg) {
        // asm setup
        self.out
            .push_str("; Compiled using the Lazuli compiler for Lazuli.... stuff\n");
        self.out.push_str(
            "BITS 64\nDEFAULT REL\n\nsegment .text\nglobal main\nextern ExitProcess\n\nmain:\n",
        ); // default doc: https://www.nasm.us/xdoc/2.13.02rc2/html/nasmdoc6.html (don't really know the difference right now TODO)

        //TODO figure out if iter is beter or worse in this situation
        for stmt in prog.stmts.iter() {
            self.gen_stmt(stmt);
            // print!("{:?}", stmt);
            if self.exit_code != EXIT_SUCCES {
                break;
            }
        }
    }

    fn gen_stmt(&mut self, stmt: &NodeStmt) {
        match &stmt.var {
            VarStmt::NONE => todo!(),
            VarStmt::RET(var_stmt) => {
                self.gen_expr(&var_stmt.expr);
                self.pop("rax");
                self.push_out("mov", "rcx, rax");
                self.push_out("call", "ExitProcess")
            }
            // TODO move assignment to own node with own gen_assign
            VarStmt::ASSIGN(var_stmt) => {
                let var = self.vars.iter_mut().find(|v| v.ident == var_stmt.ident);

                if var_stmt.var_type == NodeType::NONE.into() {
                    match var {
                        Some(v) => v.stack_loc = self.stack_size,
                        None => {self.exit_code = print_error(
                            Some(WEIRD_ERROR),
                            Some(format!(
                                "A variable with name: \"{}\" has not yet been assigned, and therefore has no type",
                                var_stmt.ident
                            )),
                        )},
                    }
                } else {
                    match var {
                        Some(_) => {
                            self.exit_code = print_error(
                                Some(WEIRD_ERROR),
                                Some(format!(
                                    "A variable with name: \"{}\" has already been assigned.",
                                    var_stmt.ident
                                )),
                            )
                        }
                        None => self.vars.push(Var {
                            var_type: var_stmt.var_type.clone(),
                            ident: var_stmt.ident.to_owned(),
                            stack_loc: self.stack_size,
                        }),
                    }
                }

                self.gen_expr(&var_stmt.expr);
            }
        }
    }

    fn gen_expr(&mut self, expr: &NodeExpr) {
        match &expr.var {
            VarExpr::NONE => printd(
                format!("NOT YET IMPLEMENTED: {:?}", expr.var),
                DebugType::REMOVE,
            ),
            VarExpr::TERM(var_expr) => self.gen_term(&var_expr),
            VarExpr::BIN(var_expr) => self.gen_binexpr(&var_expr),
            VarExpr::NEG(var_expr) => {
                self.gen_expr(&var_expr.expr);
                self.pop("rax");
                self.push_out("neg", "rax");
                self.push("rax");
            }
        }
    }

    fn gen_term(&mut self, term: &NodeTerm) {
        match &term.var {
            VarTerm::NONE => todo!(),
            VarTerm::INT_LIT(int_lit) => {
                self.push_out("mov", format!("rax, {}", int_lit.value).as_str());
                self.push("rax");
            }
            VarTerm::IDENT(ident) => {
                let var = self.vars.iter().find(|&v| v.ident == ident.ident);
                printd(
                    format!("IDENT: {:?}, stacksize: {}", var, self.stack_size),
                    DebugType::REMOVE,
                );
                match var {
                    Some(v) => self.push(&format!(
                        "QWORD [rsp + {}]",
                        (self.stack_size - v.stack_loc - 1) * 8
                    )),
                    None => {
                        self.exit_code = print_error(
                            Some(WEIRD_ERROR),
                            Some(format!(
                                "No variable declared with name: \"{}\".",
                                ident.ident
                            )),
                        )
                    }
                }
            }
        }
    }

    fn gen_binexpr(&mut self, binexpr: &NodeBinExpr) {
        match &binexpr.var {
            VarBinExpr::NONE => todo!(),
            VarBinExpr::ADD => {
                self.gen_expr(&binexpr.rhs);
                self.gen_expr(&binexpr.lhs);
                self.pop("rax");
                self.pop("rbx");
                self.push_out("add", "rax, rbx");
                self.push("rax");
            }
            VarBinExpr::SUB => {
                self.gen_expr(&binexpr.rhs);
                self.gen_expr(&binexpr.lhs);
                self.pop("rax");
                self.pop("rbx");
                self.push_out("sub", "rax, rbx");
                self.push("rax");
            }
            VarBinExpr::MUL => {
                self.gen_expr(&binexpr.rhs);
                self.gen_expr(&binexpr.lhs);
                self.pop("rax");
                self.pop("rbx");
                self.push_out("mul", "rbx");
                self.push("rax");
            }
            // 64 division -> div [value](64bit) => [rdx][rax] / [value] ==> result [rax] : quotient, [rdx] : remainder
            // only unsigned division if one of the numbers is signed (negative) it doesn't work
            VarBinExpr::DIV => {
                self.gen_expr(&binexpr.rhs);
                self.gen_expr(&binexpr.lhs);
                self.push_out("xor", "rdx, rdx");
                self.pop("rax");
                self.pop("rbx");
                self.push_out("div", "rbx");
                self.push("rax");
            }
        }
    }

    fn push_out(&mut self, instruction: &str, operation: &str) {
        self.out
            .push_str(format!("      {:<6}{}\n", instruction, operation).as_str());
    }

    /// TODO for now only 64 bit registers else stuff probably breaks in multiple places
    fn push(&mut self, register: &str) {
        self.push_out("push", register);
        self.stack_size += 1; // 8 bytes (64 bit register)
    }

    /// TODO for now only 64 bit registers else stuff probably breaks in multiple places
    fn pop(&mut self, register: &str) {
        self.push_out("pop", register);
        self.stack_size -= 1;
    }
}
