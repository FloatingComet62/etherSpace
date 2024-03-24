use colored::Colorize;
use std::{fs::{File, OpenOptions}, io::{prelude::*, ErrorKind}};

#[derive(PartialEq, PartialOrd)]
enum LogLevel {
    INFO = 0,
    WARN = 1,
    ERROR = 2
}

pub struct Log {
    log_level: LogLevel,
    output_file: Option<String>
}

impl Log {
    fn new() -> Self {
        Self {
            log_level: LogLevel::WARN,
            output_file: None
        }
    }
    pub fn info(message: &str) {
        let this = Self::new();
        if this.log_level > LogLevel::INFO {
            return;
        }
        let str = format!("[{}] {}", "Info".bold().green(), message);
        if this.output_file.is_some() {
            Log::to_file(this, &str);
            return;
        }
        println!("{}", str);
    }
    pub fn warn(message: &str) {
        let this = Self::new();
        let str = format!("[{}] {}", "Warn".bold().yellow(), message);
        if this.log_level > LogLevel::WARN {
            return;
        }
        if this.output_file.is_some() {
            Log::to_file(this, &str);
            return;
        }
        println!("{}", str);
    }
    pub fn critical(message: &str) -> ! {
        let this = Self::new();
        let str = format!("[{}] {}", "Error".bold().red(), message);
        if this.log_level > LogLevel::ERROR {
            std::process::exit(1);
        }
        if this.output_file.is_some() {
            Log::to_file(this, &str);
        } else {
            println!("{}", str);
        }
        std::process::exit(1);
    }
    pub fn critical_debug(file: &str, line: u32, message: &str) -> ! {
        let this = Self::new();
        let str = format!("[{}][{}:{}] {}", "Error".bold().red(), file, line, message);
        if this.log_level > LogLevel::ERROR {
            std::process::exit(1);
        }
        if this.output_file.is_some() {
            Log::to_file(this, &str);
        } else {
            println!("{}", str);
        }
        std::process::exit(1);
    }
    fn to_file(this: Self, data: &String) {
        let file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(this.output_file.clone().unwrap());
        match file {
            Err(e) => {
                if e.kind() == ErrorKind::NotFound {
                    match File::create(this.output_file.unwrap().clone()) {
                        Ok(mut f) => if let Err(e) = f.write(data.as_bytes()) {
                            println!("[ERROR] Failed to write to file\n{}", e);
                        },
                        Err(e) => println!("[ERROR] Failed to create the file\n{}", e),
                    }
                } else { println!("[ERROR] Failed to open the file\n{}", e) }
            },
            Ok(mut file) => if let Err(e) = writeln!(file, "{}", data) {
                println!("[ERROR] Failed to open the file\n{}", e)
            }
        }
    }
}

#[cfg(debug_assertions)]
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

#[cfg(not(debug_assertions))]
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
        crate::modules::log::Log::critical(file!(), line!(), &format!($($e),*));
    };
    (err object $obj: expr) => {
        crate::modules::log::Log::critical(
            &("\n".to_string() + &serde_yaml::to_string(&$obj).unwrap_or(format!("{:?}", $obj))),
        );
    };
}
