use log::{debug, error};

use crate::interface::CreatableInterface;
use crate::state::State;

/// Launches an emulated interface process. If the process is running,
/// returns an error.
///
/// # Arguments
///
/// * `creatable_interface` - The emulated interface that
///     should be launched
/// * `state` - The state of the application
#[tauri::command]
#[specta::specta]
pub(crate) async fn spawn_interface_process(
    creatable_interface: CreatableInterface,
    state: tauri::State<'_, State>,
) -> Result<(), String> {
    debug!("Command spawn_interface_process for interface '{creatable_interface:?}'");
    let process_manager = state.get_process_manager();
    process_manager
        .spawn_process(creatable_interface)
        .map_err(|err| {
            error!("{err:?}");
            String::from("Could not spawn process")
        })
}

/// Kills an emulated interface process. If the process is not running,
/// returns an error.
///
/// # Arguments
///
/// * `creatable_interface` - The interface whose process should be killed
/// * `state` - The state of the application
#[tauri::command]
#[specta::specta]
pub(crate) async fn kill_interface_process(
    creatable_interface: CreatableInterface,
    state: tauri::State<'_, State>,
) -> Result<(), String> {
    debug!("Command kill_interface_process for interface '{creatable_interface:?}'");
    let process_manager = state.get_process_manager();
    process_manager
        .kill_process(&creatable_interface)
        .map_err(|err| {
            error!("{err:?}");
            String::from("Could not kill process")
        })
}

/// Checks if the specified process is running at the moment
/// regardless of the configuration
///
/// # Arguments
///
/// * `creatable_interface` - Specified interface
/// * `state` - The state of the application
#[tauri::command]
#[specta::specta]
pub(crate) async fn is_interface_process_running(
    creatable_interface: CreatableInterface,
    state: tauri::State<'_, State>,
) -> Result<bool, ()> {
    let process_manager = state.get_process_manager();
    Ok(process_manager.is_process_running(&creatable_interface))
}
