pub mod scenes;

pub mod bootstrap {
  use crate::cli::scenarios::scenes::scenes_wrapper;
  /**
   * Init
   * 
   * Parse the scenario based on the main_action
   */
  pub fn init(main_action: String, sub_action: String) {
    match scenes_wrapper::Scenarios::from_str(&main_action) {
      Err(err) => println!("{:?}", err),
      Ok(res) => launch_scenario(res, sub_action)
    }
  }

  /**
   * Launch Scenario
   * 
   * Launch a scenario based on the input of the main action
   */
  fn launch_scenario(scenario: scenes_wrapper::Scenarios, sub_action: String) {
    match scenario {
      scenes_wrapper::Scenarios::Init => println!("Launch welcome scenario"),
      scenes_wrapper::Scenarios::Help => println!("Launch help scenario")
    }
  }
}
