# obs-plan — archived amendment originals

Writer = wrap P7 and registry.py migrate --apply · read by NO loop skill · cold history, never cited for current truth.

## Registry migration (U35) — 2026-10-05

<!-- U35 · obs-plan.md · ## 3. Observability Harness Contract · sha256 795fbbd812a3a32d11d7b35cbab8abe40d8c76a5fe6d384781f2f9acbe211b9e -->

## 3. Observability Harness Contract

**OTel SDK init:**

- observed absent — telemetry backends · searched: `sentry|opentelemetry|otel|prometheus|metrics` over .github/workflows/*.yml, Cargo.toml, Cross.toml, flake.nix, .github/scripts/*.py

**Logging stack (subscriber installation):**

- Native subscribers are installed with `tracing_subscriber::fmt::init()` under the `tracing` feature (apps/browser/src/main.rs:73-74; apps/readme/src/main.rs:61-62; examples/todomvc/src/main.rs:16-17)
- wasm_hello installs `tracing_wasm::set_as_global_default()` (examples/wasm_hello/src/lib.rs:105)
- The wpt runner logs through the `log` facade via `env_logger` (wpt/runner/src/main.rs:458; wpt/runner/src/main.rs:784-830)
- observed absent — tracing subscriber or exporter setup · searched: `subscriber|opentelemetry|sentry` over the 15 s05 files
- observed absent — a tracing subscriber or exporter setup · searched: `tracing_subscriber|subscriber` over the 17 listed s06 files
- observed absent — a tracing subscriber or exporter set up in these crates · searched: `subscriber` over the 21 listed s11 files

**Embedder drain of script diagnostics:**

- Embedders drain JS errors with `take_js_errors` and JS messages with `take_messages` (packages/blitz-vibey-script/src/document.rs:242-261)
- At most 256 errors are retained between drains (packages/blitz-vibey-script/src/state.rs:101-103; packages/blitz-vibey-script/src/document.rs:256)

> NOT YET MEASURED — product mode, service identity, log format JSON schema, log file location, snapshot integration, trace context propagation and heartbeat ticks: the reading recorded none of them

### Bootstrap phases (derive for route / setup-project)

- **otel-sdk-install:** no OTel SDK or telemetry backend is present — recorded absent in §2 Telemetry Strategy and §3 Observability Harness Contract (OTel SDK init).
- **pii-scrubbing-wire:** no redaction or scrubbing of logged values is present — recorded absent in §8 PII Scrubbing & Compliance.

---
