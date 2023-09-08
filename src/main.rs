#![allow(dead_code)]
#![allow(unused_variables)]

pub(crate) mod data;
pub(crate) mod request;

#[tokio::main]
async fn main() {
    let obj: data::IRawGalleryQueryResult =
        serde_json::from_str(include_str!("../vscode.json")).unwrap();
    let obj = &obj.results[0];
    obj.extensions
        .iter()
        .filter(|item| {
            item.publisher.publisher_name == "ms-python" && item.extension_name == "python"
        })
        .for_each(|item| {
            println!("{item:?}");
        });
    println!("Hello, world!");
}
