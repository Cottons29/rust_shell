use std::fmt::format;
use std::path::PathBuf;
use colored::{Color, Colorize};
use crossterm::style::Stylize;
use figlet_rs::FIGfont;
use crate::DebugPrint;
use crate::DEBUG_MODE;
use crate::dlog;
use crate::interpreter::{eval, tokenize, Parser};
use crate::test::table_printer::tabel_tester_2;

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use crate::utils::WordSplitter;

    #[test]
    fn single_quote_with_escaped_quote() {
        let temp = "'test\\'s test'".to_string();
        let words = temp.split_quote();
        assert_eq!(words, vec!["test's test"]);
    }

    #[test]
    fn single_quote_empty() {
        let temp = "''".to_string();
        let words = temp.split_quote();
        assert_eq!(words, vec![""]);
    }

    #[test]
    fn single_quote_multiple_words() {
        let temp = "'hello world' 'test case'".to_string();
        let words = temp.split_quote();
        // Note: Current implementation doesn't handle multiple quoted sections
        // This test shows expected behavior vs actual behavior
        assert_eq!(words, vec!["hello world", "test case"]);
    }

    #[test]
    fn single_quote_with_spaces() {
        let temp = "'hello world with spaces'".to_string();
        let words = temp.split_quote();
        assert_eq!(words, vec!["hello world with spaces"]);
    }

    #[test]
    fn single_quote_escaped_backslash() {
        let temp = "'test\\\\path'".to_string();
        let words = temp.split_quote();
        // This test reveals how the current implementation handles escaped backslashes
        assert_eq!(words, vec!["test\\path"]);
    }

    #[test]
    fn single_quote_no_quotes() {
        let temp = "regular text".to_string();
        let words = temp.split_quote();
        // Current implementation doesn't handle text without quotes well
        assert_eq!(words.len(), 0);
    }

    #[test]
    fn single_quote_unclosed() {
        let temp = "'unclosed quote".to_string();
        let words = temp.split_quote();
        // Test behavior with unclosed quotes
        assert_eq!(words, vec!["unclosed quote"])
    }
    
    #[test]
    fn advance_quote() {
        let temp = "'test\\'s test' test2".to_string();
        let splitter = temp.advance_split();
        assert_eq!(splitter, vec!["'test\\'s test'", "test2"])
    }
}

fn greet(font: &str){
    // Load thicc block font from file
    let font = FIGfont::from_file("figlet-fonts/Sub-Zero.flf")
        .unwrap_or_else(|_| FIGfont::standard().unwrap());
    let figure = font.convert("COTSH").unwrap();

    // Rainbow gradient 🌈
    let gradient = [
        Color::Green,
        Color::Yellow,
        Color::Red,
        Color::Magenta,
        Color::Blue,
    ];

    let ascii = figure.to_string(); // owns the data
    let lines: Vec<&str> = ascii.lines().collect(); // borrow from owned string
    let total = lines.len();

    for (i, line) in lines.iter().enumerate() {
        let color_index = i * gradient.len() / total;
        println!("{}", line.color(gradient[color_index]));
    }
}



pub fn tester() -> Result<(), Box<dyn std::error::Error>> {

    let font_path = PathBuf::from("/Users/cottons/Documents/rust_shell/figlet-fonts");

    let fonts = match font_path.read_dir(){
        Ok(fonts) => fonts,
        Err(e) => {
            eprintln!("Error reading fonts: {}", e);
            return Ok(())
        }
    };

    for font in fonts {
        match font {
            Ok(font) => {
                if font.file_name().to_str().unwrap_or_else(Default::default).contains(".flf"){
                    println!("font: {}", font.file_name().to_str().unwrap());
                    greet(font.file_name().to_str().unwrap());
                }

            },
            Err(e) => {
                eprintln!("Error reading font: {}", e);
            }
        }
    }


    Ok(())
}