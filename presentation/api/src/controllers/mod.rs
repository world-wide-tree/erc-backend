use axum::{routing::get, Router};

use self::{clinics::{clinics_routes, doctors_routes, patients_routes}, ecr::ecr_routes, users::user_routes};

pub mod users;
pub mod clinics;
pub mod ecr;
pub mod diseases;


pub fn app_routes() -> Router{
    Router::new()
        .route("/", get("handler"))
        .merge(user_routes())
        .merge(clinics_routes())
        .merge(doctors_routes())
        .merge(patients_routes())
        .merge(ecr_routes())
}