/**
 * Logger is a small decorator over println
 */
pub mod logging {
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

  pub enum HelpLogType {
    Cmd,
    Description,
    Example
  }

  /**
   * Log
   *    Main entry point for logging a message
   */
  pub fn write(level: LogType, message: &str, rest: Option<String>) {

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

  pub fn write_help(level: HelpLogType, message: &'static str) {
    match level {
      HelpLogType::Cmd => println!("{}{}", "\nNAME: \n\n".blue().bold(), message),
      HelpLogType::Description => println!("{}{}", "\nDESCRIPTION: \n\n".blue().bold(), message),
      HelpLogType::Example => println!("{}{}", "\nEXAMPLE: \n\n".green().bold(), message)
    }
  }
}