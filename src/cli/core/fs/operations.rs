extern crate dirs;

/**
 * Toolbox
 * 
 * A mod executing io::fs methods
 * for interacting with any file
 */
pub mod toolbox {
  use std::error::Error;
  use std::fs::File;
  use std::path::Path;
  use std::path::PathBuf;
  use std::io;
  use std::fs;

  /**
   * Get Home Dir
   * 
   * Return the home directory path
   */
  pub fn get_home_dir() -> PathBuf {
    let home_dir = match dirs::home_dir() {
      Some(path) => path,
      None => PathBuf::new()
    };

    return home_dir;
  }

  /**
   * Create File
   * 
   * Create a file
   */
  pub fn create_file(file_path: PathBuf) -> Result<PathBuf, std::io::Error> {
    match File::create(Path::new(&file_path)) {
      Ok(f) => f,
      Err(e) => {
        return Err(e);
      }
    };

    return Ok(file_path);
  }

  /**
   * File exist
   * 
   * Check if a file exist
   */
  pub fn file_exist(file_path: &PathBuf) -> Option<PathBuf> {
    if Path::new(&file_path).exists() {
      Some(file_path);
    }

    None
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

  /**
   * Create Folder
   * 
   * Create a folder based on the path
   */
  pub fn create_folder_from_pathbuf(path: PathBuf) -> io::Result<()> {
    fs::create_dir_all(path)
  }

  /**
   * Concat String Path
   * 
   * Concat string paths
   */
  pub fn concat_string_path(base: &String, extra: &String) -> PathBuf {
    let mut path = PathBuf::from(base);
    path.push(extra);

    return path;
  } 
}