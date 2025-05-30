// Thingsboard certificate upload logic stub
// TODO: Implement actual upload logic for Thingsboard

use anyhow::Result;
use reqwest::Client;
use std::fs;


/// Upload the device certificate to Thingsboard cloud.
/// cert_path: path to PEM certificate
/// key_path: path to PEM private key
/// tb_url: Thingsboard server URL (e.g. https://demo.thingsboard.io)
/// device_id: Device identifier
/// token: JWT or API token for Thingsboard
pub async fn upload_certificate_thingsboard(cert_path: &str, key_path: &str, tb_url: &str, device_id: &str, token: &str) -> Result<()> {
    let cert = fs::read_to_string(cert_path)?;
    let key = fs::read_to_string(key_path)?;
    let payload = serde_json::json!({
        "deviceId": device_id,
        "certificate": cert,
        "privateKey": key,
    });
    let url = format!("{}/api/certificates", tb_url);
    let client = Client::new();
    let res = client
        .post(&url)
        .bearer_auth(token)
        .json(&payload)
        .send()
        .await?;
    if res.status().is_success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Failed to upload certificate: {}", res.status()))
    }
}
