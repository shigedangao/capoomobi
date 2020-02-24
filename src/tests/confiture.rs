#[cfg(test)]
mod confiture {
    use crate::confiture::config;
    use crate::kubernetes::controllers::controller::ControllerKind;
    use crate::kubernetes::controllers::service::ServiceType;

    #[test]
    fn expect_to_load_confiture() {
        let cnf = config::load("./example");
        if cnf.is_none() {
            panic!("Expect to retrieve a confiture.json");
        }
    }

    #[test]
    fn expect_to_get_confiture_value() {
        let cnf = config::load("./example").unwrap();
        let value = cnf.get_config_confiture_map();

        // testing the values
        // deployment
        assert_eq!(value.get("web").unwrap().deployment.replicas, 1);
        assert_eq!(value.get("web").unwrap().deployment.controller, ControllerKind::Deployment);

        // service
        assert_eq!(value.get("web").unwrap().service.kind, ServiceType::NodePort);
        assert_eq!(value.get("web").unwrap().service.nodeport, 30302);
    }

    #[test]
    fn expect_to_get_ingress() {
        let cnf = config::load("./example").unwrap();
        let ingress = cnf.ingress.unwrap();

        assert_eq!(ingress.ip, "30.10.20.30");
        assert_eq!(ingress.services[0].name, "web");
        assert_eq!(ingress.services[0].path, "/web");
    }

    #[test]
    fn expect_missing_property_to_return_none() {
        let cnf = config::load("./example_failure");
        assert!(cnf.is_none());
    }
}
