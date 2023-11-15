use std::process::Child;

use dashmap::DashMap;
use log::info;

use self::process_manager_error::ProcessManagerError;

use super::process_executor::PlatformSpecificProcessExecutor;
use crate::{interface::CreatableInterface, process::process_executor::ProcessExecutor};

pub(crate) mod process_manager_error;

/// Manages the processes of emulated interfaces,
/// its creation and termination
pub(crate) struct ProcessManager {
    /// A map of all running processes
    processes: DashMap<CreatableInterface, Child>,

    /// The process executor for the current platform
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
            return Err(ProcessManagerError::ProcessAlreadyRunning(interface));
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
        let Some((_, process)) = self.processes.remove(interface) else {
            return Err(ProcessManagerError::ProcessNotRunning(*interface));
        };

        match interface {
            CreatableInterface::Pcsc => self.process_executor.kill_pcsc_process(process)?,
            CreatableInterface::Webauthn => self.process_executor.kill_webauthn_process(process)?,
        }

        Ok(())
    }
}
