/**
 * Kube Compiler
 * 
 * Compiler is use to take the datastructure and output the yaml values
 */
pub mod kube_compiler {
  use tokio::prelude::Future;
  use tokio::fs::{create_dir};
  use tokio::fs::File;
  use crate::kubernetes::generator::{Kube};
  use crate::cli::configurator::config::Helper;
  use crate::cli::core::fs::operations::toolbox;

  // Error constant
  const EMPTY_PROJECT_PATH: &str = "The project path is not set";

  /**
   * Compile Kube Vector
   */
  pub fn compile_kube_vector(kubes: Vec<Kube>) -> Result<(), &'static str> {
    let project_path = Helper::get_current_project_path();
    if let None = project_path {
      return Err(EMPTY_PROJECT_PATH);
    }

    let create_object_tasks = kubes
      .into_iter()
      .map(|kube| create_kube_folders(kube.object.name, project_path.unwrap()))
      .collect();
  }

  fn create_kube_folders(name: String, path: String) -> Future {
    let path = toolbox::concat_string_path(path, name);
    let stuff = create_dir(path)
      .and_then(|_| File::create("ntm"))
      .map(|res| println!("{:?}", res))
      .map_err(|err| println!("{:?}", err));

    return stuff;
  }
}