/// Parser module
/// 
/// # Description
/// Parser module is use to parse the action provided by the boostrap module
/// 
pub mod parser {
  use std::env;
  use crate::cli::scenarios::scenes::picker::{EnumHelper, Scenarios};
  use crate::cli::scenarios::sketch;
  use crate::errors::cli_error::{CliErr, ErrHelper, ErrCode};

  /// errors
  const UNKNOWN_SCENARIO_ERR: &str = "Scenario not found";

  /// default scenario to launch
  const DEFAULT_ACTION_SIZE: usize = 3;

  /// Arguments structure
  pub struct Arguments {
    pub main: String,
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
    let args = retrieve_args();

    if let Some(arg) = args {
      match Scenarios::from_string(&arg.main) {
        None => {
          return Err(
            CliErr::new(UNKNOWN_SCENARIO_ERR, String::new(),ErrCode::NotFound)    
          )
        },
        Some(res) => trigger_scenario(res, &arg.secondary, &arg.options)
      }

      return Ok(());
    }

    Err(CliErr::new(UNKNOWN_SCENARIO_ERR, String::new(),ErrCode::MissingFieldError))
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
  /// Return an `Arguments` option
  fn retrieve_args() -> Option<Arguments> {
    let actions: Vec<String> = env::args()
      .enumerate()
      .filter(|t| t.0 < DEFAULT_ACTION_SIZE)
      .map(|t| t.1)
      .collect();

    if actions.len() < DEFAULT_ACTION_SIZE {
      return None
    }

    let arguments: Vec<String> = env::args()
      .enumerate()
      .filter(|t| t.0 >= DEFAULT_ACTION_SIZE)
      .map(|t| t.1)
      .collect();
    
    // retrieve the actions
    let main = actions.get(1).unwrap();
    let secondary = actions.get(2).unwrap();

    let arg = Arguments {
      main: String::from(main),
      secondary: String::from(secondary),
      options: arguments
    };

    Some(arg)
  }
}
