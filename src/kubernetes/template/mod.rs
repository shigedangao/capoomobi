pub mod controller;
pub mod service;
mod helper;

pub mod common {
  use handlebars::{RenderError};
  use crate::errors::cli_error::{CliErr, ErrCode, ErrHelper};

  /// Use as an interface to create a common template builder method
  pub trait TemplateBuilder<T, Y> {
    /// Return a new strucutre
    /// 
    /// * `object` - T (Usually a structure which is from the Kube datastructure)
    fn new(object: T) -> Self;
    /// Return a Kubernetes templated by Handlebars and datastrucutre
    /// 
    /// # Return
    /// 
    /// Option<Y>
    fn template(&self) -> Option<Y>;
  }

  pub fn handle_error(err: Option<&RenderError>) {
    if let Some(details) = err {
      CliErr::new(
        "An error happened while rendering the template",
        String::from(details.desc).as_str(),
        ErrCode::RendererError
      );
    }
  }
}