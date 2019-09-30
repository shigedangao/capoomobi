use std::collections::HashMap;
use crate::docker::{lexer::lexer, parser};
use crate::cli::core::logger::logger::{log, LogType};
use crate::cli::core::input::input;
use crate::kubernetes::{tree, io};
use crate::confiture::config::conf;
use crate::errors::cli_error::ErrHelper;

/// Constant referring to the compose file which need to be parse
const COMPOSE_FILE_NAME: &str = "docker-compose.yaml";
/// Message
const PREPARE_PARSING: &str = "Preparing to parse the docker-compose.yml located on the path: ";

/// Launch
/// 
/// # Description
/// Launch the generate scenario with the command below
/// capoomobi generate <path_to_docker-compose.yaml>
/// e.g: capoomobi generate ./example
/// 
/// # Arguments
/// * `sub_action`: slice of string representing the path
pub fn launch(sub_action: &str) {
  log(
    LogType::Info,
    PREPARE_PARSING,
    Some(String::from(sub_action))
  );

  let yaml_content = match parser::yaml::parse(sub_action, COMPOSE_FILE_NAME) {
    Ok(content) => content,
    Err(e) => {
      e.log_pretty();
      panic!();
    }
  };

  let services = match lexer::get_docker_services(yaml_content) {
    Some(vector) => vector,
    None => {
      //@TODO add error code to panic
      panic!();
    }
  };

  let confiture_opts = conf::load_conf(String::new(), sub_action);
  if let Some(conf) = confiture_opts {
    let kubes = tree::tree::get_kube_abstract_tree(services, conf);
    match io::bootstrap::bootstrap::prepare_kube(&kubes) {
      Ok(()) => io::writer::writer::write_kubernetes_yaml(kubes),
      Err(e) => panic!("error {:?}", e)
    };

    return;
  }

  // @TODO put error here or something...
  panic!()
}
