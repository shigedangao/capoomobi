#[cfg(test)]
mod controller {
    use std::collections::HashMap;
    use yaml_rust::{YamlLoader, Yaml};
    use crate::docker::parser::DockerService;
    use crate::kubernetes::builder::{get_basic_objects};
    use crate::kubernetes::controllers::controller::ControllerKind;
    use crate::kubernetes::controllers::service::ServiceType;
    use crate::confiture::config::{
        ConfigConfiture,
        ConfigDeployment,
        ConfigService
    };
    use crate::kubernetes::io::output;
    use crate::kubernetes::template::controller::ControllerTmplBuilder;
    use crate::assets::loader::K8SAssetType;

    // Create a docker service to test the kube io process
    fn setup() -> String {
        let dk = DockerService {
            name       : "nginx".to_owned(),
            image      : "nginx:1.3.0".to_owned(),
            commands   : vec!["sudo nginx reload".to_owned()],
            ports      : vec!["9000:9000".to_owned()],
            labels     : vec!["app=front".to_owned(), "tier=front".to_owned()],
            environment: vec!["API_ENV=dev".to_owned()],
            volumes    : vec![]
        };

        let conf = ConfigConfiture {
            name: "nginx".to_owned(),
            deployment: ConfigDeployment {
                replicas: 3,
                controller: ControllerKind::Deployment
            },
            service: ConfigService {
                kind: ServiceType::NodePort,
                nodeport: 30320
            }
        };

        let mut map: HashMap<String, &ConfigConfiture> = HashMap::new();
        map.insert("nginx".to_owned(), &conf);

        // build kubes objects
        let kubes = get_basic_objects(&[dk], map);
        let builder = ControllerTmplBuilder{};

        // render the template
        output::render_component(
            &builder,
            &kubes[0].ctrl,
            K8SAssetType::Controller
        ).unwrap()
    }

    #[test]
    fn expect_to_render_spec() {
        let template = setup();
        let resources = YamlLoader::load_from_str(&template).unwrap();
        let raw = &resources[0];

        // testing values
        assert_eq!(raw["apiVersion"].as_str().unwrap(), "apps/v1");
        assert_eq!(raw["kind"].as_str().unwrap(), "Deployment");
    }

    #[test]
    fn expect_to_render_labels() {
        let template = setup();
        // retrieve the resources
        let resources = YamlLoader::load_from_str(&template).unwrap();
        let raw = &resources[0];

        let metadata = raw["metadata"].as_hash().unwrap();
        let app = metadata
            .get(&Yaml::from_str("labels"))
            .unwrap()
            .as_hash()
            .unwrap()
            .get(&Yaml::from_str("app"))
            .unwrap()
            .as_str()
            .unwrap();

        let name = metadata
            .get(&Yaml::from_str("name"))
            .unwrap()
            .as_str()
            .unwrap();

        // get metadata
        assert_eq!(name, "nginx");
        assert_eq!(app, "front");
    }

    #[test]
    fn expect_to_have_base_metadata() {
        let template = setup();
        let resources = YamlLoader::load_from_str(&template).unwrap();
        let raw = &resources[0];

        // spec
        // yaml value
        let spec = &raw["spec"].as_hash().unwrap();
        let replicas = spec
            .get(&Yaml::from_str("replicas"))
            .unwrap()
            .as_i64()
            .unwrap();

        let match_labels = spec
            .get(&Yaml::from_str("selector"))
            .unwrap()
            .as_hash()
            .unwrap()
            .get(&Yaml::from_str("matchLabels"))
            .unwrap()
            .as_hash()
            .unwrap();

        let app_label = match_labels
            .get(&Yaml::from_str("app"))
            .unwrap()
            .as_str()
            .unwrap();

        let tier_label = match_labels
            .get(&Yaml::from_str("tier"))
            .unwrap()
            .as_str()
            .unwrap();

        assert_eq!(replicas, 3);
        assert_eq!(app_label, "front");
        assert_eq!(tier_label, "front");
    }

    #[test]
    fn expect_to_render_containers() {
        let template = setup();
        let resources = YamlLoader::load_from_str(&template).unwrap();
        let raw = &resources[0];

        // spec
        // yaml value
        let spec = &raw["spec"].as_hash().unwrap();
        // get template
        let tmpl = spec
            .get(&Yaml::from_str("template"))
            .unwrap()
            .as_hash()
            .unwrap();

        // get spec of template
        let spec = tmpl
            .get(&Yaml::from_str("spec"))
            .unwrap()
            .as_hash()
            .unwrap();

        // get containers
        let ct = spec
            .get(&Yaml::from_str("containers"))
            .unwrap()
            .as_vec()
            .unwrap();

        let container = &ct[0].as_hash().unwrap();
        let name = container
            .get(&Yaml::String("name".to_owned()))
            .unwrap()
            .as_str()
            .unwrap();

        let image = container
            .get(&Yaml::String("image".to_owned()))
            .unwrap()
            .as_str()
            .unwrap();

        let ports = container
            .get(&Yaml::String("ports".to_owned()))
            .unwrap()
            .as_vec()
            .unwrap();

        let container_port = ports[0]
            .as_hash()
            .unwrap()
            .get(&Yaml::String("containerPort".to_owned()))
            .unwrap()
            .as_i64()
            .unwrap();

        assert_eq!(name, "nginx");
        assert_eq!(image, "nginx:1.3.0");
        assert_eq!(container_port, 9000);
    }
}

