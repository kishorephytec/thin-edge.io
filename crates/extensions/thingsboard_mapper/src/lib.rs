// Thingsboard Mapper Skeleton
// TODO: Implement the actual logic

use rumqttc::{AsyncClient, Event, MqttOptions, QoS};
use serde_json::Value;

pub struct ThingsboardMapper {
    mqtt_client: Option<AsyncClient>,
}

impl ThingsboardMapper {
    pub fn new() -> Self {
        Self { mqtt_client: None }
    }

    pub async fn start(&mut self, mqtt_host: &str, mqtt_port: u16, device_token: &str) {
        // Setup MQTT options for Thingsboard
        let mut mqttoptions = MqttOptions::new("tedge-thingsboard-mapper", mqtt_host, mqtt_port);
        mqttoptions.set_credentials(device_token, "");
        let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
        self.mqtt_client = Some(client.clone());

        // Subscribe to Thin-Edge measurement topics
        client.subscribe("te/+/+/+/+/m/+", QoS::AtLeastOnce).await.unwrap();

        // Main event loop: map and forward messages
        loop {
            match eventloop.poll().await {
                Ok(Event::Incoming(rumqttc::Packet::Publish(publish))) => {
                    // Map Thin-Edge message to Thingsboard telemetry JSON
                    if let Ok(payload) = std::str::from_utf8(&publish.payload) {
                        if let Ok(json) = serde_json::from_str::<Value>(payload) {
                            // Forward to Thingsboard telemetry topic
                            let _ = client.publish("v1/devices/me/telemetry", QoS::AtLeastOnce, false, json.to_string()).await;
                        }
                    }
                }
                Ok(_) => {}
                Err(e) => {
                    eprintln!("ThingsboardMapper error: {e}");
                    break;
                }
            }
        }
    }
}
