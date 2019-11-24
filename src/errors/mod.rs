/// CliErr module
/// 
/// # Description
/// Module use for handling custom error
pub mod cli_error {
    use std::fmt;
    use std::error::Error;
    use crate::core::logger::{log_error, LogType};

    // Constants errors
    const MISSING_FIELD_ERROR: &str = "Unable to find a field. Please provide the missing field";
    const PARSING_ERROR: &str = "Error while parsing a content";
    const SERIALIZE_ERROR: &str = "Error while serializing a resources";
    const NOT_FOUND_ERROR: &str = "Resources not found";
    const UNEXPECTED_ERROR: &str = "An unexpected error happened";

    /// ErrMessage
    /// List of enum field describing the type of error
    #[derive(Debug)]
    pub enum ErrMessage {
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
        /// * `reason` &str
        /// * `codename` ErrMessage enum value
        /// 
        /// # Return
        /// CliErr struct
        fn new(message: &str, reason: &str, codename: ErrMessage) -> Self;
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
        message: String,
        reason: String,
        code_msg: String
    }

    impl ErrHelper for CliErr {
        fn new(message: &str, reason: &str, codename: ErrMessage) -> CliErr {
            let msg = match codename {
                ErrMessage::MissingFieldError => MISSING_FIELD_ERROR,
                ErrMessage::ParsingError => PARSING_ERROR,
                ErrMessage::SerializeError => SERIALIZE_ERROR,
                ErrMessage::NotFound => NOT_FOUND_ERROR,
                _ => UNEXPECTED_ERROR
            };

            CliErr {
                message: String::from(message),
                reason: String::from(reason),
                code_msg: String::from(msg)
            }
        }

        fn log_pretty(&self) {
            log_error(
                LogType::Error,
                self.message.as_str(),
                self.code_msg.as_str(),
                Some(self.reason.clone())
            );
        }
    }

    impl Error for CliErr {
        fn description(&self) -> &str {
            &self.message
        }
    }

    impl fmt::Display for CliErr {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "message: {} reason: {} code: {}",
                self.message,
                self.reason,
                self.code_msg
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
                self.code_msg
            )
        }
    }
}