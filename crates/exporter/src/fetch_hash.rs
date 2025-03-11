use crate::models::*;
use crate::schema::marketplace::dsl::*;
use diesel::prelude::*;
use diesel::PgConnection;

pub async fn fetch_hash(conn: &mut PgConnection) -> anyhow::Result<()> {
    let record = marketplace
        .filter(hash.is_null())
        .select(Marketplace::as_select())
        .load(conn)?;

    for record in record {
        if let Ok(file_hash) = compute_hash(&record.assert_url).await {
            let _ = diesel::update(marketplace)
                .filter(name.eq(&record.name))
                .filter(publisher.eq(&record.publisher))
                .filter(version.eq(&record.version))
                .filter(engine.eq(&record.engine))
                .filter(platform.eq(&record.platform))
                .filter(assert_url.eq(&record.assert_url))
                .set(hash.eq(file_hash))
                .execute(conn);
        }
    }
    Ok(())
}

pub async fn compute_hash(url: &str) -> anyhow::Result<String> {
    let sha256 = tokio::process::Command::new("nix-prefetch-url")
        .arg(url)
        .output()
        .await?
        .stdout;

    Ok(String::from_utf8(sha256)?.trim().to_owned())
}
