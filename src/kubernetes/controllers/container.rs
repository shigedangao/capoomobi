/**
 * Container
 * 
 * Module use to create a container of a T kubernetes controller
 */
pub mod container {
  use std::collections::HashMap;
  use std::collections::BTreeMap;
  use serde::Serialize;
  use crate::cli::scenarios::scenes::scenes_helper::{EnumHelper};
  use crate::docker::lexer::compose::compose::{Service};
  use crate::kubernetes::controllers::common::{KubeHelper};

  /**
   * List of supported K8S controllers
   * by the generator
   */
  #[derive(Debug)]
  #[derive(Serialize)]
  pub enum ControllerKind {
    Deployment,
    ReplicaSet,
    StatefulSet,
    DaemonSet
  }

  /**
   * Parse string to enum ControllerKind deployment
   */
  impl EnumHelper<ControllerKind> for ControllerKind {
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
  
  /**
   * Kube Container
   * 
   * Structure representing a kubernetes container
   */
  #[derive(Debug)]
  #[derive(Serialize)]
  pub struct KubeContainer {
    pub controller_type: ControllerKind,
    pub name: String,
    pub image: String,
    pub replicas: u8,
    pub commands: Vec<String>,
    pub labels: Vec<String>,
    pub environement: Vec<String>,
  }

  impl KubeHelper<&'static str, String> for KubeContainer {
    fn get_tree_map(&self) -> BTreeMap<&'static str, String> {
      let mut tree = BTreeMap::new();
      tree.insert("name", String::from(&self.name));
      tree.insert("image", String::from(&self.image));

      return tree;
    }
  }

  /**
   * Create Kube Struct
   * 
   * Create a KubeContainer structure
   */
  pub fn create_kube_struct(docker_service: Service, option: &HashMap<&str, String>) -> KubeContainer {
    let mut controller_kind: ControllerKind = ControllerKind::Deployment;
    if let Some(controller) = option.get("controller") {
      controller_kind = ControllerKind::from_str(controller.to_lowercase().as_str()).unwrap();
    }

    let mut replica_count: u8 = 3;
    if let Some(replicas) = option.get("replicas") {
      replica_count = replicas.parse::<u8>().unwrap_or(3);
    }

    println!("value of docker_service {:?}", docker_service);

    let kube_container = KubeContainer {
      controller_type: controller_kind,
      name: docker_service.name,
      image: docker_service.image,
      replicas: replica_count,
      commands: docker_service.commands,
      labels: docker_service.labels,
      environement: docker_service.environment
    };

    return kube_container;
  }
}