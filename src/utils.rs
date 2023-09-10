pub async fn get_sha256(url: &str) -> anyhow::Result<String> {
    let sha256 = tokio::process::Command::new("nix-prefetch-url")
        .arg(url.clone())
        .output()
        .await?
        .stdout;
    Ok(String::from_utf8(sha256).unwrap().trim().to_owned())
}

pub async fn request_get_remote_object<T: for<'de> serde::Deserialize<'de>>(
    client: &reqwest::Client,
    url: &str,
) -> anyhow::Result<T> {
    let req = client.get(url).build().unwrap();
    Ok(client
        .execute(req)
        .await
        .unwrap()
        .json::<T>()
        .await
        .unwrap())
}
