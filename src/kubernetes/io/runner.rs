use futures::future::join_all;
use super::output::{run_controller};
use crate::kubernetes::tree::{Kube};


async fn run_futures(kubes: Vec<Kube>) {
    let futures = kubes
        .iter()
        .map(|k| run_controller(k.clone()));
}