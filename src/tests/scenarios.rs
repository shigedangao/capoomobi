#[cfg(test)]
mod scenarios {
    use crate::cli::scenarios::sketch;

    #[test]
    fn test_generate_to_not_panic() {
        sketch::generate::launch("./example", &[]);
    }

    #[test]
    fn test_project_list_to_not_panic() {
        sketch::project::launch("list", &[]);
    }

    #[test]
    fn test_project_current_to_not_panic() {
        sketch::project::launch("list", &[]);
    }
}
