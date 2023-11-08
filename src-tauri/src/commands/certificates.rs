use log::{debug, error};

use crate::state::State;

/// Checks if there is a certificate stored for the specified communicator hostname.
///
/// # Arguments
/// * `communicator_hostname` - The hostname of the communicator.
/// * `state` - The state of the application.
#[tauri::command]
#[specta::specta]
pub(crate) async fn is_certificate_present(
    communicator_hostname: String,
    state: tauri::State<'_, State>,
) -> Result<bool, String> {
    let certificate_path = state
        .get_filesystem()
        .get_certificate_filepath(&communicator_hostname)
        .map_err(|err| {
            error!("{err:?}");
            String::from("Could not get certificate file")
        })?;

    let exists = certificate_path.is_some();
    debug!(
        "Command is_certificate_present for hostname {communicator_hostname:?} returning {exists}"
    );
    Ok(exists)
}

/// Stores the communicator certificate that was uploaded using the front-end page
///
/// # Arguments
///
/// * `certificate_path` - The path to the certificate file.
/// * `communicator_hostname` - The hostname of the communicator.
/// * `state` - The state of the application.
#[tauri::command]
#[specta::specta]
pub(crate) async fn store_communicator_certificate(
    certificate_path: String,
    communicator_hostname: String,
    state: tauri::State<'_, State>,
) -> Result<(), String> {
    debug!("Command set_communicator_certificate_path for hostname {communicator_hostname:?} and crtificate path '{certificate_path:?}'");
    let filesystem = state.get_filesystem();
    let _ = filesystem
        .copy_cerrtificate(&certificate_path, &communicator_hostname)
        .map_err(|err| {
            error!("{err:?}");
            String::from("Could not store certificate file")
        })?;
    Ok(())
}
