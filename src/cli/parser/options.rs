/**
 * Options
 * 
 * Parse the options Vector
 */
pub mod options {
  /**
   * Parse Options
   * 
   * Parse the options from the CLI
   */
  pub fn parse_options(vec: Vec<String>, idx: usize) -> Option<String> {
    if vec.is_empty() {
      println!("vector is empty");
      return None;
    }

    match vec.get(idx) {
      Some(res) => Some(res.to_string()),
      None => None
    }
  }
}