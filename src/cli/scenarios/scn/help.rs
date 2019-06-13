use crate::cli::scenarios::scenes::scenes_helper::{EnumHelper, Help_scenarios};
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
  let parsed_action = match Help_scenarios::from_str(main_action) {
    Ok(value) => value,
    Err(_) => panic!(format!("{}", EMPTY_OPTIONS_ERROR))
  };

  match parsed_action {
    Help_scenarios::Init => describe_init(),
    Help_scenarios::Generate => describe_generate(),
    Help_scenarios::Project => describe_project(),
    Help_scenarios::Revert => describe_revert(),
    Help_scenarios::Verify => describe_verify()
  }
}

fn describe_init() {
  write_help(HelpLogType::Cmd, "capoomobi init <args>");
  write_help(
    HelpLogType::Description,
    "Initialize and set a capoomobi project based on the provided name and path"
  );
  write_help(HelpLogType::Example, "capoomobi init little_mouse ../cat");
}

fn describe_generate() {

}

fn describe_project() {

}

fn describe_revert() {

}

fn describe_verify() {

}
