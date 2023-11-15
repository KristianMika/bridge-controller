use std::process::Child;

use crate::process::process_manager::process_manager_error::ProcessManagerError;

use super::ProcessExecutor;

pub(crate) struct WindowsProcessExecutor {}

impl ProcessExecutor for WindowsProcessExecutor {
    fn new() -> Self {
        Self {}
    }

    fn create_webauthn_process(&self) -> Result<Child, ProcessManagerError> {
        todo!()
    }

    fn create_pcsc_process(&self) -> Result<Child, ProcessManagerError> {
        todo!()
    }

    fn kill_webauthn_process(&self, process: Child) -> Result<(), ProcessManagerError> {
        todo!()
    }

    fn kill_pcsc_process(&self, process: Child) -> Result<(), ProcessManagerError> {
        todo!()
    }
}
