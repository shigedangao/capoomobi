/// Conf module
/// 
/// # Description
/// Module use to retrieve the configuration of the docker-compose K8S bindings
pub mod conf {
  use std::path::PathBuf;
  use std::default::Default;
  use crate::cli::core::fs::toolbox::{get_home_dir, open_and_read_string_file};
  use crate::kubernetes::controllers::container::container::ControllerKind;
  use crate::kubernetes::controllers::service::service::ServiceType;

  const CONFITURE_FILE_NAME: &str = "confiture.json";

  /// Config Deployment structure
  struct ConfigDeployment {
    replicas: u8,
    controller: ControllerKind
  }

  /// Config Service structure
  struct ConfigService {
    service: ServiceType,
    nodeport: u64
  }
  
  /// Config structure
  struct Config {
    deployment: ConfigDeployment,
    service: ConfigService
  }

  struct Confiture {
    confitures: Vec<Config>
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
  fn retrieve_file_path(path: String) -> PathBuf {
    if path.is_empty() {
      return get_home_dir().push(CONFITURE_FILE_NAME);
    }

    PathBuf::from(path)
  }

  /// Load Conf
  /// 
  /// # Description
  /// Load the configuration file and retrieve it's contents
  /// 
  /// # Return
  /// Confiture structure
  pub fn load_conf(path: String) -> Confiture {
    let p = retrieve_file_path(path);
    let content = match open_and_read_string_file(&p) {
      Ok(c) => c,
      Err(err) => {}
    };
  }
}