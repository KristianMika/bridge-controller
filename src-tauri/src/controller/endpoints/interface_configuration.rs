use actix_web::{get, web, Responder};
use serde::Serialize;

use crate::{
    controller::{
        interface_configuration::{GroupId, InternalInterfaceConfiguration},
        state::State,
    },
    interface::CryptographicInterface,
};

#[get("/{interface}/configuration")]
pub(crate) async fn get_configuration(
    path: web::Path<CryptographicInterface>,
    data: web::Data<State>,
) -> impl Responder {
    let interface = path.into_inner();
    let repo = data.get_controller_repo();
    let Ok(Some(configuration)) = repo.get_interface_configuration(&interface) else {
        // todo: return custom error
        panic!();
    };
    let configuration: InterfaceConfiguration = configuration.into();
    web::Json(configuration)
}

#[derive(Serialize)]
struct InterfaceConfiguration {
    communicator_url: String,
    group_id: GroupId,
}

impl From<InternalInterfaceConfiguration> for InterfaceConfiguration {
    fn from(value: InternalInterfaceConfiguration) -> Self {
        Self {
            communicator_url: value.get_communicator_url().into(),
            group_id: value.into_group_id(),
        }
    }
}
