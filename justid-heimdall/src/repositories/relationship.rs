use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::HeimdallResult,
    models::relationship::{Relationship, RelationshipTuple},
};

pub struct PgRelationshipRepository {
    pool: PgPool,
}

#[async_trait::async_trait]
pub trait RelationshipRepository: Send + Sync {
    async fn create_relationship(&self, tuple: &RelationshipTuple) -> HeimdallResult<Uuid>;
    async fn get_relationship(&self, id: Uuid) -> HeimdallResult<Relationship>;
    async fn delete_relationship(&self, id: Uuid) -> HeimdallResult<()>;
    async fn list_relationships(
        &self,
        namespace: Option<&str>,
        object_id: Option<&str>,
        relation: Option<&str>,
        subject_namespace: Option<&str>,
        subject_id: Option<&str>,
        page: u32,
        page_size: u32,
    ) -> HeimdallResult<(Vec<Relationship>, u64)>;
    async fn check_relationship_exists(
        &self,
        namespace: &str,
        object_id: &str,
        relation: &str,
        subject_namespace: &str,
        subject_id: &str,
        subject_relation: Option<&str>,
    ) -> HeimdallResult<bool>;
    async fn get_relation_members(
        &self,
        namespace: &str,
        object_id: &str,
        relation: &str,
    ) -> HeimdallResult<Vec<(String, String, Option<String>)>>;
}

impl PgRelationshipRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl RelationshipRepository for PgRelationshipRepository {
    async fn create_relationship(&self, tuple: &RelationshipTuple) -> HeimdallResult<Uuid> {
        todo!()
    }

    async fn get_relationship(&self, id: Uuid) -> HeimdallResult<Relationship> {
        todo!()
    }

    async fn delete_relationship(&self, id: Uuid) -> HeimdallResult<()> {
        todo!()
    }

    async fn list_relationships(
        &self,
        namespace: Option<&str>,
        object_id: Option<&str>,
        relation: Option<&str>,
        subject_namespace: Option<&str>,
        subject_id: Option<&str>,
        page: u32,
        page_size: u32,
    ) -> HeimdallResult<(Vec<Relationship>, u64)> {
        todo!()
    }

    async fn check_relationship_exists(
        &self,
        namespace: &str,
        object_id: &str,
        relation: &str,
        subject_namespace: &str,
        subject_id: &str,
        subject_relation: Option<&str>,
    ) -> HeimdallResult<bool> {
        todo!()
    }

    async fn get_relation_members(
        &self,
        namespace: &str,
        object_id: &str,
        relation: &str,
    ) -> HeimdallResult<Vec<(String, String, Option<String>)>> {
        todo!()
    }
}
