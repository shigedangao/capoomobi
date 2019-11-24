use crate::core::logger::{log, LogType};
use crate::cli::configurator::configure;
use crate::core::fs::config::ConfigHelper;
use crate::cli::configurator::builder::{Projects};
use crate::errors::cli_error::{CliErr, ErrHelper, ErrMessage};

/// Error constant
const PATH_ERROR: &str = "Unable to retrieve absolute path {:?}";

/// Launch
/// 
/// # Description
/// Launch the init scenario it is launch with the command below 
/// capoomobi init <project_name> <path>
/// e.g: capoomobi init lilmouse ../lilcat
/// 
/// # Arguments
/// * `project_name`: slice of string
/// * `cmd_args`: reference to Vec of string
pub fn launch(project_name: &str, cmd_args: &Vec<String>) {
    // Check if the capoomobi configurator already exist
    let config = configure::exist();
    if let Some(conf) = config {
        execute(project_name, cmd_args, conf);
        return;
    }

    match configure::create_config() {
        Ok(conf) => execute(project_name, cmd_args, conf),
        Err(e) => e.log_pretty() 
    }   
}

/// Execute
/// 
/// # Description
/// Execute the creation of the project folder
/// Write the project path into the capoomobi.json file
/// 
/// # Arguments
/// * `project_name` &str
/// * `cmd_args` &Vec<String>
/// * `conf` configure::CapooConfig
fn execute(project_name: &str, cmd_args: &Vec<String>, conf: configure::CapooConfig) {
    // Retrieve path from the arguments
    // capoomobi init mouse <path>
    let path = cmd_args.get(0).unwrap_or(&String::new());
    let conf_helper = ConfigHelper::new(&path, project_name);

    let build_opt = conf_helper.build_project_folder();
    if let None = build_opt {
        CliErr::new(PATH_ERROR, path, ErrMessage::NotFound).log_pretty();
        return;
    }
    
    let build_path = build_opt.unwrap();
    let content_builder = match conf.get_content() {
        Ok(c) => c,
        Err(_) => {
            Projects {
                projects: Vec::new(),
                current: String::new()
            }
        }
    };

    content_builder.populate_project_conf(String::from(project_name), build_path)
    .and_then(|res| conf.write_json(res)) {
        Ok(()) => log(
            LogType::Success,
            "Project successfully created",
            Some(conf_helper.get_path_as_string())
        ),
        Err(err) => err.log_pretty()
    }
}
