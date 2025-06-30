#[macro_export]
macro_rules! print_error {
    ($($arg:tt)*) => {
        $crate::commands::parse_cmd::ResultPrinter::error(format!($($arg)*))
    };
}

#[macro_export]
macro_rules! print_success {
    ($($arg:tt)*) => {
        crate::commands::parse_cmd::ResultPrinter::success(format!($($arg)*))
    };
}

#[macro_export]
macro_rules! dlog {
    ($($arg:tt)*) => {
        if *DEBUG_MODE{
            let value = format!($($arg)*);
            DebugPrint::print(value);
        }
    }
    
}