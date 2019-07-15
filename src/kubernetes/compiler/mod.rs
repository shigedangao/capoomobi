/**
 * Kube Compiler
 * 
 * Compiler is use to take the datastructure and output the yaml values
 */
pub mod kube_compiler {
  use std::error::Error;
  use crate::kubernetes::generator::{Kube};
  use crate::cli::configurator::config::Helper;
  use crate::cli::core::fs::operations::toolbox;
  use crate::cli::core::logger::logging;

  // Error constant
  const EMPTY_PROJECT_PATH: &str = "The project path is not set";
  const ERROR_CREATE_FOLDER: &str = "Unable to create kubernetes folders";

  /**
   * Compile Kube Vector
   */
  pub fn compile_kube_vector(kubes: Vec<Kube>) -> Result<(), &'static str> {
    logging::write(
      logging::LogType::Info, 
      "Creating kubernetes folders...",
      None
    );
    
    for kube in kubes.into_iter() {
      let project_path = Helper::get_current_project_path();
      if let None = project_path {
        return Err(EMPTY_PROJECT_PATH);
      }

      let folder_creation_res = create_kubernetes_folder(project_path.unwrap(), kube.object.name);
      if let Err(err) = folder_creation_res {
        return Err(err);
      }
    }

    Ok(())
  }

  /**
   * Create Kubernetes folder
   * 
   * Create kubernetes folders based on the saved project path and the parsed services
   */
  fn create_kubernetes_folder(base_path: String, service_name: String) -> Result<(), &'static str> {
    let mut kube_folder_path = String::from(base_path);
    kube_folder_path.push_str("/kube");
    
    let service_full_path = toolbox::concat_string_path(&kube_folder_path, &service_name);
    match toolbox::create_folder_from_pathbuf(service_full_path) {
      Ok(()) => {
        logging::write(
          logging::LogType::Info,
          format!("{}{}", "Successfully creating folder: ", service_name).as_str(),
          None
        );
      },
      Err(err) => panic!(format!("{}{}", ERROR_CREATE_FOLDER, err.description()))
    };
    
    Ok(())
  }
}