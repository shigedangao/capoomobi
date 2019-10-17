/// Service
/// 
/// # Path
/// kubernetes/template/service.rs
/// 
/// Module use to create a template of a Kubernetes service
pub mod service {
    use handlebars::Handlebars;
    use crate::kubernetes::controllers::service::service::{KubeService};
    use crate::kubernetes::template::helper::helper::{VectorRawHelper};
    use crate::kubernetes::template::helper::common::{TemplateBuilder, handle_error};
    use crate::assets::loader::{K8SAssetType, retrieve_asset_content};
    use crate::errors::cli_error::{CliErr};

    /// Structure use to implement the service template builder
    pub struct ServiceTmplBuilder {}

    impl TemplateBuilder<KubeService, String> for ServiceTmplBuilder {
        fn render(&self, svc: &KubeService) -> Result<String, CliErr> {
            let mut handlebars = Handlebars::new();
            handlebars.register_helper("lilmouse", Box::new(VectorRawHelper));

            let content = match retrieve_asset_content(K8SAssetType::Service) {
              Ok(c) => c,
              Err(err) => {
                return Err(err);
              }
            };

            match handlebars.render_template(content.as_str(), svc) {
                Ok(p) => Ok(p),
                Err(e) => {
                    let renderer_error = e.as_render_error();
                    return Err(handle_error(&renderer_error));
                }
            }
        }
    }
}