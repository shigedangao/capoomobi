use crate::cli::core::parser::options;
use crate::cli::core::fs::utility;


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
  // fs::create_dir_all(base_path);
  println!("value of path {:?}", fs_struct.base_path);
  fs_struct.build_compose_dir();
  fs_struct.build_kube_dir();
}