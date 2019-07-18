/**
 * Kube Compiler
 * 
 * Compiler is use to take the datastructure and output the yaml values
 * @TODO rename this module as it's not a compiler at all
 */
pub mod kube_compiler {
  use crate::kubernetes::generator::{Kube};
  use crate::kubernetes::controllers::container::container::ControllerKind;
  use crate::cli::configurator::config::Helper;
  use crate::cli::core::fs::operations::toolbox;
  use crate::cli::core::logger::logging;
  use crate::errors::cli_error::{CliErr, ErrorHelper, ErrCode};

  // Errors
  const EMPTY_PROJECT_PATH: &str = "The project path is not set";
  const CREATE_FOLDER_ERROR: &str = "Unable to create kubernetes services folders";
  
  const KUBE_FOLDER: &str = "/kube";

  /**
   * Compile Kube Vector
   */
  pub fn compile_kube_vector(kubes: Vec<Kube>) -> Result<(), CliErr> {
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
      .map(|kube| create_kubernetes_folder(project_path, kube.object.name))
      .filter(|res| res.is_err())
      .collect();

    if create_folder_errors.len() > 0 {
      // @TODO see if we need to repeat this operation many times.
      for err in create_folder_errors.into_iter() {
        if let Err(e) = err {
          e.log_pretty();
        }
      }

      return Err(CliErr::new(CREATE_FOLDER_ERROR, "", ErrCode::IOError));
    }

    Ok(())
  }

  /**
   * Create Kubernetes folder
   * 
   * Create kubernetes folders based on the saved project path and the parsed services
   */
  fn create_kubernetes_folder(base_path: String, service_name: String) -> Result<(), CliErr> {
    let svc_path = toolbox::concat_string_path(&base_path, &service_name);
    match toolbox::create_folder_from_pathbuf(svc_path) {
      Ok(()) => {
        logging::write(
          logging::LogType::Info,
          format!("{}{}", "Successfully creating folder: ", service_name).as_str(),
          None
        );

        Ok(())
      },
      // @TODO see how to pass the description to the error handler
      Err(_) => Err(CliErr::new(CREATE_FOLDER_ERROR, "", ErrCode::IOError))
    }
  }

  /**
   * Create Kubernetes Empty File
   */
  fn create_kubernetes_empty_file(base_path: String, service_name: String) -> Result<(), CliErr> {
    let mut svc_folder_path = toolbox::concat_string_path(&base_path, &service_name);
    match toolbox::create_file() {
      
    }
  }
}