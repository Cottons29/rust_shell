use crossterm::style::Stylize;
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



pub fn tester() -> Result<(), Box<dyn std::error::Error>> {

    println!("{}", "\nTesting env is entering\n".red().bold());

    let input = "1 + 2 * 3 - (4 / 2)";
    let tokens = tokenize(input);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_expression(0);
    println!("AST: {:#?}", ast);
    println!("Result: {}", eval(&ast));

    Ok(())
}