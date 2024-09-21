use axum::{response::IntoResponse, routing::{delete, get, post, put}, Router};

pub fn diseases_routes() -> Router{
    Router::new()
        .route("/diseases", post(post_diseases_handler))
        .route("/diseases", get(list_diseases_handler))
        .route("/diseases/:id", get(get_diseases_handler))
        .route("/diseases/:id", put(put_diseases_handler))
        .route("/diseases/:id", delete(delete_diseases_handler))
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

) -> impl IntoResponse{
    ""
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

) -> impl IntoResponse{
    ""
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

) -> impl IntoResponse{
    ""
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

) -> impl IntoResponse{
    ""
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

) -> impl IntoResponse{
    ""
}