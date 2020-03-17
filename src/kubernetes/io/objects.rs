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

    // render the component by using the template
    let res = output::render_component(&template, &data, K8SAssetType::Ingress);
    if let Err(e) = res {
        e.log_pretty();
        return Err(());
    }

    let rendered_tmpl = res.unwrap();
    // Create the object asynchronously
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
            Err(e) => Err(CliErr::new(CREATING_FILE, &e.to_string(), ErrMessage::IOError))
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
