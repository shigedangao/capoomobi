pub mod core;
pub mod scenarios;
pub mod configurator;

use crate::errors::cli_error::{ErrHelper};

pub fn bootstrap() {
  // scenarios::welcome::welcome_scn::bootstrap();
  // Parse the arguments and launch a scenario
  match core::parser::cli_parser::parse_arguments() {
    Err(err) => err.log_pretty(),
    Ok(args) => {
      let main = args.main.to_owned();
      let sub  = args.sub.to_owned(); 
      scenarios::bootstrap::init(main.as_str(), sub.as_str(), args.options)
    }
  }
}