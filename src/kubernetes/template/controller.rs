/**
 * Controller
 * 
 * Templating controller String model
 */
pub mod controller {
  use handlebars::Handlebars;
  use crate::kubernetes::controllers::container::container::{KubeContainer};
  use crate::kubernetes::template::helper::helper::{TemplateHelper};

  /**
   * Template
   * 
   * Template the kuberntes deployment file
   */
  pub fn template(container: KubeContainer) -> Option<String> {
    let mut handlebars = Handlebars::new();
    
    let content = r#"
      apiVersion: apps/v1
      kind: {{ controller_type }}
      metadata:
        name: {{ name }}
        labels: {{ lilmouse labels 8 }}
      spec:
        replicas: {{ replicas }}
        selector:
          matchLabels: {{ lilmouse labels 8 }}
        template:
          metadata:
            labels: {{ lilmouse labels 8 }}
          spec:
            containers:
            - name: {{ name }}
              image: {{ image }}
              ports:
              - containerPort: {{ ports }}
    "#;

    // Handlebars helper
    handlebars.register_helper("lilmouse", Box::new(TemplateHelper));
    // temmplate the content
    match handlebars.render_template(content, &container) {
      Ok(p) => Some(p),
      Err(e) => {
        // @TODO use CliErr
        println!("{}", e);
        None
      }
    }
  }
}