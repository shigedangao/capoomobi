use crate::docker::yaml;

const COMPOSE_FILE_NAME: &str = "docker-compose.yaml";

/**
 * Launch
 * 
 * Launch the scenario of the generate command
 * 
 * capoomobi generate <docker-compose.yml> path
 */
pub fn launch(sub_action: &str) {
  yaml::yaml_parser::parse(COMPOSE_FILE_NAME, sub_action);
}