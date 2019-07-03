/**
 * Service
 */
pub mod service {
  use std::collections::HashMap;
  use crate::cli::scenarios::scenes::scenes_helper::{EnumHelper};
  use crate::docker::lexer::compose::compose::{Service};
  
  const UNSUPPORT_PARSE_SERVICE: &str = "Unsupported service type";
  const WRONG_FORMAT: &str = "Wrong port format for NodePort";

  /**
   * Service Type enum
   */
  #[derive(PartialEq)]
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
  pub struct KubeService {
    port: String,
    service_type: String,
    nodePort: u16,
    labels: Vec<String>
  }

  pub fn create_kube_service(docker_service: Service, option: &HashMap<&str, String>) {
    let service_type_str = option.get("service").unwrap_or(&String::from(""));
    let service_type = match ServiceType::from_str(service_type_str) {
      Ok(svc) => svc,
      Err(e) => panic!(e)
    };

    if docker_service.ports.len() == 1 && service_type == ServiceType::NodePort {
      panic!(WRONG_FORMAT);
    }

    return kube_service;
  }
}