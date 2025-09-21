use crate::trick_models::{Trick, TrickCreateInput, TrickReplaceInput};
use crate::trick_repository::TrickRepository;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::sync::Arc;
use uuid::Uuid;

pub async fn create_trick(
    State(repo): State<Arc<TrickRepository>>,
    Json(input): Json<TrickCreateInput>,
) -> Json<Trick> {
    let new_trick = repo.create(input).await;
    Json(new_trick)
}

pub async fn find_tricks(State(repo): State<Arc<TrickRepository>>) -> Json<Vec<Trick>> {
    let all_tricks = repo.find_all().await;
    Json(all_tricks)
}

pub async fn find_trick_by_id(
    State(repo): State<Arc<TrickRepository>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let trick = repo.find_by_id(id).await;
    match trick {
        None => StatusCode::NOT_FOUND.into_response(),
        Some(trick) => Json(trick).into_response(),
    }
}

pub async fn replace_trick(
    State(repo): State<Arc<TrickRepository>>,
    Path(id): Path<Uuid>,
    Json(input): Json<TrickReplaceInput>,
) -> Json<Trick> {
    let replaced_trick = repo.replace(id, input).await;
    Json(replaced_trick)
}

pub async fn delete_trick(State(repo): State<Arc<TrickRepository>>, Path(id): Path<Uuid>) {
    repo.delete_by_id(id).await
}
