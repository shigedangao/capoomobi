use crate::docker::yaml;
use crate::docker::yaml::compose;
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
  let yaml_content = match yaml::yaml_parser::parse(sub_action, COMPOSE_FILE_NAME) {
    Ok(content) => content,
    Err(e) => {
      return logging::write(logging::LogType::Error, e, None);
    }
  };

  compose::compose::get_docker_service_structure(yaml_content);
}