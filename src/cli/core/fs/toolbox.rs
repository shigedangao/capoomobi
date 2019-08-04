extern crate dirs;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::io;
use std::fs;

/// Get Home Dir
/// 
/// # Description
/// Return the home directory e.g ~/ ... path
/// 
/// # Return
/// PathBuf
pub fn get_home_dir() -> PathBuf {
  match dirs::home_dir() {
    Some(path) => path,
    None => PathBuf::new()
  }
}

/// Create File
/// 
/// # Description
/// Create a file based on a file path
/// 
/// # Arguments
/// * `file_path` PathBuf
/// 
/// # Return
/// Result<PathBuf, io::Error>
pub fn create_file(file_path: PathBuf) -> Result<PathBuf, std::io::Error> {
  match File::create(Path::new(&file_path)) {
    Ok(_) => Ok(file_path),
    Err(e) => {
      return Err(e);
    }
  }
}

/// File Exist
/// 
/// # Description
/// Check if a file exist return the PathBuf if true
/// 
/// # Return
/// PathBuf of the file
pub fn file_exist(file_path: &PathBuf) -> Option<PathBuf> {
  if Path::new(&file_path).exists() {
    Some(file_path);
  }

  None
}

/// Open And Read String File
/// 
/// # Description
/// Open a file and return a string representation of the content
/// 
/// # Return
/// Result<String>
pub fn open_and_read_string_file(file_path: &PathBuf) -> io::Result<String> {
  let file = fs::read_to_string(Path::new(file_path))?;
  Ok(file)
}

/// Create Folder From Pathbuf
/// 
/// # Description
/// Create a folder from a PathBuf
/// 
/// # Return
/// Result
pub fn create_folder_from_pathbuf(path: PathBuf) -> io::Result<()> {
  fs::create_dir_all(path)
}

/// Concat String Path
/// 
/// # Description
/// Concat 2 string and return a PathBuf
/// 
/// # Arguments
/// * `base` Reference to a string
/// * `extra` Reference to a string
/// 
/// # Return
/// PathBuf
pub fn concat_string_path(base: &String, extra: &String) -> PathBuf {
  let mut path = PathBuf::from(base);
  path.push(extra);

  return path;
}

/// Get Absolute Path
/// 
/// # Description
/// Get Absolute path of a given PathBuf
/// 
/// # Arguments
/// * `path` PathBuf
/// 
/// # Return
/// Result<PathBuf>
pub fn get_absolute_path(path: &PathBuf) -> std::io::Result<PathBuf> {
  let path = fs::canonicalize(path)?;
  Ok(path)
}

/// Get Path As String
/// 
/// # Description
/// Return a path as a string
/// 
/// # Arguments
/// * `path` PathBuf
/// 
/// # Return
/// Return a string
pub fn get_path_as_string(path: &PathBuf) -> String {
  let path_str = match path.to_str() {
    Some(p) => p,
    None => ""
  };

  String::from(path_str)
}

/// Write Json Content
/// 
/// # Description
/// Write json content to a file
/// 
/// # Arguments
/// * `path` PathBuf
/// * `json` String
/// 
/// # Return
/// io::Result<()>
pub fn write_json_content(path: &PathBuf, json: String) -> std::io::Result<()> {
  fs::write(Path::new(&path), json.as_bytes())?;
  Ok(())
}