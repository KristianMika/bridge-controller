use std::process::{Child, Command};

use log::debug;

use crate::process::process_manager::process_manager_error::ProcessManagerError;

use super::ProcessExecutor;

pub(crate) struct LinuxProcessExecutor {}

impl ProcessExecutor for LinuxProcessExecutor {
    fn new() -> Self {
        Self {}
    }

    fn create_webauthn_process(&self) -> Result<Child, ProcessManagerError> {
        let softfido_child = Command::new("softfido")
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
        let _usb_ip_attach = Command::new("usbip")
            .arg("attach")
            .arg("--remote")
            .arg("127.0.0.1")
            .arg("--busid")
            .arg("1-1")
            .spawn()?;
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
}
