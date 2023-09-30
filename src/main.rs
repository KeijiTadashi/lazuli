use std::fs::remove_file;
use std::process::Command;
use std::rc::Rc;
use std::{env, process::ExitCode};

mod asm_generator;
mod colourtest;
mod global;
mod lzl_error;
mod nodes;
mod parser;
mod tokenizer;
mod tokens;
use colored::Colorize;

use crate::colourtest::testcolours;
use crate::global::*;
use crate::lzl_error::*;
use crate::parser::*;
use crate::tokenizer::*;
use crate::tokens::*;

fn main() -> ExitCode {
    printd("Started lazuli compiler".to_string(), DebugType::MESSAGE);
    let args: Vec<String> = env::args().collect(); // arg[0] is lazuli
    let exit_code: u8;
    let mut tokens: Vec<Token>;

    let input_file: Rc<str>;
    let mut output_file: String;
    let mut output_file_type: OutputFileType = OutputFileType::EXE;
    // let mut debug: bool = false;
    let mut keep_intermediate: bool = false;

    'main: loop {
        if args.len() < 2 {
            exit_code = print_error(
                Some(MISSING_ARGUMENT),
                Some(
                    "No input file provided:\nlazuli <input[.lzl]>\nUse \"lazuli [-help | -h | -?]\" for more info.".to_string()
                ),
            );
            testcolours();
            break;
        } else if ["-help", "-h", "-?"].contains(&args[1].as_str()) {
            println!("lazuli <input[.lzl]> [-o <output[.exe | .asm | .obj]> | -out <output[.exe | .asm | .obj]>] [[-d | -debug] | [-da | -debugall]] [-k | -keep]\n\
            All optional arguments can be in any order and are case sensitive.\n\
            {:<10}Show additional debug information.\n\
            {:<10}Show all debug information.\n\
            {:<10}Keep the intermediate files that are created (.asm and .obj).\n\
            {:<10}Specify the name of the output file. If the extension is .asm or .obj you get the corresponding intermediate file",
            "-debug", "-debugall", "-keep", "-out");
            exit_code = EXIT_SUCCES;
            break;
        } else {
            // Go through all options in args and set corresponding variables
            input_file = if args[1].ends_with(".lzl") {
                args[1].to_string()
            } else {
                format!("{}.lzl", &args[1])
            }
            .into();
            output_file = input_file.to_string();
            let mut i: usize = 2;
            while i < args.len() {
                let a = args[i].as_str();
                match a {
                    "-o" | "-out" => {
                        i += 1;
                        output_file = args[i].to_string();
                        if output_file.ends_with(".asm") {
                            output_file_type = OutputFileType::ASM
                        } else if output_file.ends_with(".obj") {
                            output_file_type = OutputFileType::OBJ
                        } else if output_file.ends_with(".exe") {
                            output_file_type = OutputFileType::EXE
                        } else {
                            output_file.push_str(".exe") // just so I can remove it again later...
                        }
                    }
                    "-d" | "-debug" => unsafe { DEBUG = true },
                    "-da" | "-debugall" => unsafe {
                        DEBUGALL = true;
                        DEBUG = true
                    },
                    "-k" | "-keep" => keep_intermediate = true,
                    _ => {
                        exit_code = print_error(
                            Some(INVALID_ARGUMENT),
                            Some(format!(
                                "{} is not a valid argument. See \"lazuli [-help | -h | -?]\"",
                                a
                            )),
                        );
                        break 'main;
                    }
                }
                i += 1;
            }
            output_file.truncate(output_file.len() - 4); // remove .lzl | .exe | .obj | .asm

            printd(
                "Finished analyzing CLI arguments".to_string(),
                DebugType::MESSAGE,
            );

            let tokenizer_result = tokenize(input_file.clone());
            match tokenizer_result {
                Ok(t) => tokens = t,
                Err(e) => {
                    exit_code = e;
                    break;
                }
            }
        }

        printd(format!("Tokenized {}", input_file), DebugType::MESSAGE);
        printd(
            format!("Amount of tokens found: {}\n{:?}", tokens.len(), tokens),
            DebugType::NONE,
        );

        tokens.reverse();
        let mut parser = Parser::new(tokens);
        let parse_result = parser.parse();
        match parse_result {
            Ok(r) => {
                printd(format!("parse result:\n{:?}", r), DebugType::MESSAGE);
                let mut gen = asm_generator::AsmGenerator::new();

                match gen.generate_asm(output_file.to_owned(), r) {
                    Ok(_) => {}
                    Err(e) => {
                        exit_code = e;
                        break 'main;
                    }
                }
            }
            Err(e) => {
                exit_code = e;
                break;
            }
        }

        if output_file_type == OutputFileType::ASM {
            exit_code = EXIT_SUCCES;
            break;
        }

        // nasm -f win64 -o hello_world.obj hello_world.asm
        let run_nasm = Command::new("cmd")
            .args([
                "/C",
                "nasm",
                "-f",
                "win64",
                "-o",
                &format!("{}.obj", output_file),
                &format!("{}.asm", output_file),
            ])
            .output();

        if run_nasm.is_err() {
            exit_code = print_error(
                Some(WEIRD_ERROR),
                Some("Couldn't create .obj file from .asm".to_owned()),
            );
            break;
        }

        printd(format!("Created {}.obj", output_file), DebugType::CREATE);

        if !keep_intermediate {
            let _ = remove_file(format!("{}.asm", output_file));
            printd(format!("Removed {}.asm", output_file), DebugType::REMOVE);
        }
        if output_file_type == OutputFileType::OBJ {
            exit_code = EXIT_SUCCES;
            break;
        }

        // link hello_world.obj /subsystem:console /out:hello_world_basic.exe kernel32.lib legacy_stdio_definitions.lib msvcrt.lib
        let run_link = Command::new("cmd")
            .args([
                "/C",
                "call",
                "C:\\Program Files (x86)\\Microsoft Visual Studio\\2022\\BuildTools\\VC\\Auxiliary\\Build\\vcvars64.bat",
                "&&",
                "link",
                &format!("{}.obj", output_file),
                "/subsystem:console",
                &format!("/out:{}.exe", output_file),
                "kernel32.lib",
                "legacy_stdio_definitions.lib",
                "msvcrt.lib",
            ])
            .output();

        if run_link.is_err() {
            exit_code = print_error(
                Some(WEIRD_ERROR),
                Some("Couldn't create .exe file from .obj".to_owned()),
            );
            break;
        }

        printd(format!("Created {}.exe", output_file), DebugType::CREATE);

        if !keep_intermediate {
            let _ = remove_file(format!("{}.obj", output_file));
            printd(format!("Removed {}.obj", output_file), DebugType::REMOVE);
        }

        //BREAK OUT OF LOOP AFTER ALL OPPERATIONS ARE COMPLEATED
        exit_code = EXIT_SUCCES;
        break;
    }

    if exit_code == EXIT_SUCCES {
        printd(
            format!("Finished compiling, exited with: {}", exit_code),
            DebugType::MESSAGE,
        );
        return ExitCode::from(0);
    } else {
        println!(
            "{}",
            format!("Failed to compile, exited with: {}", exit_code).bright_red()
        );
        return ExitCode::from(exit_code);
    }
    // return ExitCode::from(if exit_code == EXIT_SUCCES {
    //     0
    // } else {
    //     exit_code
    // });
}
