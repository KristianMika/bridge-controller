use super::ProcessExecutor;

struct WindowsProcessExecutor {}

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
}
