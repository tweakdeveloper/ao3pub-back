use axum::extract::Path;

pub async fn get_metadata(Path(work_id): Path<String>) -> String {
  format!("fetching work {work_id}'s metadata")
}
