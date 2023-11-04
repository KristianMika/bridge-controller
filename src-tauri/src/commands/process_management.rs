use log::{debug, error};

use crate::process::creatable_interface::CreatableInterface;
use crate::state::State;

#[tauri::command]
#[specta::specta]
pub(crate) async fn spawn_interface_process(
    state: tauri::State<'_, State>,
    creatable_interface: CreatableInterface,
) -> Result<(), String> {
    debug!("Command spawn_interface_process for interface '{creatable_interface:?}'");
    let process_manager = state.get_process_manager();
    process_manager
        .spawn_process(creatable_interface)
        .map_err(|err| {
            error!("{err}");
            String::from("Could not spawn process")
        })
}

#[tauri::command]
#[specta::specta]
pub(crate) async fn kill_interface_process(
    state: tauri::State<'_, State>,
    creatable_interface: CreatableInterface,
) -> Result<(), String> {
    debug!("Command kill_interface_process for interface '{creatable_interface:?}'");
    let process_manager = state.get_process_manager();
    process_manager
        .kill_process(&creatable_interface)
        .map_err(|err| {
            error!("{err}");
            String::from("Could not kill process")
        })
}
