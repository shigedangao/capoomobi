use futures::future::{lazy};
use crate::assets::loader::{K8SAssetType};
use crate::kubernetes::tree::{Kube};
use crate::kubernetes::controllers::controller::{KubeController};
use crate::kubernetes::controllers::service::{KubeService};
use crate::kubernetes::template::controller::controller::{ControllerTmplBuilder};
use crate::kubernetes::template::service::service::{ServiceTmplBuilder};
use crate::kubernetes::template::helper::common::TemplateBuilder;
use crate::core::errors::cli_error::{ErrHelper};

/// Compile Kubernetes Yaml
/// 
/// # Description
/// Display the output of a Kubernetes Yaml
/// 
/// # Arguments
/// * `kubes` Vec<kubes>
pub fn compile_kubernetes_yaml(kubes: Vec<Kube>) {
    tokio::run(lazy(|| {
        for k in kubes.into_iter() {
            display_controller(&k.ctrl);
            println!("---");
            display_service(&k.svc);
        }

        Ok(())
    }))
}

/// Display Controller
/// 
/// # Description
/// Display the value of the controller
/// 
/// # Arguments
/// * `kube` KubeContainer reference
fn display_controller(kube: &KubeController) {
    let ctrl = ControllerTmplBuilder{};
    match ctrl.render(kube, K8SAssetType::Controller) {
        Ok(s) => println!("{}", s),
        Err(e) => e.log_pretty()
    }
}

/// Display Service
/// 
/// # Description
/// Display the value of the service
/// 
/// # Arguments
/// * `kube` KubeService reference
fn display_service(kube: &KubeService) {
    let svc = ServiceTmplBuilder{};
    match svc.render(kube, K8SAssetType::Service) {
        Ok(s) => println!("{}", s),
        Err(e) => e.log_pretty()
    }
}