use log::{debug, error};

use crate::{interface::CryptographicInterface, state::State, FrontEndInterfaceConfiguration};

#[tauri::command]
#[specta::specta]
pub(crate) async fn set_interface_configuration(
    state: tauri::State<'_, State>,
    cryptographic_interface: CryptographicInterface,
    configuration: FrontEndInterfaceConfiguration,
) -> Result<(), String> {
    debug!("A command for setting a {cryptographic_interface:?} configuration has been invoked: {configuration:#?}");
    let repo = state.get_controller_repo();
    repo.set_interface_configuration(configuration.into(), cryptographic_interface)
        .map_err(|err| {
            error!("{err}");
            String::from("Could not store interface configuration")
        })
}

#[tauri::command]
#[specta::specta]
pub(crate) async fn get_interface_configuration(
    state: tauri::State<'_, State>,
    cryptographic_interface: CryptographicInterface,
) -> Result<Option<FrontEndInterfaceConfiguration>, String> {
    let repo = state.get_controller_repo();
    let configuration: Option<FrontEndInterfaceConfiguration> = repo
        .get_interface_configuration(&cryptographic_interface)
        .map_err(|err| {
            error!("{err}");
            String::from("Could not get interface configuration")
        })?
        .and_then(|configuration| Some(configuration.into()));

    Ok(configuration)
}
