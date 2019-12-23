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
    Example,
    Action
}

/// Log Error
/// 
/// # Description
/// Decorator around the log method in order to display the error code properly
/// 
/// # Arguments
/// * `level` LogType
/// * `message` slice of string
/// * `code` ErrMessage
/// * `rest` Option<String>
pub fn log_error(level: LogType, message: &str, code: &str, rest: Option<String>) {
    log(level, message, rest);
    println!("{} {}", "Error message:".yellow().bold().underline(), code);
}

/// Log
/// 
/// # Arguments
/// * `level` LogType
/// * `message` slice of string
/// * `rest` Option<String>
pub fn log(level: LogType, message: &str, rest: Option<String>) {
    let colored_mess = match level {
        LogType::Info => format!("{}: {}", "Info".blue().bold(), message.blue()),
        LogType::Error => format!("{}: {}", "Error".red().bold(), message.red()),
        LogType::Success => format!("{}: {}", "Success".green().bold(), message.green()),
        LogType::Warning => format!("{}: {}", "Warning".yellow().bold(), message.yellow())
    };

    println!("{}", colored_mess);
    if let Some(r) = rest {
        println!("{} {}", "Details:".yellow().bold(), r)
    }
}

/// Log Help
/// 
/// # Description
/// Execute a println for each kind of HelpLog
/// 
/// # Arguments
/// * `level` HelpLogType
/// * `message` String
pub fn log_help(level: HelpLogType, message: String) {
    match level {
        HelpLogType::Cmd => println!("{}{}", "\nName: \n\n".blue().bold(), message),
        HelpLogType::Description => println!("{}{}", "\nDescription: \n\n".blue().bold(), message),
        HelpLogType::Action => println!("{}{}", "\nAction: \n\n".green().bold(), message),
        HelpLogType::Example => println!("{}{}", "\nExample: \n\n".green().bold(), message)
    }
}