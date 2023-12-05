use log::*;

use openvsx::apis::{configuration::Configuration, registry_api_api};

async fn get_matched_version_of(
    config: &Configuration,
    namespace: &str,
    extension: &str,
    engine_ver: &semver::Version,
) -> Option<String> {
    match registry_api_api::get_version_references1(config, namespace, extension, None, None).await
    {
        Ok(res) => {
            for ver in res.versions {
                todo!()
            }
            todo!()
        }
        Err(err) => {
            warn!("Error happend when get matched version of {namespace}.{extension} for {engine_ver}");
            None
        }
    }
}
