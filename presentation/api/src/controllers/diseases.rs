use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, routing::{delete, get, post, put}, Json, Router};
use domain::services::AppServices;
use crate::models::diseases::*;
pub fn diseases_routes(state: Arc<AppServices>) -> Router{
    Router::new()
        .route("/diseases", post(post_diseases_handler))
        .route("/diseases", get(list_diseases_handler))
        .route("/diseases/:id", get(get_diseases_handler))
        .route("/diseases/:id", put(put_diseases_handler))
        .route("/diseases/:id", delete(delete_diseases_handler))
        .with_state(state)
}
#[utoipa::path(
    tag = "",
    post,
    path = "/diseases",
    responses(
        (status = 200, body = ())
    )
)]
pub async fn post_diseases_handler(
    State(state): State<Arc<AppServices>>,
    Json(dto): Json<PostDieaseasApiDto>
) -> impl IntoResponse{
    match state.diseases_services().create(dto.into()).await{
        Ok(rst) => (StatusCode::OK, Json(DieseasApiDto::from(rst))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
    
}
#[utoipa::path(
    tag = "",
    get,
    path = "/diseases",
    responses(
        (status = 200, body = ())
    )
)]
pub async fn list_diseases_handler(
    State(state): State<Arc<AppServices>>
) -> impl IntoResponse{
    match state.diseases_services().list().await{
        Ok(rst) => (StatusCode::OK, Json(rst.into_iter().map(DieseasApiDto::from).collect::<Vec<_>>())).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}
#[utoipa::path(
    tag = "",
    get,
    path = "/diseases/{id}",
    responses(
        (status = 200, body = ())
    )
)]
pub async fn get_diseases_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppServices>>,
) -> impl IntoResponse{
    match state.diseases_services().get_by_id(id).await{
        Ok(rst) => (StatusCode::OK, Json(DieseasApiDto::from(rst))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}
#[utoipa::path(
    tag = "",
    put,
    path = "/diseases/{id}",
    responses(
        (status = 200, body = ())
    )
)]
pub async fn put_diseases_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppServices>>,
    Json(dto): Json<PutDieaseasApiDto>
) -> impl IntoResponse{
    match state.diseases_services().update(id, dto.into()).await{
        Ok(rst) => (StatusCode::OK, Json(DieseasApiDto::from(rst))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}
#[utoipa::path(
    tag = "",
    delete,
    path = "/diseases/{id}",
    responses(
        (status = 200, body = ())
    )
)]
pub async fn delete_diseases_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppServices>>,
) -> impl IntoResponse{
    match state.diseases_services().delete(id).await{
        Ok(rst) => (StatusCode::OK, Json(rst)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}