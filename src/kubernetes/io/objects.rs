use serde::{Serialize};
use crate::kubernetes::template::ingress::IngressTmplBuilder;
//use crate::kubernetes::io::output::{write_component};

/// Objcts
/// 
/// # Description
/// List of K8S Objects (exclude from controller & services)
pub enum Objects {
    Ingress
}

/// Create
/// 
/// # Description
/// Write an object T with the provided name and template
pub fn create<T: Serialize>(data: T, name: String, kind: Objects) {
    let template = match kind {
        Objects::Ingress => IngressTmplBuilder {}
    };

    //let task = 
}