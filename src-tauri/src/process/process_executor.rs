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

pub(crate) trait ProcessExecutor {
    fn new() -> Self;
    fn create_webauthn_process(&self) -> Result<Child, ProcessManagerError>;
    fn create_pcsc_process(&self) -> Result<Child, ProcessManagerError>;
}
