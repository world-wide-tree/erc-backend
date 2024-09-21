use std::sync::Arc;

use axum::{response::IntoResponse, routing::{delete, get, post, put}, Router};
use domain::services::AppServices;

pub fn clinics_routes(state: Arc<AppServices>) -> Router{
    Router::new()
        .route("/clinics", post(post_clinics_handler))
        .route("/clinics", get(list_clinics_handler))
        .route("/clinics/:id", get(get_clinics_handler))
        .route("/clinics/:id", put(put_clinics_handler))
        .route("/clinics/:id", delete(delete_clinics_handler))
}
#[utoipa::path(
    tag = "",
    post,
    path = "/clinics",
    responses(
        (status = 200, body = ())
    )
)]
pub async fn post_clinics_handler(

) -> impl IntoResponse{
    ""
}
#[utoipa::path(
    tag = "",
    get,
    path = "/clinics",
    responses(
        (status = 200, body = ())
    )
)]
pub async fn list_clinics_handler(

) -> impl IntoResponse{
    ""
}
#[utoipa::path(
    tag = "",
    get,
    path = "/clinics/{id}",
    responses(
        (status = 200, body = ())
    )
)]
pub async fn get_clinics_handler(

) -> impl IntoResponse{
    ""
}
#[utoipa::path(
    tag = "",
    put,
    path = "/clinics/{id}",
    responses(
        (status = 200, body = ())
    )
)]
pub async fn put_clinics_handler(

) -> impl IntoResponse{
    ""
}
#[utoipa::path(
    tag = "",
    delete,
    path = "/clinics/{id}",
    responses(
        (status = 200, body = ())
    )
)]
pub async fn delete_clinics_handler(

) -> impl IntoResponse{
    ""
}


pub fn doctors_routes(state: Arc<AppServices>) -> Router{
    Router::new()
        .route("/doctors", post(post_doctors_handler))
        .route("/doctors", get(list_doctors_handler))
        .route("/doctors/:id", get(get_doctors_handler))
        .route("/doctors/:id", put(put_doctors_handler))
        .route("/doctors/:id", delete(delete_doctors_handler))
}
#[utoipa::path(
    tag = "",
    post,
    path = "/doctors",
    responses(
        (status = 200, body = ())
    )
)]
pub async fn post_doctors_handler(

) -> impl IntoResponse{
    ""
}
#[utoipa::path(
    tag = "",
    get,
    path = "/doctors",
    responses(
        (status = 200, body = ())
    )
)]
pub async fn list_doctors_handler(

) -> impl IntoResponse{
    ""
}
#[utoipa::path(
    tag = "",
    get,
    path = "/doctors/{id}",
    responses(
        (status = 200, body = ())
    )
)]
pub async fn get_doctors_handler(

) -> impl IntoResponse{
    ""
}
#[utoipa::path(
    tag = "",
    put,
    path = "/doctors/{id}",
    responses(
        (status = 200, body = ())
    )
)]
pub async fn put_doctors_handler(

) -> impl IntoResponse{
    ""
}
#[utoipa::path(
    tag = "",
    delete,
    path = "/doctors/{id}",
    responses(
        (status = 200, body = ())
    )
)]
pub async fn delete_doctors_handler(

) -> impl IntoResponse{
    ""
}



pub fn patients_routes(state: Arc<AppServices>) -> Router{
    Router::new()
        .route("/patients", post(post_patients_handler))
        .route("/patients", get(list_patients_handler))
        .route("/patients/:id", get(get_patients_handler))
        .route("/patients/:id", put(put_patients_handler))
        .route("/patients/:id", delete(delete_patients_handler))
}
#[utoipa::path(
    tag = "",
    post,
    path = "/patients",
    responses(
        (status = 200, body = ())
    )
)]
pub async fn post_patients_handler(

) -> impl IntoResponse{
    ""
}
#[utoipa::path(
    tag = "",
    get,
    path = "/patients",
    responses(
        (status = 200, body = ())
    )
)]
pub async fn list_patients_handler(

) -> impl IntoResponse{
    ""
}
#[utoipa::path(
    tag = "",
    get,
    path = "/patients/{id}",
    responses(
        (status = 200, body = ())
    )
)]
pub async fn get_patients_handler(

) -> impl IntoResponse{
    ""
}
#[utoipa::path(
    tag = "",
    put,
    path = "/patients/{id}",
    responses(
        (status = 200, body = ())
    )
)]
pub async fn put_patients_handler(

) -> impl IntoResponse{
    ""
}
#[utoipa::path(
    tag = "",
    delete,
    path = "/patients/{id}",
    responses(
        (status = 200, body = ())
    )
)]
pub async fn delete_patients_handler(

) -> impl IntoResponse{
    ""
}