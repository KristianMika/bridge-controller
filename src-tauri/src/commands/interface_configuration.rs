use crate::{interface::CryptographicInterface, state::State, FrontEndInterfaceConfiguration};

#[tauri::command]
#[specta::specta]
pub(crate) async fn set_interface_configuration(
    state: tauri::State<'_, State>,
    cryptographic_interface: CryptographicInterface,
    configuration: FrontEndInterfaceConfiguration,
) -> Result<(), String> {
    let repo = state.get_controller_repo();
    repo.set_interface_configuration(configuration.into(), cryptographic_interface)
        .unwrap();
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub(crate) async fn get_interface_configuration(
    state: tauri::State<'_, State>,
    cryptographic_interface: CryptographicInterface,
) -> Result<Option<FrontEndInterfaceConfiguration>, String> {
    let repo = state.get_controller_repo();
    let Some(configuration) = repo
        .get_interface_configuration(&cryptographic_interface)
        .unwrap() else {return Ok(None)};
    Ok(Some(configuration.into()))
}
