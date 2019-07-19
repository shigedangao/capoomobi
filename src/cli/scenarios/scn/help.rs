use crate::cli::scenarios::scenes::scenes_helper::{EnumHelper, HelpScenarios};
use crate::cli::core::logger::logging::{HelpLogType, write_help};

// Error constants
const EMPTY_OPTIONS_ERROR: &str = "Please select an option";

/**
 * Launch
 * 
 * Launch the scenario of the help command
 * 
 * capoomobi help
 */
pub fn launch(main_action: &str) {
  let parsed_action = match HelpScenarios::from_str(main_action) {
    Some(value) => value,
    None => panic!(format!("{}", EMPTY_OPTIONS_ERROR))
  };

  match parsed_action {
    HelpScenarios::Init => describe_init(),
    HelpScenarios::Generate => describe_generate(),
    HelpScenarios::Project => describe_project(),
    HelpScenarios::Revert => describe_revert(),
    HelpScenarios::Verify => describe_verify()
  }
}

/**
 * Describe Init
 * 
 * Describe the init command of the capoomobi
 */
fn describe_init() {
  write_help(HelpLogType::Cmd, "capoomobi init <args>");
  write_help(
    HelpLogType::Description,
    "Initialize and set a capoomobi project based on the provided name and path"
  );
  write_help(HelpLogType::Example, "capoomobi init little_mouse ../cat");
}

/**
 * Describe Generate
 * 
 * Describe the generate command
 */
fn describe_generate() {
  write_help(HelpLogType::Cmd, "capoomobi generate <args>");
  write_help(
    HelpLogType::Description,
    "Generate a set of Kubernetes files from your docker-compose.yml"
  );
  write_help(HelpLogType::Example, "capoomobi generate");
}

/**
 * Describe Project
 * 
 * Describe the project command
 */
fn describe_project() {
  write_help(HelpLogType::Cmd, "capoomobi describe <args>");
  write_help(
    HelpLogType::Description,
    "Describe a project based on it's name"
  );
  write_help(HelpLogType::Example, "capoomobi describe jasmine");
}

/**
 * Describe Revert
 * 
 * Describe the revert command
 */
fn describe_revert() {
  write_help(HelpLogType::Cmd, "capoomobi revert <args>");
  write_help(
    HelpLogType::Description,
    "Revert the project to a previous version"
  );
  write_help(HelpLogType::Example, "capoomobi revert 10 jasmine");
}

/**
 * Describe Verify
 * 
 * Describe the verify command
 */
fn describe_verify() {
  write_help(HelpLogType::Cmd, "capoomobi verify <args>");
}
