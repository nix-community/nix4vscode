use std::pin::pin;
use std::str::FromStr;

use code_api::code::is_version_valid;
use code_api::code::AssetType;
use code_api::code::HttpClient;
use code_api::code::IRawGalleryExtension;
use code_api::code::TargetPlatform;
use futures::future::join_all;
use futures::stream;
use futures::StreamExt;

use semver::Version;
use tracing::debug;
use tracing::error;
use tracing::trace;

use crate::config::Config;
use crate::jinja::AssetUrlContext;
use crate::jinja::Generator;
use crate::jinja::NixContext;
use crate::utils;

pub struct CodeNix {
    config: Config,
    client: HttpClient,
}

impl CodeNix {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            client: HttpClient::new().unwrap(),
        }
    }

    pub async fn get_extensions(&mut self, generator: Generator<'static>) -> Vec<NixContext> {
        let mut obj = vec![];
        {
            let mut iter = self
                .client
                .get_extension_response(self.config.handled_extensions.clone())
                .filter_map(|item| async move {
                    match item {
                        Ok(v) => Some(v),
                        Err(_) => None,
                    }
                })
                .flat_map(|item| stream::iter(item.extensions));

            let mut iter = pin!(iter);
            while let Some(val) = iter.next().await {
                if self
                    .config
                    .contains(&val.publisher.publisher_name, &val.extension_name)
                {
                    obj.push(val);
                }
            }
        }

        let futures: Vec<_> = obj
            .into_iter()
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
            } else if res.iter().any(|ctx: &NixContext| {
                item.publisher.publisher_name == ctx.publisher && ctx.name == item.extension_name
            }) {
                continue;
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
                "{}-{}-{:?}-{:?}",
                item.publisher.publisher_name,
                item.extension_name,
                asset_url,
                version.target_platform
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
                    name: item.extension_name.to_lowercase(),
                    publisher: item.publisher.publisher_name.to_lowercase(),
                    version: version.version.clone(),
                    asset_url: if has_asset_url {
                        Some(asset_url.clone())
                    } else {
                        None
                    },
                    sha256: sha256.clone(),
                    platform: target_platform,
                });

            res.extend(a);
        }

        trace!(?res);

        res
    }
}
