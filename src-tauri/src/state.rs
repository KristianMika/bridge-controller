use std::sync::Arc;

use crate::{
    controller::controller_repo::ControllerRepo, filesystem::FileSystem,
    process::process_manager::ProcessManager,
};

/// Holds the state of the application
pub(crate) struct State {
    /// A reference to the DB repository
    controller_repo: Arc<dyn ControllerRepo>,
    /// Filesystem object creates and manages files and directories
    filesystem: FileSystem,
    /// Manages interface processes based on the requests from the front-end
    process_manager: Arc<ProcessManager>,
}

impl State {
    pub fn new(
        controller_repo: Arc<dyn ControllerRepo>,
        filesystem: FileSystem,
        process_manager: Arc<ProcessManager>,
    ) -> Self {
        Self {
            controller_repo,
            filesystem,
            process_manager,
        }
    }

    pub fn get_controller_repo(&self) -> &Arc<dyn ControllerRepo> {
        &self.controller_repo
    }

    pub fn get_filesystem(&self) -> &FileSystem {
        &self.filesystem
    }

    pub fn get_process_manager(&self) -> &Arc<ProcessManager> {
        &self.process_manager
    }
}
