use crossterm::style::Stylize;
use crate::utils::DebugTool;

pub struct ResultPrinter;

impl ResultPrinter {
    pub fn error<T>(error : T)  where T : std::fmt::Display{
        // DebugTool::print("ResultPrinter::error");
        println!("{}", format!("{}", error).red())
    }
}