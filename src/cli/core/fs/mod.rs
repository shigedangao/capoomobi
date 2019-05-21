pub mod config;

/**
 * Fs module
 */
pub mod utility {
  use std::fs;
  // Constant use defining the available folders in the project
  const COMPOSE_PATH: &str = "/compose";
  const KUBE_PATH: &str = "/kube";

  /**
   * Fs struct
   * 
   * The fs struct store the base_path
   */
  pub struct Fs {
    pub base_path: String
  }

  impl Fs {
    /**
     * Build Dir
     * 
     * Build the compose dir
     * <base_path>/<COMPOSE_PATH>
     */
    pub fn build_compose_dir(&self) {
      let mut compose_path = String::from(self.base_path.as_str());
      compose_path.push_str("/");
      compose_path.push_str(COMPOSE_PATH);

      match build_dir(compose_path.as_str()) {
        Err(e) => println!("error {:?}", e),
        Ok(()) => println!("folder compose create")
      }
    }

    /**
     * Build Kube Dir
     * 
     * Build the kube dir
     * <base_path>/<KUBE_PATH>
     */
    pub fn build_kube_dir(&self) {
      let mut kube_path = String::from(self.base_path.as_str());
      kube_path.push_str("/");
      kube_path.push_str(KUBE_PATH);

      match build_dir(kube_path.as_str()) {
        Err(e) => println!("Error {:?}", e),
        Ok(()) => println!("Folder kube create")
      }
    }
  }

  /**
   * Build Base Path
   * 
   * Build the base path which is use across the CLI
   */
  pub fn build_base_path(name: &str, optional_path: &str) -> Fs {
    let mut base_path = String::new();

    if optional_path.is_empty() {
      base_path.push_str("./");    
    } else {
      base_path.push_str(optional_path);
      base_path.push_str("/");
    }

    base_path.push_str(name);
    let str_struct = Fs {
      base_path: base_path
    };

    return str_struct;
  }

  /**
   * Build dir
   * 
   * Build a directory and return a Result
   */
  fn build_dir(path: &str) -> std::io::Result<()> {
    fs::create_dir_all(path)?;
    Ok(())
  }
}