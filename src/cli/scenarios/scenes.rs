pub mod scenes_wrapper {
  // Scenario available
  #[derive(Debug)]
  pub enum Scenarios {
    Welcome,
    Help,
  }

  impl Scenarios {
    pub fn from_str(action: &str) -> Result<Scenarios, &'static str> {
      match action {
        "welcome" => Ok(Scenarios::Welcome),
        "help" => Ok(Scenarios::Help),
        _ => return Err("")
      }
    }
  }
}
