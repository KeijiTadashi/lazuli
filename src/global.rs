use colored::Colorize;
use once_cell::sync::OnceCell;
use std::time::{Duration, Instant};

// new global file after renaming the previous one to lzl_error... oh well
pub static mut DEBUG: bool = false;
pub static mut DEBUGALL: bool = false;
static START_TIME: OnceCell<Instant> = OnceCell::new();
static mut TIME_SINCE_LAST: Duration = Duration::MAX;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum OutputFileType {
    EXE,
    ASM,
    OBJ,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum DebugType {
    CREATE,
    REMOVE,
    MESSAGE, //change to a better name
    NONE,
}

pub fn printd(s: String, t: DebugType) {
    let total_time = START_TIME.get_or_init(|| Instant::now()).elapsed();
    let step_time = total_time
        .checked_sub(unsafe { TIME_SINCE_LAST })
        .unwrap_or_else(|| Duration::ZERO);
    unsafe { TIME_SINCE_LAST = total_time }

    let msg = format!(
        "[{:>8} | {:<7}] {}",
        format!("{:.1?}", step_time),
        format!("{:.3}s", total_time.as_secs_f64()),
        s
    );
    if t == DebugType::MESSAGE {
        println!("{}", msg.bright_blue());
    } else if t == DebugType::NONE && unsafe { DEBUGALL } {
        println!("{}", msg.bright_white());
    } else if unsafe { DEBUG } {
        if t == DebugType::CREATE {
            println!("{}", msg.bright_green())
        } else if t == DebugType::REMOVE {
            println!("{}", msg.bright_purple())
        }
    }
}

// pub fn printd(s: String) {
//     if unsafe { DEBUG } {
//         println!("{}", s);
//     }
// }
// #[macro_export]
// macro_rules! log {
//     ($($args: tt)*) => {
//         let s = format!($($args)*);
//         println!("a");
//     }
// }

// #[macro_export]
// macro_rules! printd {
//     if DEBUG
//     {
//     () => { ... };
//     ($($arg:tt)*) => { .. };
//     }
// }
