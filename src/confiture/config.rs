/// Conf module
/// 
/// # Description
/// Module use to retrieve the configuration of the docker-compose K8S bindings
pub mod conf {
    use std::path::PathBuf;
    use std::error::Error;
    use std::collections::HashMap;
    use serde::{Deserialize};
    use serde_json;
    use crate::core::fs::toolbox::{get_absolute_path, open_and_read_string_file};
    use crate::core::logger::{log, LogType};
    use crate::kubernetes::controllers::container::ControllerKind;
    use crate::kubernetes::controllers::service::ServiceType;

    /// Constant
    const CONFITURE_FILE_NAME: &str = "./confiture.json";

    /// Config Deployment structure
    #[derive(Deserialize, Debug, Clone)]
    pub struct ConfigDeployment {
        pub replicas: u8,
        pub controller: ControllerKind
    }

    /// Config Service structure
    #[derive(Deserialize, Debug, Clone)]
    pub struct ConfigService {
        pub kind: ServiceType,
        #[serde(default)]
        pub nodeport: u16
    }

    /// Config structure
    #[derive(Deserialize, Debug, Clone)]
    pub struct Config {
        pub deployment: ConfigDeployment,
        pub service: ConfigService,
        name: String
    }

    /// Ingress Service
    #[derive(Deserialize, Debug)]
    pub struct IngressService {
        pub name: String,
        pub path: String
    }

    /// Ingress
    #[derive(Deserialize, Debug)]
    pub struct Ingress {
        pub ip: String,
        pub services: Vec<IngressService>
    }

    /// Confiture
    #[derive(Deserialize, Debug)]
    pub struct Confiture {
        confitures: Vec<Config>,
        ingress: Option<Ingress>
    }

    /// Retrieve File Path
    /// 
    /// # Description
    /// Retrieve the file path in the PathBuf format
    /// 
    /// # Arguments
    /// * `path` String
    /// 
    /// # Return
    /// PathBuf
    fn retrieve_file_path(path: String, folder: &str) -> PathBuf {
        if path.is_empty() {
            let mut o_path = PathBuf::from(String::from(folder));
            o_path.push(CONFITURE_FILE_NAME);

            return match get_absolute_path(&o_path) {
                Ok(p) => p,
                Err(_) => {
                    return PathBuf::new();
                }
            }
        }

        PathBuf::from(path)
    }

    /// Get Hashmap From Confiture
    /// 
    /// # Description
    /// Get a hashmap from the confiture struct
    /// 
    /// # Arguments
    /// * `conf` Confiture struct
    /// 
    /// # Return
    /// HashMap<String, Config>
    fn get_hashmap_from_confiture(conf: Confiture) -> HashMap<String, Config> {
        let mut map = HashMap::new();
        for c in conf.confitures {
            map.insert(String::from(&c.name), c);
        }

        map
    }

    /// Load Conf
    /// 
    /// # Description
    /// Load the configuration file and retrieve it's contents
    /// 
    /// # Param
    /// * `path` String
    /// * `target_folder` &str
    /// 
    /// # Return
    /// HashMap<String, Config>
    pub fn load_conf(path: String, target_folder: &str) -> Option<HashMap<String, Config>> {
        let p = retrieve_file_path(path, target_folder);
        let content = match open_and_read_string_file(&p) {
            Ok(c) => c,
            Err(err) => {
                log(LogType::Warning, err.description(), None);
                return None;
            }
        };

        let confiture: Confiture = match serde_json::from_str(&content) {
            Ok(c) => c,
            Err(err) => {
                log(LogType::Warning, err.description(), None);
                return None;
            }
        };

        let map = get_hashmap_from_confiture(confiture);
        Some(map)
    }
}