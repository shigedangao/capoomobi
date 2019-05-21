/**
 * Config util
 * 
 * A mod executing io::fs methods
 * for interacting with the ~/.capoomobi.json
 */
pub mod config_util {
  use std::fs::File;
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
  pub fn create(path: &str) {
    match File::create(path) {
      Ok(_) => println!("Config file create"),
      Err(e) => println!("Unable to create config file {:?}", e)
    }
  } 

  /**
   * Exist
   * 
   * Check if the config file exist
   * @TODO add result output
   */
  pub fn exist(path: String) {
    let str_path = path.as_str();
    let f = File::open(str_path);
    match f {
      Ok(_) => println!("file exist"),
      Err(e) => {
        println!("Error while opening file {:?}", e);
        create(str_path);
      }
    }
  }
}