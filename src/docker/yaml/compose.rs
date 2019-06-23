/**
 * Compose
 * 
 * List of struct defining the docker-compose file
 */
pub mod compose {
  use yaml_rust::{yaml};

  // constant error
  const EMPTY_YAML_CONTENT: &str = "Unable to parse empty content of docker-compose.yaml file";
  const SVC_NOT_ARR: &str = "Services attribute is not an array";

  // Service represent a service in the compose file
  // e.g services.portainer
  #[derive(Debug, Default)]
  pub struct Service {
    name: String,
    image: String,
    port: String,
    command: Vec<String>,
    ports: Vec<String>,
    labels: Vec<String>
  }

  // Compose Services
  // Representing the list of services gather from the 
  // docker-compose.yaml
  #[derive(Debug, Default)]
  pub struct ComposeServices {
    name: String,
    service: Vec<Service>
  }

  /**
   * Get Supported attributes
   * 
   * Return a vector of the supported list of attributes
   */
  fn get_supported_attributes() -> Vec<&'static str> {
    let vec: Vec<&'static str> = vec![
      "name",
      "image",
      "port",
      "labels",
      "replicas"
    ];

    return vec;
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
        .map(|yaml| get_service(yaml.1))
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
  fn get_service(yaml_service: yaml::Yaml) -> Service {
    let vec: Vec<&str> = get_supported_attributes()
        .into_iter()
        .map(|key| yaml_service[key].as_str().unwrap_or(""))
        .collect();


    Service {
      name: String::from(vec[0]),
      image: String::from(vec[1]),
      port: String::from(vec[2]),
      ..Default::default()
    }
  }
}