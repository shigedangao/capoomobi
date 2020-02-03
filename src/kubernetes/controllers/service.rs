/// Service
///
/// Module use to create a K8S Service datastructure
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use crate::docker::parser::{DockerService};
use crate::confiture::config::{ConfigService};

/// Constant
pub const SVC_SUFFIX: &str = "-svc";
const SERVICE_FILENAME: &str = "service.yaml";
const PORT_SEPARATOR: &str = ":";

/// Service Type
///
/// List supported K8S Service
#[derive(Serialize, Deserialize, Clone, Debug, Copy, PartialEq)]
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
    pub name: String,
    pub host_port: u16,
    pub target_port: u16,
    pub kind: ServiceType,
    pub labels: Vec<String>,
    pub nodeport: u16,
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
    pub fn new(dk: DockerService, option: &ConfigService, kube_path: &PathBuf) -> Option<KubeService> {
        let mut svc_path = PathBuf::from(kube_path);
        svc_path.push(SERVICE_FILENAME);

        let mut svc_name = String::new();
        svc_name.push_str(dk.name.as_str());
        svc_name.push_str(SVC_SUFFIX);

        if dk.ports.is_empty() {
            return None;
        }

        let mapped_ports = get_ports(&dk.ports[0]);
        let svc = KubeService {
            name: svc_name,
            host_port: mapped_ports[0],
            target_port: mapped_ports[1],
            kind: option.kind,
            labels: dk.labels,
            nodeport: option.nodeport,
            path: svc_path
        };

        Some(svc)
    }
}

/// Get Ports
///
/// # Description
/// Retrieve ports
///
/// # Arguments
/// * `ports` &str
///
/// # Return
/// Vec<u16>
pub fn get_ports(ports: &str) -> Vec<u16> {
    let mapped_ports: Vec<u16> = ports
        .split(PORT_SEPARATOR)
        .map(|port| port.parse::<u16>().unwrap_or(0))
        .collect();

    mapped_ports
}
