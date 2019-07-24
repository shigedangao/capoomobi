/**
 * Template helper
 */
pub mod helper {
  use handlebars::{Handlebars, HelperDef, RenderContext, Helper, Context, JsonRender, HelperResult, Output, RenderError};
  
  #[derive(Clone, Copy)]
  pub struct TemplateHelper;

  impl HelperDef for TemplateHelper {
    fn call<'reg: 'rc, 'rc>(&self, h: &Helper, _: &Handlebars, _: &Context, rc: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
      let param = h.param(0).unwrap();
      let value = param.value().as_array();

      if let Some(v) = value {
        for key in v {
          out.write(format!("- {}", key).as_str())?;
        }
      }

      Ok(())
    }
  }
}