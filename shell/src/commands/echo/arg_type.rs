use std::fmt::Display;
use crate::utils::WordSplitter;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum EchoArg {
    Plain(String),
    Flag(String),
    QuoteTxt(String),
    DoubleQuoteTxt(String),
    WriteInto(String),
    AppendInto(String)
}

impl Display for EchoArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl EchoArg {

    pub fn new(text: &str)-> Self{
        use EchoArg::*;
        match text{
            text if text.starts_with('-') => Flag(text.to_string()),
            text if text.starts_with('"') && text.ends_with('"') => DoubleQuoteTxt(text.to_string()),
            text if text.starts_with('\'') && text.ends_with('\'') => QuoteTxt(text.to_string()),
            text if text.eq(">") => WriteInto(text.to_string()),
            text if text.eq(">>") => AppendInto(text.to_string()),
            _ => Plain(text.to_string())
        }
    }

    pub fn value(&self)-> String{
        use EchoArg::*;
        match self{
            Plain(text) => text.into(),
            Flag(text) => text.into(),
            QuoteTxt(text) =>  text.split_quote().join("").into(),
            DoubleQuoteTxt(text) => text.split_double_quote().join("").into(),
            WriteInto(text) => text.into(),
            AppendInto(text) => text.into()
        }
    }
}
