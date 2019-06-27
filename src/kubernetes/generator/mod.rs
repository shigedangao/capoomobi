/**
 * Generator
 * 
 * A mod helping to generate an abstract syntax tree
 * representing a Kubernetes files
 */
pub mod generator {
  use std::collections;
  use crate::docker::lexer;

  /**
   * Get Kubernets Struct
   * 
   * Get a structure representing the abstract tree
   * of a kubernetes file
   */
  pub fn get_kubernetes_struct(docker_services: lexer::compose::Serice, options: Vec<HashMap<&str, String>>) {

  }
}