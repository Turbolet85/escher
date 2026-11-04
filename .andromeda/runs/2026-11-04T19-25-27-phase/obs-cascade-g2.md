@carried n=1 label=obs-plan/log-coverage fact=fct-gxkjmvep
why: The probe reads through dioxus-native-dom, and its verdicts must not rest on these mutation trace events, which carry page content and are on only in some builds; the probe must also add no log site beside them.
quote: Every DOM mutation (assign_node_id, create_placeholder, create_text_node, append/insert/replace, remove_node, push_root, set_node_text, load_template, set_attribute) is logged at debug through `trace!` (packages/dioxus-native-dom/src/mutation_writer.rs:141-227; packages/dioxus-native-dom/src/mutation_writer.rs:327; packages/dioxus-native-dom/src/mutation_writer.rs:410)
@carried n=1 run=2 label=obs-plan fact=fct-dxerzkok
quote: dioxus-native debug logs record text-node contents and attribute values
@carried n=2 label=obs-plan/log-coverage fact=fct-7dblu2wz
why: The chunk re-verifies the MountedData hazard in dioxus-native-dom at HEAD after the 2026-11-01 sync, which added the mounted-event guard recorded here as silent; the probe must keep that crate's silence and add no site there.
quote: no site added: the restyle narrowing, `NetHandler::error` and the failed-stylesheet path, blitz-paint's `blend` module and its blend layer, `init_stylo_prefs`, `is_inline_with_only_block_children` and its paint skip, dioxus-native-dom's mounted-event guard and the `deallocate_anonymous_block` fix add no env read, print, `tracing` site or wall-clock read (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/sink.md)
@carried n=3 label=obs-plan/error-capture-reporting fact=fct-dih52bkv
why: Re-verifying the MountedData panic must install no panic hook and no error-reporting capture; the reading must state the capture that already exists in the sink-installing binaries, not add any.
quote: observed absent — an error reporting service or panic hook · searched: `sentry|panic::set_hook|catch_unwind` over the 17 listed s06 files
@carried n=3 run=2 label=obs-plan fact=fct-rcgwsxfj
quote: escher-telemetry's hook, installed by `init` in `seven_guis_native` and in `escher-session`, chains: it takes the previous hook, logs one ERROR event at target `escher_telemetry::panic` with `panic.file`, `panic.line`, `panic.column` and `panic.payload` (redacted, §8), then runs the previous hook — std's default still prints the raw message to stderr and the exit code is unchanged (packages/escher-telemetry/src/panic.rs:4-24)
@carried n=3 run=3 label=obs-plan fact=fct-6pasqyd4
quote: ERROR at target `escher_telemetry::panic`, message `panic`, fields `panic.file`, `panic.line`, `panic.column`, `panic.payload` (redacted), once per panic (packages/escher-telemetry/src/panic.rs:11-21)
@carried n=4 label=obs-plan/pii-scrubbing-compliance fact=fct-hlttx6z4
why: The founder's reading must state that the MountedData panic's raw message reaches stderr past escher's scrub in a sink-installing binary, without the probe adding a hook to change that.
quote: Past escher's scrub: the chained std panic hook prints the raw panic message to stderr
@carried n=4 run=2 label=obs-plan fact=fct-lrj2spd7
quote: the sink's lines read 0 id, name and typed-text needles in process, in the `escher-session` host and in the windowed stand
@carried n=5 label=obs-plan/pii-scrubbing-compliance fact=fct-q64qhjfz
why: The reading's capture statement must rest on the sink redacting panic.payload under escher_telemetry::panic; the dioxus_native engine prefix printing only safe fields is why the probe's evidence must come from assertions and not from engine log lines.
quote: an event whose target — or a bridged `log` record's `log.target` — starts with `blitz`, `dioxus_native`, `stylo_taffy`, `accesskit_xplat`, `debug_timer` or `js_console` (`ENGINE_TARGET_PREFIXES`) prints only `node_id`, `status`, `waiting_nodes`, `property`, `log.module_path`, `log.file`, `log.line`, every other field (the message included) as `{name}=[redacted]`; an event whose target starts with `escher_` (`ESCHER_TARGET_PREFIXES` — the underscore is part of the prefix: `escher_telemetry`, `escher_telemetry::panic`, `escher_driver` and `escher_stand_probe` are admitted, `escher` alone is not) prints every field except `url`, `href`, `src`, `html`, `text`, `value`, `attrs`, `path`, `request`, `error` and `panic.payload`, which are redacted
@carried n=5 run=2 label=obs-plan fact=fct-cbcilbst
quote: an engine target (`ENGINE_TARGET_PREFIXES`) prints only the safe fields, the rest `[redacted]`
@binding n=1
**error-capture-reporting ↔ pii-scrubbing-compliance**: the reading must name the MountedData panic's capture as it stands. In `seven_guis_native` and `escher-session` the chained hook logs one ERROR at `escher_telemetry::panic` with `panic.payload` redacted, and std's default hook still prints the raw `RefCell already borrowed` message to stderr. The probe must add no `panic::set_hook`, no reporting service and no log line, so the s06 search keeps 0 hits (obs-plan/error-capture-reporting fct-dih52bkv) (obs-plan fct-rcgwsxfj) (obs-plan/pii-scrubbing-compliance fct-hlttx6z4) (obs-plan/pii-scrubbing-compliance fct-q64qhjfz).
@binding n=2
**log-coverage ↔ pii-scrubbing-compliance**: dioxus-native-dom's mutation_writer `trace!` sites carry text-node contents and attribute values, and under the sink the `dioxus_native` engine prefix prints only safe fields. The probe must therefore give the same verdicts whether or not engine tracing is compiled in, must read nothing from those events, and must add no log site, print or field in dioxus-native-dom. Nothing it reads (component names, handler or event names, signal values) may be logged or printed (obs-plan/log-coverage fct-gxkjmvep) (obs-plan fct-dxerzkok) (obs-plan/pii-scrubbing-compliance fct-q64qhjfz).
@binding n=3
**log-coverage ↔ the chunk's re-read of events.rs**: the hazard's coordinates (`events.rs:162`, `poll`) must be re-verified at HEAD after the 2026-11-01 sync, which brought dioxus-native-dom's mounted-event guard. The reading must state the coordinates as re-measured, with each claim marked measured or read, and the guard must stay free of any env read, print or `tracing` site (obs-plan/log-coverage fct-7dblu2wz).
