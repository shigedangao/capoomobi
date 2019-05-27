/**
 * Configure mod
 * 
 * This mod allow us to
 * - read
 * - parse
 * - set
 * 
 * values in the config.json 
 */
pub mod configure {
  use crate::cli::core::fs::config::config_util;
  use std::io::ErrorKind;
  use std::path::PathBuf;

  // Constant referencing the error
  const CONFIG_GENERATE_ERROR: &str = "Unable to generate the config file"; 

  /**
   * Exist Or Create
   * 
   * Check if the configuration file exist
   * If it exist do nothing otherwise create the configuration file
   */
  pub fn exist_or_create() -> Result<(), String> {
    match config_util::open() {
      Ok(f) => f,
      Err(e) => {
        let kind = e.kind();

        if kind == ErrorKind::NotFound {
          config_util::create();
          return Ok(());
        }

        let err = format!("{}{:?}", CONFIG_GENERATE_ERROR, kind);
        return Err(err);
      }
    };

    return Ok(());
  }

  /**
   * Write
   * 
   * Write the project into the .capoomobi JSON file
   */
  pub fn write(project_name: &str, path: PathBuf) {
    println!("project name: {:?}", project_name);
    println!("path of the project {:?}", path);
  }
}