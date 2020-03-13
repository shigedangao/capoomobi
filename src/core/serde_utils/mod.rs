use serde::{Serialize, Deserialize};
use super::errors::cli_error::{CliErr, ErrHelper, ErrMessage};
use crate::core::errors::message::core::{SERIALIZE_ERROR, DESERIALIZE_ERROR};

/// SerdeUtil
///
/// # Description
/// Trait use to implement serialize a data T to a String
pub trait SerdeUtil {
    /// Serialize
    ///
    /// # Description
    /// Serialize any data and return the generated string
    ///
    /// # Arguments
    /// * `d` T
    ///
    /// # Return
    /// Result<String, CliErr>
    fn serialize(&self) -> Result<String, CliErr> where Self : Serialize {
        match serde_json::to_string_pretty(&self) {
            Ok(p) => Ok(p),
            Err(err) => Err(CliErr::new(SERIALIZE_ERROR, &err.to_string(), ErrMessage::SerializeError))
        }
    }
}

/// Deserialize
///
/// # Description
/// Deserialize any data and return the wishes type
///
/// # Arguments
/// * `d` String
///
/// # Return
/// Result<T, &str>
pub fn deserialize<'de, T>(d: &'de str) -> Result<T, CliErr> where T: Deserialize<'de> {
    match serde_json::from_str(d) {
        Ok(res) => Ok(res),
        Err(err) => Err(CliErr::new(DESERIALIZE_ERROR, &err.to_string(), ErrMessage::SerializeError))
    }
}
