use vscode_derive::api;

use super::*;

pub type Query = IQueryState;

#[api]
pub struct IQueryState {
    pub page_number: usize,
    pub page_size: usize,
    pub sort_by: SortBy,
    pub sort_order: SortOrder,
    pub flags: Flags,
    pub criteria: Vec<ICriterium>,
    pub asset_types: Vec<String>,
    pub source: Option<String>,
}

impl IQueryState {
    pub const DEFAULT_PAGE_SIZE: usize = 10;
}

impl Default for IQueryState {
    fn default() -> Self {
        Self {
            page_number: 1,
            page_size: Self::DEFAULT_PAGE_SIZE,
            sort_by: SortBy::default(),
            sort_order: SortOrder::default(),
            flags: Flags::default(),
            criteria: vec![],
            asset_types: vec![],
            source: None,
        }
    }
}

/// extensionGalleryService.ts#Query
#[api(Default)]
pub struct QueryBuilder {
    state: IQueryState,
}

impl QueryBuilder {
    pub fn new(state: Query) -> Self {
        Self { state }
    }

    pub fn with_page(mut self, page_number: usize, page_size: Option<usize>) -> Self {
        self.state.page_number = page_number;
        if let Some(s) = page_size {
            self.state.page_size = s;
        }
        self
    }

    pub fn with_filter(mut self, filter_type: FilterType, values: Vec<String>) -> Self {
        if values.is_empty() {
            return self;
        }

        self.state
            .criteria
            .extend(values.into_iter().map(|item| ICriterium {
                filter_type: filter_type.clone(),
                value: item,
            }));
        self
    }

    pub fn with_sortby(mut self, sort_by: SortBy) -> Self {
        self.state.sort_by = sort_by;
        self
    }

    pub fn with_sort_order(mut self, order: SortOrder) -> Self {
        self.state.sort_order = order;
        self
    }

    pub fn with_flags(mut self, flags: Vec<Flags>) -> Self {
        flags.into_iter().for_each(|item| self.state.flags |= item);
        self
    }

    pub fn with_asset_types(mut self, asset_types: Vec<String>) -> Self {
        self.state.asset_types = asset_types;
        self
    }

    pub fn with_source(mut self, source: String) -> Self {
        self.state.source = Some(source);
        self
    }

    pub fn build(self) -> IQueryState {
        self.state
    }
}

#[api(Default)]
pub struct IExtensionCriteria {
    target_platform: TargetPlatform,
    compatible: bool,
    include_pre_release: IncludePreRelease,
    versions: Vec<CriteriaVersion>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum IncludePreRelease {
    Boolean(bool),
    Infos(CriteriaPreVersion),
}

impl Default for IncludePreRelease {
    fn default() -> Self {
        Self::Boolean(false)
    }
}

#[api(Default)]
pub struct CriteriaPreVersion {
    id: IExtensionIdentifier,
    include_pre_release: bool,
}

#[api(Default)]
pub struct CriteriaVersion {
    id: IExtensionIdentifier,
    version: String,
}

#[api(Default)]
pub struct IExtensionIdentifier {
    id: String,
    uuid: Option<String>,
}

#[api(Default)]
struct IRawGalleryExtensionsResult {
    gallery_extensions: Vec<IRawGalleryExtension>,
    total: usize, // context?: IStringDictionary<string>;
}
