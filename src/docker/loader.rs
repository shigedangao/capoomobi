/// Yaml
/// 
/// # Description
/// 
/// Yaml parser module is use to extract the content of a yaml file
use std::path::PathBuf;
use std::error::Error;
use yaml_rust::{YamlLoader, yaml};
use crate::core::fs::toolbox;
use crate::core::errors::cli_error::{CliErr, ErrMessage, ErrHelper};

/// Error constant
const UNABLE_READ_ERR: &str  = "Unable to open the docker-compose.yaml file";
const UNABLE_PARSE_ERR: &str = "Unable to parse the docker-compose.yaml for reason: ";
const ABS_PATH_ERROR: &str   = "Unable to generate absolute path";

/// Parse
/// 
/// # Description
/// Retrieve the yaml content
/// 
/// # Arguments
/// * `path` slice of string
/// * `file_name` slice of string
/// 
/// # Return
/// Result<Vec<yaml::Yaml>, CliErr>
pub fn load(path: &str, file_name: &str) -> Result<Vec<yaml::Yaml>, CliErr> {
    let mut paths = PathBuf::from(path);
    paths.push(file_name);

    let docker_compose_path = toolbox::get_absolute_path(&paths);
    if let Err(e) = docker_compose_path {
        Err(CliErr::new(ABS_PATH_ERROR, e.description(), ErrMessage::IOError));
    }

    let content = toolbox::open_file(&docker_compose_path.unwrap());
    if let Err(err) = content {
        Err(CliErr::new(UNABLE_READ_ERR, err.description(), ErrMessage::IOError));
    }

    match parse_yaml_tree(content.unwrap()) {
        Ok(yaml_content) => Ok(yaml_content),
        Err(err) => Err(CliErr::new(UNABLE_PARSE_ERR, err.description(), ErrMessage::IOError))
    }
}

/// Parse Yaml Tree
///  
/// # Description
/// Retrieve the `yaml-rust` library yaml representation
/// 
/// # Argument
/// * `content` String
/// 
/// # Return
/// Result<Vec<yaml::Yaml>, yaml_rust::ScanError>
fn parse_yaml_tree(content: String) -> Result<Vec<yaml::Yaml> , yaml_rust::ScanError> {
    let yaml_file = YamlLoader::load_from_str(content.as_str());

    return match yaml_file {
        Ok(yaml_content) => Ok(yaml_content),
        Err(e) => Err(e)
    };
}
