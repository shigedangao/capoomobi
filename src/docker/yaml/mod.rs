mod compose;

/**
 * Yaml Parser
 * 
 * Parse a a .yaml file based on the provided path
 */
pub mod yaml_parser {
  use std::collections::HashMap;
  use std::path::PathBuf;
  use crate::cli::core::fs::operations;
  use crate::cli::core::fs::utility;
  use yaml_rust::{YamlLoader};

  // Error constant
  const UNABLE_READ_ERR: &str = "Unable to open the docker-compose.yaml file";

  /**
   * Parse
   * 
   * Open and parse a yaml file
   */
  pub fn parse(path: &str, file_name: &str) -> Result<(), &'static str> {
    let mut paths = PathBuf::new();
    paths.push(path);
    paths.push(file_name);

    let compose_file_path = match utility::get_abs_path(&paths) {
      Ok(p) => p,
      Err(_) => return Err("err")
    };

    let content = operations::toolbox::open_get_str_content(compose_file_path);
    if let Ok(value) = content {
       return match parse_yaml_tree(value) {
         Ok(()) => Ok(()),
         Err(e) => return Err(e)
       }
    }

    Err(UNABLE_READ_ERR)
  }

  /**
   * Parse Yaml Tree
   * 
   * Parse the yaml tree
   */
  fn parse_yaml_tree(content: String) -> Result<(), &'static str> {
    let yaml_file = YamlLoader::load_from_str(content.as_str());

    if let Ok(yaml_content) = yaml_file {
      println!("value of yaml {:?}", yaml_content[0]);
    }

    Ok(())
  }
}