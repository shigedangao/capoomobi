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
  use std::fs;

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
   * Create a file
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
   * File exist
   * 
   * Check if a file exist
   */
  pub fn file_exist(file_path: &str) -> bool {
    let mut home_dir = get_home_dir();
    home_dir.push(file_path);

    Path::new(&home_dir).exists()
  }

  /**
   * Open and get str content
   * 
   * Open a file 
   */
  pub fn open_get_str_content(file_path: PathBuf) -> io::Result<String> {
    let file = fs::read_to_string(Path::new(&file_path))?;
    Ok(file)
  }
}