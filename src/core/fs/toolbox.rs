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
/// Result<(), io::Error>
pub fn create_file(file_path: &PathBuf) -> Result<(), std::io::Error> {
    match File::create(file_path) {
      Ok(_) => Ok(()),
      Err(e) => Err(e)
    }
}


/// Open File
/// 
/// # Description
/// Open a file and return a string representation of the content
/// 
/// # Arguments
/// * `file_path` &PathBuf
/// 
/// # Return
/// Result<String>
pub fn open_file(file_path: &PathBuf) -> io::Result<String> {
    let file = fs::read_to_string(file_path)?;
    Ok(file)
}

/// Create Folder From Pathbuf
/// 
/// # Description
/// Create a folder from a PathBuf
/// 
/// # Return
/// Result
pub fn create_folder_from_pathbuf(path: &PathBuf) -> io::Result<()> {
    fs::create_dir_all(path)
}

/// Delete Folder From Pathbug
/// 
/// # Description
/// Delete a folder from a pathbuf reference
/// 
/// # Arguments
/// * `path` Reference to a pathbuf
/// 
/// # Return
/// io::Result
pub fn delete_folder_from_pathbuf(path: &PathBuf) -> io::Result<()> {
    fs::remove_dir_all(path)
}

/// Concat String Path
/// 
/// # Description
/// Concat 2 string and return a PathBuf
/// 
/// # Arguments
/// * `base` &str
/// * `extra` &str
/// 
/// # Return
/// PathBuf
pub fn concat_string_path(base: &str, extra: &str) -> PathBuf {
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