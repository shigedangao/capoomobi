use crate::core::errors::cli_error::{CliErr, ErrMessage, ErrHelper};
use crate::core::errors::message::cli::{DELETE_PROJECT};
use crate::core::configurator::configure::{exist, CapooConfig};
use crate::core::logger::{log, LogType};
use crate::cli::scenarios::sketch::args;
use crate::core::fs::toolbox;
use crate::core::serde_utils::{SerdeUtil};

// Errors

/// Project
///
/// # Description
/// Project allow you to see the current & switch the project to an other. This scenario allow you to change project below is an example
/// capoomobi project current
/// capoomobi project switch <project_name>
///
/// # Arguments
/// * `project_name` slice of a string
pub fn launch(main_action: &str, options: &[String]) {
    let arg = match args::retrieve_options_by_idx(options, 0) {
        Some(arg) => arg,
        None => String::new()
    };

    let cnf = exist();
    if let Some(conf) = cnf {
        match main_action {
            "current" => show_current_project(conf),
            "switch"  => switch_project(conf, arg),
            "list"    => list_project(conf),
            "delete"  => delete_project(conf, arg),
            _ => show_current_project(conf)
        }
    }
}

/// Show Current Project
///
/// # Description
/// Show the current setted project
///
/// # Arguments
/// * `configuration` CapooConfig struct
fn show_current_project(configuration: CapooConfig) {
    match configuration.get_content() {
        Ok(p) => log(LogType::Info, "the current project in use is:", Some(p.current)),
        Err(err) => err.log_pretty()
    }
}

/// List Project
///
/// # Description
/// list the known project
///
/// # Arguments
/// * `configuration` CapooConfig struct
fn list_project(configuration: CapooConfig) {
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
/// * `conf` CapooConfig struct
/// * `pname` name of the project
fn switch_project(conf: CapooConfig ,pname: String) {
    let projects_opt = conf.get_content();
    if let Err(e) = projects_opt {
        e.log_pretty();
        return;
    }

    let projects = projects_opt.unwrap();
    let new_projects_opt = projects.switch_project(&pname);
    if let Err(e) = new_projects_opt {
       e.log_pretty();
       return;
    }

    let new_projects = new_projects_opt.unwrap();
    match new_projects
        .serialize()
        .and_then(|res| conf.write_json_file(res)) {
            Ok(()) => log(LogType::Success, "project has been change to: ", Some(pname)),
            Err(err) => err.log_pretty()
        };
}

/// Delete Project
///
/// # Description
/// Delete a project from the list of setted project
///
/// # Arguments
/// * `conf` CapooConfig
/// * `pname` String
fn delete_project(conf: CapooConfig, pname: String) {
    let projects_opt = conf.get_content();
    if let Err(e) = projects_opt {
        e.log_pretty();
        return;
    }

    let projects = projects_opt.unwrap();
    let new_projects = projects.delete_project_by_name(&pname);
    if let Err(e) = new_projects {
        e.log_pretty();
        return;
    }

    let projects = new_projects.unwrap();
    let serialize_res = projects.0.serialize().and_then(|res| conf.write_json_file(res));
    if let Err(e) = serialize_res {
        e.log_pretty();
        return;
    }

    match toolbox::delete_folder_from_pathbuf(&projects.1) {
        Ok(_) => log(LogType::Success, "Project has been deleted name: ", Some(pname)),
        Err(err) => CliErr::new(DELETE_PROJECT, &err.to_string(), ErrMessage::IOError).log_pretty()
    }
}
