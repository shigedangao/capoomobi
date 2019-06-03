/**
 * Json mod
 * 
 * This mod manipulate struct regarding JSON
 * It mainly manipulate raw string from capoomobi.json
 */
pub mod json_util {
  use std::path::PathBuf;
  use serde::{Serialize, Deserialize};
  use serde_json;

  // Structure refering to a project
  #[derive(Serialize, Deserialize, Debug)]
  struct Project {
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
   * Generate the project config for the .capoomobi.json file
   * 
   */
  pub fn generate_project_conf(project_name: String, path: PathBuf) -> Result<String, &'static str> {
    let str_path = match path.to_str() {
      Some(p) => p,
      None => ""
    };

    if str_path.is_empty() {
      return Err("Unable to generate path for encoding purposes")
    }

    let project = Project {
      name: project_name,
      path: str_path.to_owned()
    };

    // @todo replace by an already filled accumulator
    let mut vec: Vec<Project> = Vec::new();
    vec.push(project);

    let projects = Projects {
      projects: vec,
      current: String::from("foo")
    };

    let serialized_projects = serde_json::to_string(&projects);

    if let Ok(content) = serialized_projects {
      Ok(content)
    } else {
      Err("Unable to generate project list for capoomobi.json")
    }

  }
}