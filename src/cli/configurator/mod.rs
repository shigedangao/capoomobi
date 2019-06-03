pub mod json;

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
  use std::path::PathBuf;
  use std::path::Path;
  use std::io::ErrorKind;
  use std::fs;

  // Constant referencing the error
  const CONFIG_GENERATE_ERROR: &str = "Unable to generate the config file for reason:"; 

  // ConfigureFile is a struct which is holding the reference of the config file
  pub struct Configure {
    pub path: PathBuf
  }

  impl Configure {
    /**
     * Write
     * 
     * Write JSON into the .capoomobi.json file
     */
    pub fn write_json(&self, json: String) -> std::io::Result<()> {
      fs::write(Path::new(&self.path), json.as_bytes())?;
      Ok(())
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
  pub fn exist_or_create() -> Result<Configure, String> {
    let existing_path = match config_util::open() {
      Ok(f) => f,
      Err(e) => {
        let kind = e.kind();
        if kind != ErrorKind::NotFound {
          return Err(
            format!("{}{:?}", CONFIG_GENERATE_ERROR, kind)
          );
        }

        let created_path = match config_util::create() {
          Ok(p) => p,
          Err(e) => {
            return Err(
              format!("{}{:?}", CONFIG_GENERATE_ERROR, e)
            );
          }
        };

        let create_conf = Configure{
          path: created_path
        };

        return Ok(create_conf);
      }
    };

    let existing_config = Configure{
      path: existing_path
    };

    return Ok(existing_config);
  }
}