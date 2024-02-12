use super::*;
use vscode_derive::api;

#[api(Default)]
pub struct ICriterium {
    pub filter_type: FilterType,
    pub value: String,
}

impl ICriterium {
    pub fn const_vscode() -> Self {
        Self {
            filter_type: FilterType::Target,
            value: "Microsoft.VisualStudio.Code".to_string(),
        }
    }

    // pub fn const_exclude() -> Self {
    //     Self {
    //         filter_type: FilterType::EXCLUDE_WITH_FLAGS,
    //         value: RequestFlags::,
    //     }
    // }
}
