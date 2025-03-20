use sqlx::PgPool;
use uuid::Uuid;

use crate::{error::HeimdallResult, models::namespace::PermissionDefinition};

pub struct PgPermissionRepository {
    pool: PgPool,
}

#[async_trait::async_trait]
pub trait PermissionRepository: Send + Sync {
    async fn get_permission_relations(
        &self,
        namespace: &str,
        permission: &str,
    ) -> HeimdallResult<Vec<String>>;
    async fn create_or_update_permission(
        &self,
        namespace: &str,
        permission: &str,
        relation_expressions: &[String],
    ) -> HeimdallResult<Uuid>;
    async fn delete_permission(&self, namespace: &str, permission: &str) -> HeimdallResult<()>;
    async fn get_permission_for_namespace(
        &self,
        namespace: &str,
    ) -> HeimdallResult<Vec<PermissionDefinition>>;
}

impl PgPermissionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl PermissionRepository for PgPermissionRepository {
    async fn get_permission_relations(
        &self,
        namespace: &str,
        permission: &str,
    ) -> HeimdallResult<Vec<String>> {
        todo!()
    }

    async fn create_or_update_permission(
        &self,
        namespace: &str,
        permission: &str,
        relation_expressions: &[String],
    ) -> HeimdallResult<Uuid> {
        todo!()
    }

    async fn delete_permission(&self, namespace: &str, permission: &str) -> HeimdallResult<()> {
        todo!()
    }

    async fn get_permission_for_namespace(
        &self,
        namespace: &str,
    ) -> HeimdallResult<Vec<PermissionDefinition>> {
        todo!()
    }
}
