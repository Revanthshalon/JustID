use crate::services::ServiceContainer;

pub struct AppState {
    pub service_container: ServiceContainer,
}

impl AppState {
    pub fn new() -> Self {
        // NOTE: Pass the configuration file to appstate, inside which the app state initializes
        // pool and shared it with repository
        todo!()
    }
}
