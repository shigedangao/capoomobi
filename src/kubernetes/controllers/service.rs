/// Service
/// 
/// Module use to create a K8S Service datastructure
pub mod service {
  use serde::Serialize;
  use std::collections::HashMap;
  use crate::docker::lexer::lexer::Service;
  use crate::kubernetes::controllers::helper::{KubeEnumHelper};

  const UNSUPPORT_CONTROLLER: &str = "Unsupport type of controller";
  const EMPTY_PORT: &str = "Ports value is empty";

  /// Service Type
  /// 
  /// List supported K8S Service
  #[derive(PartialEq)]
  #[derive(Debug)]
  #[derive(Serialize)]
  pub enum ServiceType {
    ClusterIP,
    NodePort
  }

  impl KubeEnumHelper<ServiceType> for ServiceType {
    fn from_str(service_type: &str) -> Option<ServiceType> {
      match service_type {
        "clusterip" => Some(ServiceType::ClusterIP),
        "nodeport" => Some(ServiceType::NodePort),
        _ => None
      }
    }
  }

  /// Kube Service
  /// 
  /// Structure use to store the value of a K8S service
  #[derive(Debug)]
  #[derive(Serialize)]
  pub struct KubeService {
    name: String,
    svc_port: u16,
    target_port: u16,
    service_type: ServiceType,
    labels: Vec<String>,
    nodeport: u16
  }

  /// Create Kube Service
  /// 
  /// # Description
  /// Create a kubernetes service
  /// 
  /// # Arguments
  /// * `name` - Pointer which reference to a String
  /// * `docker_ports` - Pointer which reference to a List of String
  /// * `labels` - Pointer which refrence to a List of String
  /// * `service_type_str` - Pointer which reference to a String
  /// 
  /// # Return
  /// * `KubeService` return a KubeService object
  pub fn create_kube_service(svc: &Service, option: &HashMap<&str, String>) -> KubeService {
    let mut service_name = String::from(&svc.name);
    service_name.push_str("-svc");

    let svc_type = option.get("service").unwrap_or(&String::new()).to_owned();
    let service_type = match ServiceType::from_str(svc_type.to_lowercase().as_str()) {
      Some(svc) => svc,
      None => panic!(format!("{}", UNSUPPORT_CONTROLLER))
    };


    if svc.ports.is_empty() {
      panic!(EMPTY_PORT);
    }

    let mapped_ports: Vec<u16> = svc.ports[0]
      .split(':')
      .into_iter()
      .map(|port| port.parse::<u16>().unwrap_or(0))
      .collect();

    let nodeport = option.get("nodeport").unwrap_or(&String::new()).to_owned();

    let kube_service = KubeService {
      name: service_name,
      svc_port: mapped_ports[0],
      target_port: mapped_ports[1],
      service_type: service_type,
      labels: svc.labels.clone(),
      nodeport: nodeport.parse::<u16>().unwrap_or(0)
    };

    return kube_service
  }
}