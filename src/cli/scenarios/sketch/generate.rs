use super::args::{GenerateOptions, retrieve_cmd_options};
use crate::docker::{loader, parser};
use crate::core::logger::{log, LogType};
use crate::kubernetes::builder;
use crate::kubernetes::io::{
    folder,
    runner,
    display,
    objects
};
use crate::confiture::config;
use crate::confiture::config::{Confiture, ConfigIngress};
use crate::docker::parser::DockerService;
use crate::core::errors::cli_error::{CliErr, ErrHelper, ErrMessage};
use crate::core::errors::message::cli::{
    GET_DOCKER_SERVICE_LIST,
    GET_CONFITURE,
    GENERATE_ERROR,
    INGRESS_CONFIG
};

/// Constant referring to the compose file which need to be parse
const COMPOSE_FILE_NAME: &str = "docker-compose.yaml";

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
    let config = prepare(sub_action);
    if config.is_none() {
        CliErr::new(GET_CONFITURE, "", ErrMessage::NotFound).log_pretty();
        return;
    }

    let args = retrieve_cmd_options(options);
    let (confiture, docker) = config.unwrap();
    execute_with_options(docker, confiture, args);
}

/// Execute With Options
/// 
/// # Description
/// Execute a scenario depending of the given options
/// 
/// # Arguments
/// * `dk` DockerService
/// * `options` args::GenerateOptions
fn execute_with_options(dk: Vec<DockerService>, conf: Confiture, options: Option<GenerateOptions>) {
    let map = conf.get_config_confiture_map();
    let kube_objects = builder::get_basic_objects(&dk, map);
    
    if options.is_none() {
        create_kubes_files(kube_objects);
        return;
    }

    match options.unwrap() {
        GenerateOptions::Print => display::render_kubes_objects(kube_objects),
        GenerateOptions::Ingress => create_ingress_file(&dk, conf.ingress)
    }
}

/// Prepare
/// 
/// # Description
/// Load the yaml configuration and retrieve the Docker struct representing the services
/// 
/// # Arguments
/// * `path` &str
/// 
/// # Return
/// Option<Vec<Kube>>
fn prepare(path: &str) -> Option<(config::Confiture, Vec<DockerService>)> {
    // get the yaml builder
    let yaml_content = match loader::load(path, COMPOSE_FILE_NAME) {
        Ok(content) => content,
        Err(e) => {
            e.log_pretty();
            return None;
        }
    };

    // get a vector of docker services
    let docker_svc = match parser::get_docker_services(yaml_content) {
        Some(vector) => vector,
        None => {
            CliErr::new(GET_DOCKER_SERVICE_LIST, "", ErrMessage::ParsingError).log_pretty();
            return None;
        }
    };
    
    // load the configuration file
    let conf_opts = config::load_conf(String::new(), path);
    if conf_opts.is_none() {
        return None
    }

    Some((conf_opts.unwrap(), docker_svc))
}

/// Create Kubes Files
/// 
/// # Description
/// Create kubernetes files from the Vector of Kube services
/// 
/// # Arguments
/// * `kubes` Vec<Kube>
fn create_kubes_files(kubes: Vec<builder::Kube>) {
    let res = folder::create(&kubes).and_then(|_| runner::run(kubes));
    match res {
        Ok(()) => log(LogType::Success, "Successfully creating the Kubernetes files", None),
        Err(()) => CliErr::new(GENERATE_ERROR, "", ErrMessage::IOError).log_pretty()
    };
}

/// Create Ingress File
/// 
/// # Description
/// Create an ingress file based on the DockerServices and the confiture.json
/// 
/// # Arguments
/// * `dk` &Vec<DockerService>
/// * `ing` Option<ConfigIngress>
fn create_ingress_file(dk: &Vec<DockerService>, ing: Option<ConfigIngress>) {
    let ingress = builder::get_ingress_object(&dk, ing);
    if ingress.is_none() {
        CliErr::new(INGRESS_CONFIG, "", ErrMessage::MissingFieldError).log_pretty();
        return;
    }

    let res = objects::create(ingress.unwrap(), "ingress.yaml", objects::Objects::Ingress);
    match res {
        Ok(()) => log(LogType::Success, "Successfully creating the ingress file", None),
        Err(()) => CliErr::new(GENERATE_ERROR, "", ErrMessage::IOError).log_pretty()
    }
}