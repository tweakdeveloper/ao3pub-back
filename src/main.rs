use std::error::Error;

use axum::{routing::get, Router};

use ao3pub_back::routing;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new().route("/", get(routing::root::get_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:20732").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
