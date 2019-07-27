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
  use crate::kubernetes::template::helper::helper::{TemplateHelper};
  use crate::kubernetes::template::common::{TemplateBuilder, handle_error};

  /// Structure use to implement the controller template builder
  pub struct ControllerTmplBuilder {
    controller: KubeContainer
  }

  impl TemplateBuilder<KubeContainer, String> for ControllerTmplBuilder {
    fn new(object: KubeContainer) -> ControllerTmplBuilder {
      ControllerTmplBuilder {
        controller: object
      }
    }

    fn template(&self) -> Option<String> {
      let mut handlebars = Handlebars::new();
      // Handlebars helper
      handlebars.register_helper("lilmouse", Box::new(TemplateHelper));

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
        ports:
        - containerPort: {{ ports }}
      ";

      match handlebars.render_template(content, &self.controller) {
        Ok(p) => Some(p),
        Err(e) => {
          let renderer_error = e.as_render_error();
          handle_error(renderer_error);
          None
        }
      }
    }
  }
}