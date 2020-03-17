/// Boostrap module
///
/// # Description
/// Module use to trigger the CLI
pub mod manager {
    use crate::cli::scenarios::parser;

    /// Start
    ///
    /// # Description
    /// Boostrap method use to launch the CLI
    pub fn start() {
        parser::trigger_command();
    }
}
