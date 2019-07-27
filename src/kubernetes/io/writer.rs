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
  use crate::kubernetes::template::service::service;

  /**
   * Async Yaml Writer
   * 
   * Write a set of yaml asynchronously
   */
  pub fn write_kubernetes_yaml(kubes: Vec<Kube>) {
    tokio::run(lazy(|| {
      for kube in kubes.into_iter() {
        tokio::spawn(lazy(move || {
          let controller_path = PathBuf::from(&kube.object.controller_path);
          
          let option = controller::template(kube.object);
          if let Some(template) = option {
            match write_yaml(controller_path, template) {
              Ok(()) => {
                println!("success !!!!");
                return Ok(())
              },
              Err(err) => panic!(err)
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