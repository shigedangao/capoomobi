/// Output module
/// 
/// # Description
/// This module is use to write the files in an asynchronous way
use serde::{Serialize};
use crate::kubernetes::template::helper::common::TemplateBuilder;
use crate::assets::loader::{K8SAssetType};
use crate::core::errors::cli_error::{CliErr};

/// Render Component
/// 
/// # Description
/// Write a K8S Component (object) to a file
/// 
/// # Arguments
/// * `tmpl` impl TemplateBuilder
/// * `cmp` &T impl serde::Serialize
/// * `k8s_type` K8SAssetType
/// * `path` PathBuf
/// 
/// # Return
/// Result<(), CliErr>
pub fn render_component<T: Serialize>(tmpl: &impl TemplateBuilder, cmp: &T, k8s_type: K8SAssetType) -> Result<String, CliErr> {
    let render_res = tmpl.render(cmp, k8s_type);
    render_res
}

