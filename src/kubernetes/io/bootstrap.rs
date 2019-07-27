/**
 * Kube IO
 * 
 * IO is use to prepare folders & files for being write later on
 */
pub mod bootstrap {
  use std::path::PathBuf;
  use crate::kubernetes::tree::tree::{Kube};
  use crate::cli::core::fs::operations::toolbox;
  use crate::cli::core::logger::logging;
  use crate::errors::cli_error::{CliErr, ErrHelper, ErrCode};

  /**
   * Prepare Kube
   * 
   * Prepare the folder and the kubernetes files
   */
  pub fn prepare_kube(kubes: &Vec<Kube>) -> Result<(), CliErr> {
    logging::write(
      logging::LogType::Info, 
      "Creating kubernetes folders...",
      None
    );
    
    for kube in kubes.into_iter() {
      create_kubernetes_folder(&kube.object.path);
      create_file(&kube.object.controller_path, "controller");
      create_file(&kube.object.service_path, "service");
    }

    Ok(())
  }

  /**
   * Create Kubernetes folder
   * 
   * Create kubernetes folders based on the saved project path and the parsed services
   */
  fn create_kubernetes_folder(path: &PathBuf) {
    match toolbox::create_folder_from_pathbuf(PathBuf::from(path)) {
      Ok(()) => {
        logging::write(
          logging::LogType::Info,
          "Successfully creating folder",
          None
        );
      },
      Err(err) => {
        CliErr::new("", "", ErrCode::IOError).log_pretty();
        panic!(err);
      }
    }
  }

  /**
   * Create Kubernetes Empty File
   * 
   * Create kubernetes file based on the path and the service name
   */
  fn create_file(path: &PathBuf, message: &str) {
    match toolbox::create_file(PathBuf::from(path)) {
      Ok(_) => {
        logging::write(
          logging::LogType::Success,
          format!("Successfully initialize {} file", message).as_str(),
          None
        );
      },
      Err(err) => {
        CliErr::new("", "", ErrCode::IOError).log_pretty();
        panic!(err);
      }
    }
  }
}