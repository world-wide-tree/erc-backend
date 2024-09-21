use std::sync::Arc;

use axum::{response::IntoResponse, routing::{delete, get, post, put}, Router};
use domain::services::AppServices;

pub fn ecr_routes(state: Arc<AppServices>) -> Router{
    Router::new()
        .route("/ecr", post(post_ecr_handler))
        .route("/ecr", get(list_ecr_handler))
        .route("/ecr/:id", get(get_ecr_handler))
        .route("/ecr/:id", put(put_ecr_handler))
        .route("/ecr/:id", delete(delete_ecr_handler))
}
#[utoipa::path(
    tag = "",
    post,
    path = "/ecr",
    responses(
        (status = 200, body = ())
    )
)]
pub async fn post_ecr_handler(

) -> impl IntoResponse{
    ""
}
#[utoipa::path(
    tag = "",
    get,
    path = "/ecr",
    responses(
        (status = 200, body = ())
    )
)]
pub async fn list_ecr_handler(

) -> impl IntoResponse{
    ""
}
#[utoipa::path(
    tag = "",
    get,
    path = "/ecr/{id}",
    responses(
        (status = 200, body = ())
    )
)]
pub async fn get_ecr_handler(

) -> impl IntoResponse{
    ""
}
#[utoipa::path(
    tag = "",
    put,
    path = "/ecr/{id}",
    responses(
        (status = 200, body = ())
    )
)]
pub async fn put_ecr_handler(

) -> impl IntoResponse{
    ""
}
#[utoipa::path(
    tag = "",
    delete,
    path = "/ecr/{id}",
    responses(
        (status = 200, body = ())
    )
)]
pub async fn delete_ecr_handler(

) -> impl IntoResponse{
    ""
}