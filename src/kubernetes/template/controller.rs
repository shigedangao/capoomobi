/// Controller
/// 
/// # Path
/// kubernetes/template/controller.rs
/// 
/// # Description
/// Module use to template a K8S controller
pub mod controller {
    use handlebars::Handlebars;
    use crate::kubernetes::controllers::container::container::{KubeContainer};
    use crate::kubernetes::template::helper::helper::{VectorRawHelper};
    use crate::kubernetes::template::helper::common::{TemplateBuilder, handle_error};
    use crate::assets::loader::{K8SAssetType, retrieve_asset_content};
    use crate::errors::cli_error::{CliErr};

    /// Controller Tmpl Builder
    /// 
    /// # Description
    /// Struct use to build the controller template
    pub struct ControllerTmplBuilder {}

    impl TemplateBuilder<KubeContainer, String> for ControllerTmplBuilder {
        fn render(&self, controller: &KubeContainer) -> Result<String, CliErr> {
            let mut handlebars = Handlebars::new();
            // Handlebars helper
            handlebars.register_helper("lilmouse", Box::new(VectorRawHelper));
            
            // Should I put this in an external file ?
            let content = match retrieve_asset_content(K8SAssetType::Controller) {
              Ok(c) => c,
              Err(err) => {
                return Err(err);
              }
            };

            match handlebars.render_template(content.as_str(), controller) {
                Ok(p) => Ok(p),
                Err(e) => {
                    let renderer_error = e.as_render_error();
                    return Err(handle_error(&renderer_error));
                }
            }
        }
    }
}