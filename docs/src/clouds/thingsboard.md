# Thingsboard Cloud Integration for Thin-Edge.io

## Overview
This document describes the integration of Thingsboard as a supported cloud endpoint in Thin-Edge.io. It covers configuration, certificate management, connection logic, and mapper details.

## Configuration
Add a Thingsboard section to your `tedge.toml`:

```toml
[thingsboard.default]
url = "YOUR_THINGSBOARD_URL"
root_cert_path = "/etc/tedge/thingsboard-trusted-root-certificates.pem"

[thingsboard.default.device]
id = "YOUR_DEVICE_ID"
key_path = "/etc/tedge/device-certs/tedge-private-key.pem"
cert_path = "/etc/tedge/device-certs/tedge-certificate.pem"
csr_path = "/etc/tedge/device-certs/tedge.csr"

[thingsboard.default.mapper]
timestamp = true
timestamp_format = "unix"

[thingsboard.default.bridge]
topic_prefix = "tb"
keepalive_interval = "60s"

[thingsboard.default]
topics = "te/+/+/+/+/m/+,te/+/+/+/+/e/+,te/+/+/+/+/a/+,te/+/+/+/+/status/health"
```

## Certificate Management
- Device certificates can be generated and uploaded to Thingsboard using the CLI.
- The upload logic uses the Thingsboard REST API for certificate registration.

## Connection Logic
- The connection check logic queries the Thingsboard device API to verify connectivity.

## Mapper
- The Thingsboard mapper subscribes to Thin-Edge measurement topics and forwards mapped telemetry to Thingsboard via MQTT.

## System Service
- The mapper can be managed as a systemd service: `tedge-mapper-thingsboard.service`.

## Testing
- Ensure the Thingsboard feature is enabled in your build.
- Use the CLI to connect, upload certificates, and verify data flow to Thingsboard.

## Example
```sh
sudo tedge connect thingsboard
sudo systemctl start tedge-mapper-thingsboard
```

## See Also
- [Thingsboard Documentation](https://thingsboard.io/docs/)
- [Thin-Edge.io Documentation](https://thin-edge.io/docs/)
