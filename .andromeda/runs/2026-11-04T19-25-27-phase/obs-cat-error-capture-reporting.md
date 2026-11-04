@carried n=1 label=obs-plan fact=fct-rcgwsxfj
why: The MountedData panic's reading must state what a panic meets in the windowed path, matching the chained hook as it stands, and the probe must leave that hook and its redaction exactly as they are.
quote: escher-telemetry's hook, installed by `init` in `seven_guis_native` and in `escher-session`, chains: it takes the previous hook, logs one ERROR event at target `escher_telemetry::panic` with `panic.file`, `panic.line`, `panic.column` and `panic.payload` (redacted, §8), then runs the previous hook — std's default still prints the raw message to stderr and the exit code is unchanged (packages/escher-telemetry/src/panic.rs:4-24)
@carried n=2 label=obs-plan/error-capture-reporting fact=fct-dih52bkv
why: The probe's measurement of the panic must add no panic hook, error-reporting service or product-side `catch_unwind`, so this observed absence still holds after the chunk.
quote: observed absent — an error reporting service or panic hook · searched: `sentry|panic::set_hook|catch_unwind` over the 17 listed s06 files
@carried n=3 label=obs-plan fact=fct-effx6n7y
why: The probe reads the dioxus-native bridge and must add no error class or report path to it, so dioxus-native errors still go out only as these events.
quote: dioxus-native: errors are reported only as `tracing` warn/error events (packages/dioxus-native/src/assets.rs:52-53; packages/dioxus-native/src/link_handler.rs:14-15)
@carried n=4 label=obs-plan fact=fct-zwruciiy
why: The probe and its reading for the founder must bring in no error-reporting client or service.
quote: Error-reporting clients and services observed absent over the reading's s02, examples, s05, s06 and s11 slices and the 16 listed files.
@binding n=1
**Panic hook ↔ the body's §6 logged events**: if the MountedData panic is reached under `seven_guis_native` or `escher-session`, it must come out as the one ERROR event at `escher_telemetry::panic` with its payload redacted, while std's raw message still goes to stderr (obs-plan fct-rcgwsxfj; obs-plan fct-6pasqyd4). The headless stand installs no sink, so the headless measurement goes through std's default hook alone (obs-plan fct-icom2cq7). The reading must name which path each verdict was measured on.
@binding n=2
**Panic-hook absence ↔ the body's §7 Absent**: the probe is test or example code that measures. It must add no `panic::set_hook`, no product `catch_unwind` and no reporting client, so the category's absence and the body's absence both still hold (obs-plan/error-capture-reporting fct-dih52bkv; obs-plan fct-zwruciiy).
@binding n=3
**Panic capture ↔ the founder's fork on the fix**: a MountedData fix is an engine change for every document and is left to the founder. Whatever he chooses must keep the chained hook and its redacted payload as they are (obs-plan fct-rcgwsxfj).
