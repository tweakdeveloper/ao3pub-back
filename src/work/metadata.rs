use std::sync::Arc;

use axum::{
  extract::{Path, State},
  Json,
};
use scraper::{Html, Selector};
use serde::Serialize;

use crate::{error::AppError, state::AppState};

#[derive(Serialize)]
pub struct GetMetadataResponse {
  pub title: String,
}

pub async fn get_metadata(
  State(state): State<Arc<AppState>>,
  Path(work_id): Path<String>,
) -> Result<Json<GetMetadataResponse>, AppError> {
  let work_res = state.ao3_client.get(&format!("/works/{work_id}")).await?;
  let work_doc = Html::parse_document(&work_res);

  let work_selector = Selector::parse("div#workskin").unwrap();
  let work = work_doc
    .select(&work_selector)
    .next()
    .ok_or(anyhow::anyhow!("work not present in HTML response"))?;

  let title_selector = Selector::parse("h2.title.heading").unwrap();
  let title = work
    .select(&title_selector)
    .next()
    .ok_or(anyhow::anyhow!("no title element"))?;
  let title = title.text().collect::<Vec<&str>>().join("");

  Ok(Json(GetMetadataResponse { title }))
}
