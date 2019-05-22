/**
 * Config util
 * 
 * A mod executing io::fs methods
 * for interacting with the ~/.capoomobi.json
 */
pub mod config_util {
  use std::fs::File;
  use std::path::Path;
  use std::io;
  // Constant defining the paths available
  const CONFIG_FILE_PATH: &str = "~/.capoomobi.json";
  /**
   * Read
   * 
   * Read the capoomobi file
   */
  pub fn read(path: String) {

  }

  /**
   * Create
   * 
   * Create the capoomobi file
   * /!\ When the config file can't be create the CLI should panic
   */
  pub fn create() {
    match File::create(Path::new(CONFIG_FILE_PATH)) {
      Ok(_) => println!("Config file has been create"),
      Err(e) => panic!("An error occured while creating the file {:?}", e)
    }
  }

  /**
   * Exist
   * 
   * Check if the config file exist
   * @TODO add result output
   */
  pub fn exist() -> io::Result<std::fs::File> {
    let f = File::open(Path::new(CONFIG_FILE_PATH))?;
    Ok(f)
  }
}