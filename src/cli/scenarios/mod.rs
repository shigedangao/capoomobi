pub mod scenes;
mod scn;

pub mod bootstrap {
  use crate::cli::scenarios::scenes::scenes_wrapper;
  use crate::cli::scenarios::scn;
  /**
   * Init
   * 
   * Parse the scenario based on the main_action
   */
  pub fn init(main_action: String, sub_action: String, options: Vec<String>) {
    match scenes_wrapper::Scenarios::from_str(&main_action) {
      Err(err) => println!("err {:?}", err),
      Ok(res) => launch_scenario(res, sub_action, options)
    }
  }

  /**
   * Launch Scenario
   * 
   * Launch a scenario based on the input of the main action
   */
  fn launch_scenario(scenario: scenes_wrapper::Scenarios, sub_action: String, options: Vec<String>) {
    match scenario {
      scenes_wrapper::Scenarios::Init => scn::init::launch(sub_action, options),
      scenes_wrapper::Scenarios::Help => println!("Launch help scenario")
    }
  }
}
