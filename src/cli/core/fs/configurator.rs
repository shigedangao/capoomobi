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
    pub project_path: PathBuf
  }

  impl ConfiguratorIO {
    /// Build Compose Folder
    /// 
    /// # Description
    /// Build composer folder where the docker-compose file will be store
    /// 
    /// # Arguments
    /// * `self` ConfiguratorIO struct
    /// 
    /// # Return
    /// * `self` reference to ConfiguratorIO struct
    pub fn build_compose_folder(&self) -> Result<&Self, CliErr> {
      let mut path = PathBuf::from(&self.project_path);
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

    /// Build Kube Folder
    /// 
    /// # Description
    /// Build kube folder where the K8S folders & files will be store
    /// 
    /// # Arguments
    /// * `self` ConfiguratorIO struct
    /// 
    /// # Return
    /// * `self` reference to ConfiguratorIO struct
    pub fn build_kube_folder(&self) -> Result<&Self, CliErr> {
      let mut path = PathBuf::from(&self.project_path);
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

    /// New
    /// 
    /// # Description
    /// New return a new structure instance of the ConfiguratorIO structure
    /// 
    /// # Arguments
    /// * `project_name` slice of a string
    /// * `folder_path`: String
    /// 
    /// # Return
    /// Return a new ConfiguratorIO structure
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
        project_path: absolute_path
      }
    }
  }
}