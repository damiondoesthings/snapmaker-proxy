use serde::{Deserialize, Serialize};
use tokio::sync::watch;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all(deserialize="camelCase"))]
pub struct PrinterStatus {
    pub status: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub homed: bool,
    pub nozzle_temperature: f64,
    pub nozzle_target_temperature: f64,
    pub heated_bed_temperature: f64,
    pub heated_bed_target_temperature: f64,
    #[serde(default)]
    pub work_speed: f64,
    #[serde(default)]
    pub file_name: String,
    #[serde(default)]
    pub progress: f64,
    #[serde(default)]
    pub estimated_time: f64,
    #[serde(default)]
    pub elapsed_time: f64,
    #[serde(default)]
    pub remaining_time: f64,
    pub print_status: String,
    #[serde(default)]
    pub enclosure: EnclosureStatus
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default, rename_all(deserialize="camelCase"))]
pub struct EnclosureStatus {
    led: u8,
    fan: u8
}

impl Default for PrinterStatus {
    fn default() -> Self {
        Self {
            status: "IDLE".to_string(),
            x: 0.0,
            y: 0.0,
            z: 0.0,
            homed: false,
            nozzle_temperature: 0.0,
            nozzle_target_temperature: 0.0,
            heated_bed_temperature: 0.0,
            heated_bed_target_temperature: 0.0,
            work_speed: 0.0,
            file_name: "No file loaded".to_string(),
            progress: 0.0,
            estimated_time: 0.0,
            elapsed_time: 0.0,
            remaining_time: 0.0,
            print_status: "Idle".to_string(),
            enclosure: EnclosureStatus::default()
        }
    }
}

pub fn create_status_watch() -> (watch::Sender<PrinterStatus>, watch::Receiver<PrinterStatus>) {
    let default_status = PrinterStatus::default();
    watch::channel(default_status)
}
