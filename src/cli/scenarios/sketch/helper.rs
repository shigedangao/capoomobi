/// Retrieve options by idx
/// 
/// # Description
/// Retrieve an optional String by it's index
/// 
/// # Arguments
/// * `vec` Reference to a vector of string
/// * `idx` usize
/// 
/// # Return
/// * `options` option of string
pub fn retrieve_options_by_idx(vec: &Vec<String>, idx: usize) -> Option<String> {
  if vec.is_empty() {
    return None;
  }

  match vec.get(idx) {
    Some(res) => Some(res.to_string()),
    None => None
  }
}