use actix_web::{get, web, Responder};

use crate::controller::state::State;

#[get("/{communicator_hostname}/certificate_path")]
pub(crate) async fn get_communicator_certificate_path(
    path: web::Path<String>,
    data: web::Data<State>,
) -> impl Responder {
    let communicator_hostname = path.into_inner();
    let filesystem = data.get_filesystem();
    let filepath = filesystem
        .get_certificate_filepath(&communicator_hostname)
        .unwrap();
    // TODO check if cert exists
    // TODO: errorhandling
    filepath.to_str().unwrap().to_string()
}
