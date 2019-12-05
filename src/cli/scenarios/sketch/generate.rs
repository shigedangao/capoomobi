use super::args;
use crate::docker::{loader, parser};
use crate::core::logger::{log, LogType};
use crate::kubernetes::tree;
use crate::kubernetes::io::{bootstrap, runner, display};
use crate::confiture::config;
use crate::core::errors::cli_error::{CliErr, ErrHelper, ErrMessage};
use crate::core::errors::message::cli::{
    GET_DOCKER_SERVICE_LIST,
    GET_CONFITURE
};

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
pub fn launch(sub_action: &str, options: &Vec<String>) {
    log(LogType::Info, PREPARE_PARSING, Some(String::from(sub_action)));

    let yaml_content = match loader::load(sub_action, COMPOSE_FILE_NAME) {
        Ok(content) => content,
        Err(e) => {
            e.log_pretty();
            return;
        }
    };

    let services = match parser::get_docker_services(yaml_content) {
        Some(vector) => vector,
        None => {
            CliErr::new(GET_DOCKER_SERVICE_LIST, "", ErrMessage::ParsingError).log_pretty();
            return;
        }
    };

    let confiture_opts = config::load_conf(String::new(), sub_action);
    if let Some(conf) = confiture_opts {
        let kubes = tree::get_kube_abstract_tree(services, conf);
        let cmd_opt = args::retrieve_cmd_options(options);

        // For the moment only one option is support as such we're not checking the value of the cmd
        if let Some(cmd) = cmd_opt {
            execute_with_options(cmd, kubes);
            return;
        }

        return match bootstrap::bootstrap::prepare_kube(&kubes) {
            Ok(()) => runner::run(kubes),
            Err(e) => e.log_pretty()
        };
    }

    CliErr::new(GET_CONFITURE, "", ErrMessage::MissingFieldError).log_pretty();
}

/// Execute With Options
/// 
/// # Description
/// Execute a scenario depending of the given options
/// 
/// # Arguments
/// * `options` args::GenerateOptions
fn execute_with_options(options: args::GenerateOptions, kubes: Vec<tree::Kube>) {
    match options {
        args::GenerateOptions::Print => display::compile_kubernetes_yaml(kubes),
        args::GenerateOptions::Ingress => {
            
        },

    }
}
