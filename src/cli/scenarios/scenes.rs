pub mod scenes_wrapper {
  // Scenario available
  #[derive(Debug)]
  pub enum Scenarios {
    Init,
    Help,
  }

  impl Scenarios {
    pub fn from_str(action: &str) -> Result<Scenarios, &'static str> {
      match action {
        "init" => Ok(Scenarios::Init),
        "help" => Ok(Scenarios::Help),
        _ => return Err("")
      }
    }
  }
}