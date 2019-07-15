/**
 * Error Helper trait
 * 
 * Define a set of common method to use the custom errors
 */
trait ErrorHelper<T> {
  fn new(message: String, reason: String, code: u8) -> T
};

/**
 * Cli Errors
 */
pub mod cli_errors {
  
  /**
   * Cli Error
   */
  struct CliErr {
    message: String,
    reason: String,
    code: u8
  }

  impl ErrorHelper<CliErr> for CliErr {
    fn new(message: String, reason: String, code: u8) -> CliErr {
      CliErr {
        message: message,
        reason: reason,
        code: code
      }
    }
  }
}

/**
 * Docker errors
 */
pub mod docker_errors {
  /**
   * Docker Err
   */
  struct DockerErr {
    message: String,
    code: u8
  }

  impl ErrorHelper<DockerErr> for DockerErr {
    fn new(message: String, _: String, code: u8) -> DockerErr {
      DockerErr {
        message: message,
        code: code
      }  
    }
  }
}

/**
 * Kubernetes errors
 */
pub mod kubernetes_errors {

}