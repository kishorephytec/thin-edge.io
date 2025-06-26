// Thingsboard connection logic stub
// TODO: Implement actual connection check and logic


use tedge_config::TEdgeConfig;
use tedge_config::tedge_toml::ProfileName;
use crate::cli::connect::command::DeviceStatus;
use crate::cli::connect::ConnectError;
use anyhow::Result;
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS, Transport, TlsConfiguration};
use std::time::Duration;
use tokio::time::timeout;
use std::fs;
use serde_json::json;
use reqwest::Client;


pub async fn check_device_status_thingsboard(
    tedge_config: &TEdgeConfig,
    profile: Option<&ProfileName>,
) -> Result<DeviceStatus, ConnectError> {
    println!("[DEBUG] Entered check_device_status_thingsboard");
    let client_id = "localPC"; // Use the CN from your cert
    let mut mqtt_options = tedge_config
        .mqtt_config()?
        .with_session_name(client_id)
        .rumqttc_options()?;
    mqtt_options.set_keep_alive(Duration::from_secs(10));

    let (client, mut event_loop) = AsyncClient::new(mqtt_options, 10);
    // Try to subscribe to a Thingsboard topic
    let topic = "v1/devices/me/attributes";
    client.subscribe(topic, QoS::AtLeastOnce).await.map_err(|e| ConnectError::Custom(format!("MQTT subscribe failed: {e}")))?;

    // Wait for SUBACK or error
    let check = timeout(Duration::from_secs(5), async {
        loop {
            match event_loop.poll().await {
                Ok(Event::Incoming(Packet::SubAck(_))) => return Ok(()),
                Ok(Event::Incoming(Packet::ConnAck(_))) => {},
                Ok(_) => {},
                Err(e) => return Err(e),
            }
        }
    }).await;

    match check {
        Ok(Ok(())) => Ok(DeviceStatus::Connected),
        Ok(Err(e)) => Err(ConnectError::Custom(format!("MQTT error: {e}"))),
        Err(_) => Err(ConnectError::Custom("MQTT connection check timed out".to_string())),
    }
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
