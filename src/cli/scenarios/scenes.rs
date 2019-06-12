pub mod scenes_helper {
  // Trait to parse to get enum from string
  pub trait EnumHelper<T> {
    fn from_str(action: &str) -> Result<T, &'static str>;
  }

  // Scenario available for the main action
  // capoomobi <main_action>
  #[derive(Debug)]
  pub enum Scenarios {
    Init,
    Help,
  }

  // Scenarios available for the Help command
  // capoomobi help <actions>
  #[derive(Debug)]
  pub enum Help_scenarios {
    Init,
    Generate,
    Project,
    Revert,
    Verify
  }

  impl EnumHelper<Scenarios> for Scenarios {
    fn from_str(action: &str) -> Result<Scenarios, &'static str> {
      match action {
        "init" => Ok(Scenarios::Init),
        "help" => Ok(Scenarios::Help),
        _ => Err("")
      }
    }
  }

  impl EnumHelper<Help_scenarios> for Help_scenarios {
    fn from_str(action: &str) -> Result<Help_scenarios, &'static str> {
      match action {
        "init" => Ok(Help_scenarios::Init),
        "generate" => Ok(Help_scenarios::Generate),
        "project" => Ok(Help_scenarios::Project),
        "revert" => Ok(Help_scenarios::Revert),
        "verify" => Ok(Help_scenarios::Verify),
        _ => Err("")
      }
    }
  }
}
