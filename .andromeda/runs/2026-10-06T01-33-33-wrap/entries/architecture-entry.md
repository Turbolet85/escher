
## 2026-10-06-telemetry-bootstrap — escher-telemetry registered; logging pattern and tracing gating rescoped
**Section:** §Stack and Technologies (Parallelism and misc · Testing) · §Conventions → Feature gating · §Standard Contracts · §Occupied Resources (Process-wide state and threads · Environment variables · Names) · §Infrastructure Patterns → Directory structure · §Cross-cutting Patterns → Logging and timing · Inherited Defaults (Publishability · Optional capabilities) · §Existing Scopes · every section citing `Cargo.toml`, `tests/blitz-tests/Cargo.toml`, `examples/seven_guis/Cargo.toml` or `examples/seven_guis/src/main.rs` lines
**Change:**
- New crate `escher-telemetry` (`publish = false`, no features, workspace member + `[workspace.dependencies]` path entry) in Names, the `packages/` tree, Existing Scopes (modules format, panic under `init`) and Publishability; the published-`packages/` clause now excludes blitz-test-harness and escher-telemetry.
- Stack: `tracing-log "0.2"` beside tracing / tracing-subscriber; blitz-tests dev-deps gain escher-telemetry, tracing, tracing-log.
- `RUST_LOG` registered (escher_telemetry's `EnvFilter`, default `warn`; also the upstream `fmt::init()` installs).
- Process-wide state: in `seven_guis_native` the global subscriber, the `LogTracer` bridge, a chaining panic hook, `OnceLock<ServiceIdentity>` + `Mutex<()>` statics, log targets `escher_telemetry` / `escher_telemetry::panic`; no thread.
- Standard Contracts: the telemetry bootstrap — `init` / `init_with_writer`, `ServiceIdentity` + `service_identity!()`, `InitOutcome`, `InitError::ForeignSubscriber`, the stderr line shape, the allowlist scrub, the panic event.
- Logging and timing: was "`fmt::init()` runs only under the `tracing` feature" as the subscriber pattern; now escher binaries install `escher_telemetry::init` (stderr, identity, scrub, log bridge, chaining hook) and the upstream `fmt::init()` installs stay feature-gated, stdout, unscrubbed.
- Feature gating / Optional capabilities: was "tracing gated per call site" workspace-wide; now the engine and upstream crates' rule, escher-telemetry ungated.
- 62 `file:line` citations re-pointed by the chunk's measured line shifts.
**Why:** the telemetry bootstrap chunk added the crate, its env read and process-wide installs, and adopted it in the stand; the per-call-site gate stays the engine crates' rule.
**Ref:** .andromeda/runs/2026-10-06T01-33-33-wrap/
