use crate::core::logger::{log, LogType};
use crate::core::configurator::configure;
use crate::core::fs::config::ConfigHelper;
use crate::core::configurator::builder::{Projects};
use crate::core::errors::cli_error::{ErrHelper};
use crate::core::serde_utils::SerdeUtil;

/// Message
const PROJECT_CREATED: &str = "Project successfully create";

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
pub fn launch(project_name: &str, cmd_args: &[String]) {
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
/// * `cmd_args` &[String]
/// * `conf` configure::CapooConfig
fn execute(project_name: &str, cmd_args: &[String], conf: configure::CapooConfig) {
    // Retrieve path from the arguments
    // capoomobi init mouse <path>
    let empty_str = String::new();
    let path = cmd_args.get(0).unwrap_or(&empty_str);
    let conf_helper = ConfigHelper::new(&path, project_name);

    let build_opt = conf_helper.build_project_folder();
    if let Err(err) = build_opt {
        return err.log_pretty();
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

    match content_builder
        .add(project_name, build_path)
        .and_then(|res| res.serialize())
        .and_then(|res| conf.write_json_file(res)) {
            Ok(_) => log(LogType::Success, PROJECT_CREATED, Some(conf_helper.get_path_as_string())),
            Err(e) => e.log_pretty()
        }
}
