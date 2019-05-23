use crate::cli::core::parser::options;
use crate::cli::core::fs::utility;
use crate::cli::configurator::configure;

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

  // Checking or creating if the config file exist
  match configure::exist_or_create() {
    Ok(_) => println!("Yay file has been generated"),
    Err(e) => println!("No file can not be create reason: {:?}", e)
  }
}