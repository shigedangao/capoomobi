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
pub fn launch(name: String, options: Vec<String>) {
  let optional_path = match options::parser_utils::parse_options(options, 3) {
    Some(p) => p, 
    None => "./".to_owned()
  };

  println!("value of path {:?}", optional_path);
  let base_path = concat!("lol");
  fs::create_dir_all(base_path);
}