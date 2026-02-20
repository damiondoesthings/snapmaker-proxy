pub(crate) mod enclosure;
pub(crate) mod index;
pub(crate) mod controls;
pub(crate) mod upload;
pub(crate) mod version;

pub(crate) use enclosure::*;
pub(crate) use index::*;
pub(crate) use controls::*;
use std::sync::Arc;
use tokio::sync::watch;
use tera::Tera;
pub(crate) use upload::*;
pub(crate) use version::*;

#[derive(Clone)]
pub struct AppState {
    pub snapmaker_token: String,
    pub status_watch: watch::Receiver<crate::status::PrinterStatus>,
    pub tera: Arc<Tera>,
}
