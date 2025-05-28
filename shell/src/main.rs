use std::env::args;
use once_cell::unsync::Lazy;
use crate::commands::CmdParser;
use crate::utils::{DebugTool, Input, WordSplitter};

mod utils;
mod commands;

pub const DEBUG_MODE: Lazy<bool> = Lazy::new(|| args().any(|arg| arg == "--debug"));


fn main() -> Result<(), Box<dyn std::error::Error>> {
    DebugTool::print("is launching...");
    let mut cmd_parser = CmdParser::default();
    loop{
        let input = String::read_line("$ ");
        let words = input.split_words_by_space()?;
        if words.is_empty(){
            continue;
        }
        if words.first().unwrap() == "exit"{
            match words.len() {
                1 => {
                    commands::exit(0)
                },
                2 => {
                    let code = words[1].parse::<i32>()?;
                    commands::exit(code)
                }
                _ => commands::exit(0),
            }
        }
        cmd_parser = CmdParser::new(input, Some(cmd_parser.get_current_dir().into()))?.execute_cmd()?
    }
}