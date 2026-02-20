use super::AppState;
use actix_web::{HttpResponse, Responder, get, web};
use tera::Context;

#[get("/")]
pub async fn get_index(data: web::Data<AppState>) -> impl Responder {
    let status = data.status_watch.borrow();
    let mut context = Context::new();
    context.insert("status", &*status);

    match data.tera.render("index.html.tera", &context) {
        Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
        Err(e) => {
            log::error!("Failed to render index template: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to render template")
        }
    }
}

#[get("/api/status")]
pub async fn get_status(data: web::Data<AppState>) -> impl Responder {
    let status = data.status_watch.borrow();
    HttpResponse::Ok().json(&*status)
}

#[get("/render/status")]
pub async fn get_rendered_status(data: web::Data<AppState>) -> impl Responder {
    let status = data.status_watch.borrow();
    let mut context = Context::new();
    context.insert("status", &*status);

    match data.tera.render("status.html.tera", &context) {
        Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
        Err(e) => {
            log::error!("Failed to render status template: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to render template")
        }
    }
}

#[get("/render/controls")]
pub async fn get_rendered_controls(data: web::Data<AppState>) -> impl Responder {
    let status = data.status_watch.borrow();
    let mut context = Context::new();
    context.insert("status", &*status);
    match data.tera.render("controls.html.tera", &context) {
        Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
        Err(e) => {
            log::error!("Failed to render status template: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to render template")
        }
    }
}
