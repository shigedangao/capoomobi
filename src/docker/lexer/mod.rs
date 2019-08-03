pub mod compose;

/**
 * Yaml Parser
 * 
 * Parse a a .yaml file based on the provided path
 */
pub mod yaml_parser {
  use std::path::PathBuf;
  use crate::cli::core::fs::toolbox;
  use yaml_rust::{YamlLoader, yaml};

  // Error constant
  const UNABLE_READ_ERR: &str = "Unable to open the docker-compose.yaml file";
  const UNABLE_PARSE_ERR: &str = "Unable to parse the docker-compose.yaml for reason: ";
  const ABS_PATH_ERROR: &str = "Unable to generate absolute path";

  /**
   * Parse
   * 
   * Open and parse a yaml file
   */
  pub fn parse(path: &str, file_name: &str) -> Result<Vec<yaml::Yaml>, &'static str> {
    let mut paths = PathBuf::new();
    paths.push(path);
    paths.push(file_name);

    let compose_file_path = match toolbox::get_absolute_path(&paths) {
      Ok(p) => p,
      Err(_) => return Err(ABS_PATH_ERROR)
    };

    let content = toolbox::open_and_read_string_file(compose_file_path);
    if let Ok(value) = content {
       return match parse_yaml_tree(value) {
         Ok(yaml_content) => Ok(yaml_content),
         Err(e) => {
           panic!(format!("{}{:?}", UNABLE_PARSE_ERR, e));
         }
       }
    }

    Err(UNABLE_READ_ERR)
  }

  /**
   * Parse Yaml Tree
   * 
   * Parse the yaml tree
   */
  fn parse_yaml_tree(content: String) -> Result<Vec<yaml::Yaml> , yaml_rust::ScanError> {
    let yaml_file = YamlLoader::load_from_str(content.as_str());

    return match yaml_file {
      Ok(yaml_content) => Ok(yaml_content),
      Err(e) => Err(e)
    };
  }
}