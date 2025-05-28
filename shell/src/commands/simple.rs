use crossterm::cursor::MoveTo;
use std::path::PathBuf;

pub struct ClearCommand;

impl ClearCommand {
    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        use crossterm::{terminal, execute};
        use std::io::{stdout, Write};
        
        // Clear the screen using crossterm
        execute!(
            stdout(),
            terminal::Clear(terminal::ClearType::All),
            MoveTo(0, 0)
        )?;
        Ok(())
    }
}


pub struct PwdCommand;

impl PwdCommand {
    pub fn run(current_path: PathBuf) {
        println!("{}", current_path.display());
    }
}