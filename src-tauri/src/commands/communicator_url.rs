use log::{debug, error};

use crate::state::State;

#[tauri::command]
#[specta::specta]
pub(crate) async fn set_communicator_certificate_path(
    state: tauri::State<'_, State>,
    certificate_path: String,
    communicator_url: String,
) -> Result<(), String> {
    debug!("A command for setting a communicator certificate path for communicator {communicator_url} has been invoked with path: {certificate_path}");
    let filesystem = state.get_filesystem();
    let _ = filesystem
        .copy_cerrtificate(&certificate_path, &communicator_url)
        .map_err(|err| {
            error!("{err}");
            String::from("Could not store certificate file")
        })?;
    Ok(())
}
