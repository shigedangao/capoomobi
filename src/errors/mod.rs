/// CliErr module
/// 
/// # Description
/// Module use for handling custom error
pub mod cli_error {
    use std::fmt;
    use crate::cli::core::logger::logger::{log, LogType};

    /// ErrCode
    /// List of enum field describing the type of error
    pub enum ErrCode {
        ParsingError,
        SerializeError,
        MissingFieldError,
        IOError,
        NotFound,
        RendererError
    }

    /// Err Helper
    /// 
    /// # Description
    /// Trait which implement a set of function for the error
    pub trait ErrHelper {
        /// New
        /// 
        /// # Description
        /// Return a new CLiErr struct
        /// 
        /// # Arguments
        /// * `message` slice of a static str
        /// * `reason` String
        /// * `codename` ErrCode enum value
        /// 
        /// # Return
        /// CliErr struct
        fn new(message: &'static str, reason: String, codename: ErrCode) -> Self;
        /// Log Pretty
        /// 
        /// # Description
        /// Log the error in a pretty way
        fn log_pretty(&self); 
    }
    
    /// CliErr
    /// 
    /// # Description
    /// structure use to handle information about the error
    pub struct CliErr {
        message: &'static str,
        reason: String,
        code: u8
    }

    impl ErrHelper for CliErr {
        fn new(message: &'static str, reason: String, codename: ErrCode) -> CliErr {
            let code = match codename {
                ErrCode::MissingFieldError => 44,
                ErrCode::ParsingError => 50,
                ErrCode::SerializeError => 42,
                ErrCode::NotFound => 44,
                _ => 44
            };

            CliErr {
                message: message,
                reason: reason,
                code: code
            }
        }

        fn log_pretty(&self) {
            log(
                LogType::Error,
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