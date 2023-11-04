use log::{debug, error};

use crate::{interface::CryptographicInterface, state::State};

/// Returns a list of tools for which there is a configuration present.
/// Value None present in the list means that the configuration is tool-independent.
///
/// # Arguments
///
/// * `cryptographic_interface` - The cryptographic interface
///     for which the configured tools should be returned
/// * `state` - The state of the application
#[tauri::command]
#[specta::specta]
pub(crate) async fn get_configured_tools(
    cryptographic_interface: CryptographicInterface,
    state: tauri::State<'_, State>,
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
