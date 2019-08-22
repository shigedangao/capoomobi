use serde_json;
use crate::errors::cli_error::{ErrHelper};
use crate::cli::configurator::configure::configure;
use crate::cli::core::logger::logger::{log, LogType};
use crate::cli::scenarios::sketch::helper;

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

  let configuration = match configure::bootstrap_capoo() {
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
    _ => show_current_project(configuration)
  }
}

/// Show Current Project
/// 
/// # Description
/// Show the current setted project
/// 
/// # Arguments
/// * `configuration` configure::ConfigureCapoo struct
fn show_current_project(configuration: configure::ConfigureCapoo) {
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
/// * `configuration` configure::ConfigureCapoo struct
fn list_project(configuration: configure::ConfigureCapoo) {
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
/// * `configuration` configure::ConfigureCapoo struct
/// * `project_name` name of the project
fn switch_project(configuration: configure::ConfigureCapoo ,project_name: String) {
  let mut capoo_projects = match configuration.get_content() {
    Ok(p) => p,
    Err(err) => {
      err.log_pretty();
      panic!();
    }
  };

  let clone_project = &capoo_projects.projects;
  let has_project = clone_project
    .into_iter()
    .filter(|p| p.name == project_name)
    .last();

  if let None = has_project {
    panic!(format!("Unable to find project name with the name of {:?}", project_name));
  }

  capoo_projects.current = String::from(&project_name);
  let serialized_projects = serde_json::to_string(&capoo_projects);

  let result = match serialized_projects {
    Ok(json) => configuration.write_json(json),
    Err(err) => panic!(err)
  };

  match result {
    Ok(()) => log(
      LogType::Success,
      "project has been change to: ",
      Some(project_name)
    ),
    Err(err) => err.log_pretty()
  }
}