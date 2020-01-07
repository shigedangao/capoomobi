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
use crate::core::errors::cli_error::{CliErr, ErrHelper, ErrMessage};
use crate::core::serde_utils::{SerdeUtil};
use crate::core::errors::message::core::{
    PATH_GENERATE_ERROR,
    PATH_GENERATE_REASON,
    DELETE_ERROR_MESSAGE,
    SWITCH_ERROR_MESSAGE,
    PROJECT_NAME_EMPTY
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

// List of method for manipulating the Projects struct
impl Projects {
    /// Add
    ///
    /// # Description
    /// Add a new project to the projects structure
    ///
    /// # Arguments
    /// * `self` Projects
    /// * `pname` &str
    /// * `path` PathBuf
    ///
    /// # Return
    /// Result<Self, CliErr>
    pub fn add(mut self, pname: &str, path: PathBuf) -> Result<Self, CliErr> {
        if pname.is_empty() {
            return Err(CliErr::new(PROJECT_NAME_EMPTY, "", ErrMessage::NotFound));
        }

        let pstr = path.to_str().unwrap_or("");
        if pstr.is_empty() {
            return Err(CliErr::new(PATH_GENERATE_ERROR, PATH_GENERATE_REASON, ErrMessage::ParsingError));
        }

        let new_project = Project {
            name: String::from(pname),
            path: String::from(pstr)
        };

        self.projects.push(new_project);
        self.current = String::from(pname);

        Ok(self)
    }

    /// Get Project By Name
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
    fn get_project_by_name(&self, name: String) -> Option<(usize, String)> {
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
    /// * `name` &str
    ///
    /// # Return
    /// Result<(&Self, PathBuf)>
    pub fn delete_project_by_name(mut self, name: &str) -> Result<(Self, PathBuf), CliErr> {
        let project_opt = self.get_project_by_name(String::from(name));
        if project_opt.is_none() {
            return Err(CliErr::new(DELETE_ERROR_MESSAGE, "", ErrMessage::NotFound));
        }

        let project = project_opt.unwrap();
        self.projects.remove(project.0);

        Ok((self, PathBuf::from(project.1)))
    }

    /// Switch Project
    ///
    /// # Description
    /// Switch the setted project to an other
    ///
    /// # Arguments
    /// * `self` Projets struct
    /// * `name` &str
    ///
    /// # Return
    /// Result<&Self, CliErr>
    pub fn switch_project(mut self, name: &str) -> Result<Self, CliErr> {
        let project = self.projects
            .iter()
            .filter(|p| p.name == name)
            .last();

        if project.is_none() {
            return Err(CliErr::new(SWITCH_ERROR_MESSAGE, "", ErrMessage::NotFound));
        }

        self.current = String::from(name);
        Ok(self)
    }
}

impl SerdeUtil for Projects {}
