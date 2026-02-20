use std::time::Duration;

use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_web::{Error, HttpResponse, post, web};

use crate::{http_endpoints::AppState, snapmaker_client};

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(limit = "200MB")]
    file: TempFile,
    print: Text<bool>,
}

#[post("/api/files/local")]
pub(crate) async fn handle_upload(
    MultipartForm(form): MultipartForm<UploadForm>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let file_path = form.file.file.path();
    let file_name = match form.file.file_name {
        Some(x) => x,
        None => return Ok(HttpResponse::BadRequest().body("No filename provided")),
    };
    match snapmaker_client::upload_file_to_snapmaker(&data.snapmaker_token, file_path, &file_name)
        .await
    {
        Ok(_) => (),
        Err(e) => {
            return Ok(HttpResponse::InternalServerError()
                .body(format!("Upload Error from snapmaker: {e:?}")));
        }
    };
    if form.print.0 {
        tokio::time::sleep(Duration::from_secs(2)).await;
        match snapmaker_client::start_print(&data.snapmaker_token).await {
            Ok(_) => (),
            Err(e) => {
                return Ok(HttpResponse::InternalServerError()
                    .body(format!("Print start Error from snapmaker: {e:?}")));
            }
        }
    };
    Ok(HttpResponse::Created().body("Success"))
}
