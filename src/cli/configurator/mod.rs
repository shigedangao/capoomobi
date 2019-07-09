pub mod builder;
pub mod config;

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
  use crate::cli::core::fs::operations::toolbox;
  use crate::cli::configurator::builder::builder;
  use std::path::PathBuf;
  use std::path::Path;
  use std::fs;

  // Constant defining the paths available
  const CONFIG_FILE_PATH: &str = ".capoomobi.json";

  // Constant referencing the error
  const CONFIG_GENERATE_ERROR: &str = "Unable to generate the config file for reason:"; 
  const FILE_NOT_PARSABLE_ERROR: &str = "Unable to parse the config file for reason:";
  const DECODE_ERROR: &str = "Unable to parse the content of the config file for reason:";

  // Configure is a struct which is holding the reference of the config file
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
    if toolbox::file_exist(CONFIG_FILE_PATH) {
      let mut g_path = toolbox::get_home_dir();
      g_path.push(CONFIG_FILE_PATH);
      
      return Ok(Configure{
        path: g_path
      });
    }

    match toolbox::create(CONFIG_FILE_PATH) {
      Ok(created_path) => Ok(Configure{
        path: created_path
      }),
      Err(e) => {
        return Err(
          format!("{}{:?}", CONFIG_GENERATE_ERROR, e)
        );
      }
    }
  }

  /**
   * Read Config Files
   * 
   * Read the config file and return the set of json objects
   */
  pub fn read_config_file() -> Result<builder::Projects, String>{
    let mut config_file_path = toolbox::get_home_dir();
    config_file_path.push(CONFIG_FILE_PATH);
   
    let contents = match toolbox::open_get_str_content(config_file_path) {
      Ok(c) => c,
      Err(e) => {
        return Err(format!("{}{:?}", FILE_NOT_PARSABLE_ERROR, e));
      }
    };

    let projects = match builder::parse_str_to_struct(&contents) {
      Ok(p) => p,
      Err(e) => {
        return Err(format!("{}{:?}", DECODE_ERROR, e));
      }
    };

    Ok(projects)
  }
}