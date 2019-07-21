pub mod writer;

/**
 * Kube IO
 * 
 * IO is use to prepare folders & files for being write later on
 */
pub mod kube_io {
  use std::path::PathBuf;
  use crate::kubernetes::tree::Tree::{Kube};
  use crate::cli::configurator::config::Helper;
  use crate::cli::core::fs::operations::toolbox;
  use crate::cli::core::logger::logging;
  use crate::errors::cli_error::{CliErr, ErrorHelper, ErrCode};

  // Errors
  const EMPTY_PROJECT_PATH: &str = "The project path is not set";
  const CREATE_FOLDER_ERROR: &str = "Unable to create kubernetes services folders";
  const INITIALIZE_KUBE_FILE: &str = "Unable to initialize kubernetes controller file";
  const INITIALIZE_SVC_FILE: &str = "Unable to initialize kubernetes service file";

  // Constants
  const KUBE_FOLDER: &str = "/kube";
  const CONTROLLER_FILENAME: &str = "controller.yaml";
  const SERVICE_FILENAME: &str = "service.yaml";

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
    
    let mut project_path = match Helper::get_current_project_path() {
      Some(p) => p,
      None => {
        return Err(CliErr::new(EMPTY_PROJECT_PATH, "", ErrCode::NotFound));
      }
    };

    project_path.push_str(KUBE_FOLDER);
    // Create the main folder for each services
    let create_folder_errors: Vec<Result<(), CliErr>> = kubes
      .into_iter()
      .map(|kube| {
        create_kubernetes_folder(&project_path, &kube.object.name)
          .and_then(create_controller_empty_file)
          .and_then(create_service_empty_file)
      })
      .filter(|res| res.is_err())
      .collect();

    if create_folder_errors.len() > 0 {
      return Err(CliErr::new(CREATE_FOLDER_ERROR, "", ErrCode::IOError));
    }

    Ok(())
  }

  /**
   * Create Kubernetes folder
   * 
   * Create kubernetes folders based on the saved project path and the parsed services
   */
  fn create_kubernetes_folder(base_path: &String, service_name: &String) -> Result<PathBuf, CliErr> {
    let svc_path = toolbox::concat_string_path(&base_path, &service_name);
    match toolbox::create_folder_from_pathbuf(svc_path) {
      Ok(()) => {
        let target_path = toolbox::concat_string_path(&base_path, &service_name);
        logging::write(
          logging::LogType::Info,
          format!("{}{}", "Successfully creating folder: ", service_name).as_str(),
          None
        );

        Ok(target_path)
      },
      // @TODO see how to pass the description to the error handler
      Err(_) => Err(CliErr::new(CREATE_FOLDER_ERROR, "", ErrCode::IOError))
    }
  }

  /**
   * Create Kubernetes Empty File
   * 
   * Create kubernetes file based on the path and the service name
   */
  fn create_controller_empty_file(base_path: PathBuf) -> Result<PathBuf, CliErr> {
    // Controller file path
    // Need of cloning the base_path as we need to reuse it later on
    let mut file_path = PathBuf::from(base_path.clone());
    file_path.push(CONTROLLER_FILENAME);
    match toolbox::create_file(file_path) {
      Ok(_) => {
        logging::write(
          logging::LogType::Success,
          "Successfully initialize deployment file",
          None
        );

        Ok(base_path)
      },
      Err(_) => Err(CliErr::new(INITIALIZE_KUBE_FILE, "", ErrCode::IOError))
    }
  }

  /**
   * Create Service Empty File
   * 
   * Create service kubernetes file
   */
  fn create_service_empty_file(base_path: PathBuf) -> Result<(), CliErr> {
    // Service file path
    let mut service_path = PathBuf::from(base_path);
    service_path.push(SERVICE_FILENAME);
    match toolbox::create_file(service_path) {
      Ok(_) => {
        logging::write(
          logging::LogType::Success,
          "Successfully initialize service file",
          None
        );

        Ok(())
      },
      Err(_) => Err(CliErr::new(INITIALIZE_SVC_FILE, "", ErrCode::IOError))
    }
  }
}