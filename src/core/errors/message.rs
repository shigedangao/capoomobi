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
}

pub mod template {
    pub const RENDERING: &str = "An error happened while rendering the template";
}

pub mod io {
    pub const CREATE_KUBE_FOLDER: &str = "Unable to create kubernetes folder";
    pub const CREATE_KUBE_FILES: &str = "Unable to create kubernetes file";
}

pub mod core {
    pub const PATH_GENERATE_ERROR: &str     = "Error while generating absolute path";
    pub const PATH_GENERATE_REASON: &str    = "An error occured while converting the path";
    pub const DELETE_ERROR_MESSAGE: &str = "Unable to delete project";
    pub const SWITCH_ERROR_MESSAGE: &str = "Unable to switch project";
}
