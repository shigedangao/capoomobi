/**
 * Service
 */
pub mod service {
  /**
   * Structure representing a Kubernetes service
   */
  pub struct KubeService {
    port: String,
    service_type: String,
    labels: Vec<String>
  }
}