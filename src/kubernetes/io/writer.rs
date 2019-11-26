/// Writer
/// 
/// # Path
/// template/writer.rs
/// 
/// # Description
/// Module use to write K8S controller & service files
pub mod writer {
    use std::fs;
    use std::path::PathBuf;
    use futures::future::{lazy, ok, err, FutureResult, Future};
    use crate::core::errors::cli_error::{ErrHelper};
    use crate::assets::loader::{K8SAssetType};
    use crate::kubernetes::tree::{Kube};
    use crate::kubernetes::controllers::controller::{KubeController};
    use crate::kubernetes::controllers::service::{KubeService};
    use crate::kubernetes::template::controller::controller::{ControllerTmplBuilder};
    use crate::kubernetes::template::service::service::{ServiceTmplBuilder};
    use crate::kubernetes::template::helper::common::TemplateBuilder;

    /// Write Kubernetes Yaml
    /// 
    /// # Description
    /// Module use to write a kubernetes template asynchronously
    /// 
    /// # Arguments
    /// * `kubes` List of K8S Kube structure
    pub fn write_kubernetes_yaml(kubes: Vec<Kube>) {
        tokio::run(lazy(|| {
            for kube in kubes.into_iter() {
                tokio::spawn(
                    lazy(move || {
                        return write_controller(&kube.ctrl, kube.ctrl.path)
                            .and_then(move |_| write_service(&kube.svc, kube.svc.path));
                    })
                );
            }

            Ok(())
        }));
    }

    /// Write Controller
    /// 
    /// # Description
    /// Write controller yaml file
    /// 
    /// # Arguments
    /// * `kube` Reference to a the KubeContainer
    /// * `path` PathBuf
    /// 
    /// # Return
    /// FutureResult<(), ()>
    fn write_controller(kube: &KubeController, path: PathBuf) -> FutureResult<(), ()> {
        // write controller
        let controller = ControllerTmplBuilder{};
        let tmpl = controller.render(kube, K8SAssetType::Controller);

        let res = match tmpl {
            Ok(t) => write_yaml(path, t),
            Err(e) => {
                e.log_pretty();
                return err::<(), ()>(());
            }
        };

        if let Err(e) = res {
            panic!(e);
        }

        ok::<(), ()>(())
    }

    /// Write Service
    /// 
    /// # Description
    /// Write service yaml file
    /// 
    /// # Arguments
    /// * `svc` Reference to a the KubeService
    /// * `path` PathBuf
    /// 
    /// # Return
    /// FutureResult<(), ()>
    fn write_service(svc: &KubeService, path: PathBuf) -> FutureResult<(), ()> {
        let svc_path = PathBuf::from(path);

        // write services
        let service = ServiceTmplBuilder{};
        let tmpl = service.render(svc, K8SAssetType::Service);

        let res = match tmpl {
            Ok(t) => write_yaml(svc_path, t),
            Err(e) => {
                e.log_pretty();
                return err::<(), ()>(());
            }
        };

        if let Err(e) = res {
            panic!(e);
        }

        ok::<(), ()>(())
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
}