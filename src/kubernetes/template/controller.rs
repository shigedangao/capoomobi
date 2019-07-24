/**
 * Controller
 * 
 * Templating controller String model
 */
pub mod controller {
  use yaml_rust::{YamlLoader, Yaml};
  use handlebars::Handlebars;
  use crate::kubernetes::controllers::container::container::{KubeContainer};
  use crate::kubernetes::template::helper::helper::{TemplateHelper};

  /**
   * Template
   * 
   * Template the kuberntes deployment file
   */
  pub fn template(container: KubeContainer) {
    let mut handlebars = Handlebars::new();
    
    // basic kuberntes controller template
    // @TODO create helper


    //     labels: {{ #each labels as |label| }}
    //      - {{ label }}{{ /each }}
    let content = r#"
      apiVersion: apps/v1
      kind: {{ controller_type }}
      metadata:
        name: {{ name }}
        labels:
          {{ lilmouse labels }}
      spec:
        replicas: {{ replicas }}
        selector:
          matchLabels: {{ labels }}
        template:
          metadata:
            labels: {{ labels  }}
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
      Ok(p) => println!("{}", p),
      Err(e) => println!("{}", e)
    };
  }
}