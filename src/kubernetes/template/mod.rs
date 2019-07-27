pub mod controller;
pub mod service;
mod helper;

pub mod common {
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
}