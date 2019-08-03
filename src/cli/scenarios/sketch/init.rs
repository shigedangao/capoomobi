use crate::cli::core::logger::logging;
use crate::cli::configurator::configure;
use crate::cli::configurator::builder::builder;
use crate::cli::core::fs::configurator::configurator::ConfiguratorIO;
use crate::cli::core::fs::toolbox;
use crate::errors::cli_error::ErrHelper;

/// Error constant
const PATH_ERROR: &str = "Unable to retrieve absolute path {:?}";

/// Launch
/// 
/// # Description
/// Launch the init scenario it is launch with the command below 
/// capoomobi init <project_name> <path>
/// e.g: capoomobi init lilmouse ../lilcat
/// 
/// # Arguments
/// * `project_name`: slice of string
/// * `options`: reference to Vec of string
pub fn launch(project_name: &str, options: &Vec<String>) {
  let project_path = match retrieve_options_by_idx(options, 0) {
    Some(p) => p, 
    None => String::new()
  };

  let config_io = ConfiguratorIO::new(project_name, project_path);
  let io_result = config_io
    .build_compose_folder()
    .and_then(|res: &ConfiguratorIO| res.build_kube_folder());

  if let Err(err) = io_result {
    err.log_pretty();
    panic!(err);
  }

  // Generate the absolute path from the current set path
  let absolute_path = match toolbox::get_absolute_path(&config_io.project_path) {
    Ok(p) => p,
    Err(e) => panic!(format!("{}{:?}", PATH_ERROR, e))
  };

  // Checking or creating if the config file exist
  let configurator = match configure::exist_or_create() {
    Ok(f) => f,
    Err(e) => panic!(e)
  };

  let json_str = match builder::generate_project_conf(String::from(project_name), absolute_path) {
    Ok(content) => content,
    Err(e) => panic!(e)
  };

  match configurator.write_json(json_str) {
    Ok(_) => logging::write(
      logging::LogType::Success,
      "Project successfully created",
      Some(toolbox::get_path_as_string(&config_io.project_path))
    ),
    Err(e) => panic!(e)
  }
}

/// Retrieve options by idx
/// 
/// # Description
/// Retrieve an optional String by it's index
/// 
/// # Arguments
/// * `vec` Reference to a vector of string
/// * `idx` usize
/// 
/// # Return
/// * `options` option of string
fn retrieve_options_by_idx(vec: &Vec<String>, idx: usize) -> Option<String> {
  if vec.is_empty() {
    return None;
  }

  match vec.get(idx) {
    Some(res) => Some(res.to_string()),
    None => None
  }
}