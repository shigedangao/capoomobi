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
    Generate
  }

  // Scenarios available for the Help command
  // capoomobi help <actions>
  #[derive(Debug)]
  pub enum HelpScenarios {
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
        "generate" => Ok(Scenarios::Generate),
        _ => Err("")
      }
    }
  }

  impl EnumHelper<HelpScenarios> for HelpScenarios {
    fn from_str(action: &str) -> Result<HelpScenarios, &'static str> {
      match action {
        "init" => Ok(HelpScenarios::Init),
        "generate" => Ok(HelpScenarios::Generate),
        "project" => Ok(HelpScenarios::Project),
        "revert" => Ok(HelpScenarios::Revert),
        "verify" => Ok(HelpScenarios::Verify),
        _ => Err("")
      }
    }
  }
}
