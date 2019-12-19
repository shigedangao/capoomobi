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

/// Create Controller
/// 
/// # Description
/// Create the controller files asynchronously
/// We return an array of Future that are executed in an async std thread
/// 
/// # Arguments
/// * `k` &Vec<Kube>
/// 
/// Return
/// Vec<impl Future<Output = io::Result<()>>>
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

/// Create Service
/// 
/// # Description
/// Create the service files asynchronously. We're retrieving a vector of
/// future that are going to be resolve in a async std thread
/// 
/// # Arguments
/// * `k` &Vec<Kube>
/// 
/// # Return
/// Vec<impl Future<Output = io::Result<()>>>
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

/// Parse Output
/// 
/// # Description
/// Parse the output of the executed future and return the results
/// 
/// # Arguments
/// * `res` Vec<Result<(), io::Error>>
/// 
/// # Return
/// Vec<Result<(), CliErr>>
fn parse_output(res: Vec<Result<(), io::Error>>) -> Vec<Result<(), CliErr>> {
    res.into_iter()
        .filter(|v| v.is_err())
        .map(|v| {
            let err = v.unwrap_err();
            return Err(CliErr::new(CREATING_FILE, err.description(), ErrMessage::IOError));
        })
        .collect()
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
    let ctrl_fut = create_controller(&k);
    let svc_fut  = create_service(&k);

    let ctrl_task = task::spawn(async move {
        let tasks = join_all(ctrl_fut).await;
        let out: Vec<Result<(), CliErr>> = parse_output(tasks);
        if out.len() > 0 {
            return Err(out);
        }

        return Ok(());
    });

    let svc_task = task::spawn(async move {
        let tasks = join_all(svc_fut).await;
        let out: Vec<Result<(), CliErr>> = parse_output(tasks);
        if out.len() > 0 {
            return Err(out);
        }

        return Ok(());
    });

    let res = task::block_on(async {
        let sres = svc_task.await;
        let cres = ctrl_task.await;

        if let Err(e) = sres {
            print_errors(e);
            return Err(());
        }

        if let Err(e) = cres {
            print_errors(e);
            return Err(());
        }

        Ok(())
    });

    res
}
