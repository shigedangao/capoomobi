/**
 * Service
 */
pub mod service {
  use crate::cli::scenarios::scenes::scenes_helper::{EnumHelper};
  
  const UNSUPPORT_PARSE_SERVICE: &str = "Unsupported service type";
  const EMPTY_PORT: &str = "Ports value is empty";

  /**
   * Service Type enum
   */
  #[derive(PartialEq)]
  #[derive(Debug)]
  pub enum ServiceType {
    ClusterIP,
    NodePort
  }

  /**
   * Parse string to enum ServiceType
   */
  impl EnumHelper<ServiceType> for ServiceType {
    fn from_str(service_type: &str) -> Result<ServiceType, &'static str> {
      match service_type {
        "clusterip" => Ok(ServiceType::ClusterIP),
        "nodeport" => Ok(ServiceType::NodePort),
        _ => Err(UNSUPPORT_PARSE_SERVICE)
      }
    }
  }

  /**
   * Structure representing a Kubernetes service
   */
  #[derive(Debug)]
  pub struct KubeService {
    container_port: u16,
    target_port: u16,
    service_type: ServiceType,
    labels: Vec<String>
  }

  /**
   * Create Kube Service
   * 
   * Create the service object data structure which represent a service in Kubernetes
   */
  pub fn create_kube_service(docker_ports: &Vec<String>, labels: &Vec<String>, service_type_str: &String) -> KubeService {
    let service_type = match ServiceType::from_str(service_type_str.to_lowercase().as_str()) {
      Ok(svc) => svc,
      Err(e) => panic!(e)
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
      container_port: mapped_ports[0],
      target_port: mapped_ports[1],
      service_type: service_type,
      labels: labels.clone()
    };

    return kube_service
  }
}