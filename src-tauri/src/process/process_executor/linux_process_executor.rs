use std::process::{Child, Command};

use log::{debug, error};

use crate::process::process_manager::process_manager_error::ProcessManagerError;

use super::ProcessExecutor;

pub(crate) struct LinuxProcessExecutor {}

impl ProcessExecutor for LinuxProcessExecutor {
    fn new() -> Self {
        Self {}
    }

    fn create_webauthn_process(&self) -> Result<Child, ProcessManagerError> {
        let mut softfido_child = Command::new("softfido")
            .arg("--cryptoki-bridge-mode")
            .arg("--pkcs11-module")
            .arg("/usr/lib/libcryptoki_bridge.so")
            .env("USED_AS_FIDO", "1")
            .spawn()?;
        debug!(
            "SoftFIDO process has been spawned with PID {}",
            softfido_child.id()
        );
        std::thread::sleep(std::time::Duration::from_millis(500));
        let usbip_attach_status = Command::new("bash")
            .arg("-c")
            // A temporary sollution that will by fixed by writing proper udev rules.
            // As of now, I was not able to accomplish that, but it should be possible.
            // To make this command work, the current user has to have configured
            // passwordless sudo. This requirement will also be dropped
            .arg("sudo usbip attach --remote 127.0.0.1 --busid 1-1")
            .output()?
            .status;
        if !usbip_attach_status.success() {
            softfido_child.kill()?;
            softfido_child.wait()?;
            error!("usbip attach failed, returned: {usbip_attach_status}");
            return Err(ProcessManagerError::InterfaceEmulationError);
        }
        debug!("usbip attach process has been spawned");

        Ok(softfido_child)
    }

    fn create_pcsc_process(&self) -> Result<Child, ProcessManagerError> {
        let pcsc_child = Command::new("sh")
            .arg("vicc")
            .arg("-t")
            .arg("meesign")
            .spawn()?;
        debug!(
            "PC/SC process has been spawned with PID {}",
            pcsc_child.id()
        );

        Ok(pcsc_child)
    }

    fn kill_webauthn_process(&self, mut process: Child) -> Result<(), ProcessManagerError> {
        let usbip_dettach_status = Command::new("bash")
            .arg("-c")
            // A temporary sollution that will by fixed by writing proper udev rules.
            // As of now, I was not able to accomplish that, but it should be possible.
            // To make this command work, the current user has to have configured
            // passwordless sudo. This requirement will also be dropped
            .arg("sudo usbip detach -p 00")
            .output()?
            .status;

        if !usbip_dettach_status.success() {
            error!("usbip dettach failed, returned: {usbip_dettach_status}");
            return Err(ProcessManagerError::InterfaceEmulationError);
        }

        process.kill()?;
        process.wait()?;
        Ok(())
    }

    fn kill_pcsc_process(&self, mut process: Child) -> Result<(), ProcessManagerError> {
        process.kill()?;
        process.wait()?;
        Ok(())
    }
}
