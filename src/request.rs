mod enums;
mod flags;
mod request_body;

pub use enums::*;
pub use flags::*;
pub use request_body::*;

// pub fn create_request_body(flags: u32, page_number: u64, page_size: u64) -> String {
//     let obj = RequestBody {
//         filters: vec![IQueryState {
//             criteria: vec![ICriterium {
//                 filter_type: FilterType::TARGET,
//                 value: "Microsoft.VisualStudio.Code".into(),
//             }],
//             page_number,
//             page_size,
//             sort_by: SortBy,
//             sort_order: 0,
//         }],
//         asset_types: vec![],
//         flags,
//     };
//
//     serde_json::to_string(&obj).unwrap()
// }
