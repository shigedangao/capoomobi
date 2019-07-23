/**
 * Controller
 * 
 * Templating controller String model
 */
pub mod controller {
  use yaml_rust::{YamlLoader, Yaml};
  use handlebars::Handlebars;
  use crate::kubernetes::controllers::container::container::{KubeContainer};

  /**
   * Template
   * 
   * Template the kuberntes deployment file
   */
  pub fn template(container: KubeContainer) {
    let reg = Handlebars::new();
    
    // basic kuberntes controller template
    // @TODO create helper
    let content = r#"
      apiVersion: apps/v1
      kind: {{ controller_type }}
      metadata:
        name: {{ name }}
        labels:
          {{ #each labels as |label| }}
            {{- label}}
          {{ /each }}
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

    // temmplate the content
    match reg.render_template(content, &container) {
      Ok(p) => println!("{}", p),
      Err(e) => println!("{}", e)
    };
  }
}