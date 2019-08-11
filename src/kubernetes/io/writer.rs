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
  use futures::future::{lazy, ok, err, FutureResult, Future};
  use crate::kubernetes::tree::tree::{Kube};
  use crate::kubernetes::controllers::container::container::{KubeContainer};
  use crate::kubernetes::controllers::service::service::{KubeService};
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
        let controller_path = PathBuf::from(&kube.object.controller_path);
        let service_path = PathBuf::from(&kube.object.service_path);

        tokio::spawn(
          lazy(move || {
            return write_controller(&kube.object, controller_path)
              .and_then(move |_| write_service(&kube.service, service_path));
          })
        );
      }

      Ok(())
    }));
  }
  
  /// Write Controller
  /// 
  /// # Description
  /// Write controller yaml file
  /// 
  /// # Arguments
  /// * `kube` Reference to a the KubeContainer
  /// * `path` PathBuf
  /// 
  /// # Return
  /// FutureResult<(), ()>
  fn write_controller(kube: &KubeContainer, path: PathBuf) -> FutureResult<(), ()> {
    let ctrl_path = PathBuf::from(path);

    // write controller
    let controller = ControllerTmplBuilder{};
    let controller_tmpl = controller.render(kube);

    let ctrl_writer_result = match controller_tmpl {
      Some(t) => write_yaml(ctrl_path, t),
      None => {
        return err::<(), ()>(());
      }
    };

    if let Err(e) = ctrl_writer_result {
      panic!(e);
    }

    ok::<(), ()>(())
  }

  /// Write Service
  /// 
  /// # Description
  /// Write service yaml file
  /// 
  /// # Arguments
  /// * `svc` Reference to a the KubeService
  /// * `path` PathBuf
  /// 
  /// # Return
  /// FutureResult<(), ()>
  fn write_service(svc: &KubeService, path: PathBuf) -> FutureResult<(), ()> {
    let svc_path = PathBuf::from(path);

    // write services
    let service = ServiceTmplBuilder{};
    let service_tmpl = service.render(svc);

    let svc_writer_result = match service_tmpl {
      Some(tmpl) => write_yaml(svc_path, tmpl),
      None => {
        return err::<(), ()>(());
      }
    };

    if let Err(e) = svc_writer_result {
      panic!(e);
    }

    ok::<(), ()>(())
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