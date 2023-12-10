use colored::Colorize;

pub struct Log {}

impl Log {
    pub fn info(message: &str) {
        println!("[{}] {}", "Info".bold().green(), message);
    }
    pub fn warn(message: &str) {
        println!("[{}] {}", "Warn".bold().yellow(), message);
    }
    pub fn critical(message: &str) {
        println!("[{}] {}", "Error".bold().red(), message);
    }
}
