/// Common module
///
/// # Description
/// Module which handle the trait use by the template
///
/// # Path
/// kubernetes/template
pub mod common {
    use handlebars::{RenderError, Handlebars};
    use serde::{Serialize};
    use crate::assets::loader::{K8SAssetType, retrieve_asset_content};
    use crate::core::errors::cli_error::{CliErr, ErrMessage, ErrHelper};
    use crate::core::errors::message::template::RENDERING;
    use crate::kubernetes::template::formatter;

    /// Use as an interface to create a common template builder method
    pub trait TemplateBuilder {
        /// Render
        ///
        /// # Description
        /// Return a Kubernetes templated by Handlebars and datastrucutre
        ///
        /// # Arguments
        /// * `&self` Self
        /// * `data` &T
        /// * `kind` K8SAssetType
        ///
        /// # Return
        /// Result<Y, CliErr>
        fn render<T>(&self, data: &T, kind: K8SAssetType) -> Result<String, CliErr> where T : Serialize {
            let mut handlebars = Handlebars::new();
            handlebars.register_helper("lilmouse", Box::new(formatter::LilMouseHelper));

            let content_opt = retrieve_asset_content(kind);
            if let Err(e) = content_opt {
                return Err(e);
            }

            let content = content_opt.unwrap();
            match handlebars.render_template(content.as_str(), data) {
                Ok(p) => Ok(p),
                Err(e) => {
                    let renderer_error = e.as_render_error();
                    Err(handle_error(renderer_error))
                }
            }
        }
    }

    /// Handle Error
    ///
    /// # Description
    /// Method use to handle comment templating error
    ///
    /// # Arguments
    /// * `err` Option<&RenderError>
    pub fn handle_error(err: Option<&RenderError>) -> CliErr {
        if let Some(details) = err {
            let detail = &details.desc;
            return CliErr::new(RENDERING, detail.as_str(), ErrMessage::RendererError);
        }

        CliErr::new(RENDERING, "", ErrMessage::RendererError)
    }
}
