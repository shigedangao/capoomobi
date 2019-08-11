pub mod controller;
pub mod service;
mod helper;

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
    fn render(&self, data: &T) -> Option<Y>;
  }

  /// Handle Error
  /// 
  /// # Description
  /// Method use to handle comment templating error
  /// 
  /// # Arguments
  /// * `err` Option ptr of RenderError ptr
  pub fn handle_error(err: &Option<&RenderError>) {
    if let Some(details) = err {
      let detail = &details.desc;

      CliErr::new(
        "An error happened while rendering the template",
        format!("{}", detail),
        ErrCode::RendererError
      );
    }
  }
}