
#[tokio::main]
async fn main() {
    let mut mapper = thingsboard_mapper::ThingsboardMapper::new();
    // TODO: Replace with real config or CLI args
    let mqtt_host = "localhost";
    let mqtt_port = 1883;
    let device_token = "YOUR_DEVICE_TOKEN";
    mapper.start(mqtt_host, mqtt_port, device_token).await;
}