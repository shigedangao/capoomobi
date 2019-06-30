/**
 * Compose
 * 
 * List of struct defining the docker-compose file
 */
pub mod compose {
  use yaml_rust::{yaml};
  use std::collections::HashMap;

  // constant error
  const EMPTY_YAML_CONTENT: &str = "Unable to parse empty content of docker-compose.yaml file";
  const SVC_NOT_ARR: &str = "Services attribute is not an array";

  // Service represent a service in the compose file
  // e.g services.portainer
  #[derive(Debug)]
  pub struct Service {
    pub name: String,
    pub image: String,
    pub command: String,
    pub label: String,
    pub commands: Option<Vec<String>>,
    pub ports: Option<Vec<String>>,
    pub labels: Option<Vec<String>>,
    pub environment: Option<Vec<String>>,
    pub volumes: Option<Vec<String>>
  }

  // Enumeration Field Kind
  // Use to choice which type of field to filter
  enum FieldKind {
    SingleField,
    ArrayField
  }

  /**
   * Get Supported attributes
   * 
   * Return a vector of the supported list of attributes
   */
  fn get_supported_attributes(field: FieldKind) -> Vec<&'static str> {
    match field {
      FieldKind::SingleField => {
        return vec![
          "image",
          "command",
          "labels",
        ];
      },
      FieldKind::ArrayField => {
        return vec![
          "command",
          "ports",
          "labels",
          "environment",
          "volumes",
        ];
      }
    };
  }

  /**
   * Get docker service structure
   * 
   * Generate a struct containing which represent the content
   * of a docker-compose file
   */
  pub fn get_docker_service_structure(content: Vec<yaml::Yaml>) -> Result<Vec<Service>, &'static str> {
    if content.is_empty() {
      return Err(EMPTY_YAML_CONTENT);
    }

    let compose_content  = &content[0];
    let services_content = compose_content["services"].to_owned();
    let services_hash = services_content.into_hash();

    if let Some(hashes) = services_hash {
      let iter = hashes.into_iter();

      let services: Vec<Service> = iter
        .map(|yaml| get_service(yaml.0, yaml.1))
        .collect();

      return Ok(services);
    }

    Err(SVC_NOT_ARR)
  }


  /**
   * Get Service
   * 
   * Get attribute value for each services
   */
  fn get_service(service_name: yaml::Yaml, yaml_service: yaml::Yaml) -> Service {
    let str_field_vec: Vec<&str> = get_supported_attributes(FieldKind::SingleField)
        .into_iter()
        .map(|key| yaml_service[key].as_str().unwrap_or(""))
        .collect();

    let mut array_attributes = HashMap::new();
    let attributes = get_supported_attributes(FieldKind::ArrayField);

    for attr in attributes.into_iter() {
      let vec = yaml_service[attr].as_vec();
      if let Some(array) = vec {
        let str_vec_fields: Vec<String> = array
          .into_iter()
          .map(|value| value.as_str().unwrap_or(""))
          .map(|each| String::from(each))
          .collect();
        
        array_attributes.insert(attr, str_vec_fields);        
      } else {
        array_attributes.insert(attr, Vec::new());
      }
    }

    Service {
      name: String::from(service_name.as_str().unwrap_or("unknown")),
      // Single line field
      image: String::from(str_field_vec[0]),
      command: String::from(str_field_vec[1]),
      label: String::from(str_field_vec[2]),
      // Array fields
      commands: array_attributes.get("command").cloned(),
      ports: array_attributes.get("ports").cloned(),
      labels: array_attributes.get("label").cloned(),
      environment: array_attributes.get("environment").cloned(),
      volumes: array_attributes.get("volumes").cloned(),
    }
  }
}