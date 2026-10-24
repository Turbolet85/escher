# obs · Error Capture & Reporting — category read

## Carried
- (.andromeda/obs-plan-detail/error-capture-reporting.md:8) The take must add no error-reporting client or service. Upstream's delta brings a new dependency (`mimalloc`, an allocator) and WPT-runner changes, so the take's manifests and added lines must still match no error-reporting search term.
  > - observed absent — an error-reporting client · searched: `sentry|analytics|gtag` over the 21 s02 files
- (.andromeda/obs-plan-detail/error-capture-reporting.md:11) No panic hook or `catch_unwind` may arrive in the engine crates with the take. escher-telemetry's chaining hook stays the only escher-side hook.
  > - observed absent — an error reporting service or panic hook · searched: `sentry|panic::set_hook|catch_unwind` over the 17 listed s06 files
- (.andromeda/obs-plan.md:204) escher-telemetry's chaining panic hook must stay unchanged by the take: one redacted ERROR event, then the previous hook, with the exit code unchanged. The sync is out of scope for escher's own crates, and `packages/escher-telemetry` is not in upstream's delta.
  > - escher-telemetry's hook, installed by `init` in `seven_guis_native` and in `escher-session`, chains: it takes the previous hook, logs one ERROR event at target `escher_telemetry::panic` with `panic.file`, `panic.line`, `panic.column` and `panic.payload` (redacted, §8), then runs the previous hook — std's default still prints the raw message to stderr and the exit code is unchanged (packages/escher-telemetry/src/panic.rs:4-24)
- (.andromeda/obs-plan.md:203) The WPT runner's panic hook, crash message and pending-item log are in reach of `f8723f495` (#1247, WPT runner) and `9eeebb302` (mimalloc in the WPT runner). Where the take carries those commits, the cited lines must be re-read and their cites moved.
  > - The wpt runner's panic hook captures message, file, line, column and a forced backtrace (wpt/runner/src/panic_backtrace.rs:12-38)
- (.andromeda/obs-plan.md:205) Same as the item above: the WPT report's error carriage is upstream-only surface, and the take re-reads its cites.
  > - A crashed WPT test's panic message is carried into the report's `message` field (wpt/runner/src/report.rs:95)
  > - `pump_net_provider` logs pending items before panicking on its 500 ms timeout (wpt/runner/src/test_runners/mod.rs:390-396)
- (.andromeda/obs-plan.md:211) blitz-dom's resource-load error capture sits in `document.rs`, which upstream's delta reaches (`06ecec886`, `3954ecb85`). After the take it must still be a feature-gated `tracing::warn!` with an `error` field, and its cite re-read.
  > - blitz-dom: resource load errors are logged as `tracing::warn!` with `error` field and not propagated further (packages/blitz-dom/src/document.rs:1438-1457)
- (.andromeda/obs-plan.md:212) The stylesheet parse calls pass no error reporter. Under a take that moves Stylo (`06ecec886`, v0.23-pre), these call sites move with Stylo's API. The reading must be re-measured on the take's tree and no error reporter added by us.
  > - blitz-dom: stylesheets are parsed with no error reporter (`None, // error_reporter`) (packages/blitz-dom/src/net.rs:166; packages/blitz-dom/src/net.rs:276; packages/blitz-dom/src/document.rs:1351)
- (.andromeda/obs-plan.md:214) The SVG parse-error capture lives in `layout/construct.rs`, one of the four predicted conflicts. Its resolution must keep the warn event and its `error` field, feature-gated, and the cite must be re-read.
  > - blitz-dom layout: SVG parse errors are captured into the `error` field of a warn event and not propagated (packages/blitz-dom/src/layout/construct.rs:479-489)
- (.andromeda/obs-plan.md:213) blitz-dom's log-and-fallback recoveries are engine code that the take may move. A recovery must stay logged under the `tracing` feature, never turned into a panic or an unconditional print.
  > - blitz-dom: recoverable failures are logged and a fallback is used, as with font decompression (packages/blitz-dom/src/util.rs:25-29)
- (.andromeda/obs-plan.md:217) `286874cc1` syncs `packages/dioxus-native(-dom)` from the Dioxus repo. Where the take carries it, dioxus-native's error reporting must still be `tracing` events only, and the cites re-read.
  > - dioxus-native: errors are reported only as `tracing` warn/error events (packages/dioxus-native/src/assets.rs:52-53; packages/dioxus-native/src/link_handler.rs:14-15)
- (.andromeda/obs-plan.md:229) The section's standing absence must still hold on the take's tree. The scope's sink re-check reads upstream's added lines for every new `tracing`/`log` site, print and env read, and an error-reporting client is among what that read must not find.
  > - Error-reporting clients and services observed absent over the reading's s02, examples, s05, s06 and s11 slices and the 16 listed files.

## Bindings
- **§7 error classes (warn/error events) ↔ obs body §8 Scrubbing**: every engine error event that the take moves or adds reaches escher's sink only under an engine target, safe fields only, and any other target is dropped whole. This is the scope's «sink allowlist re-checked» (.andromeda/obs-plan.md:211; .andromeda/obs-plan.md:253).
- **§7 escher panic hook ↔ obs body §6 Logged events**: the hook's ERROR event is the logged `escher_telemetry::panic` line, and both stay unchanged by the take (.andromeda/obs-plan.md:204; .andromeda/obs-plan.md:183).
- **§7 error-reporting absence ↔ security §Dependencies**: `mimalloc` is the take's one new crate. It is reviewed by hand and licence-checked, and it must bring no error-reporting client (.andromeda/obs-plan-detail/error-capture-reporting.md:8; .andromeda/obs-plan.md:229).
- **§7 Feature-gated capture ↔ obs body §2 Telemetry mechanism**: a resolved conflict in `construct.rs` or `document.rs` keeps each error log site behind `#[cfg(feature = "tracing")]` with its no-op path (.andromeda/obs-plan.md:214; .andromeda/obs-plan.md:35).
