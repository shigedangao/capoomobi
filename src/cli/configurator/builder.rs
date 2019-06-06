/**
 * Json mod
 * 
 * This mod manipulate struct regarding JSON
 * It mainly manipulate raw string from capoomobi.json
 */
pub mod json_util {
  use crate::cli::configurator::configure;
  use std::path::PathBuf;
  use serde::{Serialize, Deserialize};
  use serde_json;

  // Errors
  const PATH_GENERATE_ERROR: &str = "Unable to generate path for encoding purposes";
  const PROJECT_GENERATE_ERROR: &str = "Unable to generate new config for capoomobi.json";

  // Structure refering to a project
  #[derive(Serialize, Deserialize, Debug)]
  pub struct Project {
    name: String,
    path: String,
  }

  // Structure refering to a set of projects
  #[derive(Serialize, Deserialize, Debug)]
  pub struct Projects {
    projects: Vec<Project>,
    current: String,
  }

  /**
   * Generate Project Conf
   *  
   * Populate the .capoomobi.json configuration file with the following format
   * 
   * "projects": [
   *  {
   *    "name": <project_name>,
   *    "path": <project_path>
   *  },
   *  {
   *   ...
   *  }
   * ],
   * "current": <project_name>
   */
  pub fn generate_project_conf(project_name: String, path: PathBuf) -> Result<String, &'static str> {
    let str_path = match path.to_str() {
      Some(p) => p,
      None => ""
    };

    if str_path.is_empty() {
      return Err(PATH_GENERATE_ERROR);
    }

    // Read the capoomobi.json file and parse the content
    // If the file is empty generate a Projects empty struct which will be populate
    // by the new configuration
    let mut project_in_config = match configure::read_config_file() {
      Ok(projects) => projects,
      Err(_) => Projects {
        projects: Vec::new(),
        current: String::from("")
      }
    };

    let project = Project {
      name: project_name.clone(),
      path: str_path.to_owned()
    };

    project_in_config.projects.push(project);
    project_in_config.current = project_name;

    let serialized_projects = serde_json::to_string(&project_in_config);
    if let Ok(content) = serialized_projects {
      Ok(content)
    } else {
      Err(PROJECT_GENERATE_ERROR)
    }
  }

  /**
   * Parse Str To Struct
   * 
   * Parse the values of the config file into a struct
   */
  pub fn parse_str_to_struct(config_values: &String) -> std::io::Result<Projects> {
    let p: Projects = serde_json::from_str(&config_values.to_owned())?;
    Ok(p)
  }
}