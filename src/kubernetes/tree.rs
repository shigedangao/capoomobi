
/// Tree
/// 
/// # Description
/// Module helping to generate a K8S struct with the following syntax
/// - Object (Controller)
/// - Service
use std::iter::Iterator;
use std::collections::HashMap;
use std::path::PathBuf;
use crate::docker::parser::{DockerService};
use crate::kubernetes::controllers::controller::{KubeController};
use crate::kubernetes::controllers::service::{KubeService};
use crate::confiture::config::{Config};
use crate::core::configurator::config;
use crate::core::fs::toolbox;

/// Structure wrapping the Controller & the Service
#[derive(Debug)]
pub struct Kube {
    pub ctrl: KubeController,
    pub svc: KubeService,
    pub project_path: PathBuf
}

/// Get Kube Abstract Tree
/// 
/// # Description
/// Retrieve a List of Kube datastructure
/// 
/// # Arguments
/// * `dk_vec` List of Docker Services
/// * `options` Confiture configuration struct
/// 
/// # Return
/// * - `Kube vector` List of Kube
pub fn get_kube_abstract_tree(dk_vec: Vec<DockerService>, options: HashMap<String, Config>) -> Vec<Kube> {
    let kube_containers: Vec<Kube> = dk_vec
        .into_iter()
        .filter(|service| options.get(&service.name).is_some())
        .filter_map(|dk| {
            let base_path = get_kube_path(&dk.name).unwrap_or(PathBuf::new());
            let option = options.get(&dk.name).unwrap();

            let kube_svc = KubeService::new(dk.clone(), &option.service, &base_path);
            let kube_ctrl = KubeController::new(dk.clone(), &option.deployment, &base_path);

            if kube_svc.is_none() {
                return None;
            }

            Some(
                Kube {
                    ctrl: kube_ctrl.unwrap(),
                    svc: kube_svc.unwrap(),
                    project_path: base_path
                }
            )
        })
        .collect();
        
    kube_containers
}

/// Get Kube Path
/// 
/// # Description
/// Create the destination path of the service based on the setted project path
/// 
/// # Return
/// Optional PathBuf
fn get_kube_path(name: &String) -> Option<PathBuf> {
    let project_path_opts = config::get_current_project_path();
    if let None = project_path_opts {
        return None;
    }

    let path_str = project_path_opts.unwrap();
    Some(toolbox::concat_string_path(&path_str, &name))
}