use axum::response::IntoResponse;
use reqwest::StatusCode;

#[allow(dead_code)]
pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
  fn into_response(self) -> axum::response::Response {
    (StatusCode::INTERNAL_SERVER_ERROR, "something went wrong!").into_response()
  }
}

impl<E> From<E> for AppError
where
  E: Into<anyhow::Error>,
{
  fn from(err: E) -> Self {
    Self(err.into())
  }
}
