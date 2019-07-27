/// Service
/// 
/// Module use to create a K8S Service datastructure
pub mod service {
  use serde::Serialize;
  use crate::cli::scenarios::scenes::scenes_helper::{EnumHelper};
  
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

  impl EnumHelper<ServiceType> for ServiceType {
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
    labels: Vec<String>
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
  pub fn create_kube_service(name: &String, docker_ports: &Vec<String>, labels: &Vec<String>, service_type_str: &String) -> KubeService {
    let mut service_name = String::from(name);
    service_name.push_str("-svc");

    let service_type = match ServiceType::from_str(service_type_str.to_lowercase().as_str()) {
      Some(svc) => svc,
      None => panic!(format!("{}", UNSUPPORT_CONTROLLER))
    };

    if docker_ports.is_empty() {
      panic!(EMPTY_PORT);
    }

    let mapped_ports: Vec<u16> = docker_ports[0]
      .split(':')
      .into_iter()
      .map(|port| port.parse::<u16>().unwrap_or(0))
      .collect();

    let kube_service = KubeService {
      name: service_name,
      svc_port: mapped_ports[0],
      target_port: mapped_ports[1],
      service_type: service_type,
      labels: labels.clone()
    };

    return kube_service
  }
}