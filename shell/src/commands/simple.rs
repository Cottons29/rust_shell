use crossterm::cursor::MoveTo;

pub struct ClearCommand;

impl ClearCommand {
    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        use crossterm::{execute, terminal};
        use std::io::stdout;

        // Clear the screen using crossterm
        execute!(
            stdout(),
            terminal::Clear(terminal::ClearType::All),
            MoveTo(0, 0)
        )?;
        Ok(())
    }
}

