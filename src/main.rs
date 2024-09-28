use std::process;

use axum::{routing::get, Router};
use tokio::{net::TcpListener, signal};

#[tokio::main]
async fn main() {
  pretty_env_logger::init_custom_env("BACKEND_LOG_LEVEL");
  log::info!("starting ao3pub-back v{}", env!("CARGO_PKG_VERSION"));

  let app = Router::new()
    .route("/", get(root))
    .route("/work/:work_id/metadata", get(ao3pub::work::get_metadata));

  let listener = match TcpListener::bind("0.0.0.0:8080").await {
    Ok(listener) => listener,
    Err(error) => {
      log::error!("couldn't bind to port: {error}");
      process::exit(1);
    }
  };

  log::info!("ao3pub-back started");

  if let Err(error) = axum::serve(listener, app)
    .with_graceful_shutdown(graceful_shutdown())
    .await
  {
    log::error!("error serving app: {error}");
    process::exit(1);
  }
}

async fn graceful_shutdown() {
  let ctrl_c = async {
    if let Err(error) = signal::ctrl_c().await {
      log::error!("failed to install ctrl+C handler: {error}");
      process::exit(1);
    }
  };

  #[cfg(unix)]
  let sigterm = async {
    match signal::unix::signal(signal::unix::SignalKind::terminate()) {
      Ok(mut sig) => sig.recv().await,
      Err(error) => {
        log::error!("failed to install SIGTERM handler: {error}");
        process::exit(1);
      }
    }
  };

  #[cfg(not(unix))]
  let sigterm = std::future::pending::<()>();

  tokio::select! {
    _ = ctrl_c => {
      log::info!("ao3pub-back received ctrl+C");
    },
    _ = sigterm => {
      log::info!("ao3pub-back received SIGTERM");
    },
  }
}

async fn root() -> &'static str {
  "howdy y'all!!!"
}
