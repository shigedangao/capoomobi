pub mod scenes;
mod scn;

pub mod bootstrap {
  use crate::cli::scenarios::scenes::scenes_helper;
  use crate::cli::scenarios::scenes::scenes_helper::EnumHelper;
  use crate::cli::scenarios::scn;
  use crate::cli::core::logger::logging;

  // errors
  const UNKNOWN_SCENARIO_ERR: &str = "Scenarios not found";

  /**
   * Init
   * 
   * Parse the scenario based on the main_action
   */
  pub fn init(main_action: &str, sub_action: &str, options: Vec<String>) {
    match scenes_helper::Scenarios::from_str(main_action) {
      None => logging::write(logging::LogType::Error, UNKNOWN_SCENARIO_ERR, None),
      Some(res) => launch_scenario(res, sub_action, options)
    }
  }

  /**
   * Launch Scenario
   * 
   * Launch a scenario based on the input of the main action
   */
  fn launch_scenario(scenario: scenes_helper::Scenarios, sub_action: &str, options: Vec<String>) {
    match scenario {
      scenes_helper::Scenarios::Init => scn::init::launch(sub_action, options),
      scenes_helper::Scenarios::Help => scn::help::launch(sub_action),
      scenes_helper::Scenarios::Generate => scn::generate::launch(sub_action)
    }
  }
}
