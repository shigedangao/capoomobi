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
            Some(res) => res,
            None => panic!("No services has been founded")
        };
    }
}
