/// Output module
/// 
/// # Description
/// This module is use to write the files in an asynchronous way
use std::path::PathBuf;
use std::fs;
use std::error::Error;
use serde::{Serialize};
use crate::kubernetes::template::helper::common::TemplateBuilder;
use crate::assets::loader::{K8SAssetType};
use crate::core::errors::cli_error::{CliErr, ErrMessage, ErrHelper};


pub fn write_component<T: Serialize>(tmpl: impl TemplateBuilder, cmp: &T, k8s_type: K8SAssetType, path: PathBuf) -> Result<(), CliErr> {
    let render_res = tmpl.render(cmp, k8s_type);
    if let Err(e) = render_res {
        return Err(e);
    }

    let tmpl_str = render_res.unwrap();
    let write_res = write_yaml(path, tmpl_str);

    if let Err(e) = write_res {
        println!("error {:?}", e.description());
        return Err(CliErr::new("", e.description(), ErrMessage::IOError));
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