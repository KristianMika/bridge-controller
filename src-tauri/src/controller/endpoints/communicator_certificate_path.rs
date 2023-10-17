use actix_web::{get, web, HttpResponse, Responder};
use log::debug;
use serde::Serialize;

use crate::controller::state::State;

#[get("/{communicator_hostname}/certificate_path")]
pub(crate) async fn get_communicator_certificate_path(
    path: web::Path<String>,
    data: web::Data<State>,
) -> impl Responder {
    let communicator_hostname = path.into_inner();
    let filesystem = data.get_filesystem();
    let Ok(filepath) = filesystem.get_certificate_filepath(&communicator_hostname) else {
        return HttpResponse::InternalServerError().finish();
    };
    // TODO check if cert exists
    let filepath = filepath.to_str().unwrap().to_string();
    let certificate_response = CertificateResponse::new(Some(filepath));
    debug!(
        "GET /{communicator_hostname:?}/certificate_path -> {:#?}",
        certificate_response
    );
    HttpResponse::Ok().json(web::Json(certificate_response))
}

#[derive(Serialize, Debug)]
struct CertificateResponse {
    certificate_path: Option<String>,
}

impl CertificateResponse {
    fn new(certificate_path: Option<String>) -> Self {
        Self { certificate_path }
    }
}
