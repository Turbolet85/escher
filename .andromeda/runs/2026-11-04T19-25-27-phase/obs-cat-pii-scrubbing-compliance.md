@carried n=1 label=obs-plan/pii-scrubbing-compliance fact=fct-q64qhjfz
why: The probe must leave escher's sink rule and its five scrub sets as they stand, add no emitter under an admitted target, and edit no engine call site.
quote: an event whose target starts with `escher_` (`ESCHER_TARGET_PREFIXES` — the underscore is part of the prefix: `escher_telemetry`, `escher_telemetry::panic`, `escher_driver` and `escher_stand_probe` are admitted, `escher` alone is not) prints every field except `url`, `href`, `src`, `html`, `text`, `value`, `attrs`, `path`, `request`, `error` and `panic.payload`, which are redacted;
@carried n=1 run=2 label=obs-plan/pii-scrubbing-compliance fact=fct-q64qhjfz
quote: the five scrub sets and the rule are unchanged
@carried n=1 run=3 label=obs-plan/pii-scrubbing-compliance fact=fct-q64qhjfz
quote: no engine call site is edited
@carried n=2 label=obs-plan/pii-scrubbing-compliance fact=fct-zofw4lw2
why: Whatever Dioxus runtime records the probe trips (component, handler, signal reads) must stay dropped by the sink, and no allowlist change may admit them.
quote: a record from any target outside the two prefix sets — Stylo's `style::*`, `selectors::matching`, `dioxus_core::*` and `dioxus_signals::*` on both binaries
@carried n=3 label=obs-plan/pii-scrubbing-compliance fact=fct-hlttx6z4
why: The MountedData panic the probe re-measures passes through the chained std hook, which prints the raw panic message to stderr past escher's scrub, so the probe's evidence must carry only the mechanism and coordinates, never a value it read.
quote: Past escher's scrub: the chained std panic hook prints the raw panic message to stderr, and the allowlisted `log.file` can still carry a host path, now only for a bridged `log` record under an engine target (`js_console`)
@carried n=3 run=2 label=obs-plan/pii-scrubbing-compliance fact=fct-hlttx6z4
quote: The sink allowlist re-read after the upstream take read at `e42ee1f25` holds: no scrub set or violation schema moves, `packages/escher-telemetry/` and `packages/escher-driver/` unchanged; `host_log` and `mcp_log` read 0 needles at `trace` under both builds; the census names its 2 hits, and the twelve commits and escher's companions add no env read, print, `tracing` site or wall-clock read
@carried n=4 label=obs-plan/pii-scrubbing-compliance fact=fct-v4kxqyim
why: Any probe code added in or beside `packages/dioxus-native-dom/` (the bridge's `dioxus_document.rs`) must hold the same zero added-lines census: no log site, print or env read.
quote: the added lines hold no log site, print or env read, the added-lines census reading 0 (packages/dioxus-native-dom/src/dioxus_document.rs:31-45;
@carried n=5 label=obs-plan fact=fct-dxerzkok
why: The bridge's debug logs already record text-node contents and attribute values as-is, so the probe must read component, handler and state links without enabling, widening or relying on those log sites.
quote: dioxus-native debug logs record text-node contents and attribute values; asset logs record the full request (packages/dioxus-native-dom/src/mutation_writer.rs:172; packages/dioxus-native-dom/src/mutation_writer.rs:224; packages/dioxus-native-dom/src/mutation_writer.rs:410; packages/dioxus-native/src/assets.rs:48)
@carried n=6 label=obs-plan fact=fct-rcgwsxfj
why: A probe check that measures the MountedData panic must keep the panic hook's shape: `panic.payload` redacted on the sink, std's raw message on stderr, and the exit code unchanged.
quote: logs one ERROR event at target `escher_telemetry::panic` with `panic.file`, `panic.line`, `panic.column` and `panic.payload` (redacted, §8), then runs the previous hook — std's default still prints the raw message to stderr and the exit code is unchanged
@carried n=7 label=obs-plan fact=fct-6pnxbanc
why: A probe check in `tests/blitz-tests/` that reads the shared fixtures must keep them silent and add no print or sink site to the readers' count.
quote: is silent: no print, `tracing` call, subscriber, env read, file, socket, listener or `blitz_net` in it, the silence census reading 0 over it
@binding n=1
**Probe values never logged ↔ obs-plan §6 Log Coverage and security §Logging & Monitoring**: the scope's «nothing a probe reads … is printed, logged or put in a field» has to meet the sink's scrub and drop rules unchanged. The probe adds no emitter, field or allowlist entry, and its checks' readings go into assertions only (obs-plan/pii-scrubbing-compliance fct-q64qhjfz) (obs-plan/pii-scrubbing-compliance fct-zofw4lw2) (obs-plan fct-6pnxbanc)
@binding n=2
**MountedData panic reading ↔ obs-plan §7 Error Capture & Reporting**: the measured panic (`RefCell already borrowed` at `events.rs:162`) leaves through the chained hook. The sink line keeps `panic.payload` redacted and std prints the raw message. The founder's reading names the mechanism and coordinates only, never a component name, handler or signal value (obs-plan fct-rcgwsxfj) (obs-plan/pii-scrubbing-compliance fct-hlttx6z4)
@binding n=3
**The founder's MountedData fork ↔ an engine change for every document**: whichever option he picks lands as an engine change. It owes the added-lines census at 0 and `host_log` and `mcp_log` at 0 needles under both builds, so each option brought to him names that cost (obs-plan/pii-scrubbing-compliance fct-v4kxqyim) (obs-plan/pii-scrubbing-compliance fct-hlttx6z4)
