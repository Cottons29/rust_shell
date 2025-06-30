use colored::Colorize;

use crate::DEBUG_MODE;

pub struct DebugPrint;

impl DebugPrint {
    pub fn print<T: std::fmt::Display>(value: T) -> () {
        if !*DEBUG_MODE{
            return ();
        }
        let value = format!("{}", value).bright_green();
        let debug = "Debug: ".bright_green().bold();
        println!("{}{}", debug, value);
    }
}

