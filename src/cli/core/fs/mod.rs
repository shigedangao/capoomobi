pub mod operations;

/**
 * Fs module
 */
pub mod utility {
  use std::fs;
  use std::path::PathBuf;
  // Constant use defining the available folders in the project
  const COMPOSE_PATH: &str = "compose";
  const KUBE_PATH: &str = "kube";

  /**
   * Fs struct
   * 
   * The fs struct store the base_path
   */
  pub struct Fs {
    pub base_path: PathBuf
  }

  impl Fs {
    /**
     * Build Dir
     * 
     * Build the compose dir
     * <base_path>/<COMPOSE_PATH>
     */
    pub fn build_compose_dir(&self) {
      let mut compose_path = PathBuf::from(&self.base_path);
      compose_path.push(COMPOSE_PATH);

      match build_dir(compose_path) {
        Err(e) => panic!("folder compose can not be create {:?}", e),
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
      let mut kube_path = PathBuf::from(&self.base_path);
      kube_path.push(KUBE_PATH);

      match build_dir(kube_path) {
        Err(e) => panic!("folder kube can not be create {:?}", e),
        Ok(()) => println!("Folder kube create")
      }
    }

      /**
       * Get Abs Path
       * 
       * Get the absolute path from a string
       */
      pub fn get_abs_path(&self) -> std::io::Result<PathBuf> {
        let path = fs::canonicalize(&self.base_path)?;
        Ok(path)
      }
  }

  /**
   * Build Base Path
   * 
   * Build the base path which is use across the CLI
   */
  pub fn build_base_path(name: &str, optional_path: &str) -> Fs {
    let mut base_path = PathBuf::new();

    if optional_path.is_empty() {
      base_path.push("./");    
    } else {
      base_path.push(optional_path);
    }

    base_path.push(name);
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
  fn build_dir(path: PathBuf) -> std::io::Result<()> {
    fs::create_dir_all(path)?;
    Ok(())
  }
}