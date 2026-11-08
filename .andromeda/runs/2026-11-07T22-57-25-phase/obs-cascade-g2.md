@carried n=1 label=obs-plan/log-coverage fact=fct-uz2p5hye
why: the new verb's call through Session::run must close a command span whose every pair passes the scrub, its strings Debug-quoted and its counts and bools bare.
quote: every pair, `span` included, passes the §8 scrub under the span's target; a string value is Debug-quoted (`span="command"`, `message="close"`), a number or a bool bare; a span from a target outside the allowlist writes no byte at any point
@carried n=1 run=2 label=obs-plan fact=fct-dsb5vef5
quote: A closed span's line, the sink's second record class, is written for each closed span whose target the allowlist admits, and no line when a span is created, recorded to, entered or exited
@carried n=2 label=obs-plan/log-coverage fact=fct-3ogdy56e
why: the new verb in `VERBS` moves the usage line built from it, and the graph read's accepted or refused answer must be one line of JSON on stdout, stderr empty with `RUST_LOG` unset.
quote: usage — one fixed line on stderr built from `VERBS` and the six task names (`counter` · `flight-booker` · `timer` · `crud` · `cells` · `load`), stdout empty, status 2; session error — the error object on stdout, carrying its fixed `remedy` beside `kind` and `message` since 2026-10-12-self-description, and its fixed message on stderr, status 3 (the host's was 1); accepted (status 0) and refused (status 1) — one line of JSON on stdout, stderr empty with `RUST_LOG` unset
@carried n=2 run=2 label=obs-plan fact=fct-d7dwo6bi
quote: writes each command's answer, refusal or session-error object as one uncoloured line of JSON on stdout — an answer, never a log line: the sink still writes to stderr only, and the host role ends quiet
@carried n=3 label=obs-plan/log-coverage fact=fct-gwvelgmp
why: the new verb joins the verb table the command span's `verb` field reads from, and the new MCP tool the table `tool` reads from, each line keeping closed words, bools and counts only and no graph content.
quote: one closed-span line per call to `Session::run`: INFO at target `escher_driver`, `span="command"`, with the recorded ones of `verb`, `cause`, `settled`, `busy`, `passes`, `added`, `removed`, `changed`, then `message="close"`, `time.busy`, `time.idle` — the verb table's twelve words (eight instance-level; the three session-level ones and the tool-level `help` only beside the cause `unknown-verb`), eight cause names (ratified by the founder, 2026-10-18), three busy-class words, bools and counts
@carried n=3 run=2 label=obs-plan/log-coverage fact=fct-gwvelgmp
quote: `tool` (the table's word, one of its twelve, `help` among them since 2026-10-12-self-description and `screenshot` since 2026-10-12-headless-screenshot, and `lint`) and `ending` (`accepted` · `refused` · `error`)
@carried n=3 run=3 label=obs-plan fact=fct-pzih6tk6
quote: each INFO at target `escher_driver`, closed words, bools and counts only — no shot byte, lint finding, part number or text; no line at the default `warn`; `seven_guis_native` runs none.
@carried n=4 label=obs-plan/log-coverage fact=fct-gxkjmvep
why: the graph reader, signal registry and snapshot field land in dioxus-native-dom beside these content-carrying debug logs and must add no such site.
quote: Every DOM mutation (assign_node_id, create_placeholder, create_text_node, append/insert/replace, remove_node, push_root, set_node_text, load_template, set_attribute) is logged at debug through `trace!` (packages/dioxus-native-dom/src/mutation_writer.rs:141-227; packages/dioxus-native-dom/src/mutation_writer.rs:327; packages/dioxus-native-dom/src/mutation_writer.rs:410)
@carried n=4 run=2 label=obs-plan fact=fct-dxerzkok
quote: dioxus-native debug logs record text-node contents and attribute values
@carried n=5 label=obs-plan/log-coverage fact=fct-wdv62het
why: escher-driver must still hold 2 `info_span!` sites, no event and no `#[instrument]` once the new verb, outcome and tool land, a count the chunk re-asserts.
quote: escher-driver holding 2 `info_span!` sites, no event and no `#[instrument]`
@carried n=5 run=2 label=obs-plan/log-coverage fact=fct-jok55ozp
quote: escher-driver holds 2 `info_span!`, `stand_act_spans` eight fields, and `host_log` and `mcp_log` read 0 needles at `trace` under both builds
@carried n=6 label=obs-plan/pii-scrubbing-compliance fact=fct-hlttx6z4
why: with the new verb and tool exercised, the sink-installing host's and `mcp` role's stderr at trace must still read 0 id, name and typed-sentinel needles under both builds.
quote: Typed text in a HOST's log is measured since 2026-10-10-driver-cli: a hosted `type` reaches the `escher-session` host's instance, and that host's stderr at `RUST_LOG=trace` holds 0 occurrences of the typed sentinel, 0 id needles and 0 name needles under both builds (`host_log` green as `-p seven_guis` and as `--workspace`)
@carried n=6 run=2 label=obs-plan fact=fct-lrj2spd7
quote: the sink's lines read 0 id, name and typed-text needles in process, in the `escher-session` host and in the windowed stand, and 0 bytes of a shot in the host's and `mcp` role's stderr.
@carried n=7 label=obs-plan/pii-scrubbing-compliance fact=fct-yvbbn3zt
why: the chunk's graph reads the same content the probe read (rendering component, listeners, a signal's reader scopes), and no read value may leave its code or its tests' failure messages.
quote: reads each element's rendering component, its current listeners and a held signal's reader scopes through public dioxus-core, dioxus-signals and dioxus-native-dom API, and no read value leaves it: its failure messages name a layout mode, a task or page and a link or index, never a component name tied to a node, a listener read off a node, a signal value or a node id
@carried n=8 label=obs-plan/pii-scrubbing-compliance fact=fct-q64qhjfz
why: any closed span the new verb opens is judged pair by pair under the `escher_` target rule, so no graph content may become a field that rule would print.
quote: an event whose target starts with `escher_` (`ESCHER_TARGET_PREFIXES` — the underscore is part of the prefix: `escher_telemetry`, `escher_telemetry::panic`, `escher_driver` and `escher_stand_probe` are admitted, `escher` alone is not) prints every field except `url`, `href`, `src`, `html`, `text`, `value`, `attrs`, `path`, `request`, `error` and `panic.payload`, which are redacted
@carried n=8 run=2 label=obs-plan fact=fct-cbcilbst
quote: a closed span's line is judged pair by pair by the same rule; no engine call site is edited.
@carried n=9 label=obs-plan/pii-scrubbing-compliance fact=fct-zofw4lw2
why: the graph read, built on dioxus-core and dioxus-signals, must reach stderr only through the two binaries' sink, which drops those crates' records whole.
quote: a record from any target outside the two prefix sets — Stylo's `style::*`, `selectors::matching`, `dioxus_core::*` and `dioxus_signals::*` on both binaries, and on the windowed stand also `naga::*`, `wgpu_core::*`, `wgpu_hal::*`, `winit_wayland::*`, `sctk` and `calloop::*` — is dropped
@carried n=9 run=2 label=obs-plan fact=fct-bqrgb4jq
quote: Its reach is that sink, installed by the two binaries of the seven_guis package
@binding n=1
**log-coverage (command and tool_call span lines) ↔ pii-scrubbing-compliance (sink scrub)**: the new verb's `command` line and the new tool's `tool_call` line carry only the verb table's closed word, bools and counts (obs-plan/log-coverage fct-gwvelgmp), and every pair of them passes the `escher_` target rule pair by pair (obs-plan/pii-scrubbing-compliance fct-q64qhjfz); no node, component name, handler, state name or value becomes a field, so nothing rests on the scrub's content-named set to hide it.
@binding n=2
**log-coverage (escher-driver counts) ↔ pii-scrubbing-compliance (needle readings)**: after the verb, outcome and tool land, escher-driver holds 2 `info_span!` sites and no event (obs-plan/log-coverage fct-wdv62het), `stand_act_spans` its eight fields, and `host_log` and `mcp_log`, driving the graph read at trace, read 0 id, name and typed-sentinel needles under both builds (obs-plan/pii-scrubbing-compliance fct-hlttx6z4).
@binding n=3
**log-coverage (dioxus-native-dom logs) ↔ pii-scrubbing-compliance (probe content)**: the graph reader and signal registry add no log site beside `mutation_writer`'s content-carrying `trace!` calls (obs-plan/log-coverage fct-gxkjmvep), and the component, listener and signal content they read leaves only as the read's answer, never in a log, field, print or test failure message (obs-plan/pii-scrubbing-compliance fct-yvbbn3zt).
@binding n=4
**log-coverage (escher-session endings) ↔ pii-scrubbing-compliance (sink reach)**: the graph read's answer leaves as one line of JSON on stdout (obs-plan/log-coverage fct-3ogdy56e), and anything reaching stderr passes only through the two binaries' sink, which drops `dioxus_core` and `dioxus_signals` records whole (obs-plan/pii-scrubbing-compliance fct-zofw4lw2).
