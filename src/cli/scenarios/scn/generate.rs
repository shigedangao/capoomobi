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
  match yaml::yaml_parser::parse(sub_action, COMPOSE_FILE_NAME) {
    Ok(_) => println!("generate ok"),
    Err(e) => panic!(e)
  }
}