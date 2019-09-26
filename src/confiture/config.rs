/// Conf module
/// 
/// # Description
/// Module use to retrieve the configuration of the docker-compose K8S bindings
pub mod conf {
  use std::path::PathBuf;
  use serde::{Deserialize};
  use serde_json;
  use crate::cli::core::fs::toolbox::{get_absolute_path, open_and_read_string_file};
  use crate::kubernetes::controllers::container::container::ControllerKind;
  use crate::kubernetes::controllers::service::service::ServiceType;

  const CONFITURE_FILE_NAME: &str = "./confiture.json";

  /// Config Deployment structure
  #[derive(Deserialize, Debug)]
  struct ConfigDeployment {
    replicas: u8,
    controller: ControllerKind
  }

  /// Config Service structure
  #[derive(Deserialize, Debug)]
  struct ConfigService {
    kind: ServiceType,
    nodeport: u64
  }
  
  /// Config structure
  #[derive(Deserialize, Debug)]
  struct Config {
    deployment: ConfigDeployment,
    service: ConfigService,
    name: String
  }

  /// Confiture
  #[derive(Deserialize, Debug)]
  pub struct Confiture {
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
  fn retrieve_file_path(path: String, folder: &str) -> PathBuf {
    if path.is_empty() {
      let mut o_path = PathBuf::from(String::from(folder));
      o_path.push(CONFITURE_FILE_NAME);

      return match get_absolute_path(&o_path) {
        Ok(p) => p,
        Err(err) => {
          print!("encule {:?}", err);
          return PathBuf::new();
        }
      }
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
  pub fn load_conf(path: String, target_folder: &str) -> Option<Confiture> {
    let p = retrieve_file_path(path, target_folder);

    println!("value of path {:?}", p);
    let content = match open_and_read_string_file(&p) {
      Ok(c) => c,
      Err(err) => {
        return None;
      }
    };

    // @TODO handle error
    let confiture: Confiture = match serde_json::from_str(&content) {
      Ok(c) => c,
      Err(err) => {
        println!("fils de tchoin {:?}", err);
        return None;
      }
    };

    Some(confiture)
  }
}