/// Container
///
/// Module use to create a K8S controller datastructure
use std::iter::Iterator;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use crate::docker::parser::{DockerService};
use crate::confiture::config::{ConfigDeployment};

/// Constant
const CONTROLLER_FILENAME: &str = "controller.yaml";

/// Controller Kind
///
/// List type of supported K8S controller
#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub enum ControllerKind {
    Deployment,
    ReplicaSet,
    StatefulSet,
    DaemonSet
}

/// KubeContainer
///
/// # Description
/// Structure which define the representation of a K8S controller definition
#[derive(Serialize, Debug)]
pub struct KubeController {
    pub ctrl: ControllerKind,
    pub name: String,
    pub image: String,
    pub replicas: u8,
    // Path
    pub path: PathBuf,
    // Lists
    pub commands: Vec<String>,
    pub labels: Vec<String>,
    pub env: Vec<String>,
    pub ports: Vec<u16>
}

impl KubeController {
    /// New
    ///
    /// # Description
    /// Create a new KubeController
    ///
    /// # Arguments
    /// * `dk` Docker service struct
    /// * `option` ConfigDeployment
    ///
    /// # Return
    /// Option<KubeController>
    pub fn new(dk: DockerService, option: &ConfigDeployment, kube_path: &PathBuf) -> Option<KubeController> {
        // Controller filename
        let mut ctrl_path = PathBuf::from(&kube_path);
        ctrl_path.push(CONTROLLER_FILENAME);

        let ctrl = KubeController {
            ctrl: option.controller,
            name: dk.name,
            image: dk.image,
            replicas: option.replicas,
            commands: dk.commands,
            labels: dk.labels,
            env: dk.environment,
            ports: retrieve_container_port(dk.ports),
            path: ctrl_path
        };

        Some(ctrl)
    }
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
    docker_ports
    .into_iter()
    .map(|p| {
        p.split(':')
            .enumerate()
            .filter(|(idx, _)| *idx > (0 as usize))
            .map(|(_, value)| String::from(value))
            .last()
            .unwrap_or_default()
    })
    .map(|port_string| port_string.parse::<u16>().unwrap_or(0))
    .collect()
}
