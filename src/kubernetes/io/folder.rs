/// Boostrap module
///
/// # Description
/// Module use to preparing the following phases for create K8S yaml files
/// - creating folders
use std::path::PathBuf;
use std::error::Error;
use async_std::{fs::DirBuilder, io, task};
use futures::future::{Future, join_all};
use super::{print_errors};
use crate::kubernetes::builder::{Kube};
use crate::core::logger::{log, LogType};
use crate::core::errors::cli_error::{CliErr, ErrHelper, ErrMessage};
use crate::core::errors::message::io::CREATE_KUBE_FOLDER;

/// Get Builders
///
/// # Description
/// Create a set of Future that will create the folder in an asynchronous way
///
/// # Arguments
/// * `paths` Vec<PathBuf>
///
/// # Return
/// Vec<impl Future<Output = io::Result<()>>>
fn get_builders(paths: Vec<PathBuf>) -> Vec<impl Future<Output = io::Result<()>>> {
    log(LogType::Info, "Creating kubernetes folders...", None);
    let mut futures = Vec::new();
    let mut builder = DirBuilder::new();
    builder.recursive(true);

    for p in paths.into_iter() {
        futures.push(builder.create(p));
    }

    futures
}

/// Create
///
/// # Description
/// Create the folder asynchronously
///
/// # Arguments
/// * `kubes` &[Kube]
///
/// # Return
/// Result<(), ()>
pub fn create(kubes: &[Kube]) -> Result<(), ()> {
    let paths: Vec<PathBuf> = kubes
        .iter()
        .map(|v| v.project_path.clone())
        .collect();

    let futures = get_builders(paths);
    let tasks = task::spawn(async move {
        let values = join_all(futures).await;
        let out: Vec<Result<(), CliErr>> = values
            .into_iter()
            .filter(|v| v.is_err())
            .map(|v| {
                let err = v.unwrap_err();
                Err(CliErr::new(CREATE_KUBE_FOLDER, err.description(), ErrMessage::IOError))
            })
            .collect();

        if !out.is_empty() {
            return Err(out);
        }

        Ok(())
    });

    match task::block_on(tasks) {
        Ok(()) => Ok(()),
        Err(err) => {
            print_errors(err);
            Err(())
        }
    }
}
