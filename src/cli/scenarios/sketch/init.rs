use crate::cli::core::logger::logger::{log, LogType};
use crate::cli::configurator::configure::configure;
use crate::cli::core::fs::configurator::configurator::ConfiguratorIO;
use crate::cli::core::fs::toolbox;
use crate::errors::cli_error::ErrHelper;
use crate::cli::scenarios::sketch::helper;

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
  let project_path = match helper::retrieve_options_by_idx(options, 0) {
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
  let capoo_configurator = match configure::bootstrap_capoo() {
    Ok(f) => f,
    Err(e) => {
      e.log_pretty();
      panic!();
    }
  };

  match capoo_configurator
    .generate_project_conf(String::from(project_name), absolute_path)
    .and_then(|res| capoo_configurator.write_json(res)) {
      Ok(()) => log(
        LogType::Success,
        "Project successfully created",
        Some(toolbox::get_absolute_path_as_string(&config_io.project_path))
      ),
      Err(err) => {
        err.log_pretty();
        panic!();
      }
    }
}