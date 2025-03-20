use std::sync::Arc;

use crate::{
    error::HeimdallResult,
    models::relationship::{CheckRequest, CheckResponse},
    repositories::{PermissionRepository, RelationshipRepository},
};

pub struct AuthorizationService {
    relationship_repo: Arc<dyn RelationshipRepository>,
    permission_repo: Arc<dyn PermissionRepository>,
}

impl AuthorizationService {
    pub fn new(
        relationship_repo: Arc<dyn RelationshipRepository>,
        permission_repo: Arc<dyn PermissionRepository>,
    ) -> Self {
        Self {
            relationship_repo,
            permission_repo,
        }
    }

    pub async fn check_permission(&self, request: &CheckRequest) -> HeimdallResult<CheckResponse> {
        todo!()
    }
}
