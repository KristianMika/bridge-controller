use crate::interface::CryptographicInterface;

use self::controller_repo_error::ControllerRepoError;

use super::interface_configuration::InternalInterfaceConfiguration;

mod controller_repo_error;
pub(crate) mod sled_controller_repo;
pub(crate) trait ControllerRepo: Send + Sync {
    fn set_interface_configuration(
        &self,
        configuration: InternalInterfaceConfiguration,
        interface: CryptographicInterface,
    ) -> Result<(), ControllerRepoError>;

    fn get_interface_configuration(
        &self,
        interface: &CryptographicInterface,
    ) -> Result<Option<InternalInterfaceConfiguration>, ControllerRepoError>;
}
