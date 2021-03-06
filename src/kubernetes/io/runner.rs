use async_std::{fs, io, task};
use async_std::prelude::*;
use futures::future::join_all;
use super::{print_errors};
use crate::kubernetes::io::output;
use crate::kubernetes::builder::{Kube};
use crate::kubernetes::template::controller::{ControllerTmplBuilder};
use crate::kubernetes::template::service::{ServiceTmplBuilder};
use crate::assets::loader::{K8SAssetType};
use crate::core::errors::cli_error::{CliErr, ErrHelper, ErrMessage};
use crate::core::errors::message::io::CREATING_FILE;

// Does this mod have too much responsability ? -> generate template + write it to the file in async

/// Create Controller
///
/// # Description
/// Create the controller files asynchronously
/// We return an array of Future that are executed in an async std thread
///
/// # Arguments
/// * `k` &[Kube]
///
/// Return
/// Vec<impl Future<Output = io::Result<()>>>
fn create_controller(k: &[Kube]) -> Vec<impl Future<Output = io::Result<()>>> {
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
/// * `k` &[Kube]
///
/// # Return
/// Vec<impl Future<Output = io::Result<()>>>
fn create_service(k: &[Kube]) -> Vec<impl Future<Output = io::Result<()>>> {
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
            Err(CliErr::new(CREATING_FILE, &err.to_string(), ErrMessage::IOError))
        })
        .collect()
}

/// Create default object
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
pub fn create_default_object(k: Vec<Kube>) -> Result<(), ()> {
    let ctrl_fut = create_controller(&k);
    let svc_fut  = create_service(&k);

    // Create the async task to run the vec of futures
    let ctrl_task = task::spawn(async move {
        let tasks = join_all(ctrl_fut).await;
        let out: Vec<Result<(), CliErr>> = parse_output(tasks);
        if !out.is_empty() {
            return Err(out);
        }

        return Ok(());
    });

    let svc_task = task::spawn(async move {
        let tasks = join_all(svc_fut).await;
        let out: Vec<Result<(), CliErr>> = parse_output(tasks);
        if !out.is_empty() {
            return Err(out);
        }

        return Ok(());
    });

    // run the tasks and wait for their results
    task::block_on(async {
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
    })
}
