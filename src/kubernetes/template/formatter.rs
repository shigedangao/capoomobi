use handlebars::*;
use crate::core::errors::message::template::{ARGUMENT};

#[derive(Clone, Copy)]
pub struct Dictionnary;

#[derive(Clone, Copy)]
pub struct Mapper;

#[derive(Clone, Copy)]
pub struct Lengthie;

/// Default Padding value
const DEFAULT_PADDING: u64 = 0;

impl HelperDef for Dictionnary {
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
                let value = formatter(key, ident, "-");
                out.write(value.as_str())?;
            }
        }

        Ok(())
    }
}

impl HelperDef for Mapper {
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
                let value = formatter(key, ident, "");
                out.write(value.as_str())?;
            }
        }

        Ok(())
    }
}

impl HelperDef for Lengthie {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        r: &'reg Handlebars,
        ctx: &'rc Context,
        rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output
    ) -> HelperResult {
        let param = h
            .param(0)
            .unwrap()
            .value()
            .as_array();

        if param.is_none() {
            return Ok(());
        }

        let value: Vec<&str> = param
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap_or(""))
            .filter(|v| !v.is_empty())
            .collect();

        if !value.is_empty() {
            return match h.template() {
                Some(ref t) => t.render(r, ctx, rc, out),
                None => Ok(())
            };
        }

        Ok(())
    }
}

/// Formatter
///
/// # Description
/// Format a JsonArray based on the criteria (map or array yaml)
///
/// # Return
/// String
fn formatter(key: &handlebars::JsonValue, ident: u64, prefix: &str) -> String {
    let s: Vec<String> = format!("{}", key)
        .split('=')
        .map(|value| value.replace('"', ""))
        .collect();

    if s.len() > 1 {
        let value = format!("\n {:ident$} {}{}: {}", "", prefix, s[0], s[1], ident=ident as usize);
        return value;
    }

    let value = format!("\n {:ident$}- {}{}", "", prefix, s[0], ident=ident as usize);
    value
}

