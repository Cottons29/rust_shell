use crate::utils::DebugTool;

type Output = Result<Vec<String>, Box<dyn std::error::Error>>;

pub(crate) trait WordSplitter{
    fn split_words_by_space(&self) -> Output;
    fn split_quote(&self) -> Vec<String>;
    fn split_double_quote(&self) -> Vec<String>;
    fn split_path(&self) -> Output;
    fn advance_split(&self) -> Vec<String>;

}


impl WordSplitter for String{
    fn split_words_by_space(&self) -> Output{
        let words: Vec<&str> = self.split_whitespace().collect();
        let result: Vec<String> = words.iter().map(|s| s.to_string()).collect();
        Ok(result)
    }

    fn split_quote(&self) -> Vec<String> {
        let input = self.trim();
        let mut words: Vec<String> = Vec::new();
        let mut chars = input.chars().peekable();
        
        while let Some(ch) = chars.next() {
            if ch == '\'' {
                // Start of a quoted section
                let mut word = String::new();
                let mut escaped = false;
                
                while let Some(inner_ch) = chars.next() {
                    if escaped {
                        // Handle escape sequences
                        match inner_ch {
                            '\'' => word.push('\''),  // Escaped quote
                            '\\' => word.push('\\'),  // Escaped backslash
                            'n' => word.push('\n'),   // Newline
                            't' => word.push('\t'),   // Tab
                            'r' => word.push('\r'),   // Carriage return
                            _ => {
                                // For any other character, include the backslash and the character
                                word.push('\\');
                                word.push(inner_ch);
                            }
                        }
                        escaped = false;
                    } else if inner_ch == '\\' {
                        escaped = true;
                    } else if inner_ch == '\'' {
                        // End of quoted section
                        words.push(word);
                        word = String::new();
                        break;
                    } else {
                        // Regular character inside quotes - accept any character
                        word.push(inner_ch);
                    }
                }
                
                // Handle unclosed quote - if we reach here with content, the quote wasn't closed
                if escaped || chars.peek().is_none() {
                    if !word.is_empty() {
                        words.push(word);
                    }
                }
            }
            // Skip characters outside of quotes (including whitespace)
        }
        
        words
    }

    fn split_double_quote(&self) -> Vec<String> {
        todo!()
    }

    fn split_path(&self) -> Output {
        let temp: Vec<String> = self.trim().split('/').map(|s| s.to_string()).collect();
        Ok(temp)
    }

    fn advance_split(&self) -> Vec<String> {
        let temp = self.trim();
        let mut words: Vec<String> = Vec::new();
        let mut word = String::new();
        let mut in_quote = false;
        let mut quote_char = '\0';
        let mut escaped = false;

        for ch in temp.chars() {
            match ch {
                '\\' if !escaped => {
                    escaped = true;
                    word.push(ch);
                }
                ' ' if !in_quote && !escaped => {
                    if !word.is_empty() {
                        words.push(word.trim().to_string());
                        word = String::new();
                    }
                }
                '\'' | '"' if !escaped => {
                    if !in_quote {
                        // Starting a quote
                        in_quote = true;
                        quote_char = ch;
                        word.push(ch);
                    } else if ch == quote_char {
                        // Ending the same type of quote
                        in_quote = false;
                        word.push(ch);
                        words.push(word.trim().to_string());
                        word = String::new();
                        quote_char = '\0';
                    } else {
                        // Different quote type inside quotes
                        word.push(ch);
                    }
                }
                _ => {
                    escaped = false;
                    word.push(ch);
                }
            }
        
            // Reset escaped flag after processing any character except backslash
            if ch != '\\' || escaped {
                escaped = false;
            }
        }
    
        // Push any remaining word
        if !word.is_empty() {
            words.push(word.trim().to_string());
        }
    
        words
    }
}