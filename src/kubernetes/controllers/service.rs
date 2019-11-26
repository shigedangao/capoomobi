/// Service
/// 
/// Module use to create a K8S Service datastructure
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use crate::docker::parser::{DockerService};
use crate::confiture::config::{ConfigService};

/// Constant
const EMPTY_PORT: &str = "Ports value is empty";
const SERVICE_FILENAME: &str = "service.yaml";
const SVC_SUFFIX: &str = "-svc";
const PORT_SEPARATOR: &str = ":";

/// Service Type
/// 
/// List supported K8S Service
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ServiceType {
    ClusterIP,
    NodePort,
    LoadBalancer
}

/// Kube Service
/// 
/// Structure use to store the value of a K8S service
#[derive(Debug)]
#[derive(Serialize)]
pub struct KubeService {
    pub path: PathBuf,
    name: String,
    svc_port: u16,
    target_port: u16,
    service_type: ServiceType,
    labels: Vec<String>,
    nodeport: u16,
}

impl KubeService {
    /// New
    /// 
    /// # Description
    /// Create a new KubeService
    /// 
    /// # Arguments
    /// * `dk` &DockerService
    /// * `option` &ConfigService
    /// 
    /// # Return
    /// KubeService
    pub fn new(dk: &DockerService, option: &ConfigService, kube_path: &PathBuf) -> Option<KubeService> {
        let mut svc_path = PathBuf::from(kube_path);
        svc_path.push(SERVICE_FILENAME);
        
        let mut svc_name = String::from(dk.name);
        svc_name.push_str(SVC_SUFFIX);

        if dk.ports.is_empty() {
            return None;
        }

        let mapped_ports: Vec<u16> = dk.ports[0]
            .split(PORT_SEPARATOR)
            .into_iter()
            .map(|port| port.parse::<u16>().unwrap_or(0))
            .collect();

        let svc = KubeService {
            name: svc_name,
            svc_port: mapped_ports[0],
            target_port: mapped_ports[1],
            service_type: option.kind,
            labels: dk.labels.clone(),
            nodeport: option.nodeport,
            path: svc_path
        };

        Some(svc)
    }
}
