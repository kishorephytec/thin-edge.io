// Thingsboard connection logic stub
// TODO: Implement actual connection check and logic

use tedge_config::TEdgeConfig;
use tedge_config::tedge_toml::ProfileName;
use crate::cli::connect::command::DeviceStatus;
use crate::cli::connect::ConnectError;
use reqwest::Client;
use anyhow::Result;
use serde_json::json;
use std::env;

pub async fn check_device_status_thingsboard(
    _tedge_config: &TEdgeConfig,
    _profile: Option<&ProfileName>,
) -> Result<DeviceStatus, ConnectError> {
    // TODO: Implement actual connection check for Thingsboard
    Ok(DeviceStatus::Unknown)
}

/// Check if the device is connected to Thingsboard by querying the device API.
/// If not, try to onboard (register) the device using the REST API.
pub async fn connect_and_onboard_thingsboard(
    tb_url: &str,
    device_name: &str,
    access_token: &str,
    device_type: Option<&str>,
) -> Result<DeviceStatus, ConnectError> {
    let client = Client::new();
    let device_url = format!("{}/api/device?deviceName={}", tb_url, device_name);
    let res = client
        .get(&device_url)
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(|e| ConnectError::Custom(format!("Failed to query device: {e}")))?;

    if res.status().is_success() {
        // Device exists
        return Ok(DeviceStatus::Connected);
    }

    // Device does not exist, try to create (onboard) it
    let create_url = format!("{}/api/device", tb_url);
    let payload = json!({
        "name": device_name,
        "type": device_type.unwrap_or("default"),
    });
    let create_res = client
        .post(&create_url)
        .bearer_auth(access_token)
        .json(&payload)
        .send()
        .await
        .map_err(|e| ConnectError::Custom(format!("Failed to onboard device: {e}")))?;

    if create_res.status().is_success() {
        Ok(DeviceStatus::Connected)
    } else {
        Err(ConnectError::Custom(format!(
            "Failed to onboard device: {}",
            create_res.status()
        )))
    }
}
