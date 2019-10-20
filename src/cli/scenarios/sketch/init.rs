use crate::cli::core::logger::{log, LogType};
use crate::cli::configurator::configure::configure;
use crate::cli::core::fs::bootstrap::bootstrap::ProjectPath;
use crate::cli::scenarios::sketch::helper;
use crate::errors::cli_error::{CliErr, ErrHelper, ErrCode};

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
/// * `options`: reference to Vec of string
pub fn launch(project_name: &str, options: &Vec<String>) {
    let project_path = match helper::retrieve_options_by_idx(options, 0) {
        Some(p) => p, 
        None => String::new()
    };

    let initializer = ProjectPath::new(&project_path);
    let abs_path = match initializer.build_project_folder() {
        Some(p) => p,
        None => {
            CliErr::new(
                PATH_ERROR,
                project_path,
                ErrCode::NotFound
            ).log_pretty();
            return;
        }
    };

    // Checking or creating if the config file exist
    let capoo_configurator = match configure::bootstrap_capoo() {
        Ok(f) => f,
        Err(e) => {
            e.log_pretty();
            return;
        }
    };

    match capoo_configurator
        .generate_project_conf(String::from(project_name), abs_path)
        .and_then(|res| capoo_configurator.write_json(res)) {
            Ok(()) => log(
                LogType::Success,
                "Project successfully created",
                Some(initializer.get_path_as_string())
            ),
            Err(err) => err.log_pretty()
        }
}
