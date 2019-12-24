/// Lexer
///
/// Retrieve a representation of the docker-compose.yaml file
use std::collections::HashMap;
use yaml_rust::{yaml};

// Error constant
const UNKNOWN_SERVICE_NAME: &str = "unknown";

/// Service represent a service in the compose file
/// e.g services.portainer
#[derive(Debug, Clone)]
pub struct DockerService {
    pub name: String,
    pub image: String,
    pub commands: Vec<String>,
    pub ports: Vec<String>,
    pub labels: Vec<String>,
    pub environment: Vec<String>,
    pub volumes: Vec<String>
}

/// Enumeration Field Type
/// Use to choice which type of field to filter
enum FieldType {
    Single,
    Collection
}

/// Get Supported Attributes
///
/// # Description
/// Retrieve the list of attribute that is supported by each kind of field type
///
/// # Arguments
/// * `field` FieldType
///
/// # Return
/// Vec of static str
fn get_supported_attributes(field: FieldType) -> Vec<&'static str> {
    match field {
        FieldType::Single => vec!["image", "command", "labels"],
        FieldType::Collection => vec!["command", "ports", "labels", "environment", "volumes"]
    }
}

/// Get Docker Services
///
/// # Description
/// Retrieve the list of docker services
///
/// # Arguments
/// * `content` Vector of Yaml struct
///
/// # Return
/// Option of vector services
pub fn get_docker_services(content: Vec<yaml::Yaml>) -> Option<Vec<DockerService>> {
    if content.is_empty() {
        return None;
    }

    let raw_docker_content = &content[0];
    let raw_services = raw_docker_content["services"].to_owned();
    if raw_services.is_null() || raw_services.is_badvalue() {
        return None;
    }

    let raw_hash = raw_services.into_hash();
    if let Some(hashes) = raw_hash {
        let services: Vec<DockerService> = hashes
            .into_iter()
            .map(|yaml| parse_each_yaml_content(yaml.0, yaml.1))
            .collect();

        return Some(services);
    }

    None
}

/// Retrieve Array Or Fallback
///
/// # Description
/// Retrieve an array of the fallback array
///
/// # Argument
/// * `content` Option of refrence to a string
/// * `fallback` Vec<String>
fn get_value(content: Option<&Vec<String>>, fallback: Vec<String>) -> Vec<String> {
    match content {
        Some(value) => {
            if value.is_empty() {
                return fallback;
            }

            value.to_vec()
        },
        None => fallback
    }
}

/// Parse Each Yaml Content
///
/// # Description
/// Retrieve the service struct
///
/// # Arguments
/// * `service_name` yaml::Yaml the service name (e.g: service: web)
/// * `service_content` yaml::Yaml the content of the service (e.g: service: web: image, labels...)
///
/// # Return
/// Service struct
fn parse_each_yaml_content(service_name: yaml::Yaml, service_content: yaml::Yaml) -> DockerService {
    let mut collection_attrs = HashMap::new();
    let single_type_vec: Vec<String> = get_supported_attributes(FieldType::Single)
        .into_iter()
        .map(|key| String::from(service_content[key].as_str().unwrap_or("")))
        .collect();

    for attr in get_supported_attributes(FieldType::Collection) {
        if let Some(service) = service_content[attr].as_vec() {
            let str_vec_fields: Vec<String> = service
                .iter()
                .map(|value| String::from(value.as_str().unwrap_or("")))
                .collect();

            collection_attrs.insert(attr, str_vec_fields);
        } else {
            collection_attrs.insert(attr, Vec::new());
        }
    }

    let fallback_cmd = vec![String::from(&single_type_vec[1])];
    let fallback_label = vec![String::from(&single_type_vec[2])];

    DockerService {
        name:  String::from(service_name.as_str().unwrap_or(UNKNOWN_SERVICE_NAME)),
        image: String::from(&single_type_vec[0]),
        commands: get_value(collection_attrs.get("command"), fallback_cmd),
        labels: get_value(collection_attrs.get("labels"), fallback_label),
        ports: get_value(collection_attrs.get("ports"), Vec::new()),
        environment: get_value(collection_attrs.get("environment"), Vec::new()),
        volumes: get_value(collection_attrs.get("volumes"), Vec::new())
    }
}
