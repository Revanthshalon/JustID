use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Namespace {
    pub id: Uuid,
    pub name: String,
    pub relation_definitions: RelationDefinitions,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelationDefinitions {
    pub relations: Vec<RelationDefinition>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelationDefinition {
    pub name: String,
    pub rewrites: Vec<RewriteRule>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RewriteRule {
    Union { child_relations: Vec<String> },
    Intersection { child_relations: Vec<String> },
    ComputedUserset { relation: String },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpsertNamespaceRequest {
    pub name: String,
    pub relation_definitions: RelationDefinitions,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NamespaceResponse {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionDefinition {
    pub id: Uuid,
    pub namespace: String,
    pub permission: String,
    pub relation_expressions: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionResponse {
    pub id: Uuid,
    pub namespace: String,
    pub permission: String,
}
