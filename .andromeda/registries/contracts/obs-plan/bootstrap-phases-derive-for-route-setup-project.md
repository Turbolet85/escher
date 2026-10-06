### Bootstrap phases (derive for route / setup-project)

- **otel-sdk-install:** no OTel SDK or telemetry backend is present — recorded absent in §2 Telemetry Strategy and §3 Observability Harness Contract (OTel SDK init); escher 0.1.0 ships no OTel export (ruled by the founder, 2026-10-06): no OTel crate, no egress and no `OTEL_EXPORTER_OTLP_HEADERS` credential path, with the export transport and the credential path both left undecided — a cross-version residual in `.andromeda/residuals.md`; the sink's `service.name` / `service.version` already use the OTel resource keys (§3).
- **pii-scrubbing-wire:** discharged for escher's own sink — escher-telemetry's allowlist scrub in `seven_guis_native` (§8 PII Scrubbing & Compliance → Scrubbing); still open for the upstream apps' `fmt::init()` stdout subscribers and the WPT runner's `env_logger`, which stay unscrubbed.

---
