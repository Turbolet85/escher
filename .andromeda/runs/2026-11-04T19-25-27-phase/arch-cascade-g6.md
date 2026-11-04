@carried n=1 label=architecture/occupied-resources fact=fct-crxfkvwh
why: The probe must add no listener, socket or port, so the session socket stays the tree's one served listener and no network port is served, the stand's load page included.
quote: no served network port — no TCP or UDP port a binary serves; one local listener — the driver session's Unix-domain socket `{state_dir}/session.sock`, set to mode `0600` inside a state directory the caller names and `escher_driver::serve` creates `0700`, bound at the tree's one served listener site
@carried n=1 run=2 label=architecture/occupied-resources fact=fct-b5fra4pv
quote: The stand's load page names one URL, `http://stand.test/load.css` (`LOAD_URL`, under the `.test` TLD no resolver answers), answered in process by the stand's `LoadNet` with the fixed `LOADED_CSS` and never reached on the network — no `blitz_net`, socket or listener (examples/seven_guis/src/stand.rs:154-175)
@carried n=1 run=3 label=architecture/occupied-resources fact=fct-qz4rtwlm
quote: The stand's load page's network stand-in `LoadNet` (`stand::boot_load`'s `net_provider`) answers each `fetch` from a thread of its own, one per fetch, sending the fixed `LOADED_CSS` after `LOAD_ANSWER_DELAY` (200 ms), with no socket, listener, print or env read
@carried n=1 run=4 label=architecture fact=fct-uu6lok3l
quote: one local listener, the driver session's Unix-domain socket `{state_dir}/session.sock` (`0600`, in a `0700` state directory), unix only; the `escher-session mcp` role speaks over its own stdio, with no listener; still no second served listener.
@carried n=2 label=architecture/occupied-resources fact=fct-7quovrlq
why: The probe's code must read and add no environment variable, so the list of variables read stays as it is.
quote: `RUST_LOG`, the tracing filter directives `escher_telemetry::init` reads through `EnvFilter::try_from_default_env`, defaulting to `warn` when unset or unparsable (packages/escher-telemetry/src/lib.rs:144)
@carried n=2 run=2 label=architecture fact=fct-yhllvhvr
quote: **Environment variables:** `RUST_LOG`, the filter `escher_telemetry::init` reads, `warn` when unset or unparsable; `WPT_DIR`; `PAINT_TREE_BENCH_HTML`; `MULTI_DOC_N`, `MULTI_DOC_SEQ` and `MULTI_DOC_FLAT`, read by the `multi_doc` example alone; `TMPDIR`, at two test-only sites (escher-driver's client unit test and blitz-vibey-script's `tests/dom.rs`); the CI script's GitHub variables.
@carried n=3 label=architecture/occupied-resources fact=fct-mzhbqeot
why: The probe must add no workspace crate and no binary name, so whatever probe test or example code it keeps lives inside the existing crates and binaries.
quote: binaries `blitz`, `seven_guis_native`, `escher-session`, `todomvc_native` — `escher-session` the second binary of the seven_guis package, sharing its name with no library crate
@carried n=3 run=2 label=architecture fact=fct-x4u4skbo
quote: **Names:** the workspace's crates, among them escher-telemetry and escher-driver, `publish = false` libraries; binaries `blitz`, `seven_guis_native`, `escher-session` and `todomvc_native`; crate features;
@carried n=4 label=architecture/occupied-resources fact=fct-xzzhfdyl
why: A probe check that runs its served task beside a lean one must add no state directory, and any session it hosts must take a caller-named directory under `CARGO_TARGET_TMPDIR`, which this list records.
quote: the session checks' own state directories are `ss-life`, `ss-idle` and `ss-quiet` (blitz-tests) and `hb-*`, `hl-trace`, `hl-client`, `hl-client-rec`, `hl-cells`, `hl-popup`, `cc-*`, `cf-*` and `ht-timer`
@carried n=4 run=2 label=architecture fact=fct-xxyhtkoq
quote: the driver session's state directory — a path its caller names, never an env read or a default, created `0700`, holding `session.sock` alone, removed at stop or idle expiry.
@carried n=5 label=architecture/occupied-resources fact=fct-bw7ybeh6
why: Probe code kept under examples/seven_guis/ must leave the stand Counter's ids, names, values, snapshot text and diffs unchanged, because the two committed checks are compared byte for byte with a fresh recording.
quote: two kept checks of the repository's own scripted stand-Counter sessions, one through the command line and one through the `mcp` role, holding the Counter's ids, names, values, snapshot text and diffs and the scripts' calls, no typed text, host path, name or secret and no `screenshot` step (a shot's PNG bytes differ by OS, as measured at CI#39225375577); written as the binary records them by `examples/seven_guis/tests/flows/record_checks.sh` (POSIX `sh`, the binary and an existing output directory as its two arguments, its scratch `rec/` made `mkdir -m 700`, printing only `step N` on failure), read by `stand_replay` through `Check::from_bytes`, compared byte for byte with a fresh recording by seven_guis `recording` in `rc-fresh` under `CARGO_TARGET_TMPDIR`
@carried n=5 run=2 label=architecture fact=fct-kjd5y7c4
quote: the two committed checks under `tests/blitz-tests/checks/` are the one exception to a kept check's owner-only medium.
@carried n=6 label=architecture/occupied-resources fact=fct-drud2scu
why: Link (2) and its click reading must say whether these per-kind handler counts are what the bridge exposes of a node's listeners, naming the API in the verdict.
quote: 64 event-handler count slots indexed by `DomEventKind` (packages/dioxus-native-dom/src/mutation_writer.rs:22-23; packages/dioxus-native-dom/src/mutation_writer.rs:36)
@carried n=7 label=architecture/cross-cutting-patterns fact=fct-dw3hon6z
why: A probe check that reads the snapshot read-only must be built with dioxus-native-dom's `accessibility` feature, which gates the snapshot modules.
quote: each dioxus-native feature forwarding to the same-named feature of the underlying blitz crates, and `accessibility` also to `dioxus-native-dom/accessibility`, which enables dioxus-native-dom's own optional `accesskit` and gates its `accessibility_tree` override and its `snapshot`, `snapshot_diff`, `snapshot_text` and `actionable` modules (packages/dioxus-native/Cargo.toml:16-33; packages/dioxus-native-dom/Cargo.toml:15-23; packages/dioxus-native-dom/Cargo.toml:41)
@carried n=7 run=2 label=architecture fact=fct-nmnb5xux
quote: optional capabilities are compile-time Cargo features forwarded down the crates, `accessibility` also gating the dioxus-native-dom snapshot modules;
@carried n=8 label=architecture/cross-cutting-patterns fact=fct-4onw7te5
why: The probe must not print or log a component name, handler or signal value it reads, and must add no log field or span, so the sink's drop and scrub rules see nothing new from it.
quote: writes to stderr only, stamps `service.name` / `service.version` on every line, bridges `log` records, drops every record whose target is outside the engine and escher prefix sets (`ENGINE_TARGET_PREFIXES`, `ESCHER_TARGET_PREFIXES`) at every level and whatever `RUST_LOG` names, scrubs the records that print
@carried n=8 run=2 label=architecture fact=fct-wkx6y4nz
quote: escher binaries install `escher_telemetry::init` at start — a stderr-only sink stamping the service identity, bridging `log`, dropping every record outside the engine and escher prefix sets at every level whatever `RUST_LOG` names, scrubbing what prints and chaining the panic hook; escher-driver emits its spans ungated and installs no subscriber;
@carried n=9 label=architecture/cross-cutting-patterns fact=fct-dsg4uyo4
why: The hazard's re-verification must read the shared `Rc<RefCell<BaseDocument>>` in the re-read dioxus_document.rs and events.rs as the borrow the panic trips, and links (3) and (4) must state what state the root contexts let them read.
quote: dioxus services are provided as root contexts — net provider, HTML parser provider, native document, window event handlers, shell provider, history, renderer, winit window (packages/dioxus-native/src/lib.rs:182-206; packages/dioxus-native/src/dioxus_application.rs:147-172) — and apps use `use_context_provider` / `use_context` (apps/browser/src/main.rs:121-123; apps/browser/src/tab.rs:211; apps/browser/src/toolbar.rs:46); the blitz document is shared as `Rc<RefCell<BaseDocument>>` (packages/dioxus-native-dom/src/dioxus_document.rs:86; packages/dioxus-native-dom/src/events.rs:140-144)
@carried n=9 run=2 label=architecture fact=fct-wrq3dhlq
quote: Dioxus services are root contexts; shared state is `Rc<RefCell<…>>`.
@carried n=10 label=architecture/cross-cutting-patterns fact=fct-klon5sda
why: The listener-map reading for links (2) and (3) must account for Dioxus skipping event kinds with zero handlers, and the hazard's re-read must place the mounted flush after each `poll` in dioxus_document.rs.
quote: dioxus skips event kinds with zero handlers (packages/dioxus-native-dom/src/dioxus_document.rs:346-352; packages/dioxus-native-dom/src/mutation_writer.rs:349-359); `mounted` listeners fire after `initial_build` and each `poll` (packages/dioxus-native-dom/src/mutation_writer.rs:331-336; packages/dioxus-native-dom/src/dioxus_document.rs:160-168; packages/dioxus-native-dom/src/dioxus_document.rs:232-256)
@carried n=10 run=2 label=architecture/cross-cutting-patterns fact=fct-3kximnte
quote: A queued `mounted` event whose element was removed since it was queued is skipped: `flush_queued_mounted_events` reads the element through `try_element_to_node_id` (packages/dioxus-native-dom/src/dioxus_document.rs:235-239)
@carried n=10 run=3 label=architecture fact=fct-p37qjcmu
quote: default-action events are queued and processed in order, handlers before the default action unless cancelled; sub-document and custom-widget events forwarded with adjusted coordinates; Dioxus skips event kinds with zero handlers;
@carried n=11 label=architecture/project-intent fact=fct-4ncs7geh
why: The probe must keep Dioxus at "0.7.3" with no patch, vendored copy, pin move or git dependency, Dioxus 0.8.0-alpha.3 waiting for the next version.
quote: Dioxus 0.8.0-alpha.3 (`286874cc1`), which wait for escher's next version, owned by a `.andromeda/residuals.md` line (target: next), so Stylo stays "0.22.0", Taffy the registry "0.15" and Dioxus "0.7.3", with no git dependency
@carried n=11 run=2 label=architecture fact=fct-7lmrueug
quote: git Stylo 0.23-pre, the six Taffy-main commits and Dioxus 0.8.0-alpha.3 wait for escher's next version, so Stylo stays "0.22.0", Taffy the registry "0.15" and Dioxus "0.7.3".
@carried n=11 run=3 label=architecture fact=fct-656exhoj
quote: Dioxus "0.7.3" (Cargo.toml:85-102)
@binding n=1
**Occupied Resources ↔ Cross-cutting Patterns (logging)**: «nothing crosses» binds on both sides at once. The probe adds no listener, port, env var, crate, binary or state directory (architecture/occupied-resources fct-crxfkvwh) (architecture/occupied-resources fct-7quovrlq) (architecture/occupied-resources fct-mzhbqeot), and no line the escher sink would print carries a component name, handler or signal value the probe reads (architecture/cross-cutting-patterns fct-4onw7te5).
@binding n=2
**Cross-cutting Patterns (service wiring) ↔ Cross-cutting Patterns (event processing)**: the MountedData hazard's mechanism, re-verified at HEAD, joins the shared `Rc<RefCell<BaseDocument>>` (architecture/cross-cutting-patterns fct-dsg4uyo4) with the mounted listeners firing after each `poll` and the queued mounted flush (architecture/cross-cutting-patterns fct-klon5sda) (architecture/cross-cutting-patterns fct-3kximnte). The founder's reading must name the borrow held in `render_immediate` and the `events.rs:162` coordinate on that path.
@binding n=3
**Occupied Resources (process-wide state) ↔ Cross-cutting Patterns (event processing)**: link (2)'s verdict and its `click` answer read the 64 per-`DomEventKind` handler count slots (architecture/occupied-resources fct-drud2scu), which are what lets Dioxus skip a kind with zero handlers (architecture/cross-cutting-patterns fct-klon5sda). The verdict must say whether a node's own listeners, or only per-kind counts, are exposed.
@binding n=4
**Project Intent ↔ Inherited Defaults**: the Dioxus "0.7.3" pin the upstream-sync line holds (architecture/project-intent fct-4ncs7geh) is the manifest's pin (architecture fct-656exhoj). A link that only a Dioxus change would expose stops at «not exposed without a fork-level change» and moves neither.
@binding n=5
**Cross-cutting Patterns (config management) ↔ Occupied Resources (committed checks)**: a probe check that reads the snapshot does so under `accessibility` (architecture/cross-cutting-patterns fct-dw3hon6z), read-only. The stand Counter's snapshot text and diffs then stay byte-identical to the committed checks (architecture/occupied-resources fct-bw7ybeh6).
