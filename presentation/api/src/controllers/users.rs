use axum::{response::IntoResponse, routing::{delete, get, post, put}, Router};

pub fn user_routes() -> Router{
    Router::new()
        .route("/users", post(post_users_handler))
        .route("/users", get(list_users_handler))
        .route("/users/:id", get(get_users_handler))
        .route("/users/:id", put(put_users_handler))
        .route("/users/:id", delete(delete_users_handler))
}
#[utoipa::path(
    tag = "",
    post,
    path = "/users",
    responses(
        (status = 200, body = ())
    )
)]
pub async fn post_users_handler(

) -> impl IntoResponse{
    ""
}
#[utoipa::path(
    tag = "",
    get,
    path = "/users",
    responses(
        (status = 200, body = ())
    )
)]
pub async fn list_users_handler(

) -> impl IntoResponse{
    ""
}
#[utoipa::path(
    tag = "",
    get,
    path = "/users/{id}",
    responses(
        (status = 200, body = ())
    )
)]
pub async fn get_users_handler(

) -> impl IntoResponse{
    ""
}
#[utoipa::path(
    tag = "",
    put,
    path = "/users/{id}",
    responses(
        (status = 200, body = ())
    )
)]
pub async fn put_users_handler(

) -> impl IntoResponse{
    ""
}
#[utoipa::path(
    tag = "",
    delete,
    path = "/users/{id}",
    responses(
        (status = 200, body = ())
    )
)]
pub async fn delete_users_handler(

) -> impl IntoResponse{
    ""
}