/// Helper
/// 
/// # Path
/// kubernetes/template/path
pub mod helper {
  use handlebars::{Handlebars, HelperDef, RenderContext, Helper, Context, HelperResult, Output};
  
  /// Handlebar helper struct
  #[derive(Clone, Copy)]
  pub struct VectorRawHelper;
  #[derive(Clone, Copy)]
  pub struct VectorContentHelper;

  /// Default Padding value
  const DEFAULT_PADDING: u64 = 0;

  impl HelperDef for VectorRawHelper {
    fn call<'reg: 'rc, 'rc>(&self, h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
      let values = h.param(0).unwrap().value().as_array();
      let ident = h.param(1).unwrap().value().as_u64().unwrap_or(DEFAULT_PADDING);

      if let Some(v) = values {
        for key in v {
          let value = format!("\n {:ident$}- {}", "", key, ident=ident as usize);
          out.write(value.as_str())?;
        }
      }

      Ok(())
    }
  }

  impl HelperDef for VectorContentHelper {
    fn call<'reg: 'rc, 'rc>(&self, h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
      let values = h.param(0).unwrap().value().as_array();
      let ident = h.param(1).unwrap().value().as_u64().unwrap_or(DEFAULT_PADDING);
      let prefix = h.param(2).unwrap().value().as_str().unwrap_or("");

      if let Some(v) = values {
        for key in v {
          let value = format!("\n {:ident$}- {}:{}", "", prefix, key, ident=ident as usize);
          out.write(value.as_str())?;
        }
      }

      Ok(())
    }
  }
}