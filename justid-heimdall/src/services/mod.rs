use authorization::AuthorizationService;

use crate::repositories::RepositoryContainer;

mod authorization;

pub struct ServiceContainer {
    pub authorization_service: AuthorizationService,
}

impl ServiceContainer {
    pub fn new(repository_container: RepositoryContainer) -> Self {
        Self {
            authorization_service: AuthorizationService::new(
                repository_container.relationship_repo.clone(),
                repository_container.permission_repo.clone(),
            ),
        }
    }
}
