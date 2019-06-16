mod compose;

/**
 * Yaml Parser
 * 
 * Parse a a .yaml file based on the provided path
 */
pub mod yaml_parser {
  use std::collections::BTreeMap;
  use serde_yaml;
  use crate::cli::core::fs::operations;

  // Error constant
  const UNABLE_READ_ERR: &str = "Unable to open the docker-compose.yaml file";

  /**
   * Parse
   * 
   * Open and parse a yaml file
   */
  pub fn parse(path: &str, file_name: &str) -> Result<(), &'static str> {
    let mut concat_path = String::new();
    concat_path.push_str(path);
    concat_path.push_str(file_name);

    let content = operations::toolbox::open_get_str_content(concat_path.as_str());
    if let Ok(value) = content {
       let deserialized_compose_map: BTreeMap<String, String> = match serde_yaml::from_str(value.as_str()) {
        Ok(c) => c,
        Err(e) => return Err("unwrap err")
      };
    }

    Ok(())
  }
}