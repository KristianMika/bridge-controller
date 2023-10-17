use log::{debug, error};

use crate::state::State;

#[tauri::command]
#[specta::specta]
pub(crate) async fn is_certificate_present(
    communicator_url: String,
    state: tauri::State<'_, State>,
) -> Result<bool, String> {
    debug!("A command for getting certificate {communicator_url} has been invoked");
    let certificate_path = state
        .get_filesystem()
        .get_certificate_filepath(&communicator_url)
        .map_err(|err| {
            error!("{err}");
            String::from("Could not get certificate file")
        })?;

    Ok(certificate_path.exists())
}
