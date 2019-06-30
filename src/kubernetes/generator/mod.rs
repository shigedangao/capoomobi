/**
 * Generator
 * 
 * A mod helping to generate an abstract syntax tree
 * representing a Kubernetes files
 */
pub mod generator {
  use std::collections::HashMap;
  use crate::docker::lexer::compose::compose::{Service};
  use crate::kubernetes::controllers::container::container;
  use crate::kubernetes::controllers::service::service;

  /**
   * Kube
   * 
   * Structure representing a kube file component
   */
  pub struct Kube {
    object: container::KubeContainer,
    service: service::KubeService,
  }

  /**
   * Get Kube Abstract Tree
   * 
   * Get a structure representing the tree of a k8s file
   */
  pub fn get_kube_abstract_tree(docker_services: Vec<Service>, options: Vec<HashMap<&str, String>>) {
    let objects = docker_services
      .into_iter()
      
  }
}