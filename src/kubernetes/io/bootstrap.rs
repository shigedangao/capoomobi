/// Boostrap module
/// 
/// # Description
/// Module use to preparing the following phases for create K8S yaml files
/// - creating folders
/// - creating empty yaml files
pub mod bootstrap {
    use std::path::PathBuf;
    use std::error::Error;
    use crate::kubernetes::tree::{Kube};
    use crate::cli::core::fs::toolbox;
    use crate::cli::core::logger::{log, LogType};
    use crate::errors::cli_error::{CliErr, ErrHelper, ErrMessage};

    const CREATE_KUBE_FOLDER_ERR: &str = "Unable to create kubernetes folder";
    const CREATE_KUBE_FILES_ERR: &str = "Unable to create kubernetes file";

    /// Prepare Kube
    /// 
    /// # Description
    /// Creating K8S services folders
    /// 
    /// # Arguments
    /// * `kubes` Reference to Vec<Kube>
    /// 
    /// # Return
    /// Result<(), CliErr>
    pub fn prepare_kube(kubes: &Vec<Kube>) -> Result<(), CliErr> {
        log(
            LogType::Info, 
            "Creating kubernetes folders...",
            None
        );

        for kube in kubes.into_iter() {
            let results = create_kubernetes_folder(&kube.object.path)
                .and_then(|_| create_file(&kube.object.controller_path, "controller"))
                .and_then(|_| create_file(&kube.object.service_path, "service"));

            if results.is_err() {
                return Err(results.err().unwrap());
            }
        }
    
        Ok(())
    }

    /// Create Kubernetes Folder
    /// 
    /// # Description
    /// Create K8S folder
    /// 
    /// # Arguments
    /// * `path` Reference to a PathBuf
    /// 
    /// # Return
    /// Result<(), CliErr>
    fn create_kubernetes_folder(path: &PathBuf) -> Result<(), CliErr> {
        match toolbox::create_folder_from_pathbuf(path) {
            Ok(()) => {
                log(
                    LogType::Info,
                    "Successfully creating folder",
                    None
                );

                Ok(())
            },
            Err(err) => Err(CliErr::new(CREATE_KUBE_FOLDER_ERR, String::from(err.description()), ErrMessage::IOError))
        }
    }

    /// Create File
    /// 
    /// # Description
    /// Create empty yaml files
    /// 
    /// # Arguments
    /// * `path` Reference to a PathBuf
    /// * `message` file type as a slice of string
    /// 
    /// # Return
    /// Result<(), CliErr>
    fn create_file(path: &PathBuf, message: &str) -> Result<(), CliErr> {
        match toolbox::create_file(PathBuf::from(path)) {
            Ok(_) => {
                log(
                    LogType::Success,
                    format!("Successfully initialize {} file", message).as_str(),
                    None
                );

                Ok(())
            },
            Err(err) => Err(
                CliErr::new(CREATE_KUBE_FILES_ERR, String::from(err.description()), ErrMessage::IOError)
            )
        }
    }
}