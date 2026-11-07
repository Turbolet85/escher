@carried n=1 label=obs-plan/pii-scrubbing-compliance fact=fct-hlttx6z4
why: the removed hazard was a RefCell already borrowed panic reaching this chained hook in seven_guis_native and escher-session, so the port must add no panic path or new failure path in the two render sites.
quote: Past escher's scrub: the chained std panic hook prints the raw panic message to stderr
@carried n=1 run=2 label=obs-plan fact=fct-lrj2spd7
quote: a record from a target outside the sink's allowlist never reaches stderr; the sink's lines read 0 id, name and typed-text needles in process, in the `escher-session` host and in the windowed stand, and 0 bytes of a shot in the host's and `mcp` role's stderr.
@carried n=2 label=obs-plan/pii-scrubbing-compliance fact=fct-hlttx6z4
why: the render path change reaches the escher-session host and its mcp role, so host_log and mcp_log must still pass under both builds at 0 id, name and typed-text needles at trace, and escher-driver keep its 2 info_span sites.
quote: `host_log` and `mcp_log` read 0 needles at `trace` under both builds; the census names its 2 hits, and the twelve commits and escher's companions add no env read, print, `tracing` site or wall-clock read
@carried n=3 label=obs-plan/pii-scrubbing-compliance fact=fct-yvbbn3zt
why: the two inverted pins and the other eight tests of dioxus_runtime_probe must keep this file's rule, their failure messages naming the layout mode and the outcome only and the silence census reading 0 over the file.
quote: its failure messages name a layout mode, a task or page and a link or index, never a component name tied to a node, a listener read off a node, a signal value or a node id; its reading `evidence/probe-reading.md` quotes no captured stderr, panic payload, signal value or node id, the backtrace that confirmed the hazard's panic site entering it by its coordinates alone; and it holds no print, env read, subscriber or listener, the silence census over it reading 0
@carried n=4 label=obs-plan/pii-scrubbing-compliance fact=fct-q64qhjfz
why: the replay through MutationWriter fires its debug sites exactly as the direct write did, judged at the dioxus_native engine target where only the safe fields print, so the port must add, move or widen no log site and introduce no new field name the scrub would judge, and the buffer of ids, tags, attributes and text must never be logged, fielded or printed with Debug.
quote: an event whose target — or a bridged `log` record's `log.target` — starts with `blitz`, `dioxus_native`, `stylo_taffy`, `accesskit_xplat`, `debug_timer` or `js_console` (`ENGINE_TARGET_PREFIXES`) prints only `node_id`, `status`, `waiting_nodes`, `property`, `log.module_path`, `log.file`, `log.line`, every other field (the message included) as `{name}=[redacted]`
@carried n=4 run=2 label=obs-plan fact=fct-dxerzkok
quote: dioxus-native debug logs record text-node contents and attribute values; asset logs record the full request (packages/dioxus-native-dom/src/mutation_writer.rs:172; packages/dioxus-native-dom/src/mutation_writer.rs:224; packages/dioxus-native-dom/src/mutation_writer.rs:410; packages/dioxus-native/src/assets.rs:48)
@carried n=4 run=3 label=obs-plan fact=fct-cbcilbst
quote: an engine target (`ENGINE_TARGET_PREFIXES`) prints only the safe fields, the rest `[redacted]`
@carried n=5 label=obs-plan/pii-scrubbing-compliance fact=fct-zofw4lw2
why: the new witness, the inverted pins and the re-asserted host_log and mcp_log must run silent under both the package-alone build, where engine tracing stays off, and the workspace build, where blitz-dom tracing is on, and dioxus-core's own records stay dropped by the sink.
quote: in both the engine `tracing` features stay off in a package-alone build (`-p seven_guis`), so engine events reach it only when one is turned on — which a workspace-wide build does for blitz-dom (§2 → Feature wiring); a record from any target outside the two prefix sets — Stylo's `style::*`, `selectors::matching`, `dioxus_core::*` and `dioxus_signals::*` on both binaries
@carried n=5 run=2 label=obs-plan fact=fct-bqrgb4jq
quote: engine events reach it only when an engine `tracing` feature is on, and a record from any target outside the two prefix sets is dropped
@carried n=6 label=obs-plan/ci-integration fact=fct-7467zgun
why: the buffered render must build the same document, ids, snapshot and diffs as the direct write, so stand_replay must replay the two committed checks unchanged with neither re-recorded.
quote: holding the Counter's ids, names, values, snapshot text and diffs and the scripts' calls, no typed text, host path, name, secret or `screenshot` step — read by `stand_replay` and seven_guis' freshness check
@carried n=6 run=2 label=obs-plan fact=fct-4rkn3jky
quote: two kept checks are committed in git, holding no typed text, host path or secret.
@binding n=1
**pii-scrubbing-compliance ↔ telemetry-strategy**: any log site the port adds or moves in the render path must stay behind dioxus-native-dom's tracing feature with a no-op path when it is off, and wherever it compiles in it must reach escher's sink only as an engine-target event printing the safe fields alone (obs-plan/pii-scrubbing-compliance fct-q64qhjfz) (obs-plan fct-6oskq4op)
@binding n=2
**pii-scrubbing-compliance ↔ log-coverage**: the inverted pins and the ported witness must hold the silence census at 0 over dioxus_runtime_probe and dioxus-native-dom's tests, their readings in assertions only, so the needle readings of host_log and mcp_log stay at 0 under both builds (obs-plan/pii-scrubbing-compliance fct-yvbbn3zt) (obs-plan/pii-scrubbing-compliance fct-hlttx6z4)
@binding n=3
**ci-integration ↔ pii-scrubbing-compliance**: the committed checks stand_replay replays must remain byte-unchanged, holding no typed text, host path or secret, the buffered render's identical document their witness (obs-plan/ci-integration fct-7467zgun) (obs-plan fct-4rkn3jky)
