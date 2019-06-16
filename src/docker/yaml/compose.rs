/**
 * Compose
 * 
 * List of struct defining the docker-compose file
 */
pub mod compose {

  // Service represent a service in the compose file
  // e.g services.portainer
  pub struct Service {
    name: String,
    image: String,
    command: Vec<String>,
    ports: Vec<String>,
    labels: Vec<String>
  }

  // Services
  // Representing the list of services gather from the 
  // docker-compose.yaml
  pub struct Services {
    name: String,
    services: Vec<Service>
  }
}