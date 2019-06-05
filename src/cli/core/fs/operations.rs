extern crate dirs;

/**
 * Toolbox
 * 
 * A mod executing io::fs methods
 * for interacting with any file
 */
pub mod toolbox {
  use std::fs::File;
  use std::path::Path;
  use std::path::PathBuf;
  use std::io;

  pub fn get_home_dir() -> PathBuf {
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
   */
  pub fn create(file_path: &str) -> Result<PathBuf, std::io::Error> {
    let mut home_dir = get_home_dir();
    home_dir.push(file_path);

    match File::create(Path::new(&home_dir)) {
      Ok(f) => f,
      Err(e) => {
        return Err(e);
      }
    };

    return Ok(home_dir);
  }

  /**
   * Open
   * 
   * Check if the config file exist
   */
  pub fn open(file_path: &str) -> io::Result<PathBuf> {
    let mut home_dir = get_home_dir();
    home_dir.push(file_path);

    File::open(Path::new(&home_dir))?;
    Ok(home_dir)
  }
}