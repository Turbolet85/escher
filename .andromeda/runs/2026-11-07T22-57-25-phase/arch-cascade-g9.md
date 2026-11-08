@carried n=1 label=architecture/occupied-resources fact=fct-crxfkvwh
why: the graph read must cross only the ratified session socket and mcp role, within the wire's bounds, carrying no `NodeId` and adding no second listener or port.
quote: serving three requests at wire version `v3`, a call's replies including the fixed `recording-failed` — `hello`, `stop` and, since 2026-10-10-driver-cli, `call` — so a call's element id and typed text cross in and an answer's ids, accessible names, control values (a password's or a file input's as the fixed mask), snapshot text and diff cross out, no `NodeId` among them, a request line at most 16,384 bytes and an answer at most 1,048,576; still no second served listener;
@carried n=1 run=2 label=architecture fact=fct-uu6lok3l
quote: one local listener, the driver session's Unix-domain socket `{state_dir}/session.sock` (`0600`, in a `0700` state directory), unix only; the `escher-session mcp` role speaks over its own stdio, with no listener; still no second served listener.
@carried n=2 label=architecture/occupied-resources fact=fct-xzzhfdyl
why: each new seven_guis or blitz-tests session check the plan adds for the graph read must take its own state directory under `target/tmp/` and be recorded in this list.
quote: the session checks' own state directories are `ss-life`, `ss-idle` and `ss-quiet` (blitz-tests) and `hb-*`, `hl-trace`, `hl-client`, `hl-client-rec`, `hl-cells`, `hl-popup`, `cc-*`, `cf-*` and `ht-timer` (seven_guis: `host_binary`, `host_log`, `cli_commands`, `cli_flow`, `host_timer`; the select list's two `cc-popup` and `hl-popup`, `mcp_log`'s select list test recording nothing and no `ml-popup` existing: examples/seven_guis/tests/cli_commands.rs:442; examples/seven_guis/tests/host_log.rs:296) under `CARGO_TARGET_TMPDIR`, i.e. `target/tmp/`
@carried n=3 label=architecture/occupied-resources fact=fct-p7m5lrau
why: the graph read's recorded step must keep the recording directory unnamed by the new verb, its arguments and its result fields.
quote: The recording directory: a path the launcher names by `--record <dir>` on `start`, the host role or the `mcp` role (`Session::recording_into` in process) — never an env read or a default, and named by no tool call, verb, argument or result field —
@carried n=3 run=2 label=architecture fact=fct-kjd5y7c4
quote: The recording directory is a path the launcher names by `--record <dir>`, never an env read or a default and named by no tool call, created `0700` and refused when open to its group or others;
@carried n=4 label=architecture/occupied-resources fact=fct-bw7ybeh6
why: the new «reacts to a click» field moves the committed Counter checks' snapshot text and diffs, so they must be re-recorded by `record_checks.sh` and still compare byte for byte with a fresh recording.
quote: two kept checks of the repository's own scripted stand-Counter sessions, one through the command line and one through the `mcp` role, holding the Counter's ids, names, values, snapshot text and diffs and the scripts' calls, no typed text, host path, name or secret and no `screenshot` step (a shot's PNG bytes differ by OS, as measured at CI#39225375577); written as the binary records them by `examples/seven_guis/tests/flows/record_checks.sh` (POSIX `sh`, the binary and an existing output directory as its two arguments, its scratch `rec/` made `mkdir -m 700`, printing only `step N` on failure), read by `stand_replay` through `Check::from_bytes`, compared byte for byte with a fresh recording by seven_guis `recording` in `rc-fresh` under `CARGO_TARGET_TMPDIR`
@carried n=5 label=architecture/occupied-resources fact=fct-drud2scu
why: the graph read's command and tool_call spans must keep the zero id and name needles the host and mcp log checks measure, now with no component, handler or state name on them.
quote: log target `escher_driver`, under which escher-driver's command span is emitted at INFO since 2026-10-07-driver-command-spans and its `tool_call` span since 2026-10-10-mcp-surface
@carried n=6 label=architecture/occupied-resources fact=fct-drud2scu
why: link 2 reads handlers from the vdom's dynamic attributes beside the bridge's per-kind handler counts, which the reader must leave as they count.
quote: 64 event-handler count slots indexed by `DomEventKind` (packages/dioxus-native-dom/src/mutation_writer.rs:22-23; packages/dioxus-native-dom/src/mutation_writer.rs:36)
@carried n=7 label=architecture/occupied-resources fact=fct-jzxi3spj
why: the never-logged rule over the graph's content must be witnessed by the two log checks' needles, which must cover what the graph read carries.
quote: The eight seven_guis checks' shared module `examples/seven_guis/tests/common/mod.rs` also holds what the two log checks read stderr with, stated once for `host_log` and `mcp_log` alone: `Needles` and `stand_needles` (the CRUD stand's id and name needles, booted in process through `stand::boot`),
@carried n=8 label=architecture/occupied-resources fact=fct-mzhbqeot
why: a dependency or feature the graph read adds to escher-driver must be recorded in this dependency set, and its MCP tool must sit under the server name `escher`.
quote: escher-driver, a `publish = false` library with no features of its own and no binary target, workspace member and `[workspace.dependencies]` path entry, depending on blitz-test-harness, on dioxus-native-dom with its `accessibility` feature named, since 2026-10-07-driver-command-spans, on `tracing`, since 2026-10-10-mcp-surface, on `serde_json` and, since 2026-10-12-headless-screenshot, on `png` (the workspace's, each with no feature),
@carried n=8 run=2 label=architecture fact=fct-x4u4skbo
quote: the MCP server name `escher`, allowed as `mcp__escher`.
@carried n=9 label=architecture/cross-cutting-patterns fact=fct-dw3hon6z
why: the graph reader and the new snapshot field must sit behind the `accessibility` gate of dioxus-native-dom's snapshot modules.
quote: `accessibility` also to `dioxus-native-dom/accessibility`, which enables dioxus-native-dom's own optional `accesskit` and gates its `accessibility_tree` override and its `snapshot`, `snapshot_diff`, `snapshot_text` and `actionable` modules
@carried n=9 run=2 label=architecture fact=fct-nmnb5xux
quote: optional capabilities are compile-time Cargo features forwarded down the crates, `accessibility` also gating the dioxus-native-dom snapshot modules;
@carried n=10 label=architecture/cross-cutting-patterns fact=fct-4onw7te5
why: the graph read's spans must stay ungated and hand-built with no subscriber installed, so nothing of the graph reaches a sink.
quote: escher-driver emits its command span and, since 2026-10-10-mcp-surface, its `tool_call` span through `tracing` ungated and installs no subscriber (packages/escher-driver/src/execute.rs:97-112)
@carried n=10 run=2 label=architecture fact=fct-wkx6y4nz
quote: escher-driver emits its spans ungated and installs no subscriber;
@carried n=11 label=architecture/cross-cutting-patterns fact=fct-4onw7te5
why: the never-logged rule leans on the sink dropping foreign targets and scrubbing what prints, and the graph must add no field it must print.
quote: drops every record whose target is outside the engine and escher prefix sets (`ENGINE_TARGET_PREFIXES`, `ESCHER_TARGET_PREFIXES`) at every level and whatever `RUST_LOG` names, scrubs the records that print — a closed span's fields by the rule that judges an event's — and chains the panic hook
@carried n=12 label=architecture/cross-cutting-patterns fact=fct-dsg4uyo4
why: the signal registry an app hands its signals to is a wrapper hook or context, so it must follow the Dioxus root-context and `use_context_provider` / `use_context` wiring.
quote: dioxus services are provided as root contexts — net provider, HTML parser provider, native document, window event handlers, shell provider, history, renderer, winit window (packages/dioxus-native/src/lib.rs:182-206; packages/dioxus-native/src/dioxus_application.rs:147-172) — and apps use `use_context_provider` / `use_context`
@carried n=12 run=2 label=architecture fact=fct-wrq3dhlq
quote: Dioxus services are root contexts; shared state is `Rc<RefCell<…>>`.
@carried n=13 label=architecture/cross-cutting-patterns fact=fct-klon5sda
why: link 2 must report handlers exactly, `mounted` included, against a bridge that skips zero-handler kinds and fires `mounted` after `initial_build` and each `poll`.
quote: dioxus skips event kinds with zero handlers (packages/dioxus-native-dom/src/dioxus_document.rs:356-362; packages/dioxus-native-dom/src/mutation_writer.rs:349-359); `mounted` listeners fire after `initial_build` and each `poll` (packages/dioxus-native-dom/src/mutation_writer.rs:331-336; packages/dioxus-native-dom/src/dioxus_document.rs:164-175; packages/dioxus-native-dom/src/dioxus_document.rs:239-263);
@carried n=13 run=2 label=architecture fact=fct-p37qjcmu
quote: Dioxus skips event kinds with zero handlers;
@carried n=14 label=architecture/cross-cutting-patterns fact=fct-ksdng7dt
why: the new click field rides the snapshot diff, whose reading must come from the snapshots themselves and not from the engine-touched changed set.
quote: the set is engine-touched, a superset of what a snapshot reads as changed (an attribute re-written to itself marks and changes no reading; bounds-only movement changes a reading and marks nothing);
@carried n=15 label=architecture/project-intent fact=fct-4ncs7geh
why: the registry and graph reader must work on Dioxus 0.7.3 with no fork and no git dependency.
quote: which wait for escher's next version, owned by a `residual` row in the store (target: next), so Stylo stays "0.22.0", Taffy the registry "0.15" and Dioxus "0.7.3", with no git dependency
@carried n=15 run=2 label=architecture fact=fct-7lmrueug
quote: git Stylo 0.23-pre, the six Taffy-main commits and Dioxus 0.8.0-alpha.3 wait for escher's next version, so Stylo stays "0.22.0", Taffy the registry "0.15" and Dioxus "0.7.3".
@carried n=16 label=architecture/project-intent fact=fct-ldsoar27
why: a new graph verb moves the count `VERBS` holds, so the README's «twelve» must move with it.
quote: since 2026-10-10-mcp-surface how an MCP client drives the same apps (it spawns `target/debug/escher-session mcp` as its own child and gets the driver commands as tools, the page saying twelve, the count `VERBS` holds)
@carried n=16 run=2 label=architecture fact=fct-3eyld63q
quote: one test does — the capability-statement pin reads its count of the driver commands an MCP client gets as tools —
@carried n=17 label=architecture/project-intent fact=fct-k7yo6bgj
why: the new verb and the new node field move the verb names and the verb and node-key counts the capability-statement pin holds to the schema's tables.
quote: the CI-script pin `.github/scripts/test_capability_statement.py` holds its verb and cause names and its five counts — verbs, refusal causes, session errors, node keys, exit statuses — to the driver schema's tables (§Standard Contracts → CI contracts), and no check reads its other claims;
@binding n=1
**occupied-resources ↔ project-intent**: the graph read's new verb, crossing the ratified socket and mcp role with no new listener and within the wire's bounds (architecture/occupied-resources fct-crxfkvwh), must move `VERBS`, the README's «twelve» (architecture/project-intent fct-ldsoar27) and the capability-statement pin's verb and node-key counts (architecture/project-intent fct-k7yo6bgj) together.
@binding n=2
**occupied-resources ↔ cross-cutting-patterns**: the never-logged rule over component, handler and state names must hold on the escher_driver spans (architecture/occupied-resources fct-drud2scu), stay ungated with no subscriber and scrubbed by the sink (architecture/cross-cutting-patterns fct-4onw7te5), and be witnessed by the log checks' needles (architecture/occupied-resources fct-jzxi3spj).
@binding n=3
**cross-cutting-patterns ↔ occupied-resources**: the graph reader and the click field must sit under dioxus-native-dom's `accessibility` gate (architecture/cross-cutting-patterns fct-dw3hon6z), which escher-driver reaches by naming that feature in its recorded dependency set (architecture/occupied-resources fct-mzhbqeot).
@binding n=4
**occupied-resources ↔ cross-cutting-patterns**: the click field must reach the snapshot text and diffs of the two committed Counter checks, re-recorded and compared byte for byte (architecture/occupied-resources fct-bw7ybeh6), with the diff's reading taken from the snapshots and not from the changed set (architecture/cross-cutting-patterns fct-ksdng7dt).
@binding n=5
**cross-cutting-patterns ↔ project-intent**: link 2's exact handler read, `mounted` included (architecture/cross-cutting-patterns fct-klon5sda), and the registry as a hook or context (architecture/cross-cutting-patterns fct-dsg4uyo4) must both be built on Dioxus 0.7.3 with no fork and no git dependency (architecture/project-intent fct-4ncs7geh).
