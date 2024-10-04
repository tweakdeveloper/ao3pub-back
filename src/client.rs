pub struct Ao3Client {
  ao3_base: url::Url,
  http_client: reqwest::Client,
}

impl Ao3Client {
  pub fn new() -> anyhow::Result<Self> {
    // AO3 base URL
    let ao3_base = url::Url::parse("https://archiveofourown.org")?;

    // reqwest
    let http_client = reqwest::Client::builder()
      .user_agent(format!("ao3pub v{}", env!("CARGO_PKG_VERSION")))
      .build()?;

    Ok(Self {
      ao3_base,
      http_client,
    })
  }
}

impl Ao3Client {
  pub async fn get(&self, relative_url: &str) -> anyhow::Result<String> {
    let url = self.ao3_base.join(relative_url)?;
    let response = self.http_client.get(url).send().await?.text().await?;

    Ok(response)
  }
}
