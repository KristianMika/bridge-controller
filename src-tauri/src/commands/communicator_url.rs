use crate::state::State;

#[tauri::command]
#[specta::specta]
pub(crate) async fn set_communicator_certificate_path(
    _state: tauri::State<'_, State>,
    _certificate_path: String,
) -> Result<(), String> {
    // TODO
    Ok(())
}
