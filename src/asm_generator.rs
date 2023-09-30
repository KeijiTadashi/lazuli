use std::{fs::File, io::Write};

use crate::{global::*, nodes::*};

pub struct AsmGenerator {
    out: String,
}

impl AsmGenerator {
    pub fn new() -> AsmGenerator {
        return AsmGenerator { out: String::new() };
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

        return Ok(());
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
        }
    }

    fn gen_stmt(&mut self, stmt: &NodeStmt) {
        match &stmt.var {
            VarStmt::NONE => todo!(),
            VarStmt::RET(VarStmt) => {
                self.gen_expr(&VarStmt.expr);
                self.pop("rax");
                self.push_out("mov", "rcx, rax");
                self.push_out("call", "ExitProcess")
            }
            VarStmt::ASSIGN(_) => todo!(),
        }
    }

    fn gen_expr(&mut self, expr: &NodeExpr) {
        match &expr.var {
            VarExpr::NONE => todo!(),
            VarExpr::TERM(VarExpr) => self.gen_term(&VarExpr),
        }
    }

    fn gen_term(&mut self, term: &NodeTerm) {
        match &term.var {
            VarTerm::NONE => todo!(),
            VarTerm::INT_LIT(int_lit) => {
                self.push_out("mov", format!("rax, {}", int_lit.value).as_str());
                self.push("rax");
            }
        }
    }

    fn push_out(&mut self, instruction: &str, operation: &str) {
        self.out
            .push_str(format!("      {:<6}{}\n", instruction, operation).as_str());
    }

    fn push(&mut self, register: &str) {
        self.push_out("push", register);
    }

    fn pop(&mut self, register: &str) {
        self.push_out("pop", register);
    }
}
