/// Parser module
/// 
/// # Description
/// Parser module is use to parse the action provided by the boostrap module
/// 
pub mod parser {
  use std::env;
  use crate::cli::scenarios::scenes::picker::{EnumHelper, Scenarios};
  use crate::cli::scenarios::sketch;
  use crate::cli::core::logger::logging;

  /// errors
  const UNKNOWN_SCENARIO_ERR: &str = "Scenario not found";

  /// default scenario to launch
  const DEFAULT_SCENARIO: &str = "help";

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
  /// # Arguments
  /// * `main`: &str
  /// * `secondary`: &str
  /// * `options`: Vec<String>
  pub fn parse(main: &str, secondary: &str, options: Vec<String>) {
    let args = retrieve_args();

    match Scenarios::from_string(&args.main) {
      None => logging::write(logging::LogType::Error, UNKNOWN_SCENARIO_ERR, None),
      Some(res) => trigger_scenario(res, &args.secondary, &args.options)
    }
  }

  /// Trigger Scenario
  /// 
  /// # Description
  /// Trigger the selected scenario
  /// 
  /// # Arguments
  ///`
  fn trigger_scenario(scenario: Scenarios, sub_action: String, options: Vec<String>) {
    match scenario {
      Scenarios::Init => sketch::init::launch(sub_action, options),
      Scenarios::Help => sketch::help::launch(sub_action),
      Scenarios::Generate => sketch::generate::launch(sub_action)
    }
  }

  /// Retrieve args
  /// 
  /// # Description
  /// Retrieve the arguments from the cli
  /// 
  /// # Return
  /// Return an `Arguments` struct
  fn retrieve_args() -> Arguments {
    let arguments: Vec<String> = env::args().collect();
    
    // retrieve the actions
    let main = arguments.get(0).unwrap_or(String::from(DEFAULT_SCENARIO));
    let secondary = arguments.get(1).unwrap(String::from(""));

    Arguments {
      main: main,
      secondary: secondary,
      options: arguments
    }
  }
}
