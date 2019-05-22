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
  use std::io::ErrorKind;

  /**
   * Exist Or Create
   * 
   * Check if the configuration file exist
   * If it exist do nothing otherwise create the configuration file
   */
  pub fn exist_or_create() {
    let mut fi = match config_util::exist() {
      Ok(f) => f,
      Err(e) => {
        let kind = e.kind();

        if kind == ErrorKind::NotFound {
          config_util::create();
          return;
        }

        panic!("Unable to read file");
      }
    };

    println!("Able to read path")
  }
}