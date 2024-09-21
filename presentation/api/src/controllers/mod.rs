use std::sync::Arc;

use axum::{routing::get, Router};
use domain::services::AppServices;
use utoipa::{openapi, OpenApi};



pub mod users;
pub mod clinics;
pub mod erc;
pub mod diseases;

use clinics::*;
use diseases::*;
use erc::*;
use users::*;
use utoipa_rapidoc::RapiDoc;
use utoipa_swagger_ui::SwaggerUi;

use super::models::{
    clinics::*,
    diseases::*,
    erc::*,
    users::*,
};

pub fn app_routes(state: Arc<AppServices>) -> Router{
    Router::new()
        .route("/", get("handler"))
        .merge(user_routes(state.clone()))
        .merge(clinics_routes(state.clone()))
        .merge(doctors_routes(state.clone()))
        .merge(patients_routes(state.clone()))
        .merge(ecr_routes(state.clone()))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ErcOpenApi::openapi()))
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
}


#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "Erc", description = "Erc")
    ), 
    paths(
        // Users
        post_users_handler, list_users_handler, get_users_handler, put_users_handler, delete_users_handler,
        // Clinics
        post_clinics_handler,list_clinics_handler,get_clinics_handler,put_clinics_handler,delete_clinics_handler,post_doctors_handler,list_doctors_handler,get_doctors_handler,put_doctors_handler,delete_doctors_handler,post_patients_handler,list_patients_handler,get_patients_handler,put_patients_handler,delete_patients_handler,
        // Erc
        post_ecr_handler, list_ecr_handler, get_ecr_handler, put_ecr_handler, delete_ecr_handler, 
        // Diseases
        post_diseases_handler, list_diseases_handler, get_diseases_handler, put_diseases_handler, delete_diseases_handler, 
    ),
    components(
        schemas(
            DoctorApiDto, PatientApiDto, ClinicApiDto, MedicalHistoriesApiDto, DieseasApiDto, ErcApiDto, UserApiDto, CreateUserApiDto, UpdateUserApiDto, 
        )
    )
)]
pub struct ErcOpenApi;