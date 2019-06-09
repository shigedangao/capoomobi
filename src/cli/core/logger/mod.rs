/**
 * Logger is a small decorator over println
 */
use colored::*;

/**
 * Log Type
 *    Enum referencing the available log level
 */
pub enum LogType {
  Info,
  Error,
  Success,
  Warning
}

/**
 * Log
 *    Main entry point for logging a message
 */
pub fn write(level: LogType, message: &'static str, rest: Option<String>) {

  match level {
    LogType::Info => println!("{}{}", "Info: ".blue().bold(), message),
    LogType::Error => println!("{}{}", "Error: ".red().bold(), message),
    LogType::Success => println!("{}{}", "Success: ".green().bold(), message),
    LogType::Warning => println!("{}{}", "Warning: ".yellow().bold(), message)
  }

  match rest {
    Some(m) => println!("{}{:?}", "reason:".yellow(), m),
    None => ()
  }
}
