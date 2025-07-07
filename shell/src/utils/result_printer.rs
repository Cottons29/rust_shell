use crossterm::style::Stylize;

pub struct ResultPrinter;

impl ResultPrinter {
    pub fn error<T: std::fmt::Display>(error: T) {
        println!("{}", format!("{}", error).dark_red())
    }

    pub fn success<T: std::fmt::Display>(success: T)  {
        println!("{}", format!("{}", success))
    }
}
