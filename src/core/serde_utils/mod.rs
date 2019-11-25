use std::error::Error;
use serde::{Serialize, Deserialize};
use crate::errors::cli_error::{CliErr, ErrHelper, ErrMessage};

// Error message constant
const SERIALIZE_ERROR: &str = "Unable to serialize the data type";
const DESERIALIZE_ERROR: &str = "Unable to deserialize the JSON content";


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
    fn deserialize<'de>(&self, d: &'de str) -> Result<Self, CliErr> where Self : Deserialize<'de> {
        match serde_json::from_str(d) {
            Ok(d) => Ok(d),
            Err(e) => Err(CliErr::new(DESERIALIZE_ERROR, e.description(), ErrMessage::SerializeError))
        }
    }
}