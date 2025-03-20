use std::sync::Arc;

use permission::PgPermissionRepository;
use relationship::PgRelationshipRepository;
use sqlx::PgPool;

mod permission;
mod relationship;

pub use self::permission::PermissionRepository;
pub use self::relationship::RelationshipRepository;

pub struct RepositoryContainer {
    pub relationship_repo: Arc<dyn RelationshipRepository>,
    pub permission_repo: Arc<dyn PermissionRepository>,
}

impl RepositoryContainer {
    pub fn new(pool: PgPool) -> Self {
        Self {
            relationship_repo: Arc::new(PgRelationshipRepository::new(pool.clone())),
            permission_repo: Arc::new(PgPermissionRepository::new(pool.clone())),
        }
    }
}
