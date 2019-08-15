/// Controller
/// 
/// # Path
/// kubernetes/template/controller.rs
/// 
/// # Description
/// Module use to template a K8S controller
pub mod controller {
  use handlebars::Handlebars;
  use crate::kubernetes::controllers::container::container::{KubeContainer};
  use crate::kubernetes::template::helper::helper::{VectorRawHelper};
  use crate::kubernetes::template::common::{TemplateBuilder, handle_error};

  /// Controller Tmpl Builder
  /// 
  /// # Description
  /// Struct use to build the controller template
  pub struct ControllerTmplBuilder {}

  impl TemplateBuilder<KubeContainer, String> for ControllerTmplBuilder {
    fn render(&self, controller: &KubeContainer) -> Option<String> {
      let mut handlebars = Handlebars::new();
      // Handlebars helper
      handlebars.register_helper("lilmouse", Box::new(VectorRawHelper));

      let content = "
apiVersion: apps/v1
kind: {{ controller_type }}
metadata:
  name: {{ name }}
  labels: {{ lilmouse labels 2 }}
spec:
  replicas: {{ replicas }}
  selector:
    matchLabels: {{ lilmouse labels 6 }}
  template:
    metadata:
      labels: {{ lilmouse labels 7 }}
    spec:
      containers:
      - name: {{ name }}
        image: {{ image }}
        ports: {{ #each ports as |p| }}
          - containerPort: {{p}}
        {{ /each }}";

      match handlebars.render_template(content, controller) {
        Ok(p) => Some(p),
        Err(e) => {
          println!("value of error {:?}", e);
          let renderer_error = e.as_render_error();
          handle_error(&renderer_error);
          None
        }
      }
    }
  }
}