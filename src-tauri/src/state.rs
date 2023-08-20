use crate::controller::controller_repo::ControllerRepo;

pub(crate) struct State {
    controller_repo: Box<dyn ControllerRepo>,
}

impl State {
    pub fn new(controller_repo: Box<dyn ControllerRepo>) -> Self {
        Self { controller_repo }
    }
}

impl State {
    pub fn get_controller_repo(&self) -> &Box<dyn ControllerRepo> {
        &self.controller_repo
    }
}
