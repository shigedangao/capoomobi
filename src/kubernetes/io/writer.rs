/// Writer
/// 
/// # Path
/// template/writer.rs
/// 
/// # Description
/// Module use to write K8S controller & service files
pub mod writer {
  use std::fs;
  use std::path::PathBuf;
  use futures::future::lazy;
  use crate::kubernetes::tree::tree::{Kube};
  use crate::kubernetes::template::controller::controller::{ControllerTmplBuilder};
  use crate::kubernetes::template::service::service::{ServiceTmplBuilder};
  use crate::kubernetes::template::common::TemplateBuilder;

  /// Write Kubernetes Yaml
  /// 
  /// # Description
  /// Module use to write a kubernetes template asynchronously
  /// 
  /// # Arguments
  /// * `kubes` List of K8S Kube structure
  pub fn write_kubernetes_yaml(kubes: Vec<Kube>) {
    tokio::run(lazy(|| {
      for kube in kubes.into_iter() {
        tokio::spawn(lazy(move || {
          let ctrl_path = PathBuf::from(&kube.object.controller_path);
          let svc_path = PathBuf::from(&kube.object.service_path);

          // Controller
          let controller = ControllerTmplBuilder::new(kube.object);
          let controller_tmpl = controller.template();
          if let Some(template) = controller_tmpl {
            let res = write_yaml(ctrl_path, template);
            if let Err(err) = res {
              panic!(err);
            }
          }

          // Service
          let service = ServiceTmplBuilder::new(kube.service);
          let service_tmpl = service.template();
          if let Some(tmpl) = service_tmpl {
            let res = write_yaml(svc_path, tmpl);
            if let Err(err) = res {
              panic!(err);
            }
          }

          Ok(())
        }));
      }

      Ok(())
    }));
  }

  /// Write Yaml
  /// 
  /// # Description
  /// Write content to a Yaml file
  /// 
  /// # Arguments
  /// * `p` Path of the file in the PathBuf format
  /// * `template` generated template String rendered by Handlebar
  /// 
  /// # Return
  /// * `std::io::Result<()>` 
  fn write_yaml(p: PathBuf, template: String) -> std::io::Result<()> {
    fs::write(p, template.as_bytes())?;
    Ok(())
  }
}