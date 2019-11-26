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
    use serde::{Serialize};
    use crate::core::errors::cli_error::{ErrHelper};
    use crate::assets::loader::{K8SAssetType};
    use crate::kubernetes::tree::{Kube};
    use crate::kubernetes::template::controller::controller::{ControllerTmplBuilder};
    use crate::kubernetes::template::service::service::{ServiceTmplBuilder};
    use crate::kubernetes::template::helper::common::{TemplateBuilder};

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
                        let ctrl_renderer = ControllerTmplBuilder{};
                        let svc_renderer  = ServiceTmplBuilder{};

                        let ctrl_path = PathBuf::from(&kube.ctrl.path);
                        let svc_path = PathBuf::from(&kube.svc.path);

                        match write_kube_cmp(ctrl_renderer, &kube.ctrl, K8SAssetType::Controller, ctrl_path) {
                            Ok(Ok(()), _) => Ok(()),
                            Err((e, _)) => Err(e)
                        };

                        let svc_res = write_kube_cmp(svc_renderer, &kube.svc, K8SAssetType::Service, svc_path);


                        ok::<(), ()>(())
                        //return write_controller(&kube.ctrl, kube.ctrl.path)
                        //    .and_then(move |_| write_service(&kube.svc, kube.svc.path));
                    })
                );
            }

            Ok(())
        }));
    }

    /// Write Kube Cmp
    /// 
    /// # Description
    /// Write kubernetes components (controller, service)
    /// 
    /// # Arguments
    /// * `tmpl` impl TemplateBuilder
    /// * `cmp` T: Serialize
    /// * `k8s_type` K8SAssetType
    /// * `path` PathBuf
    /// 
    /// # Return
    /// FutureResult<(), ()>
    fn write_kube_cmp<T: Serialize>(tmpl: impl TemplateBuilder, cmp: &T, k8s_type: K8SAssetType, path: PathBuf) -> FutureResult<(), ()> {
       let render_res = tmpl.render(cmp, k8s_type);
       if let Err(e) = render_res {
           e.log_pretty();
           return err::<(), ()>(());
       }

       let tmpl_str = render_res.unwrap();
       let write_res = write_yaml(path, tmpl_str);

       if let Err(e) = write_res {
           return err::<(), ()>(());
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