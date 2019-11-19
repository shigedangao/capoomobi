use futures::future::{lazy};
use crate::kubernetes::tree::{Kube};
use crate::kubernetes::controllers::container::{KubeContainer};
use crate::kubernetes::controllers::service::{KubeService};
use crate::kubernetes::template::controller::controller::{ControllerTmplBuilder};
use crate::kubernetes::template::service::service::{ServiceTmplBuilder};
use crate::kubernetes::template::helper::common::TemplateBuilder;
use crate::errors::cli_error::{ErrHelper};

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
            display_controller(&k.object);
            println!("---");
            display_service(&k.service);
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
fn display_controller(kube: &KubeContainer) {
    let ctrl = ControllerTmplBuilder{};
    match ctrl.render(kube) {
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
    match svc.render(kube) {
        Ok(s) => println!("{}", s),
        Err(e) => e.log_pretty()
    }
}