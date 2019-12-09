use super::args::{GenerateOptions, retrieve_cmd_options};
use crate::docker::{loader, parser};
use crate::core::logger::{log, LogType};
use crate::kubernetes::tree::{Kube, get_kube_abstract_tree};
use crate::kubernetes::io::{folder, runner, display};
use crate::confiture::config;
use crate::core::errors::cli_error::{CliErr, ErrHelper, ErrMessage};
use crate::core::errors::message::cli::{
    GET_DOCKER_SERVICE_LIST,
    GET_CONFITURE,
    GENERATE_ERROR
};

/// Constant referring to the compose file which need to be parse
const COMPOSE_FILE_NAME: &str = "docker-compose.yaml";
/// Message
const PREPARE_PARSING: &str = "Preparing to parse the docker-compose.yml located on the path: ";

/// Prepare
/// 
/// # Description
/// Load the yaml configuration and retrieve the Docker struct representing the services
fn prepare(path: &str) -> Option<Vec<Kube>> {
    // get the yaml tree
    let yaml_content = match loader::load(path, COMPOSE_FILE_NAME) {
        Ok(content) => content,
        Err(e) => {
            e.log_pretty();
            return None;
        }
    };

    // get an array of docker services
    let services = match parser::get_docker_services(yaml_content) {
        Some(vector) => vector,
        None => {
            CliErr::new(GET_DOCKER_SERVICE_LIST, "", ErrMessage::ParsingError).log_pretty();
            return None;
        }
    };
    
    // load the configuration file
    let conf_opts = config::load_conf(String::new(), path);
    if let Some(conf) = conf_opts {
        let kubes = get_kube_abstract_tree(services, conf);
        return Some(kubes);
    }

    None
}

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
    // Retrieve the kubernetes array which describe every services
    let kubes_opt = prepare(sub_action);
    if kubes_opt.is_none() {
        CliErr::new(GET_CONFITURE, "", ErrMessage::NotFound).log_pretty();
        return;
    }

    let kubes = kubes_opt.unwrap();
    let args  = retrieve_cmd_options(options);
    if args.is_none() {
        create_kubes_files(kubes);
        return;
    }

    execute_with_options(kubes, args.unwrap());
}

/// Execute With Options
/// 
/// # Description
/// Execute a scenario depending of the given options
/// 
/// # Arguments
/// * `options` args::GenerateOptions
fn execute_with_options(k: Vec<Kube>, options: GenerateOptions) {
    match options {
        GenerateOptions::Print => {},
        GenerateOptions::Ingress => {
            
        },
        _ => {

        }
    }
}

fn create_kubes_files(kubes: Vec<Kube>) {
    let res = folder::create(&kubes).and_then(|_| runner::run(kubes));
    match res {
        Ok(()) => log(LogType::Success, "Successfully created the Kubernetes files", None),
        Err(()) => {}
    };
}
