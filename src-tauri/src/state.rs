use crate::{controller::controller_repo::ControllerRepo, filesystem::FileSystem};

pub(crate) struct State {
    controller_repo: Box<dyn ControllerRepo>,
    filesystem: FileSystem,
}

impl State {
    pub fn new(controller_repo: Box<dyn ControllerRepo>) -> Self {
        Self {
            controller_repo,
            filesystem: FileSystem {},
        }
    }
    pub fn get_controller_repo(&self) -> &Box<dyn ControllerRepo> {
        &self.controller_repo
    }

    pub fn get_filesystem(&self) -> &FileSystem {
        &self.filesystem
    }
}
