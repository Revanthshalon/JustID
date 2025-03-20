use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Relationship {
    pub id: Uuid,
    pub namespace: String,
    pub object_id: String,
    pub relation: String,
    pub subject_namespace: String,
    pub subject_id: String,
    pub subject_relation: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipTuple {
    pub namespace: String,
    pub object_id: String,
    pub relation: String,
    pub subject: Subject,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Subject {
    Direct {
        namespace: String,
        id: String,
    },
    Nested {
        namespace: String,
        id: String,
        relation: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckRequest {
    pub subject: String,
    pub permission: String,
    pub resource: String,
    pub context: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckResponse {
    pub allowed: bool,
    pub debug_info: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRelationshipRequest {
    pub tuple: RelationshipTuple,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkCheckRequest {
    pub checks: Vec<CheckRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkCheckResponses {
    pub results: Vec<CheckResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListRelationshipsParams {
    pub namespace: Option<String>,
    pub object_id: Option<String>,
    pub relation: Option<String>,
    pub subject_namespace: Option<String>,
    pub subject_id: Option<String>,
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_page_size")]
    pub page_size: u32,
}

fn default_page() -> u32 {
    1
}

fn default_page_size() -> u32 {
    50
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListRelationshipsResponse {
    pub results: Vec<RelationshipTuple>,
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LookupObjectsRequest {
    pub subject: String,
    pub relation: String,
    pub namespace: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LookupObjectsResponse {
    pub objects: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LookupSubjectsRequest {
    pub resource: String,
    pub relation: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LookupSubjectsResponse {
    pub subjects: Vec<String>,
}
