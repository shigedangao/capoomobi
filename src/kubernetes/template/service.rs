/// Service
/// 
/// # Path
/// kubernetes/template/service.rs
/// 
/// Module use to create a template of a Kubernetes service
pub mod service {
  use handlebars::Handlebars;
  use crate::kubernetes::controllers::service::service::{KubeService};
  use crate::kubernetes::template::helper::helper::{TemplateHelper};
  use crate::kubernetes::template::common::{TemplateBuilder, handle_error};

  /// Structure use to implement the service template builder
  pub struct ServiceTmplBuilder {
    service: KubeService
  }

  impl TemplateBuilder<KubeService, String> for ServiceTmplBuilder {
    fn new(object: KubeService) -> ServiceTmplBuilder {
      ServiceTmplBuilder {
        service: object
      }
    }

    fn template(&self) -> Option<String> {
      let mut handlebars = Handlebars::new();
      handlebars.register_helper("lilmouse", Box::new(TemplateHelper));

      let content = "
apiVersion: v1
kind: Service
metadata:
  name: {{ name }}
  labels: {{ lilmouse labels 2 }}
spec:
  ports:
  - protocol: TCP
    port: {{ svc_port }}
    targetPort: {{ target_port }}
      ";

      match handlebars.render_template(content, &self.service) {
        Ok(p) => Some(p),
        Err(e) => {
          let renderer_error = e.as_render_error();
          handle_error(&renderer_error);
          None
        }
      }
    }
  }
}