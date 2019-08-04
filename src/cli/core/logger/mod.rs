/// Logging
/// 
/// # Description
/// Logging module use to wrap println
pub mod logging {
  use colored::*;
  
  /// LogType enum
  pub enum LogType {
    Info,
    Error,
    Success,
    Warning
  }

  /// HelpLogType
  /// 
  /// # Description
  /// Enum use by the `help` command
  pub enum HelpLogType {
    Cmd,
    Description,
    Example
  }

  /// Log
  /// 
  /// # Arguments
  /// * `level` LogType
  /// * `message` slice of string
  /// * `rest` Option<String>
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

  /// Write Help
  /// 
  /// # Description
  /// Execute a println for each kind of HelpLog
  pub fn write_help(level: HelpLogType, message: &'static str) {
    match level {
      HelpLogType::Cmd => println!("{}{}", "\nNAME: \n\n".blue().bold(), message),
      HelpLogType::Description => println!("{}{}", "\nDESCRIPTION: \n\n".blue().bold(), message),
      HelpLogType::Example => println!("{}{}", "\nEXAMPLE: \n\n".green().bold(), message)
    }
  }
}