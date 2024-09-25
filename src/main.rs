use std::process;

use axum::{routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
  pretty_env_logger::init_custom_env("BACKEND_LOG_LEVEL");
  log::info!("starting ao3pub-back v{}", env!("CARGO_PKG_VERSION"));

  let app = Router::new().route("/", get(root));

  let listener = match TcpListener::bind("0.0.0.0:8080").await {
    Ok(listener) => listener,
    Err(error) => {
      log::error!("couldn't bind to port: {error}");
      process::exit(1);
    }
  };

  log::info!("ao3pub-back started");

  if let Err(error) = axum::serve(listener, app).await {
    log::error!("error serving app: {error}");
    process::exit(1);
  }
}

async fn root() -> &'static str {
  "howdy y'all!!!"
}
