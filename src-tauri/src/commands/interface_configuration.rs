use log::{debug, error};

use crate::{
    controller::interface_configuration::FrontEndInterfaceConfiguration,
    interface::CryptographicInterface, state::State,
};

#[tauri::command]
#[specta::specta]
pub(crate) async fn set_interface_configuration(
    state: tauri::State<'_, State>,
    cryptographic_interface: CryptographicInterface,
    tool: Option<String>,
    configuration: FrontEndInterfaceConfiguration,
) -> Result<(), String> {
    debug!("Command set_interface_configuration for interface '{cryptographic_interface:?}' and tool '{tool:?}' and configuration: {configuration:?}");
    let repo = state.get_controller_repo();
    repo.set_interface_configuration(
        configuration.into(),
        cryptographic_interface.clone(),
        tool.clone(),
    )
    .map_err(|err| {
        error!("{err}");
        String::from("Could not store interface configuration")
    })?;

    debug!("Command set_interface_configuration for {cryptographic_interface:?} and {tool:?}");
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub(crate) async fn get_interface_configuration(
    state: tauri::State<'_, State>,
    cryptographic_interface: CryptographicInterface,
    tool: Option<String>,
) -> Result<Option<FrontEndInterfaceConfiguration>, String> {
    let repo = state.get_controller_repo();
    let configuration: Option<FrontEndInterfaceConfiguration> = repo
        .get_interface_configuration(&cryptographic_interface, &tool)
        .map_err(|err| {
            error!("{err}");
            String::from("Could not get interface configuration")
        })?
        .map(|configuration| configuration.into());
    debug!("Command get_interface_configuration for interface '{cryptographic_interface:?}' and tool '{tool:?}', returning {configuration:?}");
    Ok(configuration)
}

#[tauri::command]
#[specta::specta]
pub(crate) async fn remove_interface_configuration(
    state: tauri::State<'_, State>,
    cryptographic_interface: CryptographicInterface,
    tool: Option<String>,
) -> Result<(), String> {
    debug!("Command remove_interface_configuration for interface '{cryptographic_interface:?}' and tool '{tool:?}'");
    let repo = state.get_controller_repo();
    repo.remove_interface_configuration(&cryptographic_interface, &tool)
        .map_err(|err| {
            error!("{err}");
            String::from("Could not get interface configuration")
        })
}
