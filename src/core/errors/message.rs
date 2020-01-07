/// Cli
///
/// # Description
/// List of error message related to the CLI itself
pub mod cli {
    pub const GET_DOCKER_SERVICE_LIST: &str = "Unable to retrieve list of services in the docker-compose";
    pub const GET_CONFITURE: &str = "Unable to parse the confiture.json file as it's empty";
    pub const RETRIEVE_PATH: &str = "Unable to retrieve absolute path {:?}";
    pub const DELETE_PROJECT: &str = "Unable to delete project";
    pub const UNKNOWN_SCENARIO: &str = "Command not found / supported";
    pub const UNKNOWN_ACTION: &str = "Missing arguments for command";
    pub const GENERATE_ERROR: &str = "An error happened while writing the content to the targeted yaml files";
    pub const INGRESS_CONFIG: &str = "Ingress configuration not provided in confiture.json";
}

pub mod template {
    pub const RENDERING: &str = "An error happened while rendering the template";
    pub const ARGUMENT: &str = "Could not retrieve the argument";
}

pub mod io {
    pub const CREATE_KUBE_FOLDER: &str = "Unable to create kubernetes folder";
    pub const CREATING_FILE: &str = "Error while creating K8S file";
    pub const GET_PROJECT_PATH: &str = "Unable to retrieve the project path";
}

pub mod core {
    pub const PATH_GENERATE_ERROR: &str  = "Error while generating absolute path";
    pub const PATH_GENERATE_REASON: &str = "An error occured while converting the path";
    pub const DELETE_ERROR_MESSAGE: &str = "Unable to delete project";
    pub const SWITCH_ERROR_MESSAGE: &str = "Unable to switch project";

    // Confiture
    pub const CONFIG_GENERATE_ERROR: &str   = "Unable to generate the config file for reason:";
    pub const FILE_NOT_PARSABLE_ERROR: &str = "Unable to parse the config file for reason:";
    pub const DECODE_ERROR: &str            = "Unable to parse the content of the config file for reason:";
    pub const WRITE_JSON_ERROR: &str        = "Unable to write the capoomobi.json file";

    // Serializer trait
    pub const SERIALIZE_ERROR: &str = "Unable to serialize the data type";
}

pub mod assets {
    pub const GET_ERROR: &str = "Unable to get the requested file";
    pub const PARSE_ERROR: &str = "Something went wrong while parsing the content of a template file";
}

pub mod docker {
    // loader
    pub const UNABLE_READ: &str  = "Unable to open the docker-compose.yaml file";
    pub const UNABLE_PARSE: &str = "Unable to parse the docker-compose.yaml for reason: ";
    pub const ABS_PATH: &str   = "Unable to find the docker-compose.yaml file";
}
