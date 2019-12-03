use std::str;
use rust_embed::{RustEmbed};
use crate::core::errors::cli_error::{CliErr, ErrMessage, ErrHelper};

const GET_ERROR: &str = "Unable to get the requested file";
const PARSE_ERROR: &str = "Something went wrong while parsing the content of a template file";

#[derive(RustEmbed)]
#[folder = "static/k8s/"]
struct K8SAsset;

/// K8SAssetType
pub enum K8SAssetType {
    Controller,
    Service,
    Ingress
}

impl K8SAssetType {
    fn value(self) -> &'static str {
        match self {
            K8SAssetType::Controller => "controller_tmpl.yaml",
            K8SAssetType::Service    => "service_tmpl.yaml",
            K8SAssetType::Ingress    => "ingress_tmpl.yaml"
        }
    }
}

/// Retrieve Asset Content
/// 
/// # Description
/// Retrieve the content of a specified asset
/// 
/// # Arguments
/// * `k` K8SAssetType enum
/// 
/// # Return
/// Result<String, CliErr>
pub fn retrieve_asset_content(k: K8SAssetType) -> Result<String, CliErr> {
    let filename = K8SAssetType::value(k);
    let res = K8SAsset::get(filename);

    if let Some(file) = res {
        return match str::from_utf8(&file) {
            Ok(s) => Ok(String::from(s)),
            Err(_) => Err(CliErr::new(PARSE_ERROR, "", ErrMessage::ParsingError))
        };
    }

    Err(CliErr::new(GET_ERROR, "", ErrMessage::NotFound))
}