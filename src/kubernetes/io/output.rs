/// Output module
/// 
/// # Description
/// This module is use to write the files in an asynchronous way
use std::task::{Poll, Context};
use std::pin::{Pin};
use std::path::PathBuf;
use std::fs;
use std::error::Error;
use futures::{Future};
use serde::{Serialize};
use crate::kubernetes::tree::{Kube};
use crate::kubernetes::template::helper::common::TemplateBuilder;
use crate::assets::loader::{K8SAssetType};
use crate::core::errors::cli_error::{CliErr, ErrHelper, ErrMessage};
use crate::kubernetes::template::controller::controller::{ControllerTmplBuilder};

// struct KubeFiles<'a>{
//     kubes: &'a Kube
// }

// impl Future for KubeFiles<'_> {
//     type Output = Result<(), CliErr>;

//     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//         let ctrl_render = ControllerTmplBuilder{};

//         let ctrl = write_component(ctrl_render, &self.kubes.ctrl, K8SAssetType::Controller, self.kubes.ctrl.path);

//         match ctrl {
//             Ok(()) => Poll::Ready(Ok(())),
//             Err(e) => Poll::Ready(Err(e))
//         }
//     }
// }

pub fn run_controller(k: Kube) -> impl Future<Output = Result<(), CliErr>> {
    let ctrl_render = ControllerTmplBuilder{};

    async {
        write_component(ctrl_render, &k.ctrl, K8SAssetType::Controller, k.ctrl.path).await
    }
}

async fn write_component<T: Serialize>(tmpl: impl TemplateBuilder, cmp: &T, at: K8SAssetType, path: PathBuf) -> Result<(), CliErr> {
    let render_res = tmpl.render(cmp, at);
    if let Err(e) = render_res {
        return Err(e);
    }

    let res = render_res.unwrap();
    let output = write_yaml(path, res);

    if let Err(e) = output {
        return Err(CliErr::new("Unable to write yaml file", e.description(), ErrMessage::IOError));
    }

    Ok(())
}

/// Write Yaml
/// 
/// # Description
/// Write content to a Yaml file
/// 
/// # Arguments
/// * `p` Path of the file in the PathBuf format
/// * `template` generated template String rendered by Handlebar
/// 
/// # Return
/// * `std::io::Result<()>` 
fn write_yaml(p: PathBuf, template: String) -> std::io::Result<()> {
    fs::write(p, template.as_bytes())?;
    Ok(())
}