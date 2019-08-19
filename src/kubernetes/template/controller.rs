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
  use crate::kubernetes::template::helper::common::{TemplateBuilder, handle_error};
  use crate::errors::cli_error::{CliErr};

  /// Controller Tmpl Builder
  /// 
  /// # Description
  /// Struct use to build the controller template
  pub struct ControllerTmplBuilder {}

  impl TemplateBuilder<KubeContainer, String> for ControllerTmplBuilder {
    fn render(&self, controller: &KubeContainer) -> Result<String, CliErr> {
      let mut handlebars = Handlebars::new();
      // Handlebars helper
      handlebars.register_helper("lilmouse", Box::new(VectorRawHelper));
      
      // Should I put this in an external file ?
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
        Ok(p) => Ok(p),
        Err(e) => {
          let renderer_error = e.as_render_error();
          return Err(handle_error(&renderer_error));
        }
      }
    }
  }
}