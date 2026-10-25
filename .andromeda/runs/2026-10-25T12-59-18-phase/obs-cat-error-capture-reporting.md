# obs · Error Capture & Reporting — category read

## Carried
- (.andromeda/obs-plan.md:205) The cold-agent run's server child is `escher-session mcp`, and the stand's new screens (Cells, the select, a load in flight) run under it, so a panic there must reach `server.log` only through escher-telemetry's chaining hook, with its payload redacted and the exit code unchanged; the chunk must add no panic hook of its own and no other panic capture.
  > escher-telemetry's hook, installed by `init` in `seven_guis_native` and in `escher-session`, chains: it takes the previous hook, logs one ERROR event at target `escher_telemetry::panic` with `panic.file`, `panic.line`, `panic.column` and `panic.payload` (redacted, §8), then runs the previous hook — std's default still prints the raw message to stderr and the exit code is unchanged (packages/escher-telemetry/src/panic.rs:4-24)
- (.andromeda/obs-plan.md:212) A load in flight on the stand must not count on a failed load reaching the driver as an error: a resource load error stays a warn event and is not propagated, so the stand's load either completes or stays in flight, and the driver reads it only as the load wait's busy class `loads`.
  > blitz-dom: resource load errors are logged as `tracing::warn!` with `error` field and not propagated further (packages/blitz-dom/src/document.rs:1438-1457)
- (.andromeda/obs-plan.md:216) If the P4 fork carries the stand's load through `blitz-net`, its fetch errors are logged and never reach the handler, so a failed fetch must not stand in for a load that ends; the stand boots offline today.
  > blitz-net: fetch errors in `NetProvider::fetch` are logged and not propagated to the handler (packages/blitz-net/src/lib.rs:307-319)
- (.andromeda/obs-plan.md:230) The stand extension, the cold-agent pipe's new goals and the live run must add no error-reporting client or service.
  > Error-reporting clients and services observed absent over the reading's s02, examples, s05, s06 and s11 slices and the 16 listed files.
- (.andromeda/obs-plan-detail/error-capture-reporting.md:9) The examples slice holds the stand (`examples/seven_guis`) this chunk extends, and it must still have no error capture service afterwards.
  > observed absent — error capture service · searched: `sentry|Sentry` over the 32 slice files (examples slice)
- (.andromeda/obs-plan-detail/error-capture-reporting.md:11) The stand and its new tasks must install no panic hook (`panic::set_hook`) or `catch_unwind` capture beside escher-telemetry's own.
  > observed absent — an error reporting service or panic hook · searched: `sentry|panic::set_hook|catch_unwind` over the 17 listed s06 files

## Bindings
- **escher-session's chained panic hook ↔ §8 PII Scrubbing and §6 Log Coverage (the cold-agent server log)**: the hook's `panic.payload` is redacted by the §8 scrub, and its ERROR line lands in `target/cold-agent/server.log`, which the pipe reads only for ` span="tool_call" ` lines and their endings. A server panic during the live run therefore shows up as missing `tool_call` lines and a `counts_agree` mismatch, never as a counted wrong call (.andromeda/obs-plan.md:205)
- **Load errors not propagated ↔ arch §Occupied Resources / security plan (`blitz-net` reads `file:` unrestricted, no agent URL through it)**: how the stand carries a load in flight is the P4 fork. On the obs side neither engine path reports a failed load to the driver, so the fork's stand-in must hold the request, as the fixtures' `HeldNet` does, rather than fail it (.andromeda/obs-plan.md:212; .andromeda/obs-plan.md:216)
- **No error-reporting service ↔ arch §Occupied Resources**: adding a reporting client or service would be a new dependency or crossing, which needs registration and is outside this chunk's boundaries (.andromeda/obs-plan.md:230)
