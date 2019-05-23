extern crate dirs;

/**
 * Config util
 * 
 * A mod executing io::fs methods
 * for interacting with the ~/.capoomobi.json
 */
pub mod config_util {
  use std::fs::File;
  use std::path::Path;
  use std::path::PathBuf;
  use std::io;

  // Constant defining the paths available
  const CONFIG_FILE_PATH: &str = ".capoomobi.json";

  fn get_home_dir() -> PathBuf {
    let home_dir = match dirs::home_dir() {
      Some(path) => path,
      None => PathBuf::new()
    };

    return home_dir;
  }

  /**
   * Create
   * 
   * Create the capoomobi file
   * /!\ When the config file can't be create the CLI should panic
   */
  pub fn create() {
    let mut home_dir = get_home_dir();
    home_dir.push(CONFIG_FILE_PATH);

    match File::create(Path::new(&home_dir)) {
      Ok(_) => println!("Config file has been create"),
      Err(e) => panic!("An error occured while creating the file {:?}", e)
    }
  }

  /**
   * Open
   * 
   * Check if the config file exist
   * @TODO add result output
   */
  pub fn open() -> io::Result<std::fs::File> {
    let mut home_dir = get_home_dir();
    home_dir.push(CONFIG_FILE_PATH);

    let f = File::open(Path::new(&home_dir))?;
    Ok(f)
  }
}