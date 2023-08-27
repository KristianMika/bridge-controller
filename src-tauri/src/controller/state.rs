use std::sync::Arc;

use crate::filesystem::FileSystem;

use super::controller_repo::ControllerRepo;

#[derive(Clone)]
pub(crate) struct State {
    controller_repo: Arc<dyn ControllerRepo>,
    filesystem: FileSystem,
}

impl State {
    pub fn new(controller_repo: Arc<dyn ControllerRepo>, filesystem: FileSystem) -> Self {
        Self {
            controller_repo,
            filesystem,
        }
    }

    pub fn get_controller_repo(&self) -> Arc<dyn ControllerRepo> {
        self.controller_repo.clone()
    }

    pub fn get_filesystem(&self) -> &FileSystem {
        &self.filesystem
    }
}