#[cfg(test)]
mod services {
    use std::collections::HashMap;
    use yaml_rust::{YamlLoader, Yaml};
    use crate::docker::parser::DockerService;
    use crate::kubernetes::builder::{get_basic_objects};
    use crate::kubernetes::controllers::controller::ControllerKind;
    use crate::kubernetes::controllers::service::ServiceType;
    use crate::confiture::config::{
        ConfigConfiture,
        ConfigDeployment,
        ConfigService
    };
    use crate::kubernetes::io::output;
    use crate::kubernetes::template::service::ServiceTmplBuilder;
    use crate::assets::loader::K8SAssetType;

    // Build a service as a string
    fn setup() -> String {
        let dk = DockerService {
            name       : "nginx".to_owned(),
            image      : "nginx:1.3.0".to_owned(),
            commands   : vec!["sudo nginx reload".to_owned()],
            ports      : vec!["9000:9000".to_owned()],
            labels     : vec!["app=front".to_owned(), "tier=front".to_owned()],
            environment: vec!["API_ENV=dev".to_owned()],
            volumes    : vec![]
        };

        let conf = ConfigConfiture {
            name: "nginx".to_owned(),
            deployment: ConfigDeployment {
                replicas: 3,
                controller: ControllerKind::Deployment
            },
            service: ConfigService {
                kind: ServiceType::NodePort,
                nodeport: 30320
            }
        };

        let mut map_config: HashMap<String, &ConfigConfiture> = HashMap::new();
        map_config.insert("nginx".to_owned(), &conf);
        let kubes = get_basic_objects(&[dk], map_config);

        // init the service template builder
        let builder = ServiceTmplBuilder{};
        let service = output::render_component(
            &builder,
            &kubes[0].svc,
            K8SAssetType::Service
        ).unwrap();

        service
    }

    #[test]
    fn expect_to_get_first_level_tag() {
        let service_str = setup();
        let yaml = YamlLoader::load_from_str(&service_str).unwrap();
        // extract the service binary representation
        let service = &yaml[0];

        // get first level of value
        let version = service["apiVersion"].as_str().unwrap();
        let kind    = service["kind"].as_str().unwrap();

        assert_eq!(version, "v1");
        assert_eq!(kind, "Service");
    }

    #[test]
    fn expect_to_get_metadata() {
        let service_str = setup();
        let yaml = YamlLoader::load_from_str(&service_str).unwrap();
        // extract the service binary representation
        let service = &yaml[0];

        // get the metadata
        let metadatas = service["metadata"].as_hash().unwrap();
        let name = metadatas
            .get(&Yaml::from_str("name"))
            .unwrap()
            .as_str()
            .unwrap();

        let labels = metadatas
            .get(&Yaml::from_str("labels"))
            .unwrap()
            .as_hash()
            .unwrap();

        let app_label = labels
            .get(&Yaml::from_str("app"))
            .unwrap()
            .as_str()
            .unwrap();

        let tier_label = labels
            .get(&Yaml::from_str("tier"))
            .unwrap()
            .as_str()
            .unwrap();

        assert_eq!(name, "nginx-svc");
        assert_eq!(app_label, "front");
        assert_eq!(tier_label, "front");
    }

    #[test]
    fn test_spec_metadata() {
        let service_str = setup();
        let yaml = YamlLoader::load_from_str(&service_str).unwrap();
        // extract the service binary representation
        let service = &yaml[0];

        // get the spec body
        let spec = service["spec"].as_hash().unwrap();
        let kind = spec
            .get(&Yaml::from_str("type"))
            .unwrap()
            .as_str()
            .unwrap();

        let selector = spec
            .get(&Yaml::from_str("selector"))
            .unwrap()
            .as_hash()
            .unwrap();

        let app = selector
            .get(&Yaml::from_str("app"))
            .unwrap()
            .as_str()
            .unwrap();

        let tier = selector
            .get(&Yaml::from_str("tier"))
            .unwrap()
            .as_str()
            .unwrap();

        assert_eq!(kind, "NodePort");
        assert_eq!(app, "front");
        assert_eq!(tier, "front");
    }

