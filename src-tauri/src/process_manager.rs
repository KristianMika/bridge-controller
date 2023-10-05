use std::process::Child;

use dashmap::DashMap;
use log::info;
use serde::Deserialize;
use specta::Type;

use self::process_manager_error::ProcessManagerError;
#[cfg(target_os = "linux")]
pub(crate) use linux_process_executor::LinuxProcessExecutor as PlatformSpecificProcessExecutor;
#[cfg(target_os = "windows")]
pub(crate) use windows_process_executor::WindowsProcessExecutor as PlatformSpecificProcessExecutor;

#[cfg(target_os = "linux")]
mod linux_process_executor;

#[cfg(target_os = "windows")]
mod windows_process_executor;

mod process_manager_error;

pub(crate) trait ProcessExecutor {
    fn new() -> Self;
    fn create_webauthn_process(&self) -> Result<Child, ProcessManagerError>;
    fn create_pcsc_process(&self) -> Result<Child, ProcessManagerError>;
}

pub(crate) struct ProcessManager {
    processes: DashMap<CreatableInterface, Child>,
    process_executor: PlatformSpecificProcessExecutor,
}

impl ProcessManager {
    pub(crate) fn new(process_executor: PlatformSpecificProcessExecutor) -> Self {
        Self {
            processes: DashMap::new(),
            process_executor,
        }
    }

    pub(crate) fn spawn_process(
        &self,
        interface: CreatableInterface,
    ) -> Result<(), ProcessManagerError> {
        if self.processes.contains_key(&interface) {
            return Err(ProcessManagerError::ProcessAlreadyRunning);
        }
        let child = match interface {
            CreatableInterface::Pcsc => self.process_executor.create_pcsc_process()?,
            CreatableInterface::Webauthn => self.process_executor.create_webauthn_process()?,
        };
        info!(
            "process for interface {:?} has been spawned with PID {}",
            interface,
            child.id(),
        );
        self.processes.insert(interface, child);
        Ok(())
    }

    pub(crate) fn kill_process(
        &self,
        interface: &CreatableInterface,
    ) -> Result<(), ProcessManagerError> {
        let Some((_, mut process)) = self.processes.remove(interface) else {
            return Err(ProcessManagerError::ProcessNotRunning);
        };

        process.kill()?;
        process.wait()?;

        Ok(())
    }
}
#[derive(Eq, Hash, PartialEq, Deserialize, Type, Debug)]
#[serde(rename_all = "lowercase")]
pub(crate) enum CreatableInterface {
    Pcsc,
    Webauthn,
}
