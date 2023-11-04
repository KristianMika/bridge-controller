mod http_interface_configuration_response;
mod interface_query;

use actix_web::{get, web, HttpResponse, Responder};
use log::{debug, error};

use crate::{
    controller::{
        endpoints::interface_configuration::{
            http_interface_configuration_response::HttpInterfaceConfigurationResponse,
            interface_query::InterfaceQuery,
        },
        state::State,
    },
    interface::CryptographicInterface,
};

#[get("/{interface}/configuration")]
pub(crate) async fn get_configuration(
    interface: web::Path<CryptographicInterface>,
    query: web::Query<InterfaceQuery>,
    state: web::Data<State>,
) -> impl Responder {
    let tool = query.into_inner().into_tool();
    let interface = interface.into_inner();
    let repo = state.get_controller_repo();
    let Ok(configuration) = repo.get_interface_configuration(&interface, tool.clone()) else {
        return HttpResponse::InternalServerError().finish();
    };

    let configuration = match configuration {
        Some(configuration) => configuration,
        None => {
            // There is no configuration specific to the tool,
            // let's return the general, tool-independent configuration
            let Ok(configuration) = repo.get_interface_configuration(&interface, None) else {
                return HttpResponse::InternalServerError().finish();
            };
            let Some(configuration) = configuration else {
                return HttpResponse::NotFound().body("No configuration found");
            };
            configuration
        }
    };

    let filesystem = state.get_filesystem();
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
    let configuration = HttpInterfaceConfigurationResponse::new(
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
