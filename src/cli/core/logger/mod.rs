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

/// Log
/// 
/// # Arguments
/// * `level` LogType
/// * `message` slice of string
/// * `rest` Option<String>
pub fn log(level: LogType, message: &str, rest: Option<String>) {
    let colored_mess = match level {
        LogType::Info => message.blue().bold(),
        LogType::Error => message.red().bold(),
        LogType::Success => message.green().bold(),
        LogType::Warning => message.yellow().bold()
    };

    println!("{}", colored_mess);
    match rest {
        Some(m) => println!("{}{:?}", "result:".yellow(), m),
        None => ()
    }
}

/// Log Help
/// 
/// # Description
/// Execute a println for each kind of HelpLog
pub fn log_help(level: HelpLogType, message: &'static str) {
    match level {
        HelpLogType::Cmd => println!("{}{}", "\nName: \n\n".blue().bold(), message),
        HelpLogType::Description => println!("{}{}", "\nDescription: \n\n".blue().bold(), message),
        HelpLogType::Action => println!("{}{}", "\nAction: \n\n".green().bold(), message),
        HelpLogType::Example => println!("{}{}", "\nExample: \n\n".green().bold(), message)
    }
}