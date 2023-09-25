use std::sync::Arc;

use crate::{
    controller::controller_repo::ControllerRepo, filesystem::FileSystem,
    process_manager::ProcessManager,
};

pub(crate) struct State {
    controller_repo: Box<dyn ControllerRepo>,
    filesystem: FileSystem,
    process_manager: Arc<ProcessManager>,
}

impl State {
    pub fn new(
        controller_repo: Box<dyn ControllerRepo>,
        filesystem: FileSystem,
        process_manager: Arc<ProcessManager>,
    ) -> Self {
        Self {
            controller_repo,
            filesystem,
            process_manager,
        }
    }
    pub fn get_controller_repo(&self) -> &Box<dyn ControllerRepo> {
        &self.controller_repo
    }

    pub fn get_filesystem(&self) -> &FileSystem {
        &self.filesystem
    }

    pub fn get_process_manager(&self) -> &Arc<ProcessManager> {
        &self.process_manager
    }
}
