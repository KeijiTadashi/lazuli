use std::{collections::HashMap, fs::File, io::Write, rc::Rc};

use crate::{
    global::*,
    lzl_error::{print_error, EXIT_SUCCES, WEIRD_ERROR},
    nodes::*,
};

// struct Var<'a> {
//     var_type: &'a NodeType,
//     ident: &'a String,
#[derive(Debug)]
struct Var {
    var_type: Rc<NodeType>,
    ident: String,
    stack_loc: usize,
}

pub struct AsmGenerator {
    out: String,
    vars: Vec<Var>,
    // vars: Vec<Var<'a>>,
    exit_code: u8,
    stack_size: usize,
    scopes: Vec<usize>,
    labels: HashMap<String, u8>,
}

impl AsmGenerator {
    pub fn new() -> AsmGenerator {
        return AsmGenerator {
            out: String::new(),
            vars: vec![],
            exit_code: EXIT_SUCCES,
            stack_size: 0,
            scopes: vec![],
            labels: Default::default(),
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
                self.write_instruction("mov", "rcx, rax");
                self.write_instruction("call", "ExitProcess")
            }
            // TODO move assignment to own node with own gen_assign
            VarStmt::ASSIGN(var_stmt) => {
                let pos = self.vars.iter().position(|v| v.ident == var_stmt.ident);

                match var_stmt.var_type.as_ref() {
                    // NONE -> No identifier before var name (needs to exist already)
                    NodeType::NONE => {
                        match pos {
                            Some(i) => {
                                self.gen_expr(&var_stmt.expr);
                                self.vars[i].stack_loc = self.stack_size - 1;
                            },
                            None => self.exit_code = print_error(
                                Some(WEIRD_ERROR),
                                Some(format!(
                                    "A variable with name: \"{}\" has not yet been assigned, and therefore has no type",
                                    var_stmt.ident
                                )),
                            ),
                        }
                    }
                    NodeType::N_INT => {
                        match pos {
                            Some(_) => self.exit_code = print_error(
                                Some(WEIRD_ERROR),
                                Some(format!(
                                    "A variable with name: \"{}\" has already been assigned.",
                                    var_stmt.ident
                            ))),
                            None => {
                                self.vars.push(Var {
                                    var_type: var_stmt.var_type.clone(),
                                    ident: var_stmt.ident.to_owned(),
                                    stack_loc: self.stack_size,
                                });
                                self.gen_expr(&var_stmt.expr);
                            }
                        }
                    }
                }
            }
            VarStmt::SCOPE(var_stmt) => {
                self.gen_scope(&var_stmt);
            }
            VarStmt::IF(var_stmt) => {
                self.gen_expr(&var_stmt.expr);
                self.pop("rax");
                let lbl = self.create_label("if");
                self.write_instruction("cmp", "rax, 0");
                self.write_instruction("je", &lbl);
                self.gen_scope(&var_stmt.scope);
                self.out.push_str(format!("{}:\n", lbl).as_str());
            }
        }
    }

    fn gen_scope(&mut self, scope: &NodeScope) {
        self.scopes.push(self.vars.len());
        for stmt in scope.stmts.iter() {
            self.gen_stmt(stmt);
            if self.exit_code != EXIT_SUCCES {
                break;
            }
        }

        let scope_size = self.vars.len() - self.scopes.pop().unwrap_or(0);
        self.write_instruction("add", format!("rsp, {} * 8", scope_size).as_str());
        self.stack_size -= scope_size;
        self.vars.truncate(self.vars.len() - scope_size);
    }

    fn gen_expr(&mut self, expr: &NodeExpr) {
        match &expr.var {
            VarExpr::NONE => {
                _ = print_error(
                    Some(WEIRD_ERROR),
                    Some(format!("NOT YET IMPLEMENTED: {:?}", expr.var).to_owned()),
                )
            }
            VarExpr::TERM(var_expr) => self.gen_term(&var_expr),
            VarExpr::BIN(var_expr) => self.gen_binexpr(&var_expr),
        }
    }

    fn gen_term(&mut self, term: &NodeTerm) {
        match &term.var {
            VarTerm::NONE => todo!(),
            VarTerm::INT_LIT(int_lit) => {
                self.write_instruction("mov", format!("rax, {}", int_lit.value).as_str());
                self.push("rax");
            }
            VarTerm::IDENT(ident) => {
                let var = self.vars.iter().find(|&v| v.ident == ident.ident);

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
            VarTerm::NEG(var_term) => {
                self.gen_term(&var_term.term);
                self.pop("rax");
                self.write_instruction("neg", "rax");
                self.push("rax");
            }
            VarTerm::PAR(var_term) => {
                self.gen_expr(&var_term.expr);
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
                self.write_instruction("add", "rax, rbx");
                self.push("rax");
            }
            VarBinExpr::SUB => {
                self.gen_expr(&binexpr.rhs);
                self.gen_expr(&binexpr.lhs);
                self.pop("rax");
                self.pop("rbx");
                self.write_instruction("sub", "rax, rbx");
                self.push("rax");
            }
            VarBinExpr::MUL => {
                self.gen_expr(&binexpr.rhs);
                self.gen_expr(&binexpr.lhs);
                self.pop("rax");
                self.pop("rbx");
                self.write_instruction("mul", "rbx");
                self.push("rax");
            }
            // 64 division -> div [value](64bit) => [rdx][rax] / [value] ==> result [rax] : quotient, [rdx] : remainder
            // only unsigned division if one of the numbers is signed (negative) it doesn't work
            VarBinExpr::DIV => {
                self.gen_expr(&binexpr.rhs);
                self.gen_expr(&binexpr.lhs);
                self.write_instruction("xor", "rdx, rdx");
                self.pop("rax");
                self.pop("rbx");
                self.write_instruction("div", "rbx");
                self.push("rax");
            }
        }
    }

    fn write_instruction(&mut self, instruction: &str, operation: &str) {
        self.out
            .push_str(format!("      {:<6}{}\n", instruction, operation).as_str());
    }

    /// TODO for now only 64 bit registers else stuff probably breaks in multiple places
    fn push(&mut self, register: &str) {
        self.write_instruction("push", register);
        self.stack_size += 1; // 8 bytes (64 bit register)
    }

    /// TODO for now only 64 bit registers else stuff probably breaks in multiple places
    fn pop(&mut self, register: &str) {
        self.write_instruction("pop", register);
        self.stack_size -= 1;
    }

    fn create_label(&mut self, labelname: &str) -> String {
        let count = self.labels.entry(labelname.to_string()).or_insert(0);
        *count += 1;
        return format!("{}{}", labelname, count);
    }
}
