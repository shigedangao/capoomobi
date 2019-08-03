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
  use crate::cli::configurator::config::Helper;

  // Errors
  const EMPTY_PROJECT_PATH: &str = "The project path is not set";
  // constants
  const KUBE_FOLDER: &str = "/kube";

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

  /**
   * Get Kube Path For Service
   */
  pub fn get_kube_path_for_service(name: &String) -> Option<PathBuf> {
    let project_path_opts = Helper::get_current_project_path();
    if let None = project_path_opts {
      return None;
    }

    let mut path_str = project_path_opts.unwrap();
    path_str.push_str(KUBE_FOLDER);

    Some(concat_string_path(&path_str, &name))
  }

  /**
   * Get Abs Path
   * 
   * Get the absolute path from a string
   */
  pub fn get_absolute_path(path: &PathBuf) -> std::io::Result<PathBuf> {
    let path = fs::canonicalize(path)?;
    Ok(path)
  }

  pub fn get_path_as_string(path: &PathBuf) -> String {
    let path_str = match path.to_str() {
      Some(p) => p,
      None => ""
    };

    String::from(path_str)
  } 
}