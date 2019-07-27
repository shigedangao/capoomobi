/**
 * Template helper
 */
pub mod helper {
  use handlebars::{Handlebars, HelperDef, RenderContext, Helper, Context, JsonRender, HelperResult, Output, RenderError};
  
  // Helper struct use for creating 
  // a custom handlebar helper
  #[derive(Clone, Copy)]
  pub struct TemplateHelper;

  impl HelperDef for TemplateHelper {
    fn call<'reg: 'rc, 'rc>(&self, h: &Helper, _: &Handlebars, _: &Context, rc: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
      let array_param = h.param(0).unwrap();
      let indent_param = h.param(1).unwrap();

      // converted value
      let values = array_param.value().as_array();
      let ident = indent_param.value().as_u64().unwrap_or(0);

      if let Some(v) = values {
        for key in v {
          let value = format!("\n {:ident$}- {}", "", key, ident=ident as usize);
          out.write(value.as_str())?;
        }
      }

      Ok(())
    }
  }
}