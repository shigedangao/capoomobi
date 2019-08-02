use crate::cli::core::fs::utility;
use crate::cli::core::logger::logging;
use crate::cli::configurator::configure;
use crate::cli::configurator::builder::builder;

// Error constant
const PATH_ERROR: &str = "Unable to retrieve absolute path {:?}";

/// Launch
/// 
/// # Description
/// Launch the init scenario it is launch with the command below 
/// capoomobi init <project_name> <path>
/// e.g: capoomobi init lilmouse ../lilcat
/// 
/// # Arguments
/// * `name`: slice of string
/// * `options`: reference to Vec of string
/// 
pub fn launch(name: &str, options: &Vec<String>) {
  let optional_path = match retrieve_options_by_idx(options, 0) {
    Some(p) => p, 
    None => String::from("")
  };
  
  // Generate a struct which will be used to build the folders
  let fs_struct = utility::build_base_path(name, optional_path.as_str());
  
  // Build the project folders
  fs_struct.build_compose_dir();
  fs_struct.build_kube_dir();

  // Generate the absolute path from the current set path
  let abs_path = match utility::get_abs_path(&fs_struct.base_path) {
    Ok(p) => p,
    Err(e) => panic!(format!("{}{:?}", PATH_ERROR, e))
  };

  // Checking or creating if the config file exist
  let configurator = match configure::exist_or_create() {
    Ok(f) => f,
    Err(e) => panic!(e)
  };

  let json_str = match builder::generate_project_conf(String::from(name), abs_path) {
    Ok(content) => content,
    Err(e) => panic!(e)
  };

  match configurator.write_json(json_str) {
    Ok(_) => logging::write(
      logging::LogType::Success,
      "Project successfully created",
      Some(fs_struct.get_path_as_string())
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