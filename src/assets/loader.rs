use std::str;
use rust_embed::{RustEmbed};
use crate::errors::cli_error::{CliErr, ErrCode, ErrHelper};

const PARSE_ERROR: &str = "Something went wrong while parsing the content of a template file";

#[derive(RustEmbed)]
#[folder = "static/k8s/"]
struct K8SAsset;

/// K8SAsset_type
pub enum K8SAsset_type {
    Controller,
    Service
}

impl K8SAsset_type {
    fn value(self) -> &'static str {
        match self {
            K8SAsset_type::Controller => "controller_tmpl.yaml",
            K8SAsset_type::Service => "service_tmpl.yaml"
        }
    }
}

/// Retrieve Asset Content
/// 
/// # Description
/// Retrieve the content of a specified asset
/// 
/// # Arguments
/// * `k` K8SAsset_type enum
/// 
/// # Return
/// Option<String>
pub fn retrieve_asset_content(k: K8SAsset_type) -> Option<String> {
    let filename = K8SAsset_type::value(k);
    let result = match K8SAsset::get(filename) {
        Some(s) => s,
        None => {
            return None;
        }
    };

    match str::from_utf8(result.as_ref()) {
        Ok(s) => Some(String::from(s)),
        Err(err) => {
            CliErr::new(
                PARSE_ERROR,
                String::new(),
                ErrCode::ParsingError
            ).log_pretty();

            return None;
        }
    }
}