use log::{debug, error};

use crate::{
    controller::interface_configuration::FrontEndInterfaceConfiguration,
    interface::CryptographicInterface, state::State,
};

/// Stores a configuration for a specific cryptographic interface and tool.
/// In case the tool is None, the configuration is handled as tool-independent
///
/// # Arguments
///
/// * `cryptographic_interface` - The cryptographic interface for which
///     the configuration should be stored
/// * `tool` - The tool for which the configuration should be stored,
///     if None, the configuration is handled as tool-independent
/// * `configuration` - The configuration that should be stored
/// * `state` - The state of the application
#[tauri::command]
#[specta::specta]
pub(crate) async fn set_interface_configuration(
    cryptographic_interface: CryptographicInterface,
    tool: Option<String>,
    configuration: FrontEndInterfaceConfiguration,
    state: tauri::State<'_, State>,
) -> Result<(), String> {
    debug!("Command set_interface_configuration for interface '{cryptographic_interface:?}' and tool '{tool:?}' and configuration: {configuration:?}");
    let repo = state.get_controller_repo();
    repo.set_interface_configuration(
        configuration.into(),
        cryptographic_interface.clone(),
        tool.clone(),
    )
    .map_err(|err| {
        error!("{err:?}");
        String::from("Could not store interface configuration")
    })?;

    debug!("Command set_interface_configuration for {cryptographic_interface:?} and {tool:?}");
    Ok(())
}

/// Gets a configuration for a specific cryptographic interface and tool.
///
/// # Arguments
///
/// * `cryptographic_interface` - The cryptographic interface
///     for which the configuration should be fetched
/// * `tool` - The tool for which the configuration should be fetched,
///     if None, the general, tool-independent configuration is returned
/// * `state` - The state of the application
#[tauri::command]
#[specta::specta]
pub(crate) async fn get_interface_configuration(
    cryptographic_interface: CryptographicInterface,
    tool: Option<String>,
    state: tauri::State<'_, State>,
) -> Result<Option<FrontEndInterfaceConfiguration>, String> {
    let repo = state.get_controller_repo();
    let configuration: Option<FrontEndInterfaceConfiguration> = repo
        .get_interface_configuration(&cryptographic_interface, tool.clone())
        .map_err(|err| {
            error!("{err:?}");
            String::from("Could not get interface configuration")
        })?
        .map(|configuration| configuration.into());
    debug!("Command get_interface_configuration for interface '{cryptographic_interface:?}' and tool '{tool:?}', returning {configuration:?}");
    Ok(configuration)
}

/// Removes a configuration for a specific cryptographic interface and tool.
///
/// # Arguments
///
/// * `state` - The state of the application
/// * `cryptographic_interface` - The cryptographic interface
///     for which the configuration should be removed
/// * `tool` - The tool for which the configuration should be removed,
///     if None, the general, tool-independent configuration is removed
#[tauri::command]
#[specta::specta]
pub(crate) async fn remove_interface_configuration(
    cryptographic_interface: CryptographicInterface,
    tool: Option<String>,
    state: tauri::State<'_, State>,
) -> Result<(), String> {
    debug!("Command remove_interface_configuration for interface '{cryptographic_interface:?}' and tool '{tool:?}'");
    let repo = state.get_controller_repo();
    repo.remove_interface_configuration(&cryptographic_interface, &tool)
        .map_err(|err| {
            error!("{err:?}");
            String::from("Could not get interface configuration")
        })
}
