use std::process::Child;

#[cfg(target_os = "linux")]
mod linux_process_executor;
#[cfg(target_os = "windows")]
mod windows_process_executor;

#[cfg(target_os = "linux")]
pub(crate) use linux_process_executor::LinuxProcessExecutor as PlatformSpecificProcessExecutor;
#[cfg(target_os = "windows")]
pub(crate) use windows_process_executor::WindowsProcessExecutor as PlatformSpecificProcessExecutor;

use super::process_manager::process_manager_error::ProcessManagerError;

/// ProcessExecutor is responsible for spawning
/// new processes of emulated interfaces
pub(crate) trait ProcessExecutor {
    /// Instantiates a new ProcessExecutor
    fn new() -> Self;

    /// Launches a new WebAuthn process
    fn create_webauthn_process(&self) -> Result<Child, ProcessManagerError>;

    /// Launches a new PCSC process
    fn create_pcsc_process(&self) -> Result<Child, ProcessManagerError>;
}
