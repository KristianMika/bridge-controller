use crate::interface::CryptographicInterface;

use self::controller_repo_error::ControllerRepoError;

use super::interface_configuration::InternalInterfaceConfiguration;

mod controller_repo_error;
pub(crate) mod sled_controller_repo;

pub(crate) trait ControllerRepo: Send + Sync {
    /// Stores the interface configuration in the database.
    ///
    /// # Arguments
    ///
    /// * `configuration` - The interface configuration to store.
    /// * `interface` - The interface to store the configuration for.
    /// * `tool` - The tool to store the configuration for.
    ///     Value None means that the configuration is for all tools.
    fn set_interface_configuration(
        &self,
        configuration: InternalInterfaceConfiguration,
        interface: CryptographicInterface,
        tool: Option<String>,
    ) -> Result<(), ControllerRepoError>;

    /// Retrieves the interface configuration from the database.
    ///
    /// # Arguments
    ///
    /// * `interface` - The interface to retrieve the configuration for.
    /// * `tool` - The tool to retrieve the configuration for.
    ///     Value None means that the configuration is for all tools.
    fn get_interface_configuration(
        &self,
        interface: &CryptographicInterface,
        tool: &Option<String>,
    ) -> Result<Option<InternalInterfaceConfiguration>, ControllerRepoError>;

    /// Retrieves the list of tools for which there is a configuration
    /// present in the database.
    ///
    /// # Arguments
    ///
    /// * `interface` - The interface to retrieve the list of tools for.
    fn get_configured_tools(
        &self,
        interface: &CryptographicInterface,
    ) -> Result<Vec<Option<String>>, ControllerRepoError>;

    /// Removes the interface configuration from the database.
    ///
    /// # Arguments
    ///
    /// * `interface` - The interface to remove the configuration for.
    /// * `tool` - The tool to remove the configuration for.
    ///     Value None means that the configuration is for all tools.
    fn remove_interface_configuration(
        &self,
        interface: &CryptographicInterface,
        tool: &Option<String>,
    ) -> Result<(), ControllerRepoError>;
}
