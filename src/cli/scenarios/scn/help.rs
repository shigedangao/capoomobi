use crate::cli::scenarios::scenes::scenes_helper::{EnumHelper, Help_scenarios};

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
  println!("describe init")
}

fn describe_generate() {

}

fn describe_project() {

}

fn describe_revert() {

}

fn describe_verify() {

}
