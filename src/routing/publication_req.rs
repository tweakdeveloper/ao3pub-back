use axum::{http::StatusCode, Json};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct PublicationRequestResponse {
    req_id: String,
}

pub async fn post_handler() -> (StatusCode, Json<PublicationRequestResponse>) {
    let req_id = Uuid::new_v4().to_string();
    (
        StatusCode::ACCEPTED,
        Json(PublicationRequestResponse { req_id }),
    )
}
