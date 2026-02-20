mod config;
mod http_endpoints;
mod snapmaker_client;
mod status;

use actix_web::{App, HttpServer, middleware::Logger, web};
use log::info;

use crate::config::SERVE_ADDRESS;
use crate::http_endpoints::AppState;
use crate::snapmaker_client::keep_alive_loop;
use crate::status::create_status_watch;
use std::sync::Arc;
use tera::Tera;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    env_logger::init();
    info!("Starting Snapmaker Proxy Server");

    // Get Snapmaker token
    let token = match snapmaker_client::get_snapmaker_token().await {
        Ok(token) => {
            info!("Successfully obtained Snapmaker token");
            token
        }
        Err(e) => {
            anyhow::bail!("Failed to get Snapmaker token: {}", e);
        }
    };

    // Create status watch channel
    let (status_sender, status_receiver) = create_status_watch();

    // Initialize Tera templates
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => Arc::new(t),
        Err(e) => {
            anyhow::bail!("Failed to initialize Tera templates {e}");
        }
    };

    // Create app state with both upload and status functionality
    let app_state = web::Data::new(AppState {
        snapmaker_token: token.clone(),
        status_watch: status_receiver,
        tera: tera.clone(),
    });

    info!("Starting server on {}", SERVE_ADDRESS);

    // Spawn keepalive thread with status sender
    tokio::spawn(keep_alive_loop(token, status_sender));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_state.clone())
            .service(http_endpoints::handle_upload)
            .service(http_endpoints::get_version)
            .service(http_endpoints::get_status)
            .service(http_endpoints::get_rendered_status)
            .service(http_endpoints::get_rendered_controls)
            .service(http_endpoints::get_index)
            .service(http_endpoints::set_enclosure_light)
            .service(http_endpoints::set_enclosure_fan)
            .service(http_endpoints::pause_print)
            .service(http_endpoints::stop_print)
            .service(http_endpoints::resume_print)
            .service(actix_files::Files::new("/static", "static").show_files_listing())
    })
    .bind(SERVE_ADDRESS)?
    .run()
    .await
    .map_err(|e| anyhow::anyhow!(e))
}
