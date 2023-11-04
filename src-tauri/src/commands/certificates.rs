use log::{debug, error};

use crate::state::State;

/// Checks if there is a certificate stored for the current communicator url.
///
/// # Arguments
/// * `communicator_url` - The url of the communicator.
/// * `state` - The state of the application.
#[tauri::command]
#[specta::specta]
pub(crate) async fn is_certificate_present(
    communicator_url: String,
    state: tauri::State<'_, State>,
) -> Result<bool, String> {
    let certificate_path = state
        .get_filesystem()
        .get_certificate_filepath(&communicator_url)
        .map_err(|err| {
            error!("{err}");
            String::from("Could not get certificate file")
        })?;

    let exists = certificate_path.exists();
    debug!("Command is_certificate_present for url {communicator_url:?} returning {exists}");
    Ok(exists)
}

/// Stores the communicator certificate that was uploaded using the front-end page
///
/// # Arguments
///
/// * `certificate_path` - The path to the certificate file.
/// * `communicator_url` - The url of the communicator.
/// * `state` - The state of the application.
#[tauri::command]
#[specta::specta]
pub(crate) async fn store_communicator_certificate(
    certificate_path: String,
    communicator_url: String,
    state: tauri::State<'_, State>,
) -> Result<(), String> {
    debug!("Command set_communicator_certificate_path for url {communicator_url:?} and crtificate path '{certificate_path:?}'");
    let filesystem = state.get_filesystem();
    let _ = filesystem
        .copy_cerrtificate(&certificate_path, &communicator_url)
        .map_err(|err| {
            error!("{err}");
            String::from("Could not store certificate file")
        })?;
    Ok(())
}
