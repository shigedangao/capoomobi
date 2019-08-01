/// Boostrap module
/// 
/// # Description
/// Module use to trigger the CLI
pub mod manager {
  use crate::cli;
  use crate::cli::scenarios::parser::parser;
  use crate::errors::cli_error::{ErrHelper};

  /// Start
  /// 
  /// # Description
  /// Boostrap method use to launch the CLI
  pub fn start() {
    match cli::core::parser::cli_parser::parse_arguments() {
      Err(err) => err.log_pretty(),
      Ok(args) => {
        let main = args.main.to_owned();
        let sub  = args.sub.to_owned(); 
        parser::parse(main.as_str(), sub.as_str(), args.options)
      }
    }
  }
}