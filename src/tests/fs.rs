#[cfg(test)]
mod fs {
    use crate::core::fs::config;

    #[test]
    fn expect_new_conf() {
        let cnf = config::ConfigHelper::new("./", "capoos");
        match cnf.build_project_folder() {
            Ok(_) => {},
            Err(err) => panic!(err.to_string())
        }
    }

    #[test]
    fn expect_build_conf_empty_path() {
        let cnf = config::ConfigHelper::new("", "capoos");
        match cnf.build_project_folder() {
            Ok(_) => {},
            Err(err) => panic!(err.to_string())
        }
    }

    #[test]
    fn expect_retrieve_project_path() {
        let cnf = config::ConfigHelper::new("../", "capoos");

        if let Err(err) = cnf.build_project_folder() {
            panic!(err.to_string());
        }

        let path = cnf.get_path_as_string();
        assert!(!path.is_empty());
    }
}
