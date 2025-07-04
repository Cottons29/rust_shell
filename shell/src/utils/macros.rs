#[macro_export]
macro_rules! print_error {
    ($($arg:tt)*) => {
        $crate::utils::ResultPrinter::error(format!($($arg)*))
    };
}

#[macro_export]
macro_rules! print_success {
    ($($arg:tt)*) => {
        crate::utils::ResultPrinter::success(format!($($arg)*))
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

#[macro_export]
macro_rules! read_line {
    ($($msg:tt)*) => {
        String::read_line(&format!($($msg)*))
    };
}