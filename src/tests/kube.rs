#[cfg(test)]
mod controller {
    use std::path::PathBuf;
    use crate::kubernetes::controllers::controller::{
        KubeController,
        ControllerKind
    };
    use crate::docker::parser::DockerService;
    use crate::confiture::config::ConfigDeployment;

    // setup method use to get a default DockerService
    fn setup() -> (DockerService, ConfigDeployment) {
        let dk = DockerService {
            name: "capoo".to_string(),
            image: "shigedangao/capoomobi:latest".to_string(),
            commands: vec!["sh echo 'hey'".to_string()],
            ports: vec!["9000:9000".to_string()],
            labels: vec!["rust".to_string()],
            environment: vec!["mode=dev".to_string(), "john=doe".to_string()],
            volumes: vec![]
        };

        let conf = ConfigDeployment {
            replicas: 2,
            controller: ControllerKind::Deployment
        };

        (dk, conf)
    }


    #[test]
    fn expect_to_create_kube_controller() {
        let (dk, conf) = setup();
        let controller = KubeController::new(dk, &conf, &PathBuf::new()).unwrap();

        assert_eq!(controller.ctrl, ControllerKind::Deployment);
        assert_eq!(controller.name, "capoo");
        assert_eq!(controller.image, "shigedangao/capoomobi:latest");
        assert_eq!(controller.commands, vec!["sh echo 'hey'".to_string()]);
        assert_eq!(controller.ports, vec![9000]);
    }

    #[test]
    fn expect_create_kube_ports_empty() {
        let (mut dk, conf) = setup();
        dk.ports = vec![];
        let controller = KubeController::new(dk, &conf, &PathBuf::new()).unwrap();

        assert_eq!(controller.ctrl, ControllerKind::Deployment);
        assert_eq!(controller.name, "capoo");
        assert_eq!(controller.image, "shigedangao/capoomobi:latest");
        assert_eq!(controller.commands, vec!["sh echo 'hey'".to_string()]);
        assert_eq!(controller.ports, Vec::<u16>::new());
    }
}

#[cfg(test)]
mod service {
    use std::path::PathBuf;
    use crate::kubernetes::controllers::service::{
        ServiceType,
        KubeService
    };
    use crate::docker::parser::DockerService;
    use crate::confiture::config::ConfigService;

    // method use to get default DockerService
    fn setup() -> (DockerService, ConfigService) {
        let dk = DockerService {
            name: "capoo".to_string(),
            image: "shigedangao/capoomobi:latest".to_string(),
            commands: vec!["sh echo 'hey'".to_string()],
            ports: vec!["9000:9000".to_string()],
            labels: vec!["rust".to_string()],
            environment: vec!["mode=dev".to_string(), "john=doe".to_string()],
            volumes: vec![]
        };

        let conf = ConfigService  {
            kind: ServiceType::NodePort,
            nodeport: 9000
        };

        (dk, conf)
    }

    #[test]
    fn expect_create_service() {
        let (dk, conf) = setup();
        let service = KubeService::new(dk, &conf, &PathBuf::from("../foo")).unwrap();

        assert_eq!(service.name, "capoo-svc");
        assert_eq!(service.nodeport, 9000);
        assert_eq!(service.kind, ServiceType::NodePort);
        assert_eq!(service.host_port, 9000);
        assert_eq!(service.target_port, 9000);
        assert_eq!(service.labels, vec!["rust"]);
        assert_eq!(service.path.as_os_str(), "../foo/service.yaml");
    }

    #[test]
    fn exepct_not_create_service() {
        let (mut dk, conf) = setup();
        dk.ports = vec![];

        let service = KubeService::new(dk, &conf, &PathBuf::new());
        assert!(service.is_none())
    }
}

#[cfg(test)]
mod ingress {
    use crate::kubernetes::controllers::ingress::KubeIngress;
    use crate::docker::parser::DockerService;
    use crate::confiture::config::{ConfigIngress, ConfigIngressService};

    // method use to get the Config
    fn setup() -> (Vec<DockerService>, ConfigIngress) {
        let dk = DockerService {
            name: "capoo".to_string(),
            image: "shigedangao/capoomobi:latest".to_string(),
            commands: vec!["sh echo 'hey'".to_string()],
            ports: vec!["9000:9000".to_string()],
            labels: vec!["rust".to_string()],
            environment: vec!["mode=dev".to_string(), "john=doe".to_string()],
            volumes: vec![]
        };

        let conf = ConfigIngress {
            ip: "30.10.20.30".to_string(),
            services: vec![
                ConfigIngressService {
                    name: "capoo".to_string(),
                    path: "/capoo".to_string()
                }
            ]
        };

        (vec![dk], conf)
    }

    #[test]
    fn expect_create_ingress() {
        let (dks, conf) = setup();

        let ing = KubeIngress::new(&dks, conf);
        assert_eq!(ing.name, "ingress");
        assert_eq!(ing.ip, "30.10.20.30");
        assert_eq!(ing.backend[0].path, "/capoo");
        assert_eq!(ing.backend[0].service_name, "capoo-svc");
        assert_eq!(ing.backend[0].service_port, 9000);
    }
 }
