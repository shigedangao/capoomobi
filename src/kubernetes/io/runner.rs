use std::pin::Pin;
use std::task::{Context, Poll};
use async_std::{task};
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
            kube.ctrl.path.clone()
        );

        match res {
            Ok(()) => Poll::Ready(Ok(())),
            Err(e) => Poll::Ready(Err(e)) 
        }
    }
}

pub fn run(k: Vec<Kube>) {
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
            return Err(());
        }

        return Ok(());
    });

    println!("write task run");
    match task::block_on(write_task) {
        Ok(()) => println!("task done ! success"),
        Err(()) => println!("task error")
    }
    println!("write task done");
}