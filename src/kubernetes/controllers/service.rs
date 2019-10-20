/// Service
/// 
/// Module use to create a K8S Service datastructure
use serde::{Serialize, Deserialize};
use crate::docker::lexer::Service;
use crate::confiture::config::conf::{ConfigService};

/// Constant
const EMPTY_PORT: &str = "Ports value is empty";

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
/// * `svc` - Service reference
/// * `option` Config reference
/// 
/// # Return
/// * `KubeService` return a KubeService object
pub fn create_kube_service(svc: &Service, option: ConfigService) -> KubeService {
    let mut service_name = String::from(&svc.name);
    service_name.push_str("-svc");
    if svc.ports.is_empty() {
        panic!(EMPTY_PORT);
    }

    let mapped_ports: Vec<u16> = svc.ports[0]
        .split(':')
        .into_iter()
        .map(|port| port.parse::<u16>().unwrap_or(0))
        .collect();

    let kube_service = KubeService {
        name: service_name,
        svc_port: mapped_ports[0],
        target_port: mapped_ports[1],
        service_type: option.kind,
        labels: svc.labels.clone(),
        nodeport: option.nodeport
    };

    return kube_service
}
