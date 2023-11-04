use actix_web::{get, web, HttpResponse, Responder};
use log::{debug, error};
use serde::{Deserialize, Serialize};

use crate::{
    controller::{interface_configuration::GroupId, state::State},
    interface::CryptographicInterface,
};

#[get("/{interface}/configuration")]
pub(crate) async fn get_configuration(
    path: web::Path<CryptographicInterface>,
    query: web::Query<InterfaceQuery>,
    data: web::Data<State>,
) -> impl Responder {
    // TODO check if cert exists
    let tool = query.into_inner().tool;
    let interface = path.into_inner();
    let repo = data.get_controller_repo();
    let Ok(configuration) = repo.get_interface_configuration(&interface, &tool) else {
        return HttpResponse::InternalServerError().finish();
    };

    let configuration = match configuration {
        Some(configuration) => configuration,
        None => {
            // There is no configuration specific to the tool,
            // let's return the general, tool-independent configuration
            let Ok(configuration) = repo.get_interface_configuration(&interface, &None) else {
                return HttpResponse::InternalServerError().finish();
            };
            let Some(configuration) = configuration else {
                return HttpResponse::NotFound().body("No configuration found");
            };
            configuration
        }
    };

    let filesystem = data.get_filesystem();
    let filepath =
        match filesystem.get_certificate_filepath(configuration.get_communicator_hostname()) {
            Ok(Some(filepath)) => filepath,
            Ok(None) => return HttpResponse::NotFound().body("No certificate found"),
            Err(err) => {
                error!("Couldn't get certificate path: {err}");
                return HttpResponse::InternalServerError().finish();
            }
        };
    let filepath = filepath.to_str().unwrap().to_string();
    let configuration = InterfaceConfiguration::new(
        configuration.get_communicator_hostname().into(),
        filepath,
        configuration.into_group_id(),
    );
    debug!(
        "GET /{interface:?}/configuration?tool={tool:?} -> {:#?}",
        configuration
    );
    HttpResponse::Ok().json(web::Json(configuration))
}

#[derive(Deserialize)]
pub struct InterfaceQuery {
    tool: Option<String>,
}

#[derive(Serialize, Debug)]
struct InterfaceConfiguration {
    communicator_hostname: String,
    communicator_certificate_path: String,
    group_id: GroupId,
}

impl InterfaceConfiguration {
    fn new(
        communicator_hostname: String,
        communicator_certificate_path: String,
        group_id: GroupId,
    ) -> Self {
        Self {
            communicator_hostname,
            communicator_certificate_path,
            group_id,
        }
    }
}
