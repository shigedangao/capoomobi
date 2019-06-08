/**
 * Logger is a small decorator over println
 */
use colored::*;

/**
 * Log Type
 *    Enum referencing the available log level
 */
pub enum Log_type {
  Info,
  Error,
  Success,
  Warning
};

/**
 * Log
 *    Main entry point for logging a message
 */
pub fn write(level: Log_type, message: &'static str, rest: Option<String>) {

  match level {
    Info => println!("{}{}", "info:".blue(), message),
    Error => println!("{}{}", "error".red(), message),
    Success => println!("{}{}", "success".green(), message),
    Warning => println!("{}{}", "warning".yellow(), message)
  }

  match rest {
    Some(m) => println!("{}{:?}", "reason:".yellow(), m),
    None => None
  }
}
