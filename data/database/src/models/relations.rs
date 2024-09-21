use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HasEcrDbRelation{
    id: Thing,
    patient: Thing,
    erc: Thing,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisitsDbRelation{
    id: Thing,
    patient: Thing,
    clinic: Thing
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HasDoctorsDbRelation{
    id: Thing,
    clinic: Thing,
    doctor: Thing,
}
// TODO: Расмотреть надобность
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreatsDbRelation{
    id: Thing,
    doctor: Thing,
    patient: Thing,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecializationInDbRelation{
    id: Thing,
    doctor: Thing,
    specialization: Thing,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HasHistoryDbRelation{
    id: Thing,
    patient: Thing,
    medical_history: Thing,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosedByDbRelation{
    id: Thing,
    medical_history: Thing,
    doctor: Thing,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreatedInDbRelation{
    id: Thing,
    medical_history: Thing,
    clinic: Thing,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainsHistoryDbRelation{
    id: Thing,
    erc: Thing,
    medical_history: Thing,
}