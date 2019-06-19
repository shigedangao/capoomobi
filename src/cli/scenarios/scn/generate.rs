use crate::docker::yaml;
use crate::cli::core::logger::logging;

const COMPOSE_FILE_NAME: &str = "docker-compose.yaml";

/**
 * Launch
 * 
 * Launch the scenario of the generate command
 * 
 * capoomobi generate <docker-compose.yml> path
 */
pub fn launch(sub_action: &str) {
  match yaml::yaml_parser::parse(sub_action, COMPOSE_FILE_NAME) {
    Ok(_) => logging::write(
      logging::LogType::Success,
      "Kubernetes files generated",
      None
    ),
    Err(e) => logging::write(
      logging::LogType::Error,
      "An error occurred", 
      Some(e.to_owned())
    )
  }
}