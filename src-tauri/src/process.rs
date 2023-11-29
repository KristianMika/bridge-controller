use std::sync::Arc;

use log::error;
use strum::IntoEnumIterator;

use crate::{controller::controller_repo::ControllerRepo, interface::CreatableInterface};

use self::process_manager::{process_manager_error::ProcessManagerError, ProcessManager};

pub(crate) mod process_executor;
pub(crate) mod process_manager;

/// Spawns processes for all enabled creatable interfaces.
///
/// # Arguments
///
/// * `repo` - The controller repository
/// * `process_manager` - The process manager
#[allow(dead_code)]
pub(crate) fn spawn_enabled_interfaces(
    repo: &Arc<dyn ControllerRepo>,
    process_manager: &ProcessManager,
) -> Result<(), ProcessManagerError> {
    let is_interface_creatable = |&interface: &CreatableInterface| -> bool {
        repo.get_interface_configuration(&interface.into(), None)
            .unwrap_or_else(|err| {
                // we might want to propagate this error as well
                error!(
                    "Failed to get tool-independent interface configuration for interface {interface:?}: {err:?}"
                );
                None
            })
            .map(|configuration| configuration.is_enabled())
            .unwrap_or(false)
    };

    let enabled_creatable_interfaces = CreatableInterface::iter().filter(is_interface_creatable);

    let mut spawn_results = enabled_creatable_interfaces.map(|interface| {
        if let Err(err) = process_manager.spawn_process(interface) {
            error!("Failed to spawn process for interface {interface:?}: {err:?}");
            return Err(err);
        }
        Ok(())
    });
    // return first error or ok
    spawn_results
        .find(|result| result.is_err())
        .unwrap_or(Ok(()))
}
