// https://github.com/microsoft/vscode/blob/d187d50a482ff80dcf74c35affb09dda1a7cd2fe/src/vs/platform/extensionManagement/common/extensionGalleryService.ts
mod enums;
mod extensions;
mod flags;
mod gallery_extension;
mod http_client;
mod query;
mod request_body;
mod version;

mod generated {
    use super::IQueryState;

    include!(concat!(env!("OUT_DIR"), "/gallery.rs"));
}

pub use generated::*;

mod generated_support {
    use super::FilterType;

    include!(concat!(env!("OUT_DIR"), "/gallery_support.rs"));
}

pub use generated_support::*;

pub use gallery_extension::*;

pub use enums::*;
pub use flags::*;
pub use http_client::*;
pub use request_body::*;
pub use version::*;
