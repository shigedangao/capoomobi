use serde::{Serialize};
use crate::assets::loader::{K8SAssetType};
use crate::kubernetes::builder::{Kube};
use crate::kubernetes::template::controller::controller::{ControllerTmplBuilder};
use crate::kubernetes::template::service::service::{ServiceTmplBuilder};
use crate::kubernetes::template::helper::common::TemplateBuilder;
use crate::core::errors::cli_error::{ErrHelper};

/// Render Kubes Objects
/// 
/// # Description
/// Display the output of the Kubernetes objects
/// 
/// # Arguments
/// * `kubes` Vec<kubes>
pub fn render_kubes_objects(kubes: Vec<Kube>) {
    let ctrl_tmpl = ControllerTmplBuilder {};
    let svc_tmpl  = ServiceTmplBuilder {}; 

    for k in kubes.into_iter() {
        display_template(&ctrl_tmpl, K8SAssetType::Controller, k.ctrl);
        display_template(&svc_tmpl, K8SAssetType::Service, k.svc);
    }
}

/// Display Template
/// 
/// # Description
/// Render the template as a string and print it to the console
/// 
/// # Arguments
/// * `tmpl` &impl TemplateBuilder
/// * `t` K8SAssetType
/// * `service` T: Serialize
fn display_template<T: Serialize>(tmpl: &impl TemplateBuilder, t: K8SAssetType, service: T) {
    match tmpl.render(&service, t) {
        Ok(s) => {
            println!("{}", s);
            println!("-----");
        },
        Err(e) => e.log_pretty()
    }
}