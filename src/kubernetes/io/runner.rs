use std::pin::Pin;
use std::task::{Context, Poll};
use futures::future::Future;
use futures::future::join_all;
use futures::prelude::*;
use crate::kubernetes::io::output::{write_component};
use crate::kubernetes::tree::{Kube};
use crate::kubernetes::template::controller::controller::{ControllerTmplBuilder};
use crate::assets::loader::{K8SAssetType};
use crate::core::errors::cli_error::{CliErr};

struct KubeOutput<'a> {
    kube: &'a Kube
}

impl Future for KubeOutput<'_> {
    type Output = Result<(), CliErr>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>{
        let kube = self.kube;
        let ctrl_renderer = ControllerTmplBuilder {};

        let res = write_component(
            ctrl_renderer,
            &kube.ctrl,
            K8SAssetType::Controller,
            kube.project_path.clone()
        );

        match res {
            Ok(()) => Poll::Ready(Ok(())),
            Err(e) => Poll::Ready(Err(e)) 
        }
    }
}

pub fn run(k: Vec<Kube>) {
    let futures: Vec<KubeOutput> = k
        .iter()
        .map(|k| KubeOutput{kube: k})
        .collect();

    let f = join_all(futures);

    f.map(|v| {});
}