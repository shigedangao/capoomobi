use crate::cli::scenarios::scenes::picker::{EnumHelper, HelpScenarios};
use crate::core::logger::{HelpLogType, log_help};

/// Launch
///
/// # Description
/// Launch the help scenario
///
/// # Arguments
/// * `main_action` &str
pub fn launch(main_action: &str) {
    let parsed_action = match HelpScenarios::from_string(main_action) {
        Some(value) => value,
        None => HelpScenarios::Init
    };

    match parsed_action {
        HelpScenarios::Init => describe_init(),
        HelpScenarios::Generate => describe_generate(),
        HelpScenarios::Project => describe_project(),
        HelpScenarios::Global => describe_cli()
    }
}

/// Describe Init
///
/// # Description
/// Describe the `capoomobi init` command
fn describe_init() {
    log_help(HelpLogType::Cmd, "capoomobi init <args>".to_owned());
    log_help(
        HelpLogType::Description,
        "Initialize and set a capoomobi project based on the provided name and path".to_owned()
    );
    log_help(HelpLogType::Example, "capoomobi init little_mouse ../cat".to_owned());
}

/// Describe Generate
///
/// # Description
/// Describe the `capoomobi generate` command
fn describe_generate() {
    log_help(HelpLogType::Cmd, "capoomobi generate <args>".to_owned());
    log_help(
        HelpLogType::Description,
        "Generate K8S files which describes your docker-compose.yaml file".to_owned()
    );
    log_help(
        HelpLogType::Action,
        format!(
            "{}\n{}\n",
            "- --print: Print the manifests that will be write to the project",
            "- --ingress: Generate an ingress file based on the docker-compose.yaml & confiture.json"
        )
    );
    log_help(HelpLogType::Example, "capoomobi generate /workspace/my-docker-project".to_owned());
}

/// Describe Project
///
/// # Description
/// Describe the `capoomobi project` command
fn describe_project() {
    log_help(HelpLogType::Cmd, "capoomobi project <action> <project_name>".to_owned());
    log_help(
        HelpLogType::Description,
        "Project command allow you to do action on a project".to_owned()
    );
    log_help(
        HelpLogType::Action,
        format!(
            "{}\n{}\n{}\n{}\n",
            "- current: Show the current project in use",
            "- list: List the projects that is using capoomobi",
            "- switch: Switch the project to an other one",
            "- delete: Delete a project"
        )
    );
    log_help(HelpLogType::Example, "capoomobi projet <action> little_pretty_mouse".to_owned());
}

/// Decribe Cli
///
/// # Description
/// Describe the basis function of the CLI
fn describe_cli() {
    log_help(HelpLogType::Cmd, "capoomobi <command> <...args>".to_owned());
    log_help(
        HelpLogType::Description,
        format!(
            "{}\n{}\n{}\n",
            "- init: Create a new project",
            "- generate: Create K8S yaml resources based on the docker-compose.yaml file",
            "- project: Allow you to manipulate each project such as list, delete"
        )
    );
}
