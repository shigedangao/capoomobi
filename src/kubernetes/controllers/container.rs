pub mod container {
  use std::collections::HashMap;
  use crate::cli::core::logger::logging;
  use crate::cli::scenarios::scenes::scenes_helper::{EnumHelper};
  use crate::docker::lexer::compose::compose::{Service};

  const UNWRAP_ERR: &str = "A value can not be deserialize. Make sure that this value is set";

  /**
   * List of supported K8S controllers
   * by the generator
   */
  pub enum ControllerKind {
    Deployment,
    ReplicaSet,
    StatefulSet,
    DaemonSet
  }

  impl EnumHelper<ControllerKind> for ControllerKind {
    fn from_str(controller: &str) -> Result<ControllerKind, &'static str> {
      match controller {
        "deployment" => Ok(ControllerKind::Deployment),
        "replicaset" => Ok(ControllerKind::ReplicaSet),
        "statefulset" => Ok(ControllerKind::StatefulSet),
        "daemonset" => Ok(ControllerKind::DaemonSet),
        _ => Ok(ControllerKind::Deployment)
      }
    }
  }
  
  /**
   * Kube Container
   * 
   * Structure representing a kubernetes container
   */
  pub struct KubeContainer {
    controller_type: ControllerKind,
    name: String,
    image: String,
    replicas: u8,
    commands: Vec<String>,
    labels: Vec<String>,
    environemnt: Vec<String>,
  }

  /**
   * Create Kube Struct
   * 
   * Create a KubeContainer structure
   */
  pub fn create_kube_struct(docker_service: Service, option: HashMap<&str, String>) {
    let mut controller_kind: ControllerKind;
    if let Some(controller) = option.get("controller") {
      controller_kind = ControllerKind::from_str(controller).unwrap();
    }

    let commands = match docker_service.commands {
      Some(cmds) => cmds,
      None => panic!(format!("{}{}", UNWRAP_ERR, "Docker commands parameters"))
    };
  }
}