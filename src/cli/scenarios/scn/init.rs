use crate::cli::core::parser::options;
use crate::cli::core::fs::utility;
use crate::cli::configurator::configure;
use crate::cli::configurator::builder::json_util;

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
  
  let fs_struct = utility::build_base_path(name, optional_path.as_str());
  fs_struct.build_compose_dir();
  fs_struct.build_kube_dir();

  let abs_path = match fs_struct.get_abs_path() {
    Ok(p) => p,
    Err(e) => panic!(format!("{}{:?}", PATH_ERROR, e))
  };

  // Checking or creating if the config file exist
  let configurator = match configure::exist_or_create() {
    Ok(f) => f,
    Err(e) => panic!(e)
  };

  let json_str = match json_util::generate_project_conf(String::from(name), abs_path) {
    Ok(content) => content,
    Err(e) => panic!(e)
  };

  match configurator.write_json(json_str) {
    Ok(_) => println!("yes has been write"),
    Err(e) => panic!(e)
  }
}