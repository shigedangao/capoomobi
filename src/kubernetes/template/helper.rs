/// Helper
/// 
/// # Path
/// kubernetes/template
pub mod helper {
  use handlebars::{Handlebars, HelperDef, RenderContext, RenderError ,Helper, Context, HelperResult, Output};
  
  /// Handlebar helper struct
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
  use handlebars::{RenderError};
  use crate::errors::cli_error::{CliErr, ErrCode, ErrHelper};

  /// Use as an interface to create a common template builder method
  pub trait TemplateBuilder<T, Y> {
    /// Return a Kubernetes templated by Handlebars and datastrucutre
    /// 
    /// # Return
    /// 
    /// Option<Y>
    fn render(&self, data: &T) -> Result<Y, CliErr>;
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
        ErrCode::RendererError
      );
    }

    CliErr::new(
      "An error happened while rendering the template",
      String::new(),
      ErrCode::RendererError
    )
  }
}