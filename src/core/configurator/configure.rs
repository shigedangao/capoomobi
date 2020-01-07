/// Configure
///
/// # Description
/// Module use as an entrypoint to configure the capoomobi.json file
use std::path::{PathBuf};
use std::error::Error;
use super::builder;
use crate::core::fs::toolbox;
use crate::core::serde_utils;
use crate::core::errors::cli_error::{CliErr, ErrMessage, ErrHelper};
use crate::core::errors::message::core::{
    CONFIG_GENERATE_ERROR,
    FILE_NOT_PARSABLE_ERROR,
    DECODE_ERROR,
    WRITE_JSON_ERROR
};

/// Constant defining the paths available
const CONFIG_FILE_PATH: &str = ".capoomobi.json";

/// Capoo Config
///
/// # Description
/// Structure use to configure the capoomobi.json
#[derive(Debug)]
pub struct CapooConfig {
    pub path: PathBuf
}

impl CapooConfig {
    /// New
    ///
    /// # Description
    /// Return a new ConfigureCapoo structure
    ///
    /// # Arguments
    /// * `path` PathBuf
    ///
    /// # Return
    /// ConfigureCapoo structure
    pub fn new(path: PathBuf) -> Self {
        CapooConfig {
            path
        }
    }

    /// Get Content
    ///
    /// # Description
    /// Retrieve the content of the capoomobi file
    ///
    /// # Return
    /// Result<builder::Project, CliErr>
    pub fn get_content(&self) -> Result<builder::Projects, CliErr> {
        let content_res = toolbox::open_file(&self.path);

        if let Err(err) = content_res {
            return Err(
                CliErr::new(FILE_NOT_PARSABLE_ERROR, err.description(), ErrMessage::ParsingError)
            );
        }

        let contents = content_res.unwrap();
        let projects = match serde_utils::deserialize::<builder::Projects>(&contents) {
            Ok(p) => p,
            Err(err) => {
                return Err(
                    CliErr::new(DECODE_ERROR, err.description(), ErrMessage::ParsingError)
                )
            }
        };

        Ok(projects)
    }

    /// Write Json File
    ///
    /// # DescConfigureCapooription
    /// Decorator of the write_json_content of the toolbox
    ///
    /// # Arguments
    /// * `self`
    /// * `json` String content
    ///
    /// # Return
    /// Result<(), CliErr>
    pub fn write_json_file(&self, json: String) -> Result<(), CliErr> {
        match toolbox::write_json_content(&self.path, json) {
            Ok(_) => Ok(()),
            Err(err) => Err(
                CliErr::new(WRITE_JSON_ERROR, err.description(), ErrMessage::IOError)
            )
        }
    }
}

/// Create New Config
///
/// # Description
/// Create a new capoomobi configuration
///
/// # Return
/// Return a new ConfigureCapoo struct
pub fn create_config() -> Result<CapooConfig, CliErr> {
    let mut conf_file_path = toolbox::get_home_dir();
    conf_file_path.push(CONFIG_FILE_PATH);

    match toolbox::create_file(&conf_file_path) {
        Ok(_) => Ok(CapooConfig::new(conf_file_path)),
        Err(err) => Err(
            CliErr::new(CONFIG_GENERATE_ERROR, err.description(), ErrMessage::IOError)
        )
    }
}

/// Exist
///
/// # Description
/// Check if the config already exist
///
/// # Return
/// Option<CapooConfig>
pub fn exist() -> Option<CapooConfig> {
    let mut conf_path = toolbox::get_home_dir();
    conf_path.push(CONFIG_FILE_PATH);

    if conf_path.exists() {
        return Some(CapooConfig::new(conf_path));
    }

    None
}
