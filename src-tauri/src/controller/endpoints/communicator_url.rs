use actix_web::{get, web, Responder};
use log::debug;

use crate::{controller::state::State, interface::CryptographicInterface};

#[get("/{interface}/communicator_url")]
pub(crate) async fn get_communicator_url(
    path: web::Path<CryptographicInterface>,
    data: web::Data<State>,
) -> impl Responder {
    let interface = path.into_inner();
    let repo = data.get_controller_repo();
    let Ok(Some(configuration)) = repo.get_interface_configuration(&interface) else {
        // todo: return custom error
        panic!();
    };
    let communicator_url = String::from(configuration.get_communicator_url());
    debug!(
        "GET /{interface:?}/communicator_url -> {}",
        communicator_url
    );
    communicator_url
}
