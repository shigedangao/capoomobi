use crate::cli::core::parser::options;
use crate::cli::core::fs::utility;
use crate::cli::core::logger::logging;
use crate::cli::configurator::configure;
use crate::cli::configurator::builder::builder;

const PATH_ERROR: &str = "Unable to retrieve absolute path {:?}";

/**
 * Launch
 * 
 * Launch the scenario of the init command
 * Structure of how the cmd is called
 * 
 * capoomobi init <name> <path>
 */
pub fn launch(name: &str, options: Vec<String>) {
  let optional_path = match options::parser_utils::parse_options(options, 3) {
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