/**
 * Configure mod
 * 
 * This mod allow us to
 * - read
 * - parse
 * - set
 * 
 * values in the config.json 
 */
pub mod configure {
  use crate::cli::core::fs::config::config_util;
  // Constant defining the paths available
  const CONFIG_FILE: &str = ".capoomobi.json";
  const CONFIG_PATH: &str = "~/";

  /**
   * Read Config
   * 
   * Read the configuration file
   */
  pub fn read_config() {
    let mut path = String::from(CONFIG_PATH);
    path.push_str(CONFIG_FILE);

    config_util::exist(path);
  }
}