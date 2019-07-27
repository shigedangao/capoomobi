/**
 * Writer module
 * 
 * Module use to trigger asynchronous 
 */
pub mod writer {
  use std::fs;
  use std::path::PathBuf;
  use futures::future::lazy;
  use crate::kubernetes::tree::Tree::{Kube};
  use crate::kubernetes::template::controller::controller;
  use crate::kubernetes::template::service::service::{ServiceTmplBuilder};
  use crate::kubernetes::template::common::TemplateBuilder;

  /**
   * Async Yaml Writer
   * 
   * Write a set of yaml asynchronously
   */
  pub fn write_kubernetes_yaml(kubes: Vec<Kube>) {
    tokio::run(lazy(|| {
      for kube in kubes.into_iter() {
        tokio::spawn(lazy(move || {
          let ctrl_path = PathBuf::from(&kube.object.controller_path);
          let svc_path = PathBuf::from(&kube.object.service_path);

          let ctrl_tmpl = controller::template(kube.object);
          if let Some(template) = ctrl_tmpl {
            let res = write_yaml(ctrl_path, template);
            if let Err(err) = res {
              panic!(err);
            }
          }

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

  fn write_yaml(p: PathBuf, template: String) -> std::io::Result<()> {
    fs::write(p, template.as_bytes())?;
    Ok(())
  }
}