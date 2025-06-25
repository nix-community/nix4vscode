use std::collections::HashMap;

use octocrab::models::repos::Release;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct TagName(pub String);

pub async fn fetch_codelldb() -> anyhow::Result<HashMap<TagName, Release>> {
    let crab = octocrab::Octocrab::builder().build()?;
    let page = crab
        .repos("vadimcn", "codelldb")
        .releases()
        .list()
        .send()
        .await?;

    let mut ret = HashMap::new();
    for i in page {
        ret.insert(TagName(i.tag_name.clone()), i);
    }
    Ok(ret)
}
