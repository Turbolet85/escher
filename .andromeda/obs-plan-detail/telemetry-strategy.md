# obs-plan · Telemetry Strategy
Read whole. The master's broad view is .andromeda/obs-plan.md; this file holds its Telemetry Strategy detail.

## 2. Telemetry Strategy

<!-- detail: values · strategy -->

- Telemetry is the `tracing` crate's event macros and, in escher-driver, two span macros (`info_span!`, `command` and, since 2026-10-10-mcp-surface, `tool_call`, §4), in the engine and upstream crates compiled in only with the `tracing` cargo feature (packages/blitz-dom/src/util.rs:26-27; packages/blitz-dom/src/events/ime.rs:27-28); escher-telemetry depends on `tracing` with no feature and always compiles its startup and panic events into the two native binaries of the seven_guis package, `seven_guis_native` and `escher-session`, whose engine `tracing` features stay off in a package-alone build (`-p seven_guis`; a workspace-wide build turns blitz-dom's on — Feature wiring, below); escher-driver is the second escher crate to take `tracing` ungated (`{ workspace = true }`, no `[features]` table; the workspace requirement is 0.1.40, the lock holds 0.1.44): its command span is compiled into both binaries through `seven_guis → escher-driver`; since 2026-10-10-driver-cli it is reached in `escher-session` by every call its host role runs — one closed span per hosted call — and since 2026-10-10-mcp-surface by its `mcp` role, where each `tools/call` writes one closed `tool_call` span and every instance verb run through `Session::run` adds its `command` child — and still by no code path in the windowed `seven_guis_native` (packages/escher-driver/src/mcp.rs:118-121; packages/escher-telemetry/Cargo.toml:13-16; packages/escher-telemetry/src/lib.rs:161; packages/escher-telemetry/src/panic.rs:11-21; packages/escher-driver/Cargo.toml:13-18; as measured at escher-0.1.0/chunks/2026-10-07-driver-command-spans/report.md)

<!-- /detail -->

<!-- detail: values · strategy -->

- By invocation: a build that resolves the whole workspace (`cargo test --workspace`, `ci-leg.sh fast`, CI's test leg) turns `blitz-dom/tracing` on — blitz's default `tracing` reaches blitz-shell and blitz-html, and each of those forwards to blitz-dom — while `cargo test -p blitz-tests` and a `-p seven_guis` build leave it off; so under the workspace build a test child that installs escher's sink at `info` also receives engine events: one `INFO blitz_dom::document … message=[redacted]` beside a driver `type` (packages/blitz-shell/Cargo.toml:21; packages/blitz-html/Cargo.toml:15; packages/blitz-dom/src/document.rs:1872; as measured at escher-0.1.0/chunks/2026-10-07-driver-command-spans/evidence/feature-unification.md, from cargo's resolved feature graph, the event at escher-0.1.0/chunks/2026-10-07-driver-command-spans/report.md; test-plan §9 → Engine features by runner). Measured at 2026-10-10-driver-cli for the session host: on a snapshot · click · type flow the workspace-built `escher-session` host prints that one engine line, `blitz_dom::document` at INFO, from `info` up and no further line at `debug` or `trace`, and the host built alone (`-p seven_guis`) prints none (as measured at escher-0.1.0/chunks/2026-10-10-driver-cli/evidence/by-level.md); re-measured at 2026-10-14-stand-requirement-sweep on the same flow, the workspace-built host prints that engine line twice from `info` up — `trace` adding no line — and the host built alone still none (as measured at escher-0.1.0/chunks/2026-10-14-stand-requirement-sweep/evidence/log-counts.md). Measured since 2026-10-14-quality-gates: the windowed `seven_guis_native`'s stderr by level as a workspace build makes it, on the host's display, with 0 needles (§8; as measured at escher-0.1.0/chunks/2026-10-14-quality-gates/evidence/stderr-by-level.md)

<!-- /detail -->

<!-- detail: absence · strategy -->

- observed absent — telemetry backends · searched: `sentry|opentelemetry|otel|prometheus|metrics` over .github/workflows/*.yml, Cargo.toml, Cross.toml, flake.nix, .github/scripts/*.py
- observed absent — telemetry or tracing framework · searched: `tracing::|log::|env_logger` and `sentry|opentelemetry|metrics::` over the 32 slice files (examples slice)
- observed absent — telemetry or logging crates · searched: `tracing|log::|env_logger|sentry|opentelemetry|metrics` over the 61 slice files (test crate slice)
- observed absent — tracing or metrics libraries · searched: `tracing|opentelemetry|metrics|span!` (case-insensitive) over the 12 listed files (wpt runner slice)

<!-- /detail -->
