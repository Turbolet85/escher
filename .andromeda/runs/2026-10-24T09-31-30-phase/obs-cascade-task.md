# Task — obs · 2026-10-24-upstream-sync-polish-and-ship

## The chunk
Takes upstream DioxusLabs/blitz past the last sync. The take is either a whole merge at a measured sha or cherry-picks of the engine-code commits onto the registry pins, and which one is a P4 fork.
The engine-code commits reach blitz-dom (inline layout split, class atoms, out-of-flow flag, style traversal in config.rs, aspect-ratio, animation lookups, dropped nodes skipped in hit testing, Stylo keywords). The pin commits move git Taffy, git Stylo 0.23-pre and Dioxus 0.8.0-alpha.3, and they sync dioxus-native and dioxus-native-dom from the Dioxus repo, mutation_writer.rs and a new mutation_buffer.rs included.
Fixes the red Dependency audit with a semver-compatible tokio update. Re-reads the per-OS test tallies, and re-checks escher's sink allowlist against every new upstream log site, print and env read.
Adds no escher code, crate, span, field, env var or listener of its own. Upstream's new test targets arrive with the take.

## Needs
- escher-telemetry's sink allowlist in packages/escher-telemetry/src/format.rs: engine targets print safe fields only, escher_ targets print with content-named fields redacted, and every other target is dropped whole. The entry asks for this to be re-checked against the take's tree.
- Records from moved dependencies: a new Stylo, Taffy or Dioxus version may emit under targets outside the allowlist. Those records are dropped whole at every level, and the drop check telemetry_drop still holds.
- The engine target prefix set, re-read in case the take adds or renames a module that logs. dioxus-native-dom's new mutation_buffer.rs and the split inline layout files fall under existing crate targets.
- The rule that engine telemetry sits behind each crate's tracing feature, as cfg-gated call sites with a no-op fallback and never an unconditional print. Every upstream hunk the take carries into blitz-dom, dioxus-native-dom and dioxus-native must follow this rule.
- blitz-dom's existing feature-gated log sites and the file:line citations of their logged-as-is values. Upstream hunks move those lines in document.rs, mutator.rs, layout/construct.rs, events/pointer.rs and node/element.rs, and construct.rs and mutator.rs are predicted conflicts. A resolution must keep each site gated, and moved citations get re-pointed.
- dioxus-native-dom's mutation_writer.rs debug logs of text-node contents and attribute values, an unscrubbed-value fact. The Dioxus sync rewrites this file, so whether those log sites remain, move or change is read on the take.
- The tracing and log-times feature wiring in the Cargo.toml files of dioxus-native and dioxus-native-dom, which the Dioxus 0.8 sync touches. The tracing forward to dioxus-native-dom and the blitz crates, and the log-phase-times and log-frame-times forwards, must survive the manifest merge.
- The root manifest's telemetry dependencies (tracing, tracing-subscriber, tracing-log, env_logger, console_error_panic_hook) and blitz-dom's log-phase-times to debug_timer wiring, carried unchanged through the pin moves and the Cargo.toml and Cargo.lock conflicts.
- Engine features by runner: a workspace build turns blitz-dom/tracing on, while a -p seven_guis or -p blitz-tests build leaves it off. The capture checks run under both builds, so the merged engine's log sites reach the sink in the first build only.
- The standing capture checks host_log and mcp_log. They read the escher-session host and its mcp role at trace under both builds, and they must still read 0 id, name, typed-sentinel and shot needles, with unchanged span counts, on the take's tree.
- The silence census over upstream's new integration targets that the take carries (hit_test_dropped_nodes, stale_node_mapping): no subscriber, no env read and no print per file, as earlier syncs measured for upstream's targets.
- The upstream root example examples/multi_doc.rs, with one println! and two std::env::var reads (MULTI_DOC_N and one named by a parameter), plus a commented dbg!. Its print belongs to the examples emitter group (example output uses println!). Whether its env reads need registering against the source env-read list is read the way earlier syncs read upstream's examples.
- The clock invariant (no Instant::now or SystemTime::now in blitz-dom outside clock.rs), re-read on the take. It binds through the document clock's file set, and that set holds resolve.rs, where blitz-dom's resolve-phase timings sit.
- The in-process timing facts (blitz-dom's resolve phases, debug_timer, the log-times frame and phase timings). Their lines may move with the inline layout split and the style-traversal change, and their citations are re-read where the take moves them.
- The upstream-only apps/browser and wpt/runner, which the take changes (mimalloc as the default allocator, and the WPT runner commit). Their own logging stays as recorded (fmt::init to stdout, env_logger, the runner's panic hook). Any new log site, print or env read in their hunks is read in the delta re-check.
- The Dependency audit leg failing on fork CI and its fix by a semver-compatible tokio update. A failed leg's merged log is the only CI artifact it can produce, and that log holds no user data.

## Not needed
- escher-driver's command and tool_call spans, their eight and two fields and their closed-word rule. The chunk adds no span or field and changes no driver code, and only the capture checks' span counts are read.
- The driver's answer-only outputs and their never-logged rule: snapshot text and parts, diffs, a shot's PNG, lint findings and kept-check recordings. The chunk touches none of them.
- escher-telemetry's own startup event, panic hook, service identity and line format. The chunk does not edit the bootstrap crate.
- The agent-run harness log and the cold-agent pipe log (events.jsonl, verdict, server.log, transcript). The chunk runs neither pipeline.
- Metrics exporters, OTel SDK, SLOs and trace propagation. All are recorded absent or not measured, and a sync adds none.
- WPT score publishing to Pages and the publish-browser build logging. These run on upstream DioxusLabs/blitz only, and WPT conformance numbers are out of scope.
- blitz-vibey-script's JS error surface and its take_js_errors and take_messages drain. Nothing in the upstream delta reaches that crate.
- WASM panic hooks (console_error_panic_hook in seven_guis, todomvc, wasm_hello) and wasm_hello's tracing_wasm. The take does not touch them.
- blitz-shell and blitz-paint devtools overlays and layer counters. Stylo's manifest move touches their packages without changing these behaviours.
- The apps/browser logged events (urlbar text, visited URLs, history writes). The browser's only change in the take is the allocator.
- Local state directories (session state dir, recording dir, target/agent-run, target/cold-agent) and the two committed Counter checks. The chunk neither writes nor reads them.
- The apt-install diagnostic line and the coverage report artifact. Neither is changed by the sync.
