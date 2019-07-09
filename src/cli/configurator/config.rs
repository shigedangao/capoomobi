/**
 * Helper
 */
pub mod Helper {
  use crate::cli::configurator::configure;
  use crate::cli::configurator::builder::builder::{Projects};

  /**
   * Get Current Project Path
   * 
   * Return the actual setted project path
   */
  pub fn get_current_project_path() -> Option<String> {
    let capoos = configure::read_config_file();
    if let Err(_) = capoos {
      return None;
    }

    let unwrapped_capoos = capoos.unwrap();
    let current_name = unwrapped_capoos.current;
    let project_path = unwrapped_capoos
      .projects
      .into_iter()
      .filter(|p| p.name == current_name)
      .map(|p| p.path)
      .fold(String::new(), |_, value| value);
  
    if project_path.is_empty() {
      return None;
    }

    Some(project_path)
  }
}