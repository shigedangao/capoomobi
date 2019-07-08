use crate::docker::lexer;
use crate::docker::lexer::compose;
use crate::cli::core::logger::logging;
use crate::cli::core::input::input;
use crate::kubernetes::generator;
use std::collections::HashMap;

// Constant referring to the compose file which need to be parse
const COMPOSE_FILE_NAME: &str = "docker-compose.yaml";

/**
 * Launch
 * 
 * Launch the scenario of the generate command
 * 
 * capoomobi generate <docker-compose.yml> path
 */
pub fn launch(sub_action: &str) {
  logging::write(
    logging::LogType::Info,
    "Preparing to parse the docker-compose.yml located on the path: ",
    Some(String::from(sub_action))
  );

  let yaml_content = match lexer::yaml_parser::parse(sub_action, COMPOSE_FILE_NAME) {
    Ok(content) => content,
    Err(e) => {
      return logging::write(logging::LogType::Error, e, None);
    }
  };

  let services = match compose::compose::get_docker_service_structure(yaml_content) {
    Ok(vector) => vector,
    Err(e) => {
      return logging::write(logging::LogType::Error, e, None)
    }
  };

  let prefs = ask_services_details(&services);

  generator::get_kube_abstract_tree(services, prefs);
}

/**
 * Ask services details
 * 
 * Ask questions to users regarding the configuration
 * of the kubernetes files
 */
fn ask_services_details(services: &Vec<compose::compose::Service>) -> HashMap<String, HashMap<&'static str, String>> {
  let mut preferences: HashMap<String, HashMap<&str, String>> = HashMap::new();
  for service in services.into_iter() {
    logging::write(
      logging::LogType::Info,
      format!("{}{}", "Preparing services for: ", service.name).as_str(),
      None
    );

    let replicas = input::get_user_input("Enter number of wishes replicas (e.g: 5)");
    let service_type = input::get_user_input("Enter service type (NodePort, ClusterIP)");
    let nodeport = input::get_user_input("Enter NodePort number if needed (e.g: 30120) or (e.g: N) for no NodePort");
    let controller = input::get_user_input("Enter controller type");

    let mut prefs: HashMap<&str, String> = HashMap::new();
    prefs.insert("replicas", replicas);
    prefs.insert("service", service_type);
    prefs.insert("nodeport", nodeport);
    prefs.insert("controller", controller);

    let name = service.name.to_string();
    preferences.insert(name, prefs);
  }

  return preferences;
}