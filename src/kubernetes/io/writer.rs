/**
 * Writer module
 * 
 * Module use to trigger asynchronous 
 */
pub mod Writer {
  use futures::future::lazy;
  use crate::kubernetes::tree::Tree::{Kube};
  use crate::kubernetes::template::controller::controller;

  /**
   * Async Yaml Writer
   * 
   * Write a set of yaml asynchronously
   */
  pub fn write_kubernetes_yaml(kubes: Vec<Kube>) {
    tokio::run(lazy(|| {
      for kube in kubes.into_iter() {
        tokio::spawn(lazy(move || {
          controller::template(kube.object);
          Ok(())
        }));
      }

      Ok(())
    }));
  }
}