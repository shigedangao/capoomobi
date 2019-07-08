pub mod controllers;

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
  #[derive(Debug)]
  pub struct Kube {
    object: container::KubeContainer,
    service: service::KubeService,
  }

  /**
   * Get Kube Abstract Tree
   * 
   * Get a structure representing the tree of a k8s file
   */
  pub fn get_kube_abstract_tree(docker_services: Vec<Service>, options: HashMap<String, HashMap<&str, String>>) {
    let kube_containers: Vec<Kube> = docker_services
      .into_iter()
      .filter(|service| options.get(&service.name).is_some())
      .map(|service| {
        let option = options.get(&service.name).unwrap();
        let svc_type = option.get("service").unwrap_or(&String::from("")).to_owned();
        let kube_svc = service::create_kube_service(&service.ports, &service.labels, &svc_type);
        let kube_obj = container::create_kube_struct(service, option);
        
        return Kube {
          object: kube_obj,
          service: kube_svc
        }
      })
      .collect();

    println!("value of kube containers {:?}", kube_containers);
  }
}