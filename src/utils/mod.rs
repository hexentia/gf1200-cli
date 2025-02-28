pub mod ui;

macro_rules! log {
    ($msg:tt) => {
        println!("[#] {}", $msg);
    };
    (fatal: $msg:tt) => {
        println!("{} {}", ansi_term::Colour::Red.paint("[fatal]"), $msg)
    };
    (hl: $msg:tt) => {
        println!("{} {}", ansi_term::Colour::Green.paint("[#]"), $msg)
    };
    (warn: $msg:tt) => {
        println!("{} {}", ansi_term::Colour::Yellow.paint("[!]"), $msg)
    };
}

pub(crate) use log;
