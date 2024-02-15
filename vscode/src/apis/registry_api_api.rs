use reqwest::{header::HeaderMap, Method};

use crate::models::{
    FilterType, Flags, IExtensionCriteria, IExtensionInfo, IExtensionQueryOptions,
    IGalleryExtension, IRawGalleryQueryResult, IncludePreRelease, Query, QueryBuilder,
};

use super::Configuration;

impl Configuration {
    pub async fn get_extensions(
        &mut self,
        infos: &[IExtensionInfo],
        option: IExtensionQueryOptions,
    ) -> Vec<IGalleryExtension> {
        let mut ids = vec![];
        let mut names = vec![];

        infos.iter().for_each(|item| {
            if let Some(uuid) = &item.uuid {
                ids.push(uuid.clone());
            } else {
                names.push(item.id.clone());
            }
        });
        let query = QueryBuilder::new(Default::default())
            .with_page(1, Some(infos.len()))
            .with_filter(FilterType::ExtensionId, ids)
            .with_filter(FilterType::ExtensionName, names)
            .build();

        let criteria = IExtensionCriteria {
            target_platform: option.target_platform.unwrap(),
            compatible: option.compatible.unwrap(),
            include_pre_release: IncludePreRelease::Boolean(true),
            versions: Default::default(),
        };

        self.query_gallery_extensions(query, criteria)
            .await
            .unwrap()
    }

    async fn query_gallery_extensions(
        &mut self,
        mut query: Query,
        criteria: IExtensionCriteria,
    ) -> anyhow::Result<Vec<IGalleryExtension>> {
        let flags = query.flags;
        if (flags.contains(Flags::IncludeLatestVersionOnly))
            && flags.contains(Flags::IncludeVersions)
        {
            query.flags.remove(Flags::IncludeVersions);
        }
        if (!flags.contains(Flags::IncludeLatestVersionOnly))
            && !flags.contains(Flags::IncludeVersions)
        {
            query.flags &= Flags::IncludeLatestVersionOnly;
        }

        if !criteria.versions.is_empty() {
            query.flags.remove(Flags::IncludeVersions);
            query.flags.remove(Flags::IncludeLatestVersionOnly);
        }

        query.flags = query.flags
            & Flags::IncludeAssetUri
            & Flags::IncludeCategoryAndTags
            & Flags::IncludeFiles
            & Flags::IncludeStatistics
            & Flags::IncludeVersionProperties;

        let has_all_versions = query.flags.contains(Flags::IncludeLatestVersionOnly);
        let res = self.query_raw_gallery_extensions(query).await?;
        if has_all_versions {
            return Ok(res
                .results
                .into_iter()
                .map(|item| IGalleryExtension::create(item, criteria.clone()))
                .collect());
        }

        todo!()
    }

    async fn query_raw_gallery_extensions(
        &mut self,
        query: Query,
    ) -> anyhow::Result<IRawGalleryQueryResult> {
        let query = QueryBuilder::new(query)
            .with_flags(vec![Flags::ExcludeNonValidated])
            .with_filter(
                FilterType::Target,
                vec!["Microsoft.VisualStudio.Code".to_string()],
            )
            .with_filter(
                FilterType::ExcludeWithFlags,
                vec![Flags::Unpublished.to_string()],
            )
            .build();

        let mut map = HeaderMap::new();
        map.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );
        map.insert(
            reqwest::header::ACCEPT,
            "application/json;api-version=3.0-preview.1"
                .parse()
                .unwrap(),
        );
        map.insert(reqwest::header::ACCEPT_ENCODING, "gzip".parse().unwrap());

        let req = self
            .client
            .request(Method::POST, self.base_path.to_string())
            .headers(map)
            .json(&query)
            .build()
            .unwrap();

        let rep: IRawGalleryQueryResult = self
            .client
            .execute(req)
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        Ok(rep)
    }
}
