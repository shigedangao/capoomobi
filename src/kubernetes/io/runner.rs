use std::pin::Pin;
use std::task::{Context, Poll};
use async_std::{task};
use futures::future::Future;
use futures::future::join_all;
use super::{print_errors};
use crate::kubernetes::io::output::{write_component};
use crate::kubernetes::tree::{Kube};
use crate::kubernetes::template::controller::controller::{ControllerTmplBuilder};
use crate::kubernetes::template::service::service::{ServiceTmplBuilder};
use crate::assets::loader::{K8SAssetType};
use crate::core::errors::cli_error::{CliErr};

/// KubeOutput
/// 
/// # Description
/// Struct which will be use by the future executor to write the files
struct KubeOutput<'a> {
    kube: &'a Kube
}

impl Future for KubeOutput<'_> {
    type Output = Result<(), CliErr>;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output>{
        let kube = self.kube;
        let ctrl_renderer = ControllerTmplBuilder {};
        let svc_renderer = ServiceTmplBuilder {};

        let ctrl_res = write_component(
            ctrl_renderer,
            &kube.ctrl,
            K8SAssetType::Controller,
            kube.ctrl.path.clone()
        );

        let res = ctrl_res
            .and_then(|_| write_component(
                svc_renderer,
                &kube.svc,
                K8SAssetType::Service,
                kube.svc.path.clone())
            );

        match res {
            Ok(()) => Poll::Ready(Ok(())),
            Err(e) => Poll::Ready(Err(e)) 
        }
    }
}

/// Run
/// 
/// # Description
/// Create an array of Kube Future that are run asynchronously
/// This future write the content to the targeted yaml files
/// 
/// # Arguments
/// * `k` Vec<Kube>
/// 
/// # Return
/// Result<(), ()>
pub fn run(k: Vec<Kube>) -> Result<(), ()> {
    let write_task = task::spawn(async move {
        let futures: Vec<KubeOutput> = k
            .iter()
            .map(|k| KubeOutput{kube: k})
            .collect();

        let vec = join_all(futures).await;
        let out: Vec<Result<(), CliErr>> = vec
            .into_iter()
            .filter(|res| res.is_err())
            .collect();

        if out.len() > 0 {
            return Err(out);
        }

        return Ok(());
    });

    match task::block_on(write_task) {
        Ok(()) => Ok(()),
        Err(err) => {
            print_errors(err);
            Err(())
        }
    }
}
