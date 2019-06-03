mod json;

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
  use crate::cli::configurator::json::json_util;
  use std::fs::File;
  use std::io::ErrorKind;
  use std::path::PathBuf;

  // Constant referencing the error
  const CONFIG_GENERATE_ERROR: &str = "Unable to generate the config file for reason:"; 

  // ConfigureFile is a struct which is holding the reference of the config file
  pub struct ConfigureFile {
    pub file: File
  }

  impl ConfigureFile {
    /**
     * Write
     * 
     * Write the project into the .capoomobi JSON file
     */
    pub fn write_object(&self, project_name: &str, path: PathBuf) {
      println!("project name: {:?}", project_name);
      println!("path of the project {:?}", path);
      match json_util::generate_project_conf(String::from(project_name), path) {
        Ok(content) => println!("value of content {:?}", content),
        Err(e) => println!("Error for json {:?}", e)
      };
    }
  }
  /**
   * Exist Or Create
   * 
   * Check if the configuration file exist
   * If the error is of a kind NotFound then
   *  - create the config file
   *  - otherwise return an error
   * 
   * If the error is different then return an error
   */
  pub fn exist_or_create() -> Result<ConfigureFile, String> {
    let existing_file = match config_util::open() {
      Ok(f) => f,
      Err(e) => {
        let kind = e.kind();
        if kind != ErrorKind::NotFound {
          return Err(
            format!("{}{:?}", CONFIG_GENERATE_ERROR, kind)
          );
        }

        let file = match config_util::create() {
          Ok(conf_file) => conf_file,
          Err(e) => {
            return Err(
              format!("{}{:?}", CONFIG_GENERATE_ERROR, e)
            );
          }
        };

        let create_config_file = ConfigureFile{
          file: file
        };

        return Ok(create_config_file);
      }
    };

    let existing_config_file = ConfigureFile{
      file: existing_file
    };

    return Ok(existing_config_file);
  }
}