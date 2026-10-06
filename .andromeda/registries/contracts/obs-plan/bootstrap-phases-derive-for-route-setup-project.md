### Bootstrap phases (derive for route / setup-project)

- **otel-sdk-install:** no OTel SDK or telemetry backend is present — recorded absent in §2 Telemetry Strategy and §3 Observability Harness Contract (OTel SDK init); the opt-in export was deferred when the telemetry bootstrap landed (an egress and `OTEL_EXPORTER_OTLP_HEADERS` credential-path decision for the founder) and is carried to "Driver command spans"; the sink's `service.name` / `service.version` already use the OTel resource keys (§3).
- **pii-scrubbing-wire:** discharged for escher's own sink — escher-telemetry's allowlist scrub in `seven_guis_native` (§8 PII Scrubbing & Compliance → Scrubbing); still open for the upstream apps' `fmt::init()` stdout subscribers and the WPT runner's `env_logger`, which stay unscrubbed.

---
