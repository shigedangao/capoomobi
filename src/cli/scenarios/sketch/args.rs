use crate::cli::scenarios::scenes::picker::EnumHelper;

/// Generate Options
///
/// # Description
/// Supported command options
pub enum GenerateOptions {
    Print,
    Ingress
}

impl EnumHelper<GenerateOptions> for GenerateOptions {
    fn from_string(action: &str) -> Option<GenerateOptions> {
        match action.to_lowercase().as_str() {
            "--print" => Some(GenerateOptions::Print),
            "--ingress" => Some(GenerateOptions::Ingress),
            _ => None
        }
    }
}

/// Retrieve Cmd Options
///
/// # Description
/// Retrive the options passed to a command
///
/// # Arguments
/// * `options` &[String] (See: https://stackoverflow.com/a/40006220/7489243 on why it's better to use a slice)
///
/// # Return
/// Option<GenerateOptions>
pub fn retrieve_cmd_options(options: &[String]) -> Option<GenerateOptions> {
    let opt = match retrieve_options_by_idx(options, 0) {
        Some(p) => p,
        None => String::new()
    };

    GenerateOptions::from_string(&opt)
}

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
pub fn retrieve_options_by_idx(vec: &[String], idx: usize) -> Option<String> {
    if vec.is_empty() {
        return None;
    }

    match vec.get(idx) {
        Some(res) => Some(res.to_string()),
        None => None
    }
}
