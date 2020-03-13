/// Parser module
///
/// # Description
/// Parser module is use to parse the action provided by the boostrap module
use std::env;
use crate::cli::scenarios::scenes::picker::{Scenarios};
use crate::cli::scenarios::sketch;
use crate::core::logger::{log, LogType};

/// Constants
const DEFAULT_COMMAND_SIZE: usize = 1;

/// Trigger Scenario
///
/// # Description
/// Trigger the selected scenario
/// See: https://stackoverflow.com/a/40006220/7489243 in order to understand why it's better to pass a slice
///
/// # Arguments
/// * `scenario` Scenarios enum value
/// * `sub_action` &str
/// * `options` &[String]
fn trigger_scenario(scenario: Scenarios, sub_action: &str, options: &[String]) {
    if options.len() < 2 {
        sketch::help::launch(sub_action);
        return;
    }

    let opts = options.split_at(2);
    match scenario {
        Scenarios::Init     => sketch::init::launch(sub_action, opts.1),
        Scenarios::Help     => sketch::help::launch(sub_action),
        Scenarios::Generate => sketch::generate::launch(sub_action, opts.1),
        Scenarios::Project  => sketch::project::launch(sub_action, opts.1)
    }
}

/// Trigger command
///
/// # Description
/// Retrieve the arguments from the cli
///
/// # Return
/// Main action
/// Return an `Arguments` option
pub fn trigger_command() {
    let command: Vec<String> = env::args()
        .enumerate()
        .filter(|t| t.0 >= DEFAULT_COMMAND_SIZE)
        .map(|t| t.1)
        .collect();

    let action = match command.get(1) {
        Some(s) => &s,
        None => ""
    };

    let cmd: Vec<&str> = command.iter().map(|s| &**s).collect();
    match cmd.as_slice() {
        ["init", ..] => trigger_scenario(Scenarios::Init, action, &command),
        ["generate", ..] => trigger_scenario(Scenarios::Generate, action, &command),
        ["project", ..] => trigger_scenario(Scenarios::Project, action, &command),
        ["help", ..] => trigger_scenario(Scenarios::Help, action, &command),
        _ => log(LogType::Warning, format!("{}{}", "Command not found, name: ", command.get(0).unwrap()).as_str(), None)
    }
}
