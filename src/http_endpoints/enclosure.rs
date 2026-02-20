use super::AppState;
use actix_web::{HttpResponse, Responder, post, web};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct EnclosureLightRequest {
    value: i32,
}

#[derive(Debug, Deserialize)]
struct EnclosureFanRequest {
    value: i32,
}

#[post("/api/enclosure/light")]
pub async fn set_enclosure_light(
    data: web::Data<AppState>,
    request: web::Form<EnclosureLightRequest>,
) -> impl Responder {
    match crate::snapmaker_client::set_enclosure_light(&data.snapmaker_token, request.value as u8).await {
        Ok(_) => HttpResponse::Ok().body(request.value.to_string()),
        Err(e) => {
            log::error!("Failed to set enclosure light: {:?}", e);
            HttpResponse::InternalServerError().body(format!("Failed to set enclosure light: {}", e))
        }
    }
}

#[post("/api/enclosure/fan")]
pub async fn set_enclosure_fan(
    data: web::Data<AppState>,
    request: web::Form<EnclosureFanRequest>,
) -> impl Responder {
    match crate::snapmaker_client::set_enclosure_fan(&data.snapmaker_token, request.value as u8).await {
        Ok(_) => HttpResponse::Ok().body(request.value.to_string()),
        Err(e) => {
            log::error!("Failed to set enclosure fan: {:?}", e);
            HttpResponse::InternalServerError().body(format!("Failed to set enclosure fan: {}", e))
        }
    }
}
