@carried n=1 label=security-plan/error-handling fact=fct-bzel2phc
why: the probe adds no cause or session error (the scope's «nothing crosses»), so a served task driven through the session stays inside this closed, fixed-text refusal set.
quote: the cause set is closed at eight (ratified by the founder, 2026-10-18) — `unknown-verb`, `malformed`, `not-found`, `stale`, `disabled`, `covered`, `off-screen` and, since 2026-10-12-headless-screenshot, `fonts-unavailable` — each with a fixed non-empty `&'static str` name, meaning and remedy that holds no path;
@carried n=1 run=2 label=security-plan fact=fct-ugqfmfy4
quote: escher-driver's command refusal is a typed value, `Refusal`: a closed set of eight causes, each with a fixed name, meaning and remedy holding no path, and a `Fault` only beside `malformed`; no `String` field, so a refusal cannot hold anything a call supplied; a `SessionError` is written from a fixed table likewise; MCP protocol errors are fixed codes and messages with no `data`; of a refusal only its cause's fixed name reaches a log.
@carried n=2 label=security-plan/error-handling fact=fct-b44ukc3q
why: link 2's click reading must go through the listener map and never through one of these event-data conversions, which panic.
quote: twelve dioxus event-data conversions (cancel, animation, clipboard, composition, drag, image, media, selection, toggle, transition, resize, visible) call `unimplemented!()` (packages/dioxus-native-dom/src/events.rs:43-45; packages/dioxus-native-dom/src/events.rs:71-93; packages/dioxus-native-dom/src/events.rs:111-117; packages/dioxus-native-dom/src/events.rs:123-125; packages/dioxus-native-dom/src/events.rs:131-137)
@carried n=2 run=2 label=security-plan fact=fct-l3r3gdc2
quote: Unimplemented paths: Stylo hooks, twelve dioxus event-data conversions and the accesskit_xplat macOS and Windows adapters for UiKit and WinRt handles reach `todo!`, `unimplemented!` or `panic!`
@carried n=2 run=3 label=security-plan fact=fct-f4v2lnec
quote: Dioxus nodes: supported event-data conversions `unwrap` the platform-data downcast (packages/dioxus-native-dom/src/events.rs:47-129)
@carried n=3 label=security-plan/error-handling fact=fct-7ni72vcj
why: the chunk re-verifies this hazard's mechanism and its events.rs:162 coordinate at HEAD, headlessly through poll(None), and names it in the founder's reading without repairing it.
quote: Mounted handles from a spawned task: a task spawned from a key handler that writes a signal is first polled inside `render_immediate`, which holds the document's `RefCell`, so a `MountedData` call in that task panics with `RefCell already borrowed` (packages/dioxus-native-dom/src/events.rs:162), any app able to meet it — measured headlessly through the harness's `poll(None)`, the windowed shell running the same lines, read and not run
@carried n=3 run=2 label=security-plan fact=fct-c4rr3jfv
quote: `try_doc` returns `None` when the document is mutably borrowed (packages/dioxus-native-dom/src/events.rs:155-177)
@carried n=3 run=3 label=security-plan fact=fct-f4v2lnec
quote: `NodeHandle::node`/`node_mut` panic with "Node does not exist in the Document" (packages/dioxus-native-dom/src/events.rs:155-177)
@carried n=3 run=4 label=security-plan fact=fct-p5sm3bek
quote: Missing nodes in mounted-element operations return `MountedError::OperationFailed` wrapping `NodeNotExistErr` (packages/dioxus-native-dom/src/events.rs:179-193;
@carried n=4 label=security-plan/logging-monitoring fact=fct-4hqe6pzr
why: the probe adds no log field or span and logs nothing it reads, so where it runs under escher's sink the allowlist's lists stay as they are and no read value reaches a printed field.
quote: install `escher_telemetry::init`, a stderr-only subscriber whose formatter applies an allowlist of targets with three outcomes — an event whose target starts with `blitz`, `dioxus_native`, `stylo_taffy`, `accesskit_xplat`, `debug_timer` or `js_console` prints only `node_id`, `status`, `waiting_nodes`, `property`, `log.module_path`, `log.file`, `log.line`, every other field (the message included) as `[redacted]`; an event whose target starts with `escher_` prints every field except `url`, `href`, `src`, `html`, `text`, `value`, `attrs`, `path`, `request`, `error` and `panic.payload`, which are redacted; an event from any other target is not printed at all
@carried n=4 run=2 label=security-plan fact=fct-ggtnnptr
quote: escher's own sink: both seven_guis binaries install `escher_telemetry::init`, a stderr-only allowlist subscriber — an engine target only seven allowlisted fields, the rest `[redacted]`; an `escher_` target all but eleven content-named fields; any other target dropped whole at every level, spans by the same rule;
@carried n=5 label=security-plan/logging-monitoring fact=fct-4hqe6pzr
why: the probe measures a panic, and the raw panic message goes to stderr past the sink, so the probe's measured panics and assertion messages must embed nothing it reads.
quote: Its reach is that sink only: the upstream apps' `fmt::init()` stdout subscribers and the WPT runner's `env_logger` stay unscrubbed, the chained std panic hook still prints the raw panic message to stderr
@carried n=5 run=2 label=security-plan fact=fct-ggtnnptr
quote: its reach is that sink only: upstream's `fmt::init()` subscribers, `env_logger` and the panic hook stay unscrubbed
@carried n=6 label=security-plan/logging-monitoring fact=fct-uh4dhzjw
why: nothing the probe reads may be printed, so its test output keeps out of these stdout paths any component name, handler or signal value, and the probe reads no node through the walk_tree DOM dump.
quote: `walk_tree` prints a debug dump of the DOM to stdout (packages/blitz-dom/src/util.rs:90-154); debug_timer writes timings to stdout (packages/debug_timer/src/lib.rs:37-66); tests print diagnostics with `println!`/`eprintln!` (tests/blitz-tests/tests/br_trailing_line.rs:46-50; tests/blitz-tests/tests/text_selection_anonymous_block.rs:107)
@carried n=6 run=2 label=security-plan/logging-monitoring fact=fct-uh4dhzjw
quote: no line holds anything the caller typed — in the `mcp` role but the admitted request `id` it echoes — or an ESC byte
@carried n=6 run=3 label=security-plan fact=fct-eabjzys7
quote: no line holds anything the caller typed but the `mcp` role's echoed request `id`; debug dumps, timings and test diagnostics also go to stdout.
@binding n=1
**error-handling (the MountedData panic) ↔ logging-monitoring (the panic hook outside the sink)**: the probe re-measures the `RefCell already borrowed` panic, and that panic's raw message reaches stderr through the panic hook, past the allowlist; the probe's panics and assertions must therefore carry fixed text only and none of a component name, handler or signal value (security-plan/error-handling fct-7ni72vcj) (security-plan/logging-monitoring fct-4hqe6pzr)
@binding n=2
**error-handling (event-data conversions that panic) ↔ the scope's link 2 click reading**: link 2 must read `click` from the listener map and event names, never by building event data through a conversion that unwraps or reaches `unimplemented!()`, so the probe meets no panic path of its own making (security-plan/error-handling fct-b44ukc3q) (security-plan fct-f4v2lnec)
@binding n=3
**error-handling (the closed refusal and session-error set) ↔ logging-monitoring (escher's sink)**: a served task driven through the session adds no cause, session error or log field, so the only refusal text that reaches a log stays its cause's fixed name and the sink's lists stay as they are (security-plan/error-handling fct-bzel2phc) (security-plan fct-ugqfmfy4) (security-plan/logging-monitoring fct-4hqe6pzr)
@binding n=4
**logging-monitoring (stdout and test diagnostics) ↔ the founder's reading (values masked)**: what the probe prints on stdout or stderr and what the founder's reading states as evidence must both hold verdicts, counts and API names only, never a read component name, handler or signal value, and a password's or file input's value only as masked (security-plan/logging-monitoring fct-uh4dhzjw) (security-plan fct-eabjzys7)
