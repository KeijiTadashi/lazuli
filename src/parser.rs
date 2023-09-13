use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

use crate::lzl_error::*;
use crate::tokens::*;

pub fn parse(tokens: Vec<Token>, filename: String) -> Result<u8, u8> {
    let l = tokens.len();
    let mut f: String = "".to_string();
    // 6 space tabs, t3 for 3 letter commands "mov...", t4 for 4 letter "call.."
    const TAB: &str = "      ";
    const S3: &str = "   ";
    const S4: &str = "  ";

    // asm setup
    f.push_str("; Compiled using the Lazuli compiler for Lazuli.... stuff\n");
    f.push_str("BITS 64\nDEFAULT REL\n\nsegment .text\nglobal main\nextern ExitProcess\n\nmain:\n"); // default doc: https://www.nasm.us/xdoc/2.13.02rc2/html/nasmdoc6.html (don't really know the difference right now TODO)

    let mut i: usize = 0;

    while i < l {
        let t: &Token = &tokens[i];
        println!("I: {} => t: {:?}", i, t);
        if matches!(t.t_type, TokenType::RETURN) {
            if i + 1 < l && matches!(&tokens[i + 1].t_type, TokenType::INT) {
                if i + 2 < l && matches!(&tokens[i + 2].t_type, TokenType::SEMI) {
                    // create return in asm
                    f.push_str(&format!(
                        "{}mov{}ecx, {}\n{}call{}ExitProcess\n",
                        TAB,
                        S3,
                        tokens[i + 1].value.as_ref().unwrap(),
                        TAB,
                        S4
                    ));
                    i += 2;
                }
            }
        } else {
            return Err(print_error(
                WEIRD_ERROR,
                Some("Only \"ret ##;\" works for now".to_owned()),
            ));
        }
        i += 1;
    }

    // write string to file

    let mut file = File::create(format!("{}.asm", filename)).expect("Failed to create .asm file");
    file.write_all(f.as_bytes())
        .expect("Couldn't write to .asm file");
    file.flush().expect("Couldn't flush .asm file"); // can probably not use flush from what I found, but for now let's keep it here so stuff doesn't break TODO

    // nasm -f win64 -o hello_world.obj hello_world.asm
    let run_nasm = Command::new("cmd")
        .args([
            "/C",
            "nasm",
            "-f",
            "win64",
            "-o",
            &format!("{}.obj", filename),
            &format!("{}.asm", filename),
        ])
        .output();

    if run_nasm.is_err() {
        return Err(print_error(
            WEIRD_ERROR,
            Some("Couldn't create .obj file from .asm".to_owned()),
        ));
    }

    // link hello_world.obj /subsystem:console /out:hello_world_basic.exe kernel32.lib legacy_stdio_definitions.lib msvcrt.lib
    let run_link = Command::new("cmd")
        .args([
            "/C",
            "call",
            "C:\\Program Files (x86)\\Microsoft Visual Studio\\2022\\BuildTools\\VC\\Auxiliary\\Build\\vcvars64.bat",
            "&&",
            "link",
            &format!("{}.obj", filename),
            "/subsystem:console",
            &format!("/out:{}.exe", filename),
            "kernel32.lib",
            "legacy_stdio_definitions.lib",
            "msvcrt.lib",
        ])
        .output();

    if run_link.is_err() {
        return Err(print_error(
            WEIRD_ERROR,
            Some("Couldn't create .exe file from .obj".to_owned()),
        ));
    }

    return Ok(EXIT_SUCCES);
}
