pub mod scenes;
mod scn;

pub mod bootstrap {
  use crate::cli::scenarios::scenes::scenes_wrapper;
  use crate::cli::scenarios::scn;
  use crate::cli::core::logger;

  /**
   * Init
   * 
   * Parse the scenario based on the main_action
   */
  pub fn init(main_action: &str, sub_action: &str, options: Vec<String>) {
    match scenes_wrapper::Scenarios::from_str(main_action) {
      Err(err) => logger::write(logger::LogType::Error, err, None),
      Ok(res) => launch_scenario(res, sub_action, options)
    }
  }

  /**
   * Launch Scenario
   * 
   * Launch a scenario based on the input of the main action
   */
  fn launch_scenario(scenario: scenes_wrapper::Scenarios, sub_action: &str, options: Vec<String>) {
    match scenario {
      scenes_wrapper::Scenarios::Init => scn::init::launch(sub_action, options),
      scenes_wrapper::Scenarios::Help => logger::write(logger::LogType::Info, "launch help mod", None)
    }
  }
}
