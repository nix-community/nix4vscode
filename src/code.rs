// https://github.com/microsoft/vscode/blob/d187d50a482ff80dcf74c35affb09dda1a7cd2fe/src/vs/platform/extensionManagement/common/extensionGalleryService.ts
mod enums;
mod extensions;
mod flags;
mod gallery_extension;
mod http_client;
mod query;
mod request_body;
mod version;

use std::str::FromStr;

pub use extensions::*;
use futures::future::join_all;
pub use gallery_extension::*;

pub use enums::*;
pub use flags::*;
pub use http_client::*;
pub use query::*;
pub use request_body::*;
use semver::Version;
use tracing::debug;
use tracing::error;
use tracing::trace;
pub use version::*;

use crate::config::Config;
use crate::jinja::AssetUrlContext;
use crate::jinja::CodeExt;
use crate::jinja::CodeExtension;
use crate::jinja::Generator;
use crate::jinja::NixContext;
use crate::utils;

pub struct CodeNix {
    config: Config,
    client: HttpClient,
    code_version: String,
}

impl CodeNix {
    pub fn new(config: Config) -> Self {
        let _ = Version::from_str(&config.vscode_version).unwrap();
        Self {
            code_version: config.vscode_version.clone(),
            config,
            client: HttpClient::new().unwrap(),
        }
    }

    pub async fn query_one(
        &mut self,
        publisher: &str,
        name: &str,
    ) -> anyhow::Result<CodeExtension> {
        let res = self.client.query_one(publisher, name).await?;

        let mx_ver = res
            .versions
            .iter()
            .filter(|item: &&IRawGalleryExtensionVersion| item.get_engine().is_ok())
            .filter(|item| is_version_valid(&self.code_version, &item.get_engine().unwrap()))
            .max_by(|a, b| {
                Version::from_str(&a.version)
                    .unwrap()
                    .cmp(&Version::from_str(&b.version).unwrap())
            })
            .map(|item| item.version.clone())
            .unwrap();

        let universal = res
            .versions
            .iter()
            .filter(|item| item.version == mx_ver)
            .find(|item| item.target_platform.is_none());

        if let Some(v) = universal {
            return Ok(CodeExtension {
                publisher: publisher.to_string(),
                name: name.to_string(),
                universal: Some(CodeExt {
                    version: v.version.clone(),
                    sha256: None,
                }),
                ..Default::default()
            });
        }

        let x86_linux = res
            .versions
            .iter()
            .filter(|item| item.version == mx_ver)
            .find(|item| match item.target_platform {
                Some(ref v) => v.to_lowercase() == "linux-x64",
                None => false,
            })
            .map(|item| CodeExt {
                version: item.version.to_string(),
                sha256: None,
            });
        let aarch64_linux = res
            .versions
            .iter()
            .filter(|item| item.version == mx_ver)
            .find(|item| match item.target_platform {
                Some(ref v) => v.to_lowercase() == "linux-arm64",
                None => false,
            })
            .map(|item| CodeExt {
                version: item.version.to_string(),
                sha256: None,
            });

        let x86_darwin = res
            .versions
            .iter()
            .filter(|item| item.version == mx_ver)
            .find(|item| match item.target_platform {
                Some(ref v) => v.to_lowercase() == "darwin-x64",
                None => false,
            })
            .map(|item| CodeExt {
                version: item.version.to_string(),
                sha256: None,
            });
        let aarch64_darwin = res
            .versions
            .iter()
            .filter(|item| item.version == mx_ver)
            .find(|item| match item.target_platform {
                Some(ref v) => v.to_lowercase() == "darwin-arm64",
                None => false,
            })
            .map(|item| CodeExt {
                version: item.version.to_string(),
                sha256: None,
            });

        Ok(CodeExtension {
            publisher: publisher.to_string(),
            name: name.to_string(),
            universal: None,
            x86_linux,
            aarch64_linux,
            x86_darwin,
            aarch64_darwin,
        })
    }

    pub async fn get_extensions(&mut self, generator: Generator<'static>) -> Vec<NixContext> {
        let obj = self
            .client
            .get_extension_response(&self.config.extensions)
            .await
            .unwrap();

        let futures: Vec<_> = obj
            .results
            .into_iter()
            .flat_map(|item| item.extensions.into_iter())
            .filter(|item| {
                match self
                    .config
                    .contains(&item.publisher.publisher_name, &item.extension_name)
                {
                    true => true,
                    false => {
                        debug!(
                            "extensions be filtered {}.{}",
                            item.publisher.publisher_name, item.extension_name
                        );
                        false
                    }
                }
            })
            .map(|item| {
                trace!("aa");
                let generator = generator.clone();
                self.get_matched_versoin(item, generator)
            })
            .collect();

        join_all(futures).await.into_iter().flatten().collect()
    }

