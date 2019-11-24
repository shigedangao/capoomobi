/// Builder
/// 
/// # Description
/// Module use to manipulate the content of the .capoomobi.json
use std::path::PathBuf;
use std::error::Error;
use serde::{Serialize, Deserialize};
use serde_json;
use crate::errors::cli_error::{CliErr, ErrHelper, ErrMessage};

// Errors
const PATH_GENERATE_ERROR: &str     = "Error while generating absolute path";
const PATH_GENERATE_REASON: &str    = "An error occured while converting the path";
const PROJECT_GENERATE_ERROR: &str  = "Unable to generate new config for capoomobi.json";
const PROJECT_GENERATE_REASON: &str = "An error occured while serializing the project";

/// Structure refering to a project
#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub name: String,
    pub path: String,
}

/// Structure refering to a set of projects
#[derive(Serialize, Deserialize, Debug)]
pub struct Projects {
    pub projects: Vec<Project>,
    pub current: String,
}

/// Generate Project Conf
/// @TODO to remove
/// 
/// # Description
/// Populate the project configuration
/// 
/// # Info
/// The .capoomobi.json has the following file format
/// "projects": [
///   {
///     "name": <project_name>,
///     "path": <project_path>  
///   },
///   {...},
/// ],
/// "current": <project_name>
/// 
/// # Arguments
/// * `project_name` String
/// * `path` PathBuf
/// 
/// # Return
/// Result<String, CliErr>
pub fn populate_project_conf(pname: String, path: PathBuf) -> Result<String, CliErr> {
    let serialized_projects = serde_json::to_string(&project_in_config);
    if let Ok(content) = serialized_projects {
        return Ok(content);
    }

    Err(
        CliErr::new(
            PROJECT_GENERATE_ERROR,
            String::from(PROJECT_GENERATE_REASON),
            ErrMessage::SerializeError
        )
    )
}

/// Parse String to Struct
/// 
/// # Description
/// Parse the string content to reflect to the Projects structure
/// 
/// # Arguments
/// * `config_values` Reference to a String
/// 
/// # Return
/// io::Result<Projects>
pub fn parse_string_to_struct(config_values: &String) -> std::io::Result<Projects> {
    let p: Projects = serde_json::from_str(&config_values)?;
    Ok(p)
}

// List of method for manipulating the Projects struct
impl Projects {
    /// Add
    /// 
    /// # Description
    /// Add a new project to the projects structure
    /// 
    /// # Arguments
    /// * `self` Projects
    /// * `pname` String
    /// * `path` PathBuf
    /// 
    /// # Return
    /// Result<Self, CliErr>
    fn add(self, pname: String, path: PathBuf) -> Result<Self, CliErr> {
        let pstr = path.to_str().unwrap_or("");
        if pstr.is_empty() {
            Err(CliErr::new(PATH_GENERATE_ERROR, PATH_GENERATE_REASON, ErrMessage::ParsingError));
        }

        let new_project = Project {
            name: pname,
            path: String::from(pstr)
        };

        self.projects.push(new_project);
        self.current = pname;

        Ok(self)
    }

    /// Get Project Idx
    /// 
    /// # Description
    /// Get a project by name
    /// 
    /// # Arguments
    /// * `&self` Project struct reference
    /// * `name` String
    /// 
    /// # Return
    /// Option<(usize, String)>
    fn get_project_idx(&self, name: String) -> Option<(usize, String)> {
        let project = &(&self.projects)
            .into_iter()
            .enumerate()
            .filter(|p| p.1.name == name)
            .last();

        if let Some(p) = project {
            return Some((p.0, String::from(&p.1.path)));
        }

        None
    }
    /// Delete By Idx
    /// 
    /// # Description
    /// Delete a project by giving the index
    /// 
    /// # Arguments
    /// * `self` Projects struct
    /// * `idx` usize
    /// 
    /// # Return
    /// (bool, String, String) tupple
    pub fn delete_project_by_name(&mut self, name: &String) -> (bool, String, String) {
        let project_opts = &self.get_project_idx(String::from(name));
        if let None = project_opts {
            return (false, String::new(), String::new());
        }

        let project = project_opts.as_ref().unwrap();
        &self.projects.remove(project.0);
        match serde_json::to_string(&self) {
            Ok(json) => (true, json, String::from(&project.1)),
            Err(err) => (false, String::from(err.description()), String::new())
        }
    }

    /// Switch Project
    /// 
    /// # Description
    /// Switch the setted project to an other
    /// 
    /// # Arguments
    /// * `self` Projets struct
    /// * `name` String
    /// 
    /// # Return
    /// (bool, String) tupple
    pub fn switch_project(&mut self, name: &String) -> (bool, String) {
        let has_project = &(&self.projects)
            .into_iter()
            .filter(|p| p.name == String::from(name))
            .last();

        if let None = has_project {
            return (false, String::new());
        }

        self.current = String::from(name);
        match serde_json::to_string(self) {
            Ok(json) => (true, json),
            Err(err) => (false, String::from(err.description()))
        }
    }
}
