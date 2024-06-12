use std::error::Error;

use axum::{
    routing::{get, post},
    Router,
};

use ao3pub_back::routing;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .route("/", get(routing::root::get_handler))
        .route(
            "/publication_req",
            post(routing::publication_req::post_handler),
        );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:20732").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
