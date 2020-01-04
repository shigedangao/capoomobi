#[cfg(test)]
mod loader {
    use std::error::Error;
    use crate::docker::loader;

    #[test]
    fn expect_to_load_dk() {
        match loader::load("./example", "docker-compose.yaml") {
            Ok(res) => assert!(res.len() > 0),
            Err(err) => panic!(err)
        }
    }

    #[test]
    fn expect_to_not_load_dk() {
        match loader::load("./lol", "unexisted-docker-compose.yaml") {
            Ok(_) => panic!("Expect to not have found any file"),
            Err(err) => assert_eq!(err.description(), "Unable to find the docker-compose.yaml file")
        };
    }
}

#[cfg(test)]
mod parser {
    use std::error::Error;
    use crate::docker::parser;
    use crate::docker::loader;

    #[test]
    fn expect_to_parse_dk() {
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
    fn expect_to_not_retrieve_svc() {
        match parser::get_docker_services(vec![]) {
            Some(_) => panic!("Expect to have return None"),
            None => assert!(true)
        }
    }

    #[test]
    fn expect_to_retrieve_listed_services() {
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
    fn expect_to_map_values() {
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
        assert_eq!(contents[0].commands, vec!["sh start.sh"]);
        assert_eq!(contents[0].labels, vec!["go", "api"]);
        assert_eq!(contents[0].environment, vec!["GO111MODULE=on"]);
        assert_eq!(contents[0].ports, vec!["5000:5000"]);
    }

    #[test]
    fn expect_to_not_err_during_missing_fields() {
        let empty_vec_string: Vec<String> = Vec::new(); 

        let yaml = match loader::load("./example", "docker-compose-missing-field.yaml") {
            Ok(res) => res,
            Err(err) => panic!(err)
        };

        let contents = match parser::get_docker_services(yaml) {
            Some(res) => res,
            None => panic!("No services has been founded")
        };

        assert_eq!(contents[0].image, "golang:1.12.6-stretch");
        assert_eq!(contents[0].name, "web");
        assert_eq!(contents[0].commands, vec![""]);
        assert_eq!(contents[0].labels, vec![""]);
        assert_eq!(contents[0].environment, empty_vec_string);
        assert_eq!(contents[0].ports, empty_vec_string);
    }

    #[test]
    fn expect_load_to_fail() {
        match loader::load("./example", "docker-unformatted.yaml") {
            Ok(_) => panic!("Expect to have return an error"),
            Err(err) => assert_eq!(err.description(), "Unable to parse the docker-compose.yaml for reason: ")
        }
    }
}
