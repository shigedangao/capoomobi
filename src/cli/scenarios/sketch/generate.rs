use crate::docker::{lexer, parser};
use crate::cli::core::logger::{log, LogType};
use crate::kubernetes::tree;
use crate::kubernetes::io::{bootstrap, writer, display};
use crate::confiture::config::conf;
use crate::errors::cli_error::{CliErr, ErrHelper, ErrMessage};
use super::args;

/// Constant referring to the compose file which need to be parse
const COMPOSE_FILE_NAME: &str = "docker-compose.yaml";
/// Message
const PREPARE_PARSING: &str = "Preparing to parse the docker-compose.yml located on the path: ";
/// Errors
const ERROR_GET_DOCKER_SERVICE_LIST: &str = "Unable to retrieve list of services in the docker-compose";
const ERROR_GET_CONFITURE: &str = "Unable to parse the confiture.json file as it's empty";

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

    let yaml_content = match parser::parse(sub_action, COMPOSE_FILE_NAME) {
        Ok(content) => content,
        Err(e) => {
            e.log_pretty();
            return;
        }
    };

    let services = match lexer::get_docker_services(yaml_content) {
        Some(vector) => vector,
        None => {
            CliErr::new(
                ERROR_GET_DOCKER_SERVICE_LIST,
                String::new(),
                ErrMessage::ParsingError
            ).log_pretty();

            return;
        }
    };

    let confiture_opts = conf::load_conf(String::new(), sub_action);
    if let Some(conf) = confiture_opts {
        let kubes = tree::get_kube_abstract_tree(services, conf);
        let cmd_opt = args::retrieve_cmd_options(options);

        // For the moment only one option is support as such we're not checking the value of the cmd
        if let Some(cmd) = cmd_opt {
            execute_with_options(cmd, kubes);
            return;
        }

        match bootstrap::bootstrap::prepare_kube(&kubes) {
            Ok(()) => writer::writer::write_kubernetes_yaml(kubes),
            Err(e) => e.log_pretty()
        };

        return;
    }

    CliErr::new(
        ERROR_GET_CONFITURE,
        String::new(),
        ErrMessage::MissingFieldError
    ).log_pretty();
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
