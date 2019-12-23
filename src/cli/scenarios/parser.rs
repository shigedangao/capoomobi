/// Parser module
/// 
/// # Description
/// Parser module is use to parse the action provided by the boostrap module
use std::env;
use crate::cli::scenarios::scenes::picker::{EnumHelper, Scenarios};
use crate::cli::scenarios::sketch;
use crate::core::errors::cli_error::{CliErr, ErrHelper, ErrMessage};
use crate::core::errors::message::cli::{
    UNKNOWN_ACTION,
    UNKNOWN_SCENARIO
};

/// Message
const HELP_MESSAGE: &str = "Check the `help` command in order to understand how to use the CLI";

/// Constants
const DEFAULT_ACTION_SIZE:  usize = 3;
const DEFAULT_COMMAND_SIZE: usize = 2;

/// Arguments structure
pub struct Arguments {
    pub secondary: String,
    pub options: Vec<String>
}

/// Parse
/// 
/// # Description
/// Parse arguments provided by the bootstrap method
/// 
/// # Return
/// Return a result
pub fn parse() -> Result<(), CliErr> {
    let (cmd, args) = retrieve_args();
    match Scenarios::from_string(&cmd) {
        None => {
            let msg = format!("{} {:?} {}", "command not supported", cmd, HELP_MESSAGE);
            Err(CliErr::new(UNKNOWN_SCENARIO, msg.as_str(), ErrMessage::NotFound))
        },
        Some(res) => {
            if let Some(arg) = args {
                trigger_scenario(res, &arg.secondary, &arg.options);
                return Ok(());
            }

            if args.is_none() {
                trigger_scenario(Scenarios::Help, &String::new() , &vec![]);
                return Ok(());
            }
            
            Err(CliErr::new(UNKNOWN_ACTION, format!("{}{:?}", "Command: ", cmd).as_str(), ErrMessage::MissingFieldError))
        }
    }
}

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
    match scenario {
        Scenarios::Init     => sketch::init::launch(sub_action, options),
        Scenarios::Help     => sketch::help::launch(sub_action),
        Scenarios::Generate => sketch::generate::launch(sub_action, options),
        Scenarios::Project  => sketch::project::launch(sub_action, options)
    }
}

/// Retrieve args
/// 
/// # Description
/// Retrieve the arguments from the cli
/// 
/// # Return
/// Main action
/// Return an `Arguments` option
fn retrieve_args() -> (String, Option<Arguments>) {
    let actions: Vec<String> = env::args()
        .enumerate()
        .filter(|t| t.0 <= DEFAULT_COMMAND_SIZE)
        .map(|t| t.1)
        .collect();

    let main = match actions.get(1) {
        Some(m) => String::from(m),
        None => String::from("help")
    };

    if actions.len() < DEFAULT_ACTION_SIZE {
        return (main, None);
    }

    let arguments: Vec<String> = env::args()
        .enumerate()
        .filter(|t| t.0 >= DEFAULT_ACTION_SIZE)
        .map(|t| t.1)
        .collect();
    
    // retrieve the actions
    let secondary = actions.get(2).unwrap();
    let arg = Arguments {
        secondary: String::from(secondary),
        options: arguments
    };

    (main, Some(arg))
}