    pub async fn get_matched_versoin(
        &self,
        item: IRawGalleryExtension,
        generator: Generator<'static>,
    ) -> Vec<NixContext> {
        let vscode_ver = self.config.vscode_version.as_str();
        let mx = item
            .versions
            .iter()
            .filter(|v| match v.get_engine() {
                Ok(ver) => {
                    if !is_version_valid(vscode_ver, &ver) {
                        trace!("{ver} doesn't match {vscode_ver:?}");
                        return false;
                    }
                    trace!("{} - {}", v.version, ver);
                    true
                }
                Err(_) => {
                    debug!(
                        "Cannot get engine version for {}.{} {}",
                        item.publisher.publisher_name, item.extension_name, v
                    );
                    trace!("{v:#?}");
                    true
                }
            })
            .filter_map(|item| Version::from_str(&item.version).ok())
            .max_by(|a, b| a.cmp(b));

        let mx = mx.map(|item| item.to_string());
        trace!(?mx);

        let mut res = vec![];
        for version in &item.versions {
            if let Some(mx) = mx.as_ref() {
                if mx != &version.version {
                    continue;
                }
            }
            trace!("{:?}", version.version);
            if let Some(ref v) = version.target_platform {
                let t: TargetPlatform = v.as_str().into();
                if !matches!(
                    t,
                    TargetPlatform::LinuxX64
                        | TargetPlatform::LinuxArm64
                        | TargetPlatform::Universal
                        | TargetPlatform::Web
                        | TargetPlatform::DarwinX64
                        | TargetPlatform::DarwinArm64
                ) {
                    continue;
                }
            }
            let (has_asset_url, asset_url) = match self
                .config
                .get_asset_url(&item.publisher.publisher_name, &item.extension_name)
            {
                Some(url) => {
                    debug!(url);
                    let url = generator.render_asset_url(
                        &url,
                        &AssetUrlContext::new(
                            self.config
                                .get_system_ctx(
                                    &item.publisher.publisher_name,
                                    &item.extension_name,
                                )
                                .unwrap_or_default(),
                            version.version.clone(),
                        ),
                    );
                    (true, url)
                }
                None => (
                    false,
                    version.get_file(AssetType::Vsix).unwrap().source.clone(),
                ),
            };
            debug!(
                "{}-{}-{:?}",
                item.publisher.publisher_name, item.extension_name, asset_url
            );

            let sha256 = match utils::get_sha256(&asset_url).await {
                Ok(sha256) => sha256,
                Err(err) => {
                    error!("get sha256 failed: {err}");
                    continue;
                }
            };

            let target_platform = match version.target_platform {
                Some(ref t) => vec![t.as_str().into()],
                None => {
                    self.client
                        .get_extension_target_platform(
                            item.publisher.publisher_name.clone(),
                            item.extension_name.clone(),
                        )
                        .await
                }
            };
            trace!(?target_platform);

            let a = target_platform
                .into_iter()
                .filter(|item| {
                    matches!(
                        *item,
                        TargetPlatform::LinuxX64
                            | TargetPlatform::LinuxArm64
                            | TargetPlatform::Universal
                            | TargetPlatform::Web
                            | TargetPlatform::DarwinX64
                            | TargetPlatform::DarwinArm64
                    )
                })
                .map(|target_platform| NixContext {
                    extension_name: item.extension_name.to_lowercase(),
                    publisher_name: item.publisher.publisher_name.to_lowercase(),
                    extension_version: version.version.clone(),
                    asset_url: if has_asset_url {
                        Some(asset_url.clone())
                    } else {
                        None
                    },
                    sha256: sha256.clone(),
                    target_platform,
                });

            res.extend(a);
        }

        trace!(?res);

        res
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_query_one() {
        let mut client = CodeNix::new(Config {
            vscode_version: "1.75.0".into(),
            autogen_warning: Default::default(),
            extensions: vec![],
            system: None,
        });

        for i in [
            ("editorconfig", "editorconfig"),
            ("ms-ceintl", "vscode-language-pack-zh-hans"),
            ("pkief", "material-icon-theme"),
            ("pkief", "material-product-icons"),
            ("zhuangtongfa", "material-theme"),
            ("redhat", "vscode-xml"),
            ("redhat", "vscode-yaml"),
        ] {
            let res = client.query_one(i.0, i.1).await.unwrap();

            assert!(res.universal.is_some());
        }
    }

    #[tokio::test]
    async fn test_query_one_with() {
        let mut client = CodeNix::new(Config {
            vscode_version: "1.88.0".into(),
            autogen_warning: Default::default(),
            extensions: vec![],
            system: None,
        });

        for i in [("hashicorp", "terraform"), ("ms-toolsai", "jupyter")] {
            let res = client.query_one(i.0, i.1).await.unwrap();

            assert!(res.x86_linux.is_some());
            assert!(res.aarch64_linux.is_some());
            assert!(res.x86_darwin.is_some());
            assert!(res.aarch64_darwin.is_some());
        }
    }
}
