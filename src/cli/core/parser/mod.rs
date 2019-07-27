pub mod options;

/**
 * CLI arguments parser
 */
pub mod cli_parser {
  use crate::errors::cli_error::{CliErr, ErrHelper, ErrCode};
  
  /**
   * Cli Args
   * 
   * Structure representing the structure command of the CLI
   */
  pub struct CliArgs {
    pub main: String,
    pub sub: String,
    pub options: Vec<String>
  }

  /**
   * CLI has the following format
   * 
   * capoomobi <main_action> <sub_action>
   * A main_action & sub_action are defined on the scenarios namespace
   */
  pub fn parse_arguments() -> Result<CliArgs, CliErr> {
    let main_action = std::env::args().nth(1);
    let sub_action = std::env::args().nth(2);
    let options: Vec<String> = std::env::args().collect();

    if main_action.is_none() {
      return Err(
        CliErr::new(
          "main action is empty",
          "required one argument",
          ErrCode::MissingFieldError
        )
      );
    }

    let args = CliArgs {
      main: main_action.unwrap(),
      sub: sub_action.unwrap_or("default".to_string()),
      options: options
    };

    return Ok(args);
  }
}