/// Parser module
/// 
/// # Description
/// Parser module is use to parse the action provided by the boostrap module
/// 
pub mod parser {
    use std::env;
    use crate::cli::scenarios::scenes::picker::{EnumHelper, Scenarios};
    use crate::cli::scenarios::sketch;
    use crate::errors::cli_error::{CliErr, ErrHelper, ErrMessage};

    /// errors
    const UNKNOWN_SCENARIO_ERR: &str = "Command not found / supported";
    const UNKNOWN_ACTION_ERR: &str = "Missing arguments for command";

    /// Constants
    const DEFAULT_ACTION_SIZE: usize = 3;
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
                return Err(
                    CliErr::new(
                        UNKNOWN_SCENARIO_ERR,
                        format!(
                            "{} {:?} {}",
                            "command not supported",
                            cmd,
                            "Check the `help` command in order to understand how to use the CLI"
                        ),
                        ErrMessage::NotFound
                    )
                )
            },
            Some(res) => {
                if let Some(arg) = args {
                    trigger_scenario(res, &arg.secondary, &arg.options);
                    return Ok(());
                } else {
                    match res {
                        Scenarios::Help => {
                            trigger_scenario(res, &String::new() , &vec![]);
                            return Ok(());
                        },
                        _ => {}
                    };
                }
                
                Err(
                    CliErr::new(
                        UNKNOWN_ACTION_ERR,
                        format!("{}{:?}", "Command: ", cmd),
                        ErrMessage::MissingFieldError
                    )
                )
            }
        }
    }

    /// Trigger Scenario
    /// 
    /// # Description
    /// Trigger the selected scenario
    /// 
    /// # Arguments
    /// * `scenario` Scenarios enum value
    /// * `sub_action` Reference to a String struct
    /// * `options` Reference to a vec of String
    fn trigger_scenario(scenario: Scenarios, sub_action: &String, options: &Vec<String>) {
        match scenario {
            Scenarios::Init => sketch::init::launch(sub_action, options),
            Scenarios::Help => sketch::help::launch(sub_action),
            Scenarios::Generate => sketch::generate::launch(sub_action, options),
            Scenarios::Project => sketch::project::launch(sub_action, options)
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
}
