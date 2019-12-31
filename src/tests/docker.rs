#[cfg(test)]
mod loader {
    use crate::docker::loader;

    #[test]
    fn test_loading_docker_compose() {
        match loader::load("./example", "docker-compose.yaml") {
            Ok(res) => assert!(res.len() > 0),
            Err(err) => panic!(err)
        }
    }
}
