use crate::utils::DebugPrint;
use std::env::args;
use std::path::PathBuf;
use std::sync::Mutex;
use colored::{Color, Colorize};
use figlet_rs::FIGfont;
use once_cell::sync::Lazy;
use crate::commands::CmdParser;
use crate::commands::simple::ClearCommand;
use crate::interpreter::Interpreter;
use crate::test::tester;
use crate::utils::{Input};

mod utils;
mod commands;
mod test;
mod interpreter;
mod var_map;

pub const DEBUG_MODE: Lazy<bool> = Lazy::new(|| args().any(|arg| arg == "--debug"));
pub static CURRENT_DIR: Lazy<Mutex<PathBuf>> = Lazy::new(|| Mutex::new(PathBuf::from("/Users/cottons/Desktop")));

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("args len : {} | args : {}", args().len(), args().collect::<Vec<_>>().join(" "));
    if args().any(|arg| arg == "--test"){
        tester()?;
        return Ok(());
    }
    dlog!("Starting in debug mode");
    shell_mode()?;
    Ok(())
}

fn greet(){
    // Load thicc block font from file
    let font = FIGfont::standard().unwrap();
    let figure = font.convert("COTSH").unwrap();

    // Rainbow gradient 🌈
    let gradient = [
        Color::Green,
        Color::Yellow,
        Color::Red,
        Color::Magenta,
        Color::Blue,
    ];

    let messages = vec![
        String::new(),
        format!("Welcome to {}!", "Cotsh".blue().bold()),
        format!("Base on the {}", "GNU Bash".blue().bold()),
        format!("all the built-in cmd are written in {} 🦀", "Rust".red().bold())
    ];

    let ascii = figure.to_string(); // owns the data
    let lines: Vec<&str> = ascii.lines().collect(); // borrow from owned string
    let total = lines.len();
    println!("\n");
    for (i, line) in lines.iter().enumerate() {
        let color_index = i * gradient.len() / total;
        println!(" {}     {}", line.color(gradient[color_index]), messages.get(i).unwrap_or(&"".to_string()));
    }
    let current_os_name = std::env::consts::OS;
    println!("\nCurrent Version is ✨{} ({})", "Nightly-0.1.0".green(), current_os_name.green());
    println!("Our {} repository is at {}", "Github".green(), "https://github.com/Cottons29/rust_shell".green());
    println!("Documentation? i am not have it yet -_-\n");

}




fn shell_mode() -> Result<(), Box<dyn std::error::Error>> {
    ClearCommand::run()?;
    greet();
    loop{
        let input = {
            let current_dir = CURRENT_DIR.lock().unwrap();
            read_line!("{} -> ", &current_dir.to_string_lossy())
        };
        if input.is_empty(){
            continue;
        }
        CmdParser::new(&input)?.execute_cmd()?;
    }
}

fn interpret_mode(script_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    match Interpreter::new(script_path)?.interpret(){
        Ok(_) => {}
        Err(e) => {}
    }
    Ok(())
}