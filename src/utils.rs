pub async fn get_sha256(url: &str) -> anyhow::Result<String> {
    let sha256 = tokio::process::Command::new("nix-prefetch-url")
        .arg(url.clone())
        .output()
        .await?
        .stdout;
    Ok(String::from_utf8(sha256).unwrap().trim().to_owned())
}

