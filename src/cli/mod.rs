pub mod core;
pub mod scenarios;
mod configurator;

use crate::cli::core::logger::logging;

pub fn bootstrap() {
  // scenarios::welcome::welcome_scn::bootstrap();
  // Parse the arguments and launch a scenario
  match core::parser::cli_parser::parse_arguments() {
    Err(err) => logging::write(logging::LogType::Error, err, None),
    Ok(args) => {
      let main = args.main.to_owned();
      let sub  = args.sub.to_owned(); 
      scenarios::bootstrap::init(main.as_str(), sub.as_str(), args.options)
    }
  }
}