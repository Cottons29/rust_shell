use std::io::{self, Write};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{self, ClearType},
    cursor, execute, 
    style::Print,
};
use crossterm::style::Stylize;

pub trait Input {
    fn read_line(msg: String) -> String;
}

impl Input for String {
    fn read_line(msg: String) -> String {
        let mut input = String::new();
        let mut cursor_position = 0;
        let msg_len = msg.len();
        
        // Print the prompt
        execute!(io::stdout(), Print(msg.bold().green())).unwrap();
        io::stdout().flush().unwrap();
        
        // Enable raw mode to read characters one by one
        terminal::enable_raw_mode().unwrap();
        
        loop {
            // Read a key event
            if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read().unwrap() {
                match (code, modifiers) {
                    // Handle Enter key - finish input
                    (KeyCode::Enter, _) => {
                        execute!(io::stdout(), Print("\r\n")).unwrap();
                        break;
                    },
                    
                    // Handle Ctrl+C - exit
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                        terminal::disable_raw_mode().unwrap();
                        std::process::exit(0);
                    },
                    
                    // Handle Backspace - delete a character
                    (KeyCode::Backspace, _) => {
                        if cursor_position > 0 {
                            input.remove(cursor_position - 1);
                            cursor_position -= 1;
                            
                            // Redraw the line
                            execute!(
                                io::stdout(),
                                cursor::MoveToColumn(msg_len as u16),
                                terminal::Clear(ClearType::FromCursorDown),
                                Print(&input),
                                cursor::MoveToColumn((msg_len + cursor_position) as u16)
                            ).unwrap();
                        }
                    },
                    
                    // Handle Left arrow - move cursor left
                    (KeyCode::Left, _) => {
                        if cursor_position > 0 {
                            cursor_position -= 1;
                            execute!(
                                io::stdout(),
                                cursor::MoveLeft(1)
                            ).unwrap();
                        }
                    },
                    
                    // Handle Right arrow - move cursor right
                    (KeyCode::Right, _) => {
                        if cursor_position < input.len() {
                            cursor_position += 1;
                            execute!(
                                io::stdout(),
                                cursor::MoveRight(1)
                            ).unwrap();
                        }
                    },
                    
                    // Handle regular character input
                    (KeyCode::Char(c), _) => {
                        input.insert(cursor_position, c);
                        cursor_position += 1;
                        
                        // Redraw the line
                        execute!(
                            io::stdout(),
                            cursor::MoveToColumn(msg_len as u16),
                            terminal::Clear(ClearType::FromCursorDown),
                            Print(&input),
                            cursor::MoveToColumn((msg_len + cursor_position) as u16)
                        ).unwrap();
                    },
                    
                    // Ignore other keys
                    _ => {}
                }
            }
        }
        
        // Disable raw mode when done
        terminal::disable_raw_mode().unwrap();
        
        // If input is empty and we hit enter, we don't want to add a newline character
        // as that might cause issues with command parsing
        if input.is_empty() {
            input = String::new();
        }
        
        input
    }
}
