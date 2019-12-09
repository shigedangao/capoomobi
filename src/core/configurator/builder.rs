/// Builder
/// 
/// # Description
/// Module use to manipulate the content of the .capoomobi.json
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
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use serde_json;
use crate::core::errors::cli_error::{CliErr, ErrHelper, ErrMessage};
use crate::core::serde_utils::{SerdeUtil};
use crate::core::errors::message::core::{
    PATH_GENERATE_ERROR,
    PATH_GENERATE_REASON,
    DELETE_ERROR_MESSAGE,
    SWITCH_ERROR_MESSAGE
};

/// Structure refering to a project
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    pub name: String,
    pub path: String,
}

/// Structure refering to a set of projects
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Projects {
    pub projects: Vec<Project>,
    pub current: String,
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
    pub fn add(mut self, pname: String, path: PathBuf) -> Result<Self, CliErr> {
        let pstr = path.to_str().unwrap_or("");
        if pstr.is_empty() {
            return Err(CliErr::new(PATH_GENERATE_ERROR, PATH_GENERATE_REASON, ErrMessage::ParsingError));
        }

        let new_project = Project {
            name: String::from(&pname),
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
            .iter()
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
    /// Result<(&Self, PathBuf)>
    pub fn delete_project_by_name(mut self, name: &String) -> Result<(Self, PathBuf), CliErr> {
        let project_opt = self.get_project_idx(String::from(name));
        if let None = project_opt {
            return Err(CliErr::new(DELETE_ERROR_MESSAGE, "", ErrMessage::NotFound));
        }

        let project = project_opt.unwrap();
        &self.projects.remove(project.0);

        Ok((self, PathBuf::from(project.1)))
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
    /// Result<&Self, CliErr>
    pub fn switch_project(mut self, name: &String) -> Result<Self, CliErr> {
        let project = self.projects
            .iter()
            .filter(|p| p.name == String::from(name))
            .last();

        if let None = project {
            return Err(CliErr::new(SWITCH_ERROR_MESSAGE, "", ErrMessage::NotFound));
        }

        self.current = String::from(name);
        Ok(self)
    }
}

impl SerdeUtil for Projects {}
