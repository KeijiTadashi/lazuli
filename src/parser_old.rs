// use std::fs::File;
// use std::io::prelude::*;

// use crate::global::*;
// use crate::lzl_error::*;
// use crate::nodes::*;
// // use crate::printd;
// use crate::tokens::*;

// static mut tokenlist: Vec<Token> = vec![];

// // change to Result<NodeProg, u8>
// pub fn parse(tokens: Vec<Token>, filename: &str) -> Result<(), u8> {
//     unsafe { tokenlist = tokens };

//     let l = tokens.len();
//     let mut f: String = "".to_string();
//     // 6 space tabs, t3 for 3 letter commands "mov...", t4 for 4 letter "call.."
//     const TAB: &str = "      ";
//     const S3: &str = "   ";
//     const S4: &str = "  ";

//     // asm setup
//     f.push_str("; Compiled using the Lazuli compiler for Lazuli.... stuff\n");
//     f.push_str("BITS 64\nDEFAULT REL\n\nsegment .text\nglobal main\nextern ExitProcess\n\nmain:\n"); // default doc: https://www.nasm.us/xdoc/2.13.02rc2/html/nasmdoc6.html (don't really know the difference right now TODO)

//     let mut i: usize = 0;

//     printd(
//         "Started parsing the tokens into assembly".to_string(),
//         DebugType::MESSAGE,
//     );

//     // printd!("some text {}", "blue".blue());

//     while i < l {
//         let t: &Token = &tokens[i];
//         // println!("I: {} => t: {:?}", i, t);
//         if matches!(t.t_type, TokenType::T_RETURN) {
//             if i + 1 < l && matches!(&tokens[i + 1].t_type, TokenType::T_INT) {
//                 if i + 2 < l && matches!(&tokens[i + 2].t_type, TokenType::T_SEMI) {
//                     // create return in asm
//                     f.push_str(&format!(
//                         "{}mov{}ecx, {}\n{}call{}ExitProcess\n",
//                         TAB,
//                         S3,
//                         tokens[i + 1].value.as_ref().unwrap(),
//                         TAB,
//                         S4
//                     ));
//                     i += 2;
//                 }
//             }
//         } else {
//             return Err(print_error(
//                 Some(WEIRD_ERROR),
//                 Some("Only \"ret ##;\" works for now".to_owned()),
//             ));
//         }
//         i += 1;
//     }

//     // write string to file

//     let mut file = File::create(format!("{}.asm", filename)).expect("Failed to create .asm file");
//     file.write_all(f.as_bytes())
//         .expect("Couldn't write to .asm file");
//     file.flush().expect("Couldn't flush .asm file"); // can probably not use flush from what I found, but for now let's keep it here so stuff doesn't break TODO

//     printd(format!("Created {}.asm", filename), DebugType::CREATE);

//     parse_prog();

//     return Ok(());
// }

// fn peek() -> Option<Token> {
//     if tokenlist.len() <= 0 {
//         return None;
//     }
//     return Some(tokenlist[tokenlist.len() - 1]);
// }

// fn peek_ahead(ahead: usize) -> Option<Token> {
//     if tokenlist.len() <= ahead {
//         return None;
//     }
//     return Some(tokenlist[tokenlist.len() - ahead - 1]);
// }

// fn next() -> Option<Token> {
//     if let Some(r) = tokenlist.pop() {
//         return Some(r);
//     }
//     return None;
// }

// fn parse_prog() -> Result<NodeProg, u8> {
//     let prog: NodeProg;

//     while tokenlist.

//     return Ok(prog);
// }
