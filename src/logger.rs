use colored::Colorize;

pub fn log_error(message: &str) {
    println!("{} {}", "[ERROR]".red(), message);
}
pub fn log_info(message: &str) {
    println!("{} {}", "[ INFO]".bright_blue(), message);
}
pub fn log_success(message: &str) {
    println!("{} {}", "[   OK]".green(), message);
}
pub fn log_warn(message: &str) {
    println!("{} {}", "[ WARN]".yellow(), message);
}