use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, routing::{delete, get, post, put}, Json, Router};
use domain::services::AppServices;
use crate::models::clinics::*;

pub fn clinics_routes(state: Arc<AppServices>) -> Router{
    Router::new()
        .route("/clinics", post(post_clinics_handler))
        .route("/clinics", get(list_clinics_handler))
        .route("/clinics/:id", get(get_clinics_handler))
        .route("/clinics/:id", put(put_clinics_handler))
        .route("/clinics/:id", delete(delete_clinics_handler))
        .with_state(state)
}
#[utoipa::path(
    tag = "",
    post,
    path = "/clinics",
    responses(
        (status = 200, body = ())
    )
)]
#[axum::debug_handler]
pub async fn post_clinics_handler(
    State(state): State<Arc<AppServices>>,
    Json(dto): Json<PostClinicApiDto>,
) -> impl IntoResponse{
    match state.clinic_services().create(dto.into()).await{
        Ok(rst) => (StatusCode::OK, Json(ClinicApiDto::from(rst))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}
#[utoipa::path(
    tag = "",
    get,
    path = "/clinics",
    responses(
        (status = 200, body = ())
    )
)]
#[axum::debug_handler]
pub async fn list_clinics_handler(
    State(state): State<Arc<AppServices>>,
) -> impl IntoResponse{
    match state.clinic_services().list().await{
        Ok(rst) => (StatusCode::OK, Json(rst.into_iter().map(ClinicApiDto::from).collect::<Vec<_>>())).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}
#[utoipa::path(
    tag = "",
    get,
    path = "/clinics/{id}",
    responses(
        (status = 200, body = ())
    )
)]
#[axum::debug_handler]
pub async fn get_clinics_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppServices>>,
) -> impl IntoResponse{
    match state.clinic_services().get_by_id(id).await{
        Ok(rst) => (StatusCode::OK, Json(ClinicApiDto::from(rst))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}
#[utoipa::path(
    tag = "",
    put,
    path = "/clinics/{id}",
    responses(
        (status = 200, body = ())
    )
)]
#[axum::debug_handler]
pub async fn put_clinics_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppServices>>,
    Json(dto): Json<PutClinicApiDto>
) -> impl IntoResponse{
    match state.clinic_services().update(id, dto.into()).await{
        Ok(rst) => (StatusCode::OK, Json(ClinicApiDto::from(rst))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}
#[utoipa::path(
    tag = "",
    delete,
    path = "/clinics/{id}",
    responses(
        (status = 200, body = ())
    )
)]
#[axum::debug_handler]
pub async fn delete_clinics_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppServices>>,
) -> impl IntoResponse{
    match state.clinic_services().delete(id).await{
        Ok(rst) => (StatusCode::OK, Json(rst)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}


pub fn doctors_routes(state: Arc<AppServices>) -> Router{
    Router::new()
        .route("/doctors", post(post_doctors_handler))
        .route("/doctors", get(list_doctors_handler))
        .route("/doctors/:id", get(get_doctors_handler))
        .route("/doctors/:id", put(put_doctors_handler))
        .route("/doctors/:id", delete(delete_doctors_handler))
        .with_state(state)
}
#[utoipa::path(
    tag = "",
    post,
    path = "/doctors",
    responses(
        (status = 200, body = ())
    )
)]
#[axum::debug_handler]
pub async fn post_doctors_handler(
    State(state): State<Arc<AppServices>>,
    Json(dto): Json<PostDoctorApiDto>,
) -> impl IntoResponse{
    match state.doctor_services().create(dto.into()).await{
        Ok(rst) => (StatusCode::OK, Json(DoctorApiDto::from(rst))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}
#[utoipa::path(
    tag = "",
    get,
    path = "/doctors",
    responses(
        (status = 200, body = ())
    )
)]
#[axum::debug_handler]
pub async fn list_doctors_handler(
    State(state): State<Arc<AppServices>>,
) -> impl IntoResponse{
    match state.doctor_services().list().await{
        Ok(rst) => (StatusCode::OK, Json(rst.into_iter().map(DoctorApiDto::from).collect::<Vec<_>>())).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}
#[utoipa::path(
    tag = "",
    get,
    path = "/doctors/{id}",
    responses(
        (status = 200, body = ())
    )
)]
#[axum::debug_handler]
pub async fn get_doctors_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppServices>>,
) -> impl IntoResponse{
    match state.doctor_services().get_by_id(id).await{
        Ok(rst) => (StatusCode::OK, Json(DoctorApiDto::from(rst))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}
#[utoipa::path(
    tag = "",
    put,
    path = "/doctors/{id}",
    responses(
        (status = 200, body = ())
    )
)]
#[axum::debug_handler]
pub async fn put_doctors_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppServices>>,
    Json(dto): Json<PutDoctorApiDto>
) -> impl IntoResponse{
    match state.doctor_services().update(id, dto.into()).await{
        Ok(rst) => (StatusCode::OK, Json(DoctorApiDto::from(rst))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}
#[utoipa::path(
    tag = "",
    delete,
    path = "/doctors/{id}",
    responses(
        (status = 200, body = ())
    )
)]
#[axum::debug_handler]
pub async fn delete_doctors_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppServices>>,
) -> impl IntoResponse{
    match state.doctor_services().delete(id).await{
        Ok(rst) => (StatusCode::OK, Json(rst)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}



pub fn patients_routes(state: Arc<AppServices>) -> Router{
    Router::new()
        .route("/patients", post(post_patients_handler))
        .route("/patients", get(list_patients_handler))
        .route("/patients/:id", get(get_patients_handler))
        .route("/patients/:id", put(put_patients_handler))
        .route("/patients/:id", delete(delete_patients_handler))
        .with_state(state)
}
#[utoipa::path(
    tag = "",
    post,
    path = "/patients",
    responses(
        (status = 200, body = ())
    )
)]
#[axum::debug_handler]
pub async fn post_patients_handler(
    State(state): State<Arc<AppServices>>,
    Json(dto): Json<PostPatientApiDto>
) -> impl IntoResponse{
    match state.patient_services().create(dto.into()).await{
        Ok(rst) => (StatusCode::OK, Json(PatientApiDto::from(rst))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}
#[utoipa::path(
    tag = "",
    get,
    path = "/patients",
    responses(
        (status = 200, body = ())
    )
)]
#[axum::debug_handler]
pub async fn list_patients_handler(
    State(state): State<Arc<AppServices>>,
) -> impl IntoResponse{
    match state.patient_services().list().await{
        Ok(rst) => (StatusCode::OK, Json(rst.into_iter().map(PatientApiDto::from).collect::<Vec<_>>())).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}
#[utoipa::path(
    tag = "",
    get,
    path = "/patients/{id}",
    responses(
        (status = 200, body = ())
    )
)]
#[axum::debug_handler]
pub async fn get_patients_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppServices>>,
) -> impl IntoResponse{
    match state.patient_services().get_by_id(id).await{
        Ok(rst) => (StatusCode::OK, Json(PatientApiDto::from(rst))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}
#[utoipa::path(
    tag = "",
    put,
    path = "/patients/{id}",
    responses(
        (status = 200, body = ())
    )
)]
#[axum::debug_handler]
pub async fn put_patients_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppServices>>,
    Json(dto): Json<PutPatientApiDto>
) -> impl IntoResponse{
    match state.patient_services().update(id, dto.into()).await{
        Ok(rst) => (StatusCode::OK, Json(PatientApiDto::from(rst))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}
#[utoipa::path(
    tag = "",
    delete,
    path = "/patients/{id}",
    responses(
        (status = 200, body = ())
    )
)]
#[axum::debug_handler]
pub async fn delete_patients_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppServices>>,
) -> impl IntoResponse{
    match state.patient_services().delete(id).await{
        Ok(rst) => (StatusCode::OK, Json(rst)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}