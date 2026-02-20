use actix_web::get;
use actix_web::web::Json;
use serde::Serialize;

#[get("/api/version")]
pub(crate) async fn get_version() -> Json<OctoVersion> {
    Json(OctoVersion {
        api: "0.1".to_string(),
        server: "1.9.0".to_string(),
        text: "OctoPrint (Snapmaker Proxy)".to_string(),
    })
}

#[derive(Serialize)]
pub(crate) struct OctoVersion {
    pub(crate) api: String,
    pub(crate) server: String,
    pub(crate) text: String,
}
