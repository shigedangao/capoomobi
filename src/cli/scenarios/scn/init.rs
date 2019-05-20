use std::fs;
use crate::cli::parser::options;
use crate::cli::config;


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
    None => String::from("./")
  };

  let mut base_path = String::new();
  base_path.push_str(optional_path.as_str());
  base_path.push_str(name);
  
  // fs::create_dir_all(base_path);
  println!("value of path {:?}", base_path);
}