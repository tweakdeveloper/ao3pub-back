use crate::client::Ao3Client;

pub struct AppState {
  pub ao3_client: Ao3Client,
}

impl AppState {
  pub fn new() -> anyhow::Result<Self> {
    let ao3_client = Ao3Client::new()?;

    Ok(Self { ao3_client })
  }
}
