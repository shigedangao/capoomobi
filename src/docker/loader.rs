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
use crate::core::errors::message::docker::{
    UNABLE_READ,
    UNABLE_PARSE,
    ABS_PATH
};

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
        return Err(CliErr::new(ABS_PATH, e.description(), ErrMessage::IOError));
    }

    let content = toolbox::open_file(&docker_compose_path.unwrap());
    if let Err(err) = content {
        return Err(CliErr::new(UNABLE_READ, err.description(), ErrMessage::IOError));
    }

    match parse_yaml_builder(content.unwrap()) {
        Ok(yaml_content) => Ok(yaml_content),
        Err(err) => Err(CliErr::new(UNABLE_PARSE, err.description(), ErrMessage::IOError))
    }
}

/// Parse Yaml builder
///  
/// # Description
/// Retrieve the `yaml-rust` library yaml representation
/// 
/// # Argument
/// * `content` String
/// 
/// # Return
/// Result<Vec<yaml::Yaml>, yaml_rust::ScanError>
fn parse_yaml_builder(content: String) -> Result<Vec<yaml::Yaml> , yaml_rust::ScanError> {
    let yaml_file = YamlLoader::load_from_str(content.as_str());

    return match yaml_file {
        Ok(yaml_content) => Ok(yaml_content),
        Err(e) => Err(e)
    };
}
