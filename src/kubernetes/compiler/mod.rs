/**
 * Kube Compiler
 * 
 * Compiler is use to take the datastructure and output the yaml values
 */
pub mod kube_compiler {
  use crate::kubernetes::generator::{Kube};
  use crate::cli::configurator::config::Helper;

  /**
   * Compile Kube Vector
   */
  pub fn compile_kube_vector(kubes: Vec<Kube>) {
    let project_path = Helper::get_current_project_path();
    println!("value of project path {:?}", project_path);
  }
}