
## 2026-10-06-telemetry-bootstrap — escher's log sink scrubs; logging-redaction-wire discharged for it; RUST_LOG read
**Section:** Data Protection (At rest) · Bootstrap phases · Secret Management (Environment values read) · Logging & Monitoring (Log format and backends) · every section citing `Cargo.toml` or `tests/blitz-tests/Cargo.toml` lines
**Change:**
- `logging-redaction-wire`: was recorded absent; now discharged for escher's own sink (`escher_telemetry::init`'s allowlist scrub in `seven_guis_native`), still open for the upstream `fmt::init()` stdout subscribers and the WPT runner's `env_logger`.
- Logging & Monitoring gains escher's sink: stderr only; engine targets (`blitz`, `dioxus_native`, `stylo_taffy`, `accesskit_xplat`, `debug_timer`, `js_console`) print only `node_id`, `status`, `waiting_nodes`, `property`, `log.module_path`, `log.file`, `log.line`; `url`, `href`, `src`, `html`, `text`, `value`, `attrs`, `path`, `request`, `error`, `panic.payload` redacted at any target; its reach is that sink only — the std panic hook's raw message and `log.file` host paths stay as-is; the examples' `println!`-only bullet names `seven_guis_native`'s sink.
- Data Protection: the s05 redaction-absent search stands, qualified with escher's sink.
- Environment values read gains `RUST_LOG` (not secret).
- 3 `file:line` citations re-pointed by the chunk's measured line shifts.
**Why:** the telemetry bootstrap chunk wired the scrub at escher's subscriber; the opt-in OTel export (egress + the `OTEL_EXPORTER_OTLP_HEADERS` credential path) was deferred at P4 by the overseer delegate under the founder's standing delegation of technical forks, provisional on the founder's word, and adds no surface here.
**Ref:** .andromeda/runs/2026-10-06T01-33-33-wrap/
