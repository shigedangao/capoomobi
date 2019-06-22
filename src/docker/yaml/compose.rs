/**
 * Compose
 * 
 * List of struct defining the docker-compose file
 */
pub mod Compose {
  use yaml_rust::{yaml};

  // constant error
  const EMPTY_YAML_CONTENT: &str = "Unable to parse empty content of docker-compose.yaml file";
  const SVC_NOT_ARR: &str = "Services attribute is not an array";

  // Service represent a service in the compose file
  // e.g services.portainer
  struct Service {
    name: String,
    image: String,
    command: Vec<String>,
    ports: Vec<String>,
    labels: Vec<String>
  }

  // Services
  // Representing the list of services gather from the 
  // docker-compose.yaml
  #[derive(Debug)]
  struct Services {
    name: String
  }

  /**
   * Generate Docker Project Structure
   * 
   * Generate a struct containing which represent the content
   * of a docker-compose file
   */
  pub fn generate_docker_project_structure(content: Vec<yaml::Yaml>) -> Result<(), &'static str> {
    if content.is_empty() {
      return Err(EMPTY_YAML_CONTENT);
    }

    let compose_content  = &content[0];
    let services_content = compose_content["services"].to_owned();
    let iter = services_content.into_hash().unwrap().into_iter();

    let services: Vec<Services> = iter
      .map(|yaml| {
        let service_name = yaml.0.into_string();
        if let Some(service) = service_name {
          return Services {
            name: service
          }
        } else {
          panic!("Unable to retrieve services");
        }
      })
      .collect();

    println!("value of services {:?}", services);

    Ok(())
  }
}