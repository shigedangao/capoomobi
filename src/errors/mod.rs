/**
 * Cli Error
 * 
 * Module use to handle custom CLI errors
 * This custom error help to easily debug issues with the CLI
 */
pub mod cli_error {
  use std::fmt;
  use crate::cli::core::logger::logging;

  /**
   * Err Code
   * 
   * List of error code
   */
  pub enum ErrCode {
    ParsingError,
    SerializeError,
    MissingFieldError
  }

  /**
   * Error Helper trait
   * 
   * Define a set of common method to use the custom errors
   */
  pub trait ErrorHelper {
    /**
     * New
     * 
     * Return a new CliErr structure
     */
    fn new(message: &'static str, reason: &'static str, codename: ErrCode) -> Self;
    /**
     * Log Pretty
     * 
     * Print the log in a pretty way in order to make it more readable
     */
    fn log_pretty(&self); 
  }
  
  /**
   * Cli Err
   * 
   * Custom error handler use to better describe the errors
   * triggered by the CLI
   */
  pub struct CliErr {
    message: &'static str,
    reason: &'static str,
    code: u8
  }

  impl ErrorHelper for CliErr {
    fn new(message: &'static str, reason: &'static str, codename: ErrCode) -> CliErr {
      let code = match codename {
        ErrCode::MissingFieldError => 44,
        ErrCode::ParsingError => 50,
        ErrCode::SerializeError => 42,
        _ => 44
      };
      
      CliErr {
        message: message,
        reason: reason,
        code: code
      }
    }

    fn log_pretty(&self) {
      logging::write(
        logging::LogType::Error,
        self.message,
        Some(format!("error code: {} reason: {}", self.code, self.reason))
      );
    }
  }

  impl fmt::Display for CliErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(
        f,
        "message: {} reason: {} code: {}",
        self.message,
        self.reason,
        self.code
      )
    }
  }

  impl fmt::Debug for CliErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(
        f,
        "message: {} reason: {} code: {}",
        self.message,
        self.reason,
        self.code
      )
    }
  }
}