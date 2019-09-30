
/// Tree
/// 
/// # Description
/// Module helping to generate a K8S struct with the following syntax
/// - Object (Controller)
/// - Service
pub mod tree {
  use std::collections::HashMap;
  use crate::docker::lexer::lexer::{Service};
  use crate::kubernetes::controllers::container::container;
  use crate::kubernetes::controllers::service::service;
  use crate::confiture::config::conf::{Config};

  /// Structure wrapping the Controller & the Service
  #[derive(Debug)]
  pub struct Kube {
    pub object: container::KubeContainer,
    pub service: service::KubeService
  }

  /// Get Kube Abstract Tree
  /// 
  /// # Description
  /// Retrieve a List of Kube datastructure
  /// 
  /// # Arguments
  /// * `docker_services` List of Docker Services
  /// * `options` Confiture configuration struct
  /// 
  /// # Return
  /// * - `Kube vector` List of Kube
  pub fn get_kube_abstract_tree(docker_services: Vec<Service>, options: HashMap<String, Config>) -> Vec<Kube> {
    let kube_containers: Vec<Kube> = docker_services
      .into_iter()
      .filter(|service| options.get(&service.name).is_some())
      .map(|svc| {
        let option = options.get(&svc.name).unwrap();
        let kube_svc = service::create_kube_service(&svc, option.service.clone());
        let kube_obj = container::create_kube_container(svc, option.deployment.clone());
        
        return Kube {
          object: kube_obj,
          service: kube_svc
        }
      })
      .collect();

    return kube_containers
  }
}