use serde::{Serialize};
use super::service::SVC_SUFFIX;
use crate::confiture::config::ConfigIngress;
use crate::docker::parser::DockerService;
use crate::kubernetes::controllers::service::{get_ports};

/// Kube Ingress
/// 
/// # Description
/// Structure which define a K8S ingress object
#[derive(Serialize)]
pub struct KubeIngress {
    pub name: String,
    pub ip: String,
    pub backend: Vec<IngressBackend>
}

/// Ingress Backend
/// 
/// # Description
/// Struct representing a K8S backend
#[derive(Serialize)]
pub struct IngressBackend {
    pub service_name: String,
    pub service_port: u16,
    pub path: String
}

impl KubeIngress {
    /// New
    /// 
    /// # Description
    /// Retrieve the ingress service file
    /// 
    /// # Arguments
    /// * `nodes` &Vec<KubeService>
    /// * `config` ConfigIngress
    /// 
    /// # Return
    /// KubeIngress
    pub fn new(docker: &Vec<DockerService>, config: ConfigIngress) -> KubeIngress {
        let specified_ingress: Vec<String> = config.services
            .iter()
            .map(|s| String::from(&s.name))
            .collect();

        let n: Vec<IngressBackend> = docker
            .into_iter()
            .filter(|ns| specified_ingress.contains(&ns.name))
            .enumerate()
            .map(|(i, ns)| {
                let mut name = String::from(&ns.name);
                name.push_str(SVC_SUFFIX);
                let mapped_ports = get_ports(&ns.ports[0]);
                IngressBackend {
                    service_name: name,
                    service_port: mapped_ports[0],
                    path: String::from(&config.services[i].path)
                }
            })
            .collect();

        KubeIngress {
            name: String::from("ingress"),
            ip: config.ip,
            backend: n
        }
    }
}