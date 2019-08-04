/// Input
/// 
/// Module use to retrieve the input of the user
pub mod input {
  use std::io;
  use crate::cli::core::logger::logging;

  /// Get User Input
  /// 
  /// # Description
  /// Retrieve the user input
  /// 
  /// # Arguments
  /// * `message` slice of string
  /// 
  /// # Return
  /// string
  pub fn get_user_input(message: &str) -> String {
    logging::write(
      logging::LogType::Info,
      message, 
      None
    );

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Unable to read user input");

    input.trim().to_string()
  }
}