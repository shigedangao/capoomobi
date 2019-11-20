/// Service
/// 
/// # Path
/// kubernetes/template/service.rs
/// 
/// Module use to create a template of a Kubernetes service
pub mod service {
    use crate::kubernetes::controllers::service::{KubeService};
    use crate::kubernetes::template::helper::common::{TemplateBuilder};

    /// Structure use to implement the service template builder
    pub struct ServiceTmplBuilder {}

    impl TemplateBuilder<KubeService> for ServiceTmplBuilder {}
}