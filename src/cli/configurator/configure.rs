/// Configure
/// 
/// # Description
/// Module use as an entrypoint to configure the capoomobi.json file
use std::path::{PathBuf};
use std::error::Error;
use crate::cli::core::fs::toolbox;
use crate::cli::configurator::builder;
use crate::errors::cli_error::{CliErr, ErrMessage, ErrHelper};

/// Constant defining the paths available
const CONFIG_FILE_PATH: &str = ".capoomobi.json";

/// Constant referencing the error
const CONFIG_GENERATE_ERROR: &str = "Unable to generate the config file for reason:"; 
const FILE_NOT_PARSABLE_ERROR: &str = "Unable to parse the config file for reason:";
const DECODE_ERROR: &str = "Unable to parse the content of the config file for reason:";
const WRITE_JSON_ERROR: &str = "Unable to write the capoomobi.json file";

/// Configure Capoo
/// 
/// # Description
/// Structure use to configure the capoomobi.json
pub struct ConfigureCapoo {
    pub path: PathBuf
}

impl ConfigureCapoo {
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
        ConfigureCapoo {
            path: path
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
        let content_res = toolbox::open_and_read_string_file(&self.path);

        if let Err(err) = content_res {
            return Err(
                CliErr::new(
                    FILE_NOT_PARSABLE_ERROR,
                    format!("{}", err.description()),  
                    ErrMessage::ParsingError
                )
            );
        }

        let contents = content_res.unwrap();
        let projects = match builder::parse_string_to_struct(&contents) {
            Ok(p) => p,
            Err(err) => {
                return Err(
                    CliErr::new(
                        DECODE_ERROR,
                        format!("{}", err.description()),
                        ErrMessage::ParsingError
                    )
                )
            }
        };

        Ok(projects)
    }

    /// Write Json
    /// 
    /// # Description
    /// Decorator of the write_json_content of the toolbox
    /// 
    /// # Arguments
    /// * `self`
    /// * `json` String content
    /// 
    /// # Return
    /// Result<(), CliErr>
    pub fn write_json(&self, json: String) -> Result<(), CliErr> {
        match toolbox::write_json_content(&self.path, json) {
            Ok(_) => Ok(()),
            Err(err) => Err(
                CliErr::new(
                    WRITE_JSON_ERROR,
                    format!("{}", err.description()),
                    ErrMessage::IOError
                )
            )
        }
    }

    /// Generate Project Conf
    /// 
    /// # Description
    /// Generate a project configuration JSON representation
    /// 
    /// # Arguments
    /// * `self` self
    /// * `project_name` String
    /// * `path` PathBuf
    /// 
    /// # Return
    /// Option<String, CliErr>
    pub fn generate_project_conf(&self, project_name: String, path: PathBuf) -> Result<String, CliErr> {
        let contents_result = self.get_content();

        builder::populate_project_conf(project_name, path, contents_result)
    }
}

/// Boostrap Capoo
/// 
/// # Description
/// Use to creata new / retrieve an existing configuration file
/// 
/// # Return
/// Return a new ConfigureCapoo struct
pub fn bootstrap_capoo() -> Result<ConfigureCapoo, CliErr> {
    let mut conf_file_path = toolbox::get_home_dir();
    conf_file_path.push(CONFIG_FILE_PATH);

    if conf_file_path.exists() {
        return Ok(ConfigureCapoo::new(conf_file_path));
    }

    match toolbox::create_file(conf_file_path) {
        Ok(p) => Ok(ConfigureCapoo::new(p)),
        Err(err) => Err(
            CliErr::new(
                CONFIG_GENERATE_ERROR,
                format!("{}", err.description()),
                ErrMessage::IOError
            )
        )
    }
}