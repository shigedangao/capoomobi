/// Boostrap module
/// 
/// # Description
/// Module use to trigger the CLI
pub mod manager {
  use crate::cli::scenarios::parser::parser;
  use crate::errors::cli_error::{ErrHelper};

  /// Start
  /// 
  /// # Description
  /// Boostrap method use to launch the CLI
  pub fn start() {
    match parser::parse() {
      Err(err) => err.log_pretty(),
      Ok(()) => {}
    }
  }
}