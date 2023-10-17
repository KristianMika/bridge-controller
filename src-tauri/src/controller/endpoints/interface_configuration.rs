use actix_web::{get, web, HttpResponse, Responder};
use log::debug;
use serde::Serialize;

use crate::{
    controller::{interface_configuration::GroupId, state::State},
    interface::CryptographicInterface,
};

#[get("/{interface}/configuration")]
pub(crate) async fn get_configuration(
    path: web::Path<CryptographicInterface>,
    data: web::Data<State>,
) -> impl Responder {
    // TODO check if cert exists
    let interface = path.into_inner();
    let repo = data.get_controller_repo();
    let Ok(Some(configuration)) = repo.get_interface_configuration(&interface) else {
        return HttpResponse::NotFound().body("Configuration not found");
    };

    let filesystem = data.get_filesystem();
    let Ok(filepath) = filesystem.get_certificate_filepath(&configuration.get_communicator_url())
    else {
        return HttpResponse::InternalServerError().finish();
    };
    let filepath = filepath.to_str().unwrap().to_string();
    let configuration = InterfaceConfiguration::new(
        configuration.get_communicator_url().into(),
        filepath,
        configuration.into_group_id(),
    );
    debug!("GET /{interface:?}/configuration -> {:#?}", configuration);
    HttpResponse::Ok().json(web::Json(configuration))
}

#[derive(Serialize, Debug)]
struct InterfaceConfiguration {
    communicator_url: String,
    communicator_certificate_path: String,
    group_id: GroupId,
}

impl InterfaceConfiguration {
    fn new(
        communicator_url: String,
        communicator_certificate_path: String,
        group_id: GroupId,
    ) -> Self {
        Self {
            communicator_url,
            communicator_certificate_path,
            group_id,
        }
    }
}
