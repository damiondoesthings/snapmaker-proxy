use log::{error, info};
use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path, time::Duration};
use tokio::sync::watch::Sender;

use crate::{
    config::{SNAPMAKER_ENDPOINT, TOKEN_FILE},
    status::{EnclosureStatus, PrinterStatus},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapmakerTokenResponse {
    pub token: String,
}

pub async fn get_snapmaker_token() -> Result<String, Box<dyn std::error::Error>> {
    let auth_url = format!("{}/api/v1/connect", SNAPMAKER_ENDPOINT);
    let client = reqwest::Client::new();

    // Try to read existing token
    if let Ok(token) = fs::read_to_string(TOKEN_FILE) {
        let form_data = [("token", token)];
        match client.post(&auth_url).form(&form_data).send().await {
            Ok(response) => {
                let status = response.status();
                if status.is_success() {
                    if let Ok(body) = response.text().await {
                        if let Ok(json_response) =
                            serde_json::from_str::<SnapmakerTokenResponse>(&body)
                        {
                            let new_token = json_response.token;
                            // Save the new token
                            info!("Obtained refresh token");
                            fs::write(TOKEN_FILE, &new_token)?;
                            return Ok(new_token);
                        }
                    }
                }
            }
            Err(e) => {
                error!("Error using existing token: {}", e);
            }
        }
    }

    // If we get here, we need to request a new token
    println!("No valid token found. Requesting new token...");
    println!("Please authorize the connection on your Snapmaker touchscreen");

    match client.post(&auth_url).send().await {
        Ok(response) => {
            let status = response.status();
            if status.is_success() {
                if let Ok(body) = response.text().await {
                    if let Ok(json_response) = serde_json::from_str::<SnapmakerTokenResponse>(&body)
                    {
                        let token = json_response.token.clone();
                        // Save the token
                        println!("Obtained authorization token");
                        fs::write(TOKEN_FILE, &token)?;
                        return Ok(token);
                    }
                }
            }
            error!("Failed to get token: {}", status);
            Err("Failed to get token".into())
        }
        Err(e) => {
            error!("Error requesting new token: {}", e);
            Err(e.into())
        }
    }
}

pub async fn upload_file_to_snapmaker(
    token: &str,
    file_path: &Path,
    filename: &str,
) -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    let file_content = fs::read(file_path)?;

    // below is the UPLOAD API, but this does not work if you want to start
    // the print straight away. So instead we use the "prepare_print" API, which
    // does not save the file (or at least I can't figure out where it is saved to)
    // let file_part = Part::bytes(file_content.clone())
    //     .file_name(filename.to_string())
    //     .mime_str("application/octet-stream")?;

    // // Create multipart form with file and additional form fields
    // let form = Form::new()
    //     .part("file", file_part)
    //     .text("token", token.to_string())
    //     .text("type", "3DP".to_string());

    // let response = client.post(&upload_url).multipart(form).send().await?;

    // if !response.status().is_success() {
    //     let status = response.status();
    //     let text = response.text().await.unwrap_or_default();
    //     anyhow::bail!("Upload to Snapmaker failed {status:?} {text}",)
    // }

    // prepare
    let upload_url = format!("{}/api/v1/prepare_print", SNAPMAKER_ENDPOINT);
    let file_part = Part::bytes(file_content)
        .file_name(filename.to_string())
        .mime_str("application/octet-stream")?;
    let form = Form::new()
        .part("file", file_part)
        .text("token", token.to_string())
        .text("type", "3DP".to_string());

    let response = client.post(&upload_url).multipart(form).send().await?;
    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        anyhow::bail!("Prepare on Snapmaker failed {status:?} {text}",)
    }

    Ok(())
}

pub async fn start_print(token: &str) -> anyhow::Result<()> {
    let url = format!("{}/api/v1/start_print?token={}", SNAPMAKER_ENDPOINT, token);
    let client = reqwest::Client::new();
    let response = client.post(&url).send().await?;

    if !response.status().is_success() {
        error!("Snapmaker start failed: {}", response.status());
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        anyhow::bail!("Start print failed {status:?} {text}",)
    }
    Ok(())
}

