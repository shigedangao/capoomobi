/// Configurator
/// 
/// # Description
/// IO operation related to the capoomobi.json config file
pub mod configurator {
  use std::fs;
  use std::path::PathBuf;
  use std::error::Error;
  use crate::errors::cli_error::{CliErr, ErrCode, ErrHelper};

  /// Constant defining the folders theat need to be generated for a project
  const COMPOSE_FOLDER_PATH: &str = "compose";
  const KUBE_FOLDER_PATH: &str = "kube";

  /// Errors constants
  const COMPOSE_FOLDER_ERR: &str = "Compose folder can't be create";
  const KUBE_FOLDER_ERR: &str = "Kube folder can't be create";

  /// Configurator IO
  /// 
  /// # Description
  /// Structure which contain a set of method allowing you
  /// to manipulate the config file it contain the absolute path
  pub struct ConfiguratorIO {
    pub absolute_path: PathBuf
  }

  impl ConfiguratorIO {
    pub fn build_compose_folder(&self) -> Result<&Self, CliErr> {
      let mut path = PathBuf::from(&self.absolute_path);
      path.push(COMPOSE_FOLDER_PATH);

      match fs::create_dir_all(path) {
        Ok(()) => Ok(&self),
        Err(err) => {
          println!("{:?}", err.description());
          Err(
            CliErr::new(COMPOSE_FOLDER_ERR, String::new(), ErrCode::IOError)
          )
        }
      }
    }

    pub fn build_kube_folder(&self) -> Result<&Self, CliErr> {
      let mut path = PathBuf::from(&self.absolute_path);
      path.push(KUBE_FOLDER_PATH);

      match fs::create_dir_all(path) {
        Ok(()) => Ok(&self),
        Err(err) => {
          println!("{:?}", err.description());
          Err(
            CliErr::new(KUBE_FOLDER_ERR, String::new(), ErrCode::IOError)
          )
        }
      }
    }

    pub fn get_unmoved_path(&self) -> PathBuf {
      PathBuf::from(&self.absolute_path)
    }

    pub fn new(project_name: &str, folder_path: String) -> Self {
      let mut absolute_path = PathBuf::new();

      if folder_path.is_empty() {
        absolute_path.push("./");
      } else {
        absolute_path.push(folder_path);
      }

      absolute_path.push(project_name);
      
      // return the sturct
      ConfiguratorIO {
        absolute_path: absolute_path
      }
    }
  }
}