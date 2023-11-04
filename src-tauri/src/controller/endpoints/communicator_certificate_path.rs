mod certificate_response;

use actix_web::{get, web, HttpResponse, Responder};
use log::{debug, error};

use crate::controller::{
    endpoints::communicator_certificate_path::certificate_response::CertificateResponse,
    state::State,
};

#[get("/{communicator_hostname}/certificate_path")]
pub(crate) async fn get_communicator_certificate_path(
    hostname: web::Path<String>,
    state: web::Data<State>,
) -> impl Responder {
    let communicator_hostname = hostname.into_inner();
    let filesystem = state.get_filesystem();
    let filepath = match filesystem.get_certificate_filepath(&communicator_hostname) {
        Ok(Some(filepath)) => filepath,
        Ok(None) => return HttpResponse::NotFound().body("No certificate found"),
        Err(err) => {
            error!("Couldn't get certificate path: {err}");
            return HttpResponse::InternalServerError().finish();
        }
    };
    let filepath = filepath.to_str().unwrap().to_string();
    let certificate_response = CertificateResponse::new(Some(filepath));
    debug!(
        "GET /{communicator_hostname:?}/certificate_path -> {:#?}",
        certificate_response
    );
    HttpResponse::Ok().json(web::Json(certificate_response))
}
