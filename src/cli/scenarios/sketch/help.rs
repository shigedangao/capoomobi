use crate::cli::scenarios::scenes::picker::{EnumHelper, HelpScenarios};
use crate::cli::core::logger::logger::{HelpLogType, log_help};

/// Launch
/// 
/// # Description
/// Launch the help scenario
/// 
/// # Arguments
/// * `main_action` Reference to a String
pub fn launch(main_action: &String) {
    let parsed_action = match HelpScenarios::from_string(main_action) {
        Some(value) => value,
        None => HelpScenarios::Init
    };

    match parsed_action {
        HelpScenarios::Init => describe_init(),
        HelpScenarios::Generate => describe_generate(),
        HelpScenarios::Project => describe_project(),
        HelpScenarios::Revert => describe_revert(),
        HelpScenarios::Verify => describe_verify(),
    }
}

/// Describe Init
/// 
/// # Description
/// Describe the `capoomobi init` command
fn describe_init() {
    log_help(HelpLogType::Cmd, "capoomobi init <args>");
    log_help(
        HelpLogType::Description,
        "Initialize and set a capoomobi project based on the provided name and path"
    );
    log_help(HelpLogType::Example, "capoomobi init little_mouse ../cat");
}

/// Describe Generate
/// 
/// # Description
/// Describe the `capoomobi generate` command
fn describe_generate() {
    log_help(HelpLogType::Cmd, "capoomobi generate <args>");
    log_help(
        HelpLogType::Description,
        "Generate K8S files which describes your docker-compose.yaml file"
    );
    log_help(HelpLogType::Example, "capoomobi generate /workspace/my-docker-project");
}

/// Describe Project
/// 
/// # Description
/// Describe the `capoomobi project` command
fn describe_project() {
    log_help(HelpLogType::Cmd, "capoomobi project <action> <project_name>");
    log_help(
        HelpLogType::Description,
        "Project command allow you to do action on a project"
    );
    log_help(
        HelpLogType::Action,
        "
        - current: Show the current project in use
        - list: List the projects that is using capoomobi
        - switch: Switch the project to an other one
        - delete: Delete a project
        "
    );
    log_help(HelpLogType::Example, "capoomobi projet <action> little_pretty_mouse");
}

/// Describe revert
/// 
/// # Description
/// Describe the `capoomobi revert` command
fn describe_revert() {
    log_help(HelpLogType::Cmd, "capoomobi revert <args>");
    log_help(
        HelpLogType::Description,
        "Revert the project to a previous version"
    );
    log_help(HelpLogType::Example, "capoomobi revert 10 little_pretty_mouse");
}

/// Describe Verify
/// 
/// # Description
/// Describe the `capoomobi verify` command
fn describe_verify() {
    log_help(HelpLogType::Cmd, "capoomobi verify <args>");
}
