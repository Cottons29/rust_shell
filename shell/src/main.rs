use crate::utils::DebugPrint;
use std::env::args;
use std::path::PathBuf;
use once_cell::unsync::Lazy;
use crate::commands::CmdParser;
use crate::interpreter::Interpreter;
use crate::utils::{Input, WordSplitter};

mod utils;
mod commands;
mod test;
mod interpreter;
mod var_map;

pub const DEBUG_MODE: Lazy<bool> = Lazy::new(|| args().any(|arg| arg == "--debug"));


fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("args len : {} | args : {}", args().len(), args().collect::<Vec<_>>().join(" "));
    if args().len() == 1 {
        shell_mode()
    }else{
        interpret_mode(args().nth(1).unwrap().into())
    }
}

fn shell_mode() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd_parser = CmdParser::default();
    loop{
        let input = read_line!("~{} -> ", cmd_parser.get_current_dir().to_string_lossy());
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
        cmd_parser = CmdParser::new(&input, Some(cmd_parser.get_current_dir().into()))?.execute_cmd(false)?
    }
}

fn interpret_mode(script_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    match Interpreter::new(script_path)?.interpret(){
        Ok(_) => {}
        Err(e) => {}
    }
    Ok(())
}