    #[test]
    fn test_spec_ports_body() {
        let service_str = setup();
        let yaml = YamlLoader::load_from_str(&service_str).unwrap();
        // extract the service binary representation
        let service = &yaml[0];

        // get the spec body
        let spec = service["spec"].as_hash().unwrap();
        let ports = spec.get(&Yaml::from_str("ports"))
            .unwrap()
            .as_vec()
            .unwrap();

        let port = ports[0].as_hash().unwrap();
        let protocol = port
            .get(&Yaml::from_str("protocol"))
            .unwrap()
            .as_str()
            .unwrap();

        let port_value = port
            .get(&Yaml::from_str("port"))
            .unwrap()
            .as_i64()
            .unwrap();

        let target_port = port
            .get(&Yaml::from_str("targetPort"))
            .unwrap()
            .as_i64()
            .unwrap();

        let nodeport = port
            .get(&Yaml::from_str("nodePort"))
            .unwrap()
            .as_i64()
            .unwrap();

        assert_eq!(protocol, "TCP");
        assert_eq!(port_value, 9000);
        assert_eq!(target_port, 9000);
        assert_eq!(nodeport, 30320);
    }
}

#[cfg(test)]
mod ingress {
    use yaml_rust::{Yaml, YamlLoader};
    use crate::docker::parser::DockerService;
    use crate::kubernetes::controllers::ingress::KubeIngress;
    use crate::confiture::config::{ConfigIngress, ConfigIngressService};
    use crate::kubernetes::template::ingress::IngressTmplBuilder;
    use crate::kubernetes::io::output;
    use crate::assets::loader::K8SAssetType;

    fn setup() -> String {
        let dk = DockerService {
            name       : "nginx".to_owned(),
            image      : "nginx:1.3.0".to_owned(),
            commands   : vec!["sudo nginx reload".to_owned()],
            ports      : vec!["9000:9000".to_owned()],
            labels     : vec!["app=front".to_owned(), "tier=front".to_owned()],
            environment: vec!["API_ENV=dev".to_owned()],
            volumes    : vec![]
        };

        let config = ConfigIngress {
            ip: "10.20.30.10".to_owned(),
            services: vec![
                ConfigIngressService {
                    name: "nginx".to_owned(),
                    path: "/web".to_owned()
                }
            ]
        };

        let ingress = KubeIngress::new(&[dk], config);
        let builder = IngressTmplBuilder {};

        output::render_component(&builder, &ingress, K8SAssetType::Ingress).unwrap()
    }

    #[test]
    fn expect_to_have_basic_metadata() {
        let template = setup();
        // load string as a yaml representation
        let yaml = YamlLoader::load_from_str(&template).unwrap();
        let ingress = &yaml[0];

        assert_eq!(ingress["apiVersion"].as_str().unwrap(), "extensions/v1beta1");
        assert_eq!(ingress["kind"].as_str().unwrap(), "Ingress");
    }

    #[test]
    fn expect_to_have_ingress_metadata() {
        let template = setup();
        // load string as a yaml representation
        let yaml = YamlLoader::load_from_str(&template).unwrap();
        let ingress = &yaml[0];

        // get the metadatas
        let metadatas = ingress["metadata"].as_hash().unwrap();

        // retrieve metadata attribute
        let name = metadatas
            .get(&Yaml::from_str("name"))
            .unwrap()
            .as_str()
            .unwrap();

        let annotations = metadatas
            .get(&Yaml::from_str("annotations"))
            .unwrap()
            .as_hash()
            .unwrap();

        let ip = annotations
            .get(&Yaml::from_str("kubernetes.io/ingress.global-static-ip-name"))
            .unwrap()
            .as_str()
            .unwrap();

        assert_eq!(name, "ingress");
        assert_eq!(ip, "10.20.30.10");
    }

    #[test]
    fn test_body_spec() {
        let template = setup();

        // yaml
        let yaml = YamlLoader::load_from_str(&template).unwrap();
        let ingress = &yaml[0];
        let rules = ingress["spec"]
            .as_hash()
            .unwrap()
            .get(&Yaml::from_str("rules"))
            .unwrap()
            .as_vec()
            .unwrap();

        let paths = rules[0]
            .as_hash()
            .unwrap()
            .get(&Yaml::from_str("http"))
            .unwrap()
            .as_hash()
            .unwrap()
            .get(&Yaml::from_str("paths"))
            .unwrap()
            .as_vec()
            .unwrap();

        let nginx_path = &paths[0];
        let path = nginx_path["path"].as_str().unwrap();
        let backend = nginx_path["backend"].as_hash().unwrap();

        let service_name = backend
            .get(&Yaml::from_str("serviceName"))
            .unwrap()
            .as_str()
            .unwrap();

        let service_port = backend
            .get(&Yaml::from_str("servicePort"))
            .unwrap()
            .as_i64()
            .unwrap();

        assert_eq!(path, "/web");
        assert_eq!(service_name, "nginx-svc");
        assert_eq!(service_port, 9000);
    }
}
