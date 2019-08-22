use crate::cli::scenarios::scenes::picker::{EnumHelper, HelpScenarios};
use crate::cli::core::logger::logger::{HelpLogType, log_help};

/// Error constants
const EMPTY_OPTIONS_ERROR: &str = "Please select an option";

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
    None => panic!(format!("{}", EMPTY_OPTIONS_ERROR))
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
    "Generate a set of Kubernetes files from your docker-compose.yml"
  );
  log_help(HelpLogType::Example, "capoomobi generate");
}

/// Describe Project
/// 
/// # Description
/// Describe the `capoomobi project` command
fn describe_project() {
  log_help(HelpLogType::Cmd, "capoomobi describe <args>");
  log_help(
    HelpLogType::Description,
    "Describe a project based on it's name"
  );
  log_help(HelpLogType::Example, "capoomobi describe jasmine");
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
  log_help(HelpLogType::Example, "capoomobi revert 10 jasmine");
}

/// Describe Verify
/// 
/// # Description
/// Describe the `capoomobi verify` command
fn describe_verify() {
  log_help(HelpLogType::Cmd, "capoomobi verify <args>");
}
