use log::{debug, error};

use crate::{interface::CryptographicInterface, state::State};

#[tauri::command]
#[specta::specta]
pub(crate) async fn get_configured_tools(
    state: tauri::State<'_, State>,
    cryptographic_interface: CryptographicInterface,
) -> Result<Vec<Option<String>>, String> {
    let tools = state
        .get_controller_repo()
        .get_configured_tools(&cryptographic_interface)
        .map_err(|err| {
            error!("{err}");
            String::from("Couldn't get configured tools")
        })?;
    debug!("command get_configured_tools for {cryptographic_interface:?}, returning {tools:?}");
    Ok(tools)
}
