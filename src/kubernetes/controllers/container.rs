/// Container
/// 
/// Module use to create a K8S controller datastructure
pub mod container {
  use std::collections::HashMap;
  use std::iter::Iterator;
  use std::path::PathBuf;
  use serde::Serialize;
  use crate::docker::lexer::lexer::{Service};
  use crate::cli::configurator::config;
  use crate::cli::core::fs::toolbox;
  use crate::kubernetes::controllers::helper::{KubeEnumHelper};

  /// Constant
  const CONTROLLER_FILENAME: &str = "controller.yaml";
  const SERVICE_FILENAME: &str = "service.yaml";

  /// Controller Kind
  /// 
  /// List type of supported K8S controller
  #[derive(Debug)]
  #[derive(Serialize)]
  pub enum ControllerKind {
    Deployment,
    ReplicaSet,
    StatefulSet,
    DaemonSet
  }

  impl KubeEnumHelper<ControllerKind> for ControllerKind {
    fn from_str(controller: &str) -> Option<ControllerKind> {
      match controller {
        "deployment" => Some(ControllerKind::Deployment),
        "replicaset" => Some(ControllerKind::ReplicaSet),
        "statefulset" => Some(ControllerKind::StatefulSet),
        "daemonset" => Some(ControllerKind::DaemonSet),
        _ => Some(ControllerKind::Deployment)
      }
    }
  }
  
  /// KubeContainer
  /// 
  /// # Description
  /// Structure which define the representation of a K8S controller definition
  #[derive(Debug)]
  #[derive(Serialize)]
  pub struct KubeContainer {
    pub controller_type: ControllerKind,
    pub name: String,
    pub image: String,
    pub replicas: u8,
    // Path
    pub path: PathBuf,
    pub controller_path: PathBuf,
    pub service_path: PathBuf,
    // Lists
    pub commands: Vec<String>,
    pub labels: Vec<String>,
    pub environement: Vec<String>,
    pub ports: Vec<u16>
  }

  /// Create Kube Struct
  /// 
  /// # Description
  /// Create K8S data structure
  /// 
  /// # Arguments
  /// * `docker_service` - Service structure
  /// * `option` - Pointer reference to a HashMap<slice str, String>
  /// 
  /// # Return
  /// * `KubeContainer` return the datastructure
  /// 
  pub fn create_kube_struct(docker_service: Service, option: &HashMap<&str, String>) -> KubeContainer {
    let mut ctrl: ControllerKind = ControllerKind::Deployment;
    if let Some(controller) = option.get("controller") {
      ctrl = ControllerKind::from_str(controller.to_lowercase().as_str()).unwrap();
    }

    let mut replica_count: u8 = 3;
    if let Some(replicas) = option.get("replicas") {
      replica_count = replicas.parse::<u8>().unwrap_or(3);
    }

    let base_path = get_kube_path_for_service(&docker_service.name).unwrap_or(PathBuf::new());
    let mut controller_path = PathBuf::from(&base_path);
    controller_path.push(CONTROLLER_FILENAME);

    let mut service_path = PathBuf::from(&base_path);
    service_path.push(SERVICE_FILENAME);

    let internal_ports = retrieve_container_port(docker_service.ports);

    let kube_container = KubeContainer {
      controller_type: ctrl,
      name: docker_service.name,
      image: docker_service.image,
      replicas: replica_count,
      commands: docker_service.commands,
      labels: docker_service.labels,
      environement: docker_service.environment,
      ports: internal_ports,
      // Paths
      path: base_path,
      controller_path: controller_path,
      service_path: service_path,
    };

    return kube_container;
  }

  /// Retrieve Container Port
  /// 
  /// # Description
  /// Retrieve internal docker container ports (never though that splitting a vector of string would be hard)
  /// 
  /// # Arguments
  /// * `docker_ports` Vec<String> ports of a docker services
  /// 
  /// # Return
  /// Vec<u16>
  fn retrieve_container_port(docker_ports: Vec<String>) -> Vec<u16> {
    let internal_port: Vec<u16> = docker_ports
      .into_iter()
      .map(|p| {
        let port: String = p.split(':') 
          .into_iter()
          .enumerate()
          .filter(|(idx, _)| idx > &(0 as usize))
          .map(|(_, value)| String::from(value))
          .last()
          .unwrap_or(String::new());

        return port;
      })
      .map(|port_string| port_string.parse::<u16>().unwrap_or(0))
      .collect();
  
    return internal_port;
  }

  /// Get Kube Path For Service
  /// 
  /// # Description
  /// Retrieve the current setted project path and bind the kube folder
  /// 
  /// # Return
  /// Optional PathBuf
  fn get_kube_path_for_service(name: &String) -> Option<PathBuf> {
    let project_path_opts = config::get_current_project_path();
    if let None = project_path_opts {
      return None;
    }

    let path_str = project_path_opts.unwrap();
    Some(toolbox::concat_string_path(&path_str, &name))
  }
}