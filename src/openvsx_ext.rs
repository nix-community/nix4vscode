#![allow(unused_assignments)]

use std::str::FromStr;

use tracing::*;

use openvsx::{
    apis::{configuration::Configuration, registry_api_api},
    models::VersionReference,
};

pub async fn get_matched_version_of(
    config: &Configuration,
    namespace: &str,
    extension: &str,
    engine_ver: &semver::Version,
) -> Vec<VersionReference> {
    loop {
        let mut offset = 0usize;
        let size = 20usize;

        match registry_api_api::get_version_references1(
            config,
            namespace,
            extension,
            Some(size as i32),
            Some(offset as i32),
        )
        .await
        {
            Ok(res) => {
                // FIXME: get all available version for it.
                offset += res.versions.len();

                let mut fn_res = vec![];
                for ver in res.versions {
                    if ver.version.is_none() {
                        continue;
                    }
                    match ver.target_platform {
                        Some(ref pl) => {
                            if pl == "Unknown" {
                                continue;
                            }
                        }
                        None => continue,
                    };
                    if ver
                        .engines
                        .iter()
                        .flatten()
                        .filter_map(|(_, v)| semver::VersionReq::from_str(v).ok())
                        .any(|ver| ver.matches(engine_ver))
                    {
                        fn_res.push(ver.clone());
                    }
                }

                if !fn_res.is_empty() {
                    return fn_res;
                }
            }
            Err(err) => {
                warn!("Error happend when get matched version of {namespace}.{extension} for {engine_ver}");
                return vec![];
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_get_version() {
        let a = get_matched_version_of(
            &Default::default(),
            "redhat",
            "java",
            &semver::Version::new(1, 77, 0),
        )
        .await;
        assert!(!a.is_empty());
    }
}
