#[cfg(test)]
mod directory {
    use std::collections::HashMap;
    use crate::docker::parser::DockerService;
    use crate::kubernetes::builder::{get_basic_objects};
    use crate::kubernetes::controllers::controller::ControllerKind;
    use crate::kubernetes::controllers::service::ServiceType;
    use crate::kubernetes::controllers::ingress::{KubeIngress, IngressBackend};
    use crate::confiture::config::{
        ConfigConfiture,
        ConfigDeployment,
        ConfigService
    };
    use crate::kubernetes::io::{
        runner,
        folder,
        objects
    };

    // Create a docker service to test the kube io process
    fn setup() -> (DockerService, ConfigConfiture) {
        let dk = DockerService {
            name: "nginx".to_string(),
            image: "nginx:1.3.0".to_string(),
            commands: vec!["sudo nginx reload".to_string()],
            ports: vec!["9000:9000".to_string()],
            labels: vec!["back".to_string()],
            environment: vec!["API_ENV=dev".to_string()],
            volumes: vec![]
        };

        let conf = ConfigConfiture {
            name: "nginx".to_string(),
            deployment: ConfigDeployment {
                replicas: 3,
                controller: ControllerKind::Deployment
            },
            service: ConfigService {
                kind: ServiceType::NodePort,
                nodeport: 30320
            }
        };

        (dk, conf)
    }

    #[test]
    fn expect_to_create_kube_obj() {
        let (dk, conf) = setup();

        // create a map
        let mut map: HashMap<String, &ConfigConfiture> = HashMap::new();
        map.insert("nginx".to_string(), &conf);

        let kubes = get_basic_objects(&vec![dk], map);
        assert!(!kubes.is_empty());

        // testing controller value
        assert_eq!(kubes[0].ctrl.ctrl, ControllerKind::Deployment);
        assert_eq!(kubes[0].ctrl.name, "nginx");
        assert_eq!(kubes[0].ctrl.image, "nginx:1.3.0");
        assert_eq!(kubes[0].ctrl.commands, vec!["sudo nginx reload".to_string()]);
        assert_eq!(kubes[0].ctrl.labels, vec!["back".to_string()]);
        assert_eq!(kubes[0].ctrl.env, vec!["API_ENV=dev".to_string()]);
        assert_eq!(kubes[0].ctrl.replicas, 3);

        // testing service value
        assert_eq!(kubes[0].svc.as_ref().unwrap().name, "nginx-svc");
        assert_eq!(kubes[0].svc.as_ref().unwrap().host_port, 9000);
        assert_eq!(kubes[0].svc.as_ref().unwrap().target_port, 9000);
        assert_eq!(kubes[0].svc.as_ref().unwrap().kind, ServiceType::NodePort);
        assert_eq!(kubes[0].svc.as_ref().unwrap().labels, vec!["back".to_string()]);
        assert_eq!(kubes[0].svc.as_ref().unwrap().nodeport, 30320);
    }

    #[test]
    fn expect_create_folders() {
        let (dk, conf) = setup();

        // create a map
        let mut map: HashMap<String, &ConfigConfiture> = HashMap::new();
        map.insert("nginx".to_string(), &conf);

        let kubes = get_basic_objects(&vec![dk], map);
        // create folder
        match folder::create(&kubes) {
            Ok(_) => {},
            Err(_) => panic!("Expect to create folders")
        }
    }

    #[test]
    fn expect_to_create_kube() {
        let (dk, conf) = setup();
        // create a map
        let mut map: HashMap<String, &ConfigConfiture> = HashMap::new();
        map.insert("nginx".to_string(), &conf);

        let kubes = get_basic_objects(&vec![dk], map);
        match runner::create_default_object(kubes) {
            Ok(_) => {},
            Err(_) => panic!("Expect writing kubernetes object to not fail")
        }
    }

    #[test]
    fn expect_to_create_ingress() {
        let ingress = KubeIngress {
            name: "foo".to_string(),
            ip: "10.20.30.150".to_string(),
            backend: vec![
                IngressBackend {
                    service_name: "foo-svc".to_string(),
                    service_port: 3030,
                    path: "/foo".to_string()
                }
            ]
        };

        match objects::create(ingress, "ingress.yaml", objects::Objects::Ingress) {
            Ok(_) => {},
            Err(_) => panic!("Expect to write the ingress yaml file")
        }
    }
}
