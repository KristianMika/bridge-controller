pub(crate) use frontend_interface_configuration::FrontEndInterfaceConfiguration;
pub(crate) use internal_interface_configuration::InternalInterfaceConfiguration;

type ByteVector = Vec<u8>;
pub(crate) type GroupId = ByteVector;

mod frontend_interface_configuration;
mod internal_interface_configuration;
