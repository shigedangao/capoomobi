use serde_json;
use crate::errors::cli_error::{ErrHelper};
use crate::cli::configurator::configure;
use crate::cli::core::logger::logging;

/// Launch
/// 
/// # Description
/// Launch the switch scenario. This scenario allow you to change project below is an example
/// capoomobi switch <project_name>
/// 
/// # Arguments
/// * `project_name` slice of a string
pub fn launch(project_name: &str) {
  let capoo_configurator = match configure::configure::bootstrap_capoo() {
    Ok(conf) => conf,
    Err(err) => {
      err.log_pretty();
      panic!();
    }
  };

  let mut capoo_projects = match capoo_configurator.get_content() {
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

  capoo_projects.current = String::from(project_name);
  let serialized_projects = serde_json::to_string(&capoo_projects);

  let result = match serialized_projects {
    Ok(json) => capoo_configurator.write_json(json),
    Err(err) => panic!(err)
  };

  match result {
    Ok(()) => logging::write(
      logging::LogType::Success,
      "project has been change to: ",
      Some(String::from(project_name))
    ),
    Err(err) => err.log_pretty()
  }
}