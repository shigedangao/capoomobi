
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
  /// * `options` Collection of String Hasmap of type string asked by the CLI
  /// 
  /// # Return
  /// * - `Kube vector` List of Kube
  pub fn get_kube_abstract_tree(docker_services: Vec<Service>, options: HashMap<String, HashMap<&str, String>>) -> Vec<Kube> {
    let kube_containers: Vec<Kube> = docker_services
      .into_iter()
      .filter(|service| options.get(&service.name).is_some())
      .map(|service| {
        let option = options.get(&service.name).unwrap();
        let svc_type = option.get("service").unwrap_or(&String::from("")).to_owned();
        let kube_svc = service::create_kube_service(&service.name, &service.ports, &service.labels, &svc_type);
        let kube_obj = container::create_kube_struct(service, option);
        
        return Kube {
          object: kube_obj,
          service: kube_svc
        }
      })
      .collect();

    return kube_containers
  }
}