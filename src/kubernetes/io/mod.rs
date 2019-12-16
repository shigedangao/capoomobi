pub mod folder;
pub mod display;
pub mod output;
pub mod objects;
pub mod runner;

use crate::core::errors::cli_error::{CliErr, ErrHelper};

/// Print Errors
/// 
/// # Description
/// Print the errors from an array of Result
/// 
/// # Arguments
/// * `err` Vec<Result<T, CliErr>>
fn print_errors<T>(err: Vec<Result<T, CliErr>>) {
  for e in err {
      if let Err(er) = e {
          er.log_pretty();
      }
  }
}