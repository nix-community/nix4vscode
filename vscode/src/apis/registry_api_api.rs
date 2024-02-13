use reqwest::{header::HeaderMap, Method};

use crate::models::{
    FilterType, Flags, IExtensionCriteria, IExtensionInfo, IGalleryExtension,
    IRawGalleryExtensionsResult, IRawGalleryQueryResult, Query, QueryBuilder, TargetPlatform,
};

use super::Configuration;

impl Configuration {
    pub async fn get_extensions(extension_infos: Vec<IExtensionInfo>) -> Vec<IGalleryExtension> {
        // let mut ids = vec![];
        // let mut names = vec![];
        todo!()
    }

    pub async fn get_compatible_extension(
        extension: IGalleryExtension,
        include_pre_release: bool,
        target_platform: TargetPlatform,
    ) -> Option<IGalleryExtension> {
        todo!()
    }

    async fn query_gallery_extensions(
        mut query: Query,
        criteria: IExtensionCriteria,
    ) -> (Vec<IGalleryExtension>, usize) {
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

        todo!()
    }

    async fn query_raw_gallery_extensions(&mut self, query: Query) -> IRawGalleryExtensionsResult {
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
            .request(Method::POST, format!("{}", self.base_path))
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
        todo!()
    }
}
