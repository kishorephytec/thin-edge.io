// Thingsboard connection logic stub
// TODO: Implement actual connection check and logic

use tedge_config::TEdgeConfig;
use tedge_config::tedge_toml::ProfileName;
use crate::cli::connect::command::DeviceStatus;
use crate::cli::connect::ConnectError;
use reqwest::Client;
use anyhow::Result;

pub async fn check_device_status_thingsboard(
    _tedge_config: &TEdgeConfig,
    _profile: Option<&ProfileName>,
) -> Result<DeviceStatus, ConnectError> {
    // TODO: Implement actual connection check for Thingsboard
    Ok(DeviceStatus::Unknown)
}

/// Check if the device is connected to Thingsboard by querying the device API.
/// This is a basic implementation. Adjust endpoint and logic as per your Thingsboard setup.
pub async fn check_device_status_thingsboard(tb_url: &str, device_id: &str, token: &str) -> Result<bool> {
    let url = format!("{}/api/device/{}", tb_url, device_id);
    let client = Client::new();
    let res = client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await?;
    if res.status().is_success() {
        Ok(true)
    } else {
        Ok(false)
    }
}
