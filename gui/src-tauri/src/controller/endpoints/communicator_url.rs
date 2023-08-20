use actix_web::{get, Responder};

#[get("/communicator_url")]
pub(crate) async fn get_communicator_url() -> impl Responder {
    "meesign.local"
}
