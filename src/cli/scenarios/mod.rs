pub mod welcome;
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
      Ok(res) => println!("ok: {:?}", res)
    }
  }
}
