
use std::borrow::Cow;
use std::time::Duration;
use camino::Utf8PathBuf;
use tedge_config::models::{HostPort, MQTT_TLS_PORT, TopicPrefix};
use tedge_config::tedge_toml::ProfileName;
use crate::bridge::{BridgeConfig, BridgeLocation};
use tedge_api::mqtt_topics::{MqttSchema, EntityTopicId, Channel};

#[derive(Debug)]
pub struct BridgeConfigThingsboardParams {
    pub mqtt_host: HostPort<MQTT_TLS_PORT>,
    pub config_file: Cow<'static, str>,
    pub bridge_root_cert_path: Utf8PathBuf,
    pub remote_clientid: String,
    pub bridge_certfile: Utf8PathBuf,
    pub bridge_keyfile: Utf8PathBuf,
    pub bridge_location: BridgeLocation,
    pub topic_prefix: TopicPrefix,
    pub profile_name: Option<ProfileName>,
    pub mqtt_schema: MqttSchema,
    pub keepalive_interval: Duration,
    pub proxy: Option<rumqttc::Proxy>,
}

impl From<BridgeConfigThingsboardParams> for BridgeConfig {
    fn from(params: BridgeConfigThingsboardParams) -> Self {
        let BridgeConfigThingsboardParams {
            mqtt_host,
            config_file,
            bridge_root_cert_path,
            remote_clientid,
            bridge_certfile,
            bridge_keyfile,
            bridge_location,
            topic_prefix,
            profile_name,
            mqtt_schema,
            keepalive_interval,
            proxy,
        } = params;

        let service_name = format!("mosquitto-{}-bridge", topic_prefix);
        let health = mqtt_schema.topic_for(
            &EntityTopicId::default_main_service(&service_name).unwrap(),
            &Channel::Health,
        );

        // Thingsboard topics: forward all telemetry and attributes
        let pub_msg_topic = format!("te/+/+/+/+/m/+ out 1 {}/", topic_prefix);
        let attr_msg_topic = format!("te/+/+/+/+/a/+ out 1 {}/", topic_prefix);
        let event_msg_topic = format!("te/+/+/+/+/e/+ out 1 {}/", topic_prefix);
        let health_topic = format!("{} in 1 {}/", health.name, topic_prefix);

        Self {
            cloud_name: "thingsboard".into(),
            config_file,
            connection: if let Some(profile) = &profile_name {
                format!("edge_to_thingsboard@{profile}")
            } else {
                "edge_to_thingsboard".into()
            },
            address: mqtt_host,
            remote_username: None,
            remote_password: None,
            bridge_root_cert_path,
            remote_clientid: remote_clientid.clone(),
            local_clientid: "Thingsboard".into(),
            bridge_certfile,
            bridge_keyfile,
            bridge_location,
            use_mapper: true,
            use_agent: false,
            try_private: false,
            start_type: "automatic".into(),
            clean_session: true,
            include_local_clean_session: false,
            local_clean_session: false,
            notifications: true,
            notifications_local_only: true,
            notification_topic: health.name,
            bridge_attempt_unsubscribe: false,
            topics: vec![pub_msg_topic, attr_msg_topic, event_msg_topic, health_topic],
            connection_check_attempts: 3,
            auth_type: tedge_config::models::auth_method::AuthType::Certificate,
            mosquitto_version: None,
            keepalive_interval,
            proxy: proxy.map(crate::bridge::config::ProxyWrapper),
        }
    }
}
