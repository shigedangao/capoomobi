use std::error::Error;
use async_std::{fs, io, task};
use async_std::prelude::*;
use futures::future::join_all;
use super::{print_errors};
use crate::kubernetes::io::output;
use crate::kubernetes::builder::{Kube};
use crate::kubernetes::template::controller::controller::{ControllerTmplBuilder};
use crate::kubernetes::template::service::service::{ServiceTmplBuilder};
use crate::assets::loader::{K8SAssetType};
use crate::core::errors::cli_error::{CliErr, ErrHelper, ErrMessage};
use crate::core::errors::message::io::CREATING_FILE;

fn create_controller(k: &Vec<Kube>) -> Vec<impl Future<Output = io::Result<()>>> {
    let ctrl_tmpl = ControllerTmplBuilder {};
    let mut vec = Vec::new();

    for c in k {
        let tmpl = output::render_component(&ctrl_tmpl, &c.ctrl, K8SAssetType::Controller);
        match tmpl {
            Ok(t) => {
                let future = fs::write(c.ctrl.path.clone(), t.clone());
                vec.push(future);
            },
            Err(err) => err.log_pretty()
        }
    }

    vec
}

fn create_service(k: &Vec<Kube>) -> Vec<impl Future<Output = io::Result<()>>> {
    let svc_tmpl = ServiceTmplBuilder {};
    let mut vec = Vec::new();

    for s in k {
        if let Some(svc) = &s.svc {
            let tmpl = output::render_component(&svc_tmpl, &svc, K8SAssetType::Service);
            match tmpl {
                Ok(t) => {
                    let future = fs::write(svc.path.clone(), t.clone());
                    vec.push(future);
                },
                Err(err) => err.log_pretty()
            }
        }
    }

    vec
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
    // thought it might have been better to use thread... cloning a lot...
    let ctrl_fut = create_controller(&k);
    let svc_fut  = create_service(&k);

    let ctrl_task = task::spawn(async move {
        let tasks = join_all(ctrl_fut).await;
        let out: Vec<Result<(), CliErr>> = tasks
            .into_iter()
            .filter(|v| v.is_err())
            .map(|v| {
                let err = v.unwrap_err();
                return Err(CliErr::new(CREATING_FILE, err.description(), ErrMessage::IOError));
            })
            .collect();

        if out.len() > 0 {
            return Err(out);
        }

        return Ok(());
    });

    match task::block_on(ctrl_task) {
        Ok(()) => Ok(()),
        Err(err) => {
            print_errors(err);
            Err(())
        }
    }
}
