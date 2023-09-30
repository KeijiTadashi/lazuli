// https://man.freebsd.org/cgi/man.cgi?query=sysexits&apropos=0&sektion=0&manpath=FreeBSD+11.2-stable&arch=default&format=html
// exitcodes for std::process::exit are unsigned 8 bit integers

// TODO give better name than global

use colored::Colorize;

pub const EXIT_SUCCES: u8 = 47; // see README for reasoning
pub struct LazuliCompileError<'a> {
    pub code: u8,
    pub name: &'a str,
}

// Standard(ish) error codes
pub const INVALID_ARGUMENT: LazuliCompileError = LazuliCompileError {
    code: 22,
    name: "Invalid argument",
};

// Custom Lazuli error codes
pub const MISSING_ARGUMENT: LazuliCompileError = LazuliCompileError {
    code: 1,
    name: "Missing argument",
}; //TODO: CHANGE TO NORMAL MISSING ARG CODE

pub const WEIRD_ERROR: LazuliCompileError = LazuliCompileError {
    code: 255,
    name: "Weird (or undefined) error",
};

pub fn print_error(error: Option<LazuliCompileError>, error_message: Option<String>) -> u8 {
    let e = error.unwrap_or(WEIRD_ERROR);
    eprintln!(
        "{}\n\t{}",
        format!("E{} - {}:", e.code.to_string(), e.name.to_string()).bright_red(),
        error_message
            .unwrap_or(String::from("No error message"))
            .replace('\n', "\n\t")
            .bright_white()
    );
    return e.code;
}

// pub const EXIT_INVALID_ARGUMENT: LazuliCompileError = {code: 22, name: String::from("INVALID ARGUMENT")};
