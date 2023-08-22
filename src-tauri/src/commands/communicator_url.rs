use crate::state::State;

#[tauri::command]
#[specta::specta]
pub(crate) async fn set_communicator_certificate_path(
    state: tauri::State<'_, State>,
    certificate_path: String,
    communicator_url: String,
) -> Result<(), String> {
    let filesystem = state.get_filesystem();
    filesystem
        .copy_cerrtificate(&certificate_path, &communicator_url)
        .unwrap();
    Ok(())
}
