/**
 * CLI arguments parser
 */
pub mod cli_parser {
  /**
   * Cli Args
   * 
   * Structure representing the structure command of the CLI
   */
  pub struct CliArgs {
    pub main: String,
    pub sub: String
  }

  /**
   * CLI has the following format
   * 
   * capoomobi <main_action> <sub_action>
   * A main_action & sub_action are defined on the scenarios namespace
   */
  pub fn parse_arguments() -> Result<CliArgs, &'static str> {
    let main_action = std::env::args().nth(1);
    let sub_action = std::env::args().nth(2);

    if main_action.is_none() {
      println!("empty");
      return Err("main action is empty");
    }

    println!("action: {:?}", main_action);
    println!("path: {:?}", sub_action);

    let args = CliArgs {
      main: main_action.unwrap(),
      sub: sub_action.unwrap_or("default".to_string())
    };

    return Ok(args);
  }
}