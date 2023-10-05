use log::debug;

use crate::{process_manager::CreatableInterface, state::State};

#[tauri::command]
#[specta::specta]
pub(crate) async fn spawn_interface_process(
    state: tauri::State<'_, State>,
    creatable_interface: CreatableInterface,
) -> Result<(), String> {
    debug!("A command for spawning a {creatable_interface:?} process has been invoked");
    let process_manager = state.get_process_manager();
    process_manager
        .spawn_process(creatable_interface)
        .map_err(|err| err.to_string())
}

#[tauri::command]
#[specta::specta]
pub(crate) async fn kill_interface_process(
    state: tauri::State<'_, State>,
    creatable_interface: CreatableInterface,
) -> Result<(), String> {
    debug!("A command for killing a {creatable_interface:?} process has been invoked");
    let process_manager = state.get_process_manager();
    process_manager
        .kill_process(&creatable_interface)
        .map_err(|err| err.to_string())
}
