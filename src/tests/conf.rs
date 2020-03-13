/// We're testing the CapooConfig struct impl methods
#[cfg(test)]
mod capooconfig {
    use std::path::PathBuf;
    use serde_json;
    use crate::core::fs::toolbox;
    use crate::core::configurator::configure::{CapooConfig};

    // Setup method for the unit test
    fn setup() -> CapooConfig {
        let path = PathBuf::from("capoomobi.json");
        CapooConfig::new(path)
    }

    #[test]
    fn expect_new_capoo_instance() {
        let path = PathBuf::from("capoomobi.json");
        let res = toolbox::create_file(&path);

        if res.is_err() {
            panic!(res.unwrap_err());
        }

        let config = CapooConfig::new(path.clone());
        assert_eq!(config.path.to_str().unwrap(), path.to_str().unwrap());
    }

    #[test]
    fn expect_to_retrieve_no_project() {
        let config = setup();
        let res = toolbox::create_file(&config.path);

        if res.is_err() {
            panic!(res.unwrap_err());
        }

        if let Err(err) = config.get_content() {
            assert_eq!(&err.message.to_string(), "Unable to parse the content of the config file for reason:");
        } else {
            panic!("Expect to not retrieve any content");
        }
    }

    #[test]
    fn expect_retrieve_project() {
        let config = setup();
        let res = toolbox::create_file(&config.path);

        if res.is_err() {
            panic!(res.unwrap_err());
        }

        let obj = serde_json::json!({"projects":[{"name":"capoo","path":"capoo"}],"current":"capoo"});
        let value = serde_json::to_string_pretty(&obj).unwrap();

        match config.write_json_file(value) {
            Ok(_) => {},
            Err(err) => panic!(err.message.to_string())
        };

        match config.get_content() {
            Ok(conf) => assert_eq!(conf.projects[0].name, "capoo"),
            Err(err) => panic!(err.message.to_string())
        }
    }
}

#[cfg(test)]
mod builder {
    use std::path::PathBuf;
    use crate::core::configurator::builder::Projects;
    use crate::core::configurator::config;
    use crate::core::errors::message;

    // Method running before each test
    fn setup() -> Projects {
        Projects {
            projects: Vec::new(),
            current: String::from("capoo")
        }
    }

    #[test]
    fn expect_to_add_project() {
        let projects = setup();
        match projects.add("foo", PathBuf::from("capoomobi.json")) {
            Ok(res) => assert_eq!(res.current, "foo"),
            Err(err) => panic!(err.message.to_string())
        }
    }

    #[test]
    fn expect_to_not_create_empty_name_project() {
        let projects = setup();
        match projects.add("", PathBuf::new()) {
            Ok(_) => panic!("Expect to not have create a project with an empty name"),
            Err(err) => assert_eq!(&err.message.to_string(), message::core::PROJECT_NAME_EMPTY)
        }
    }

    #[test]
    fn expect_to_not_create_project() {
        let projects = setup();
        match projects.add("foo", PathBuf::new()) {
            Ok(_) => panic!("Expect to not add a project"),
            Err(err) => assert_eq!(&err.message.to_string(), message::core::PATH_GENERATE_ERROR)
        }
    }

    #[test]
    fn expect_to_not_delete_project() {
        let projects = setup();
        match projects.delete_project_by_name("foo") {
            Ok(_) => panic!("Expect to not find any project to be deleted"),
            Err(err) => assert_eq!(&err.message.to_string(), message::core::DELETE_ERROR_MESSAGE)
        }
    }

    #[test]
    fn expect_to_delete_project() {
        let projects = setup();
        let p = match projects.add("foo", PathBuf::from("../foo")) {
            Ok(p) => p,
            Err(err) => panic!(err.message.to_string())
        };


        match p.delete_project_by_name("foo") {
            Ok(res) => assert_eq!(res.0.projects.len(), 0),
            Err(err) => panic!(err.message.to_string())
        }

    }

    #[test]
    fn expect_to_switch_project() {
        let projects = setup();

        let p = projects.add("foo", PathBuf::from("../foo")).unwrap();
        let r = p.add("bar", PathBuf::from("../bar")).unwrap();

        match r.switch_project("foo") {
            Ok(s) => assert_eq!(s.current, "foo"),
            Err(err) => panic!(err.message.to_string())
        }
    }

    #[test]
    fn expect_to_not_switch_project() {
        let projects = setup();
        match projects.switch_project("foo") {
            Ok(_) => panic!("Expect to not have switch a project as projects vector is empty"),
            Err(err) => assert_eq!(&err.message.to_string(), message::core::SWITCH_ERROR_MESSAGE)
        }
    }

    #[test]
    fn expect_to_not_switch() {
        let projects = setup();
        let p = projects.add("bar", PathBuf::from("./bar")).unwrap();

        match p.switch_project("foo") {
            Ok(_) => panic!("Expect to not have switch to the 'foo' project"),
            Err(err) => assert_eq!(&err.message.to_string(), message::core::SWITCH_ERROR_MESSAGE)
        }

    }

    #[test]
    fn expect_to_get_pj_path() {
        match config::get_current_project_path() {
            Some(s) => assert!(!s.is_empty()),
            None => panic!("Expect to retrieve a configuration")
        }
    }
}
