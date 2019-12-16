/// Ingress
/// 
/// # Path
/// kubernetes/template/ingress.rs
/// 
/// # Description
/// Module which is use to create a Kubernetes ingress template
use crate::kubernetes::template::helper::common::{TemplateBuilder};

/// Ingress Tmpl Builder
/// 
/// # Description
/// Struct use to build the ingress template
pub struct IngressTmplBuilder {}

impl TemplateBuilder for IngressTmplBuilder {}