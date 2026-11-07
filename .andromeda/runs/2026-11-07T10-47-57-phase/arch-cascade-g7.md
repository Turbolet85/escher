@carried n=1 label=architecture/occupied-resources fact=fct-crxfkvwh
why: The buffered render must add no network port and no listener, the session socket staying the tree's one listener, as the scope's «Nothing crosses» requires.
quote: no served network port — no TCP or UDP port a binary serves; one local listener — the driver session's Unix-domain socket `{state_dir}/session.sock`
@carried n=1 run=2 label=architecture fact=fct-uu6lok3l
quote: one local listener, the driver session's Unix-domain socket `{state_dir}/session.sock` (`0600`, in a `0700` state directory), unix only; the `escher-session mcp` role speaks over its own stdio, with no listener; still no second served listener.
@carried n=2 label=architecture/occupied-resources fact=fct-bw7ybeh6
why: The committed Counter checks must still replay byte for byte after the render goes through the buffer, a witness of the ids, snapshot text and diffs the chunk re-asserts unchanged.
quote: read by `stand_replay` through `Check::from_bytes`, compared byte for byte with a fresh recording by seven_guis `recording` in `rc-fresh` under `CARGO_TARGET_TMPDIR`
@carried n=3 label=architecture/occupied-resources fact=fct-drud2scu
why: The replay into MutationWriter must keep the per-kind event-handler counts on DioxusState as the direct write kept them, whether or not the buffer lives on DioxusState.
quote: 64 event-handler count slots indexed by `DomEventKind` (packages/dioxus-native-dom/src/mutation_writer.rs:22-23; packages/dioxus-native-dom/src/mutation_writer.rs:36)
@carried n=4 label=architecture/occupied-resources fact=fct-7quovrlq
why: The buffer module must read no environment variable, the list of read variables staying as it stands.
quote: **Environment variables:** `RUST_LOG`, the tracing filter directives `escher_telemetry::init` reads through `EnvFilter::try_from_default_env`
@carried n=5 label=architecture/occupied-resources fact=fct-mzhbqeot
why: The buffer must be a module of dioxus-native-dom, not a new workspace crate, and every crate on the bridge, escher-driver's sessions included, takes the buffered render.
quote: escher-driver, a `publish = false` library with no features of its own and no binary target, workspace member and `[workspace.dependencies]` path entry, depending on blitz-test-harness, on dioxus-native-dom with its `accessibility` feature named
@carried n=5 run=2 label=architecture fact=fct-x4u4skbo
quote: **Names:** the workspace's crates, among them escher-telemetry and escher-driver, `publish = false` libraries;
@carried n=6 label=architecture/cross-cutting-patterns fact=fct-dw3hon6z
why: The buffered render must build and hold with dioxus-native-dom's accessibility feature on and off, the snapshot, its text and its diff the chunk re-asserts reading only under it.
quote: `accessibility` also to `dioxus-native-dom/accessibility`, which enables dioxus-native-dom's own optional `accesskit` and gates its `accessibility_tree` override and its `snapshot`, `snapshot_diff`, `snapshot_text` and `actionable` modules
@carried n=7 label=architecture/cross-cutting-patterns fact=fct-4onw7te5
why: Any trace the buffer module keeps must ride the optional, forwarded tracing feature, gated per call site, and carry no mutation content.
quote: tracing is an optional feature forwarded down the crate chain
@carried n=7 run=2 label=architecture/inherited-defaults fact=fct-fgkqqy76
quote: Optional capabilities: Cargo features, tracing gated per call site in the engine crates
@carried n=7 run=3 label=architecture fact=fct-jgykgl33
quote: Optional capabilities: Cargo features, tracing gated per call site in the engine crates — escher-telemetry and escher-driver (its `command` and `tool_call` spans) are ungated.
@carried n=8 label=architecture/cross-cutting-patterns fact=fct-dsg4uyo4
why: The shared Rc<RefCell<BaseDocument>> must be borrowed only after the vdom returns, once, to apply the buffer, so a polled task's MountedData call can borrow it during render.
quote: the blitz document is shared as `Rc<RefCell<BaseDocument>>` (packages/dioxus-native-dom/src/dioxus_document.rs:86; packages/dioxus-native-dom/src/events.rs:140-144)
@carried n=8 run=2 label=architecture/cross-cutting-patterns fact=fct-dsg4uyo4
quote: redraws are requested through `ShellProvider::request_redraw` after mutations, resource loads, hover, device changes, a focus set or cleared
@carried n=8 run=3 label=architecture fact=fct-wrq3dhlq
quote: Dioxus services are root contexts; shared state is `Rc<RefCell<…>>`.
@carried n=9 label=architecture/cross-cutting-patterns fact=fct-klon5sda
why: Both reshaped render sites must still fire mounted listeners after the buffer is applied and keep skipping event kinds with zero handlers.
quote: dioxus skips event kinds with zero handlers (packages/dioxus-native-dom/src/dioxus_document.rs:346-352; packages/dioxus-native-dom/src/mutation_writer.rs:349-359); `mounted` listeners fire after `initial_build` and each `poll` (packages/dioxus-native-dom/src/mutation_writer.rs:331-336; packages/dioxus-native-dom/src/dioxus_document.rs:160-168; packages/dioxus-native-dom/src/dioxus_document.rs:232-256)
@carried n=10 label=architecture/cross-cutting-patterns fact=fct-3kximnte
why: flush_queued_mounted_events must stay unchanged after the apply, still skipping a queued mounted event whose element the replayed buffer removed.
quote: A queued `mounted` event whose element was removed since it was queued is skipped: `flush_queued_mounted_events` reads the element through `try_element_to_node_id` (packages/dioxus-native-dom/src/dioxus_document.rs:235-239)
@carried n=11 label=architecture/cross-cutting-patterns fact=fct-ksdng7dt
why: The replayed buffer must clear or retarget interaction state on a removed node and mark the changed set at the mutator's sites exactly as the direct write did, so the snapshot diff reads the same.
quote: interaction state referencing a removed node is cleared or retargeted before the node is freed (packages/blitz-dom/src/document.rs:1337-1442)
@carried n=11 run=2 label=architecture/cross-cutting-patterns fact=fct-ksdng7dt
quote: the 15 mutation-flag sites of `DocumentMutator`, each marking the node whose in-document flag its own condition reads and only when that condition holds
@carried n=11 run=3 label=architecture fact=fct-wqtlmbkv
quote: interaction state on a removed node is cleared or retargeted before the node is freed
@carried n=12 label=architecture/project-intent fact=fct-4ncs7geh
why: The chunk ports only 286874cc1's buffered render onto the 0.7 bridge, the 0.8 move left waiting and Dioxus held at "0.7.3" with no git dependency.
quote: Dioxus 0.8.0-alpha.3 (`286874cc1`, whose buffered render, the MountedData hazard's fix, is to be ported onto the Dioxus 0.7 bridge by the route entry «MountedData buffered render», the 0.8 move itself still waiting), which wait for escher's next version, owned by a `residual` row in the store (target: next), so Stylo stays "0.22.0", Taffy the registry "0.15" and Dioxus "0.7.3", with no git dependency
@carried n=12 run=2 label=architecture fact=fct-7lmrueug
quote: Dioxus 0.8.0-alpha.3 wait for escher's next version, so Stylo stays "0.22.0", Taffy the registry "0.15" and Dioxus "0.7.3".
@binding n=1
**Service wiring ↔ Event processing**: the document's one `borrow_mut` is taken only after `rebuild` / `render_immediate` return, the buffer applied under it, then the mounted events flushed as before (architecture/cross-cutting-patterns fct-dsg4uyo4) — so mounted listeners still fire after `initial_build` and each `poll` and a removed element's queued event is still skipped (architecture/cross-cutting-patterns fct-klon5sda) (architecture/cross-cutting-patterns fct-3kximnte).
@binding n=2
**Occupied resources (handler counts) ↔ Invalidation and state integrity**: replaying the buffer through the existing MutationWriter must leave DioxusState's handler counts (architecture/occupied-resources fct-drud2scu) and the mutator's changed set and removed-node cleanup (architecture/cross-cutting-patterns fct-ksdng7dt) as the direct write left them, which the committed checks' byte-for-byte replay witnesses (architecture/occupied-resources fct-bw7ybeh6).
@binding n=3
**Project intent ↔ Occupied resources**: the partial port of `286874cc1` keeps Dioxus at "0.7.3" (architecture/project-intent fct-4ncs7geh) and adds no listener, port, env var or workspace crate (architecture/occupied-resources fct-crxfkvwh) (architecture/occupied-resources fct-7quovrlq) (architecture/occupied-resources fct-mzhbqeot).
@binding n=4
**Logging and timing ↔ Inherited defaults ↔ Config management**: any trace in the buffer module is gated per call site under the forwarded `tracing` feature and holds no mutation content (architecture/cross-cutting-patterns fct-4onw7te5) (architecture/inherited-defaults fct-fgkqqy76), and the module holds under both `accessibility` builds (architecture/cross-cutting-patterns fct-dw3hon6z).
