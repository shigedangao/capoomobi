/**
 * Controller
 * 
 * Templating controller String model
 */
pub mod controller {
  use yaml_rust::{YamlLoader, Yaml};
  use handlebars::Handlebars;
  use crate::kubernetes::controllers::container::container::{KubeContainer};
  use crate::kubernetes::controllers::common::KubeHelper;

  /**
   * Template
   * 
   * Template the kuberntes deployment file
   */
  pub fn template(container: KubeContainer) {
    let mut reg = Handlebars::new();
    
    // basic kuberntes controller template
    let content = r#"
      apiVersion: apps/v1
      kind: {{ kind }}
      metadata:
        name: {{ name }}
        labels: {{ labels }}
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

    let tree = container.get_tree_map();
    // temmplate the content
    match reg.render_template(content, &tree) {
      Ok(p) => println!("{}", p),
      Err(e) => println!("{}", e)
    };
  }
}