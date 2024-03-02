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
macro_rules! log {
    (info $($e: expr),*) => {
        ether_space::modules::log::Log::info(&format!($($e),*));
    };
    (info object $obj: expr) => {
        ether_space::modules::log::Log::info(
            &("\n".to_string() + &serde_yaml::to_string(&$obj).unwrap_or(format!("{:?}", $obj))),
        );
    };
    (warn $($e: expr),*) => {
        ether_space::modules::log::Log::warn(&format!($($e),*));
    };
    (warn object $obj: expr) => {
        crate::modules::log::Log::warn(
            &("\n".to_string() + &serde_yaml::to_string(&$obj).unwrap_or(format!("{:?}", $obj))),
        );
    };
    (err $($e: expr),*) => {
        crate::modules::log::Log::critical_debug(file!(), line!(), &format!($($e),*));
    };
    (err object $obj: expr) => {
        crate::modules::log::Log::critical_debug(
            &("\n".to_string() + &serde_yaml::to_string(&$obj).unwrap_or(format!("{:?}", $obj))),
        );
    };
}
