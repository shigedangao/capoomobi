#[cfg(test)]
mod loader {
    use std::error::Error;
    use crate::docker::loader;

    #[test]
    fn test_loading_docker_compose() {
        match loader::load("./example", "docker-compose.yaml") {
            Ok(res) => assert!(res.len() > 0),
            Err(err) => panic!(err)
        }
    }

    #[test]
    fn test_unknown_docker_compose() {
        match loader::load("./lol", "unexisted-docker-compose.yaml") {
            Ok(_) => panic!("Expect to not have found any file"),
            Err(err) => assert_eq!(err.description(), "Unable to find the docker-compose.yaml file")
        };
    }
}

#[cfg(test)]
mod parser {
    use crate::docker::parser;
    use crate::docker::loader;

    #[test]
    fn test_parse_docker_compose() {
        let yaml = match loader::load("./example", "docker-compose.yaml") {
            Ok(res) => res,
            Err(err) => panic!(err)
        };

        match parser::get_docker_services(yaml) {
            Some(_) => assert!(true),
            None => panic!("No services has been founded")
        };
    }

    #[test]
    fn test_parse_empty_docker_svc() {
        match parser::get_docker_services(vec![]) {
            Some(_) => panic!("Expect to have return None"),
            None => assert!(true)
        }
    }

    #[test]
    fn test_retrieve_proper_number_services() {
        let yaml = match loader::load("./example", "docker-compose.yaml") {
            Ok(res) => res,
            Err(err) => panic!(err)
        };

        let contents = match parser::get_docker_services(yaml) {
            Some(res) => res,
            None => panic!("No services has been founded")
        };

        if contents.len() != 3 {
            panic!("Expect to have retrieve 3 services");
        }
    }

    #[test]
    fn test_retrieve_proper_values() {
        let yaml = match loader::load("./example", "docker-compose.yaml") {
            Ok(res) => res,
            Err(err) => panic!(err)
        };

        let contents = match parser::get_docker_services(yaml) {
            Some(res) => res,
            None => panic!("No services has been founded")
        };

        assert_eq!(contents[0].image, "golang:1.12.6-stretch");
        assert_eq!(contents[0].name, "web");
        assert_eq!(contents[0].commands[0], "sh start.sh");
        assert_eq!(contents[0].labels, vec!["go", "api"]);
        assert_eq!(contents[0].environment, vec!["GO111MODULE=on"]);
        assert_eq!(contents[0].ports, vec!["5000:5000"]);
    }
}
