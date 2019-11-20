/// Helper
/// 
/// # Path
/// kubernetes/template
pub mod utils {
    use handlebars::{Handlebars, HelperDef, RenderContext, RenderError ,Helper, Context, HelperResult, Output};

    /// Vector Raw Halper
    /// 
    /// # Description
    /// Struct use to habdle the task of printing a list to the yaml template
    #[derive(Clone, Copy)]
    pub struct VectorRawHelper;

    /// Default Padding value
    const DEFAULT_PADDING: u64 = 0;

    impl HelperDef for VectorRawHelper {
        fn call<'reg: 'rc, 'rc>(&self, h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
            let list = h.param(0);
            if let None = list {
                return Err(RenderError::new("param array not found"));
            }

            let ident = match h.param(1) {
                Some(v) => v.value().as_u64().unwrap_or(DEFAULT_PADDING),
                None => DEFAULT_PADDING
            };

            if let Some(v) = list.unwrap().value().as_array() {
                for key in v {
                    let value = format!("\n {:ident$}- {}", "", key, ident=ident as usize);
                    out.write(value.as_str())?;
                }
            }

            Ok(())
        }
    }
}

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
    use super::utils;
    use crate::assets::loader::{K8SAssetType, retrieve_asset_content};
    use crate::errors::cli_error::{CliErr, ErrMessage, ErrHelper};

    /// Use as an interface to create a common template builder method
    pub trait TemplateBuilder<T> {
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
        fn render(&self, data: &T, kind: K8SAssetType) -> Result<String, CliErr> where T : Serialize {
            let mut handlebars = Handlebars::new();
            handlebars.register_helper("lilmouse", Box::new(utils::VectorRawHelper));

            let content = match retrieve_asset_content(kind) {
                Ok(c) => c,
                Err(err) => {
                    return Err(err);
                }
            };

            match handlebars.render_template(content.as_str(), data) {
                Ok(p) => Ok(p),
                Err(e) => {
                    let renderer_error = e.as_render_error();
                    Err(handle_error(&renderer_error))
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
    /// * `err` Option ptr of RenderError ptr
    pub fn handle_error(err: &Option<&RenderError>) -> CliErr {
        if let Some(details) = err {
            let detail = &details.desc;
            return CliErr::new(
                "An error happened while rendering the template",
                format!("{}", detail),
                ErrMessage::RendererError
            );
        }

        CliErr::new(
            "An error happened while rendering the template",
            String::new(),
            ErrMessage::RendererError
        )
    }
}