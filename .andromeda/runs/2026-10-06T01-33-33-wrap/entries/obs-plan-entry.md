
## 2026-10-06-telemetry-bootstrap — escher-telemetry: stderr subscriber, identity, line format, chaining panic hook, allowlist scrub
**Section:** §1 Instrumentation scope · §2 Telemetry mechanism · §3 Logging stack · §3 Service identity and line format · §3 → Bootstrap phases (key file) · §6 Log format · §6 Logged events · §6 Absent · §7 Panic hooks · §8 Values logged as-is · §8 Scrubbing · every section citing `Cargo.toml` or `examples/seven_guis` lines
**Change:**
- §3 Logging stack: was `fmt::init()` as the native install; now `seven_guis_native` calls `escher_telemetry::init` (Registry + `EnvFilter` from `RUST_LOG`, default `warn`, + one non-ANSI fmt layer, stderr only, `LogTracer` bridge; an `Err` is `eprintln!`ed), and `fmt::init()` is the upstream apps' stdout install.
- §3: service identity measured — `service.name` / `service.version` from the binary's `CARGO_PKG_*`; the marker keeps product mode, a JSON schema, log file, snapshot, trace context, heartbeat.
- §6: the one-line text format and `RUST_LOG` levels; events `telemetry installed` (info, `escher_telemetry`) and `panic` (ERROR, `escher_telemetry::panic`); the examples slice no longer `println!`-only.
- §2 / §1: tracing compiled only with the `tracing` feature is the engine and upstream crates' rule — escher-telemetry is ungated; §1 lists tracing-log and the escher-telemetry entity.
- §7: the chaining native hook (logs, then the previous hook; std still prints the raw message).
- §8: was "Scrubbing (absent)"; now the allowlist scrub — engine prefixes `blitz`, `dioxus_native`, `stylo_taffy`, `accesskit_xplat`, `debug_timer`, `js_console` print only `node_id`, `status`, `waiting_nodes`, `property`, `log.module_path`, `log.file`, `log.line`; `url`, `href`, `src`, `html`, `text`, `value`, `attrs`, `path`, `request`, `error`, `panic.payload` redacted everywhere — reaching escher's sink only; the slice search records kept; residue as-is: the raw panic message, `log.file` host paths.
- Bootstrap phases: `pii-scrubbing-wire` discharged for escher's sink, open for the upstream sinks; `otel-sdk-install` open, the opt-in export carried to "Driver command spans".
- 7 `file:line` citations re-pointed.
**Why:** the telemetry bootstrap chunk shipped the bootstrap; OTel export was deferred at P4 by the overseer delegate under the founder's standing delegation of technical forks (egress + credential path), provisional on the founder's word.
**Ref:** .andromeda/runs/2026-10-06T01-33-33-wrap/
