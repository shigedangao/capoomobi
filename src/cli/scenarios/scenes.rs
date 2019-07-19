pub mod scenes_helper {
  // Trait to parse to get enum from string
  pub trait EnumHelper<T> {
    fn from_str(action: &str) -> Option<T>;
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
    fn from_str(action: &str) -> Option<Scenarios> {
      match action {
        "init" => Some(Scenarios::Init),
        "help" => Some(Scenarios::Help),
        "generate" => Some(Scenarios::Generate),
        _ => None
      }
    }
  }

  impl EnumHelper<HelpScenarios> for HelpScenarios {
    fn from_str(action: &str) -> Option<HelpScenarios> {
      match action {
        "init" => Some(HelpScenarios::Init),
        "generate" => Some(HelpScenarios::Generate),
        "project" => Some(HelpScenarios::Project),
        "revert" => Some(HelpScenarios::Revert),
        "verify" => Some(HelpScenarios::Verify),
        _ => None
      }
    }
  }
}
