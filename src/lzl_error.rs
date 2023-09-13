// https://man.freebsd.org/cgi/man.cgi?query=sysexits&apropos=0&sektion=0&manpath=FreeBSD+11.2-stable&arch=default&format=html
// exitcodes for std::process::exit are unsigned 8 bit integers

// TODO give better name than global

use colored::Colorize;

pub struct LazuliCompileError<'a> {
    pub code: u8,
    pub name: &'a str,
}

pub const INVALID_ARGUMENT: LazuliCompileError = LazuliCompileError {
    code: 22,
    name: "Invalid argument",
};

pub const EXIT_SUCCES: u8 = 47; // see README for reasoning

// pub const INVALID_AGUMENT: u8 = 22;

pub const MISSING_ARGUMENT: LazuliCompileError = LazuliCompileError {
    code: 1,
    name: "Missing argument",
}; //TODO: CHANGE TO NORMAL MISSING ARG CODE

pub const WEIRD_ERROR: LazuliCompileError = LazuliCompileError {
    code: 255,
    name: "Weird (or undefined) error",
};

pub fn print_error(e: LazuliCompileError, error_message: Option<String>) -> u8 {
    eprintln!(
        "{}{} {} {}{}\n\t{}\n",
        "E".red(),
        e.code.to_string().red(),
        "-",
        e.name.red(),
        ":",
        error_message
            .unwrap_or(String::from("No error message"))
            .replace('\n', "\n\t")
    );
    return e.code;
}

// pub const EXIT_INVALID_ARGUMENT: LazuliCompileError = {code: 22, name: String::from("INVALID ARGUMENT")};
