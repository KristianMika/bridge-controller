use serde::Serialize;

use crate::controller::interface_configuration::GroupId;

#[derive(Serialize, Debug)]
pub(super) struct HttpInterfaceConfigurationResponse {
    communicator_hostname: String,
    communicator_certificate_path: String,
    group_id: GroupId,
}

impl HttpInterfaceConfigurationResponse {
    pub(crate) fn new(
        communicator_hostname: String,
        communicator_certificate_path: String,
        group_id: GroupId,
    ) -> Self {
        Self {
            communicator_hostname,
            communicator_certificate_path,
            group_id,
        }
    }
}
