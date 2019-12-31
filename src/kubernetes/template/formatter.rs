use handlebars::{Handlebars, HelperDef, RenderContext, RenderError ,Helper, Context, HelperResult, Output};
use crate::core::errors::message::template::{ARGUMENT};

/// Vector Raw Halper
/// 
/// # Description
/// Struct use to handle the task of printing a list to the yaml template (lilmouse)
#[derive(Clone, Copy)]
pub struct LilMouseHelper;

/// Default Padding value
const DEFAULT_PADDING: u64 = 0;

impl HelperDef for LilMouseHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _: &mut RenderContext,
        out: &mut dyn Output
    ) -> HelperResult {
        let list = h.param(0);
        if list.is_none() {
            return Err(RenderError::new(ARGUMENT));
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
