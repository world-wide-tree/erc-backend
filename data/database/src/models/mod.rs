pub mod users;
pub mod clinics;
pub mod diseases;
pub mod relations;
pub mod erc;


pub const USER_TABLE: &'static str = "Users";
pub const CLINIC_TABLE: &'static str = "Clinics";
pub const DISEASES_TABLE: &'static str = "Diseases";
pub const ERC_TABLE: &'static str = "Ercs";
pub const DOCTOR_TABLE: &'static str = "Doctors";
pub const PATIENT_TABLE: &'static str = "Patients";
pub const SPECIALIZATION_TABLE: &'static str = "Specializations";

pub const HAS_ERC_RELETION: &'static str = "HasErcRelation";
pub const VISITS_RELETION: &'static str = "VisitsRelation";
pub const HAS_DOCTORS_RELETION: &'static str = "HasDoctorsRelation";
pub const TREATS_RELETION: &'static str = "TreatsRelation";
pub const SPECIALIZATION_IN_RELETION: &'static str = "SpecializationInRelation";
pub const HAS_HSITORY_RELETION: &'static str = "HasHistoryRelation";
pub const DIAGNOSED_BY_RELETION: &'static str = "DiagnosedByRelation";
pub const TREATED_IN_RELETION: &'static str = "TreatedInRelation";
pub const CONTAINS_HISTORY_RELETION: &'static str = "ContainsHistoryRelation";










