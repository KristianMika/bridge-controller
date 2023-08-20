use crate::interface::CryptographicInterface;

use self::controller_repo_error::ControllerRepoError;

use super::interface_configuration::InterfaceConfiguration;

mod controller_repo_error;
pub(crate) mod sled_controller_repo;
pub(crate) trait ControllerRepo: Send + Sync {
    fn set_interface_configuration(
        &self,
        configuration: InterfaceConfiguration,
        interface: CryptographicInterface,
    ) -> Result<(), ControllerRepoError>;

    fn get_interface_configuration(
        &self,
        interface: &CryptographicInterface,
    ) -> Result<Option<InterfaceConfiguration>, ControllerRepoError>;
}