pub async fn get_status(token: &str) -> anyhow::Result<PrinterStatus> {
    let status_url = format!(
        "{}/api/v1/status?token={}&{}",
        SNAPMAKER_ENDPOINT,
        token,
        chrono::Utc::now().timestamp()
    );
    let client = reqwest::Client::new();

    let response = client.get(&status_url).send().await?;

    if !response.status().is_success() {
        anyhow::bail!("Keepalive request failed: {}", response.status());
    }
    let mut status: PrinterStatus = serde_json::from_str(&response.text().await?)?;
    // Snapmaker seems to report speed in mm/h ?!
    status.work_speed = status.work_speed / 60.0;
    Ok(status)
}

pub async fn get_enclosure_status(token: &str) -> anyhow::Result<EnclosureStatus> {
    let status_url = format!(
        "{}/api/v1/enclosure?token={}&{}",
        SNAPMAKER_ENDPOINT,
        token,
        chrono::Utc::now().timestamp()
    );
    let client = reqwest::Client::new();

    let response = client.get(&status_url).send().await?;

    if !response.status().is_success() {
        anyhow::bail!("Enclosure request failed: {}", response.status());
    }

    Ok(serde_json::from_str(&response.text().await?)?)
}

pub async fn set_enclosure_light(token: &str, value: u8) -> anyhow::Result<()> {
    let api_url = format!("{}/api/v1/enclosure", SNAPMAKER_ENDPOINT);
    let client = reqwest::Client::new();

    let response = client
        .post(&api_url)
        .form(&[("token", token), ("led", value.to_string().as_str())])
        .send()
        .await?;

    if !response.status().is_success() {
        anyhow::bail!("Set enclosure light failed: {}", response.status());
    }

    Ok(())
}

pub async fn set_enclosure_fan(token: &str, value: u8) -> anyhow::Result<()> {
    let api_url = format!("{}/api/v1/enclosure", SNAPMAKER_ENDPOINT);
    let client = reqwest::Client::new();

    let response = client
        .post(&api_url)
        .form(&[("token", token), ("fan", value.to_string().as_str())])
        .send()
        .await?;

    if !response.status().is_success() {
        anyhow::bail!("Set enclosure fan failed: {}", response.status());
    }

    Ok(())
}

pub async fn pause_print(token: &str) -> anyhow::Result<()> {
    let url = format!("{}/api/v1/pause_print?token={}", SNAPMAKER_ENDPOINT, token);
    let client = reqwest::Client::new();
    let response = client.post(&url).send().await?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        anyhow::bail!("Pause print failed {status:?} {text}",)
    }
    Ok(())
}

pub async fn stop_print(token: &str) -> anyhow::Result<()> {
    let url = format!("{}/api/v1/stop_print?token={}", SNAPMAKER_ENDPOINT, token);
    let client = reqwest::Client::new();
    let response = client.post(&url).send().await?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        anyhow::bail!("Stop print failed {status:?} {text}",)
    }
    Ok(())
}

pub async fn resume_print(token: &str) -> anyhow::Result<()> {
    let url = format!("{}/api/v1/resume_print?token={}", SNAPMAKER_ENDPOINT, token);
    let client = reqwest::Client::new();
    let response = client.post(&url).send().await?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        anyhow::bail!("Resume print failed {status:?} {text}",)
    }
    Ok(())
}

pub(crate) async fn keep_alive_loop(
    token: String,
    status_sender: Sender<PrinterStatus>,
) -> anyhow::Result<()> {
    loop {
        let enclosure = match get_enclosure_status(&token).await {
            Ok(x) => x,
            Err(e) => {
                error!("Error getting enclosure status {e:?}");
                EnclosureStatus::default()
            }
        };

        match get_status(&token).await {
            Ok(mut status) => {
                status.enclosure = enclosure;
                let _ = status_sender.send(status);
                info!("Updated printer status");
            }
            Err(e) => error!("Keepalive failed: {}", e),
        }
        // Sleep for 3 seconds before next keepalive
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
