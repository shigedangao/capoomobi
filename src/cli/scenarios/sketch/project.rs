use std::path::PathBuf;
use std::error::Error;
use crate::errors::cli_error::{CliErr, ErrMessage, ErrHelper};
use crate::cli::configurator::configure::{bootstrap_capoo, ConfigureCapoo};
use crate::core::logger::{log, LogType};
use crate::cli::scenarios::sketch::helper;
use crate::core::fs::toolbox;

// Errors
const DELETE_ERROR_MESSAGE: &str = "Unable to delete project";
const SWITCH_ERROR_MESSAGE: &str = "Unable to switch project";

/// Project
/// 
/// # Description
/// Project allow you to see the current & switch the project to an other. This scenario allow you to change project below is an example
/// capoomobi project current
/// capoomobi project switch <project_name>
/// 
/// # Arguments
/// * `project_name` slice of a string
pub fn launch(main_action: &str, options: &Vec<String>) {
    let arg = match helper::retrieve_options_by_idx(options, 0) {
        Some(arg) => arg,
        None => String::new()
    };

    let configuration = match bootstrap_capoo() {
        Ok(conf) => conf,
        Err(err) => {
            err.log_pretty();
            panic!();
        }
    };

    match main_action {
        "current" => show_current_project(configuration),
        "switch" => switch_project(configuration, arg),
        "list" => list_project(configuration),
        "delete" => delete_project(configuration, arg),
        _ => show_current_project(configuration)
    }
}

/// Show Current Project
/// 
/// # Description
/// Show the current setted project
/// 
/// # Arguments
/// * `configuration` ConfigureCapoo struct
fn show_current_project(configuration: ConfigureCapoo) {
    match configuration.get_content() {
        Ok(p) => {
            log(
                LogType::Info,
                "the current project in use is:",
                Some(p.current)
            );
        },
        Err(err) => err.log_pretty()
    }
}

/// List Project
/// 
/// # Description
/// list the known project
/// 
/// # Arguments
/// * `configuration` ConfigureCapoo struct
fn list_project(configuration: ConfigureCapoo) {
    match configuration.get_content() {
        Ok(projects) => {
            for p in projects.projects.into_iter() {
                log(
                    LogType::Info,
                    format!("* {}", p.name.as_str()).as_str(),
                    None
                );
            }
        },
        Err(err) => err.log_pretty()
    }  
}

/// Switch Project
/// 
/// # Description
/// Switch the project with the provided project name
/// 
/// # Arguments
/// * `configuration` ConfigureCapoo struct
/// * `project_name` name of the project
fn switch_project(configuration: ConfigureCapoo ,project_name: String) {
    let mut capoo_projects = match configuration.get_content() {
        Ok(p) => p,
        Err(err) => {
            err.log_pretty();
            return;
        }
    };

    let (status, output) = capoo_projects.switch_project(&project_name);
    if !status {
        CliErr::new(
            SWITCH_ERROR_MESSAGE,
            output,
            ErrMessage::IOError
        ).log_pretty();
        return;
    }

    match configuration.write_json(output) {
        Ok(()) => log(
            LogType::Success,
            "project has been change to: ",
            Some(project_name)
        ),
        Err(err) => err.log_pretty()
    }
}

/// Delete Project
/// 
/// # Description
/// Delete a project from the list of setted project
fn delete_project(configuration: ConfigureCapoo, project_name: String) {
    let mut capoo_projects = match configuration.get_content() {
        Ok(p) => p,
        Err(err) => {
            err.log_pretty();
            return;
        }
    };

    let (status, output, path) = capoo_projects.delete_project_by_name(&project_name);
    if !status {
        CliErr::new(
            DELETE_ERROR_MESSAGE,
            output,
            ErrMessage::NotFound
        ).log_pretty();
        return;
    }

    match configuration.write_json(output) {
        Ok(_) => (),
        Err(err) => err.log_pretty()
    };

    let project_path = PathBuf::from(path);
    match toolbox::delete_folder_from_pathbuf(&project_path) {
        Ok(_) => {
            log(
                LogType::Success,
                "Project has been deleted name: ",
                Some(project_name)
            )
        },
        Err(err) => {
            CliErr::new(
                DELETE_ERROR_MESSAGE,
                String::from(err.description()),
                ErrMessage::IOError
            ).log_pretty()
        }
    }
}