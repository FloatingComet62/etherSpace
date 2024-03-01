use colored::Colorize;

pub struct Log {}

impl Log {
    pub fn info(message: &str) {
        println!("[{}] {}", "Info".bold().green(), message);
    }
    pub fn warn(message: &str) {
        println!("[{}] {}", "Warn".bold().yellow(), message);
    }
    pub fn critical(message: &str) -> ! {
        println!("[{}] {}", "Error".bold().red(), message);
        std::process::exit(1);
    }
    pub fn critical_debug(file: &str, line: u32, message: &str) -> ! {
        println!("[{}][{}:{}] {}", "Error".bold().red(), file, line, message);
        std::process::exit(1);
    }
}

#[macro_export]
macro_rules! info {
    ($e: expr) => {
        Log::info($e);
    };
    (object $obj: expr) => {
        Log::info(
            &("\n".to_string() + &serde_yaml::to_string(&$obj).unwrap_or(format!("{:?}", $obj))),
        );
    };
}
#[macro_export]
macro_rules! warn {
    ($($e: expr),*) => {
        Log::warn(&format!($($e),*));
    };
}
#[macro_export]
macro_rules! critical {
    ($($e: expr),*) => {
        Log::critical_debug(file!(), line!(), &format!($($e),*));
    };
}
