use super::AppState;
use actix_web::{HttpResponse, Responder, post, web};

#[post("/api/pause_print")]
pub async fn pause_print(data: web::Data<AppState>) -> impl Responder {
    match crate::snapmaker_client::pause_print(&data.snapmaker_token).await {
        Ok(_) => HttpResponse::Ok().body("Print paused successfully"),
        Err(e) => {
            log::error!("Failed to pause print: {:?}", e);
            HttpResponse::InternalServerError().body(format!("Failed to pause print: {}", e))
        }
    }
}

#[post("/api/stop_print")]
pub async fn stop_print(data: web::Data<AppState>) -> impl Responder {
    match crate::snapmaker_client::stop_print(&data.snapmaker_token).await {
        Ok(_) => HttpResponse::Ok().body("Print stopped successfully"),
        Err(e) => {
            log::error!("Failed to stop print: {:?}", e);
            HttpResponse::InternalServerError().body(format!("Failed to stop print: {}", e))
        }
    }
}

#[post("/api/resume_print")]
pub async fn resume_print(data: web::Data<AppState>) -> impl Responder {
    match crate::snapmaker_client::resume_print(&data.snapmaker_token).await {
        Ok(_) => HttpResponse::Ok().body("Print resumed successfully"),
        Err(e) => {
            log::error!("Failed to resume print: {:?}", e);
            HttpResponse::InternalServerError().body(format!("Failed to resume print: {}", e))
        }
    }
}
