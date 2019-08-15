/// Helper
/// 
/// # Path
/// kubernetes/template/path
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