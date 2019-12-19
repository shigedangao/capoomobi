use std::error::Error;
use std::path::PathBuf;
use serde::{Serialize};
use async_std::task::{spawn, block_on};
use async_std::{fs};
use crate::kubernetes::template::ingress::IngressTmplBuilder;
use crate::kubernetes::io::output;
use crate::assets::loader::{K8SAssetType};
use crate::core::errors::cli_error::{CliErr, ErrHelper, ErrMessage};
use crate::core::configurator::config;
use crate::core::errors::message::io::{
    CREATING_FILE,
    GET_PROJECT_PATH
};

/// Objcts
/// 
/// # Description
/// List of K8S Objects (exclude from controller & services)
pub enum Objects {
    Ingress
}

/// Create
/// 
/// # Description
/// Write an object T with the provided name and template
/// 
/// # Arguments
/// * `data` T where T = Serialize
/// * `filename` String
/// * `pp` PathBuf (project path)
/// * `kind` Objects
/// 
/// # Return
/// Result<(), ()>
pub fn create<T: Serialize>(data: T, filename: &'static str, kind: Objects) -> Result<(), ()> {
    let template = match kind {
        Objects::Ingress => IngressTmplBuilder {}
    };

    let res = output::render_component(&template, &data, K8SAssetType::Ingress);
    if let Err(e) = res {
        e.log_pretty();
        return Err(());
    }

    let rendered_tmpl = res.unwrap();
    let task = spawn(async move {
        let path = config::get_current_project_path();
        if path.is_none() {
            return Err(CliErr::new(CREATING_FILE, GET_PROJECT_PATH, ErrMessage::NotFound));
        }

        let mut pp = PathBuf::from(path.unwrap());
        pp.push(filename);

        let io_res = fs::write(pp, rendered_tmpl).await;
        match io_res {
            Ok(()) => Ok(()),
            Err(e) => Err(CliErr::new(CREATING_FILE, e.description(), ErrMessage::IOError))
        }
    });

    block_on(async {
        let res = task.await;
        if let Err(e) = res {
            e.log_pretty();
            return Err(());
        }

        Ok(())
    })
}
