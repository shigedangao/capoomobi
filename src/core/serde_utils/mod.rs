use std::error::Error;
use serde::{Serialize};
use super::errors::cli_error::{CliErr, ErrHelper, ErrMessage};
use crate::core::errors::message::core::SERIALIZE_ERROR;

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
        match serde_json::to_string(&self) {
            Ok(p) => Ok(p),
            Err(e) => Err(CliErr::new(SERIALIZE_ERROR, e.description(), ErrMessage::SerializeError))
        }
    }
}