use serde::{Serialize};
use std::collections::BTreeMap;

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
    pub path: String
}

impl KubeIngress {
    fn new(nodes: BTreeMap<String, String>) {
        
    }
}