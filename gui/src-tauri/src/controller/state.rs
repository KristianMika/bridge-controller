use std::sync::Arc;

use super::controller_repo::ControllerRepo;

#[derive(Clone)]
pub(crate) struct State {
    controller_repo: Arc<dyn ControllerRepo>,
}

impl State {
    pub fn new(controller_repo: Arc<dyn ControllerRepo>) -> Self {
        Self { controller_repo }
    }

    pub fn get_controller_repo(&self) -> Arc<dyn ControllerRepo> {
        self.controller_repo.clone()
    }
}
