@task_prose part=chunk
Builds the UI graph (v030-05): one read that gives, for the current screen of a held instance, each node, the component that rendered it, its handlers, the state those handlers write and the nodes that read that state.
Link 1 and link 2 are read by the public dioxus-core vdom walk the stable ids already run, link 2 through the dynamic attributes (exact, `mounted` included); links 3 and 4 come through a new escher-side signal registry in dioxus-native-dom, and the seven_guis stand's signals move onto it.
Adds the «reacts to a click» reading as a closed boolean snapshot field, which absorbs residual res-gvelrxja and moves `NODE_FIELDS` (thirteen today), the schema help texts, `describe()` and every pin that counts them; values stay masked and nothing is logged.
Carries the canvas direction: reserve a virtual-node id form that collides with none of the four stable-id tiers, key the graph on the agent-visible accesskit node with an optional owner element, and let a handler on an owner serve sub-targets. A contract or boundary change from this is the founder's fork, asked at P4.
The read crosses the driver's carriers (a verb, an outcome, its written form, its MCP tool, its recorded step). Expected crates: dioxus-native-dom, escher-driver, seven_guis. No blitz-dom engine change is expected.
@task_file n=1 path=packages/dioxus-native-dom/src/element_id.rs
@task_item n=1 part=needs
The Dioxus DOM bridge's stable ids: `DioxusDocument::element_id` / `element_ids` and their four tiers (author key, anchored path, component path, document path). The graph's link 1 reuses their vdom walk. The reserved virtual-node id form must use a separator that no id of the four tiers can hold.
@task_item n=2 part=needs
`DioxusDocument::unkeyed_actionable` and the rule that every actionable element reads an author key: it detects an id that holds a `/`, so the reserved virtual-node separator must not change what it reports.
@task_item n=3 part=needs
The snapshot, its text form and its diff under `accessibility`: they read the screen read-only and hold no `NodeId`. The new «reacts to a click» closed boolean joins the node's fields, and `NODE_FIELDS`, the schema help texts, `describe()` and every count pin move with it. Under the 2026-10-10 rule this is no widening.
@task_item n=4 part=needs
`build_accessibility_tree` and its AccessKit `TreeUpdate`, which builds the agent-visible nodes and their bounds. The graph is keyed on that node with an optional owner element, never on a DOM `NodeId`.
@task_item n=5 part=needs
The Dioxus integration on Dioxus 0.7.3 (dioxus-core 0.7.10), with no fork and no engine change. The registry reads link 4 through `Readable::subscribers()` inside `VirtualDom::in_runtime` and each subscriber's `ReactiveContext::origin_scope()`. A scope's hooks are `pub(crate)`, so a signal is found only through the wrapper hook or context an app hands it to.
@task_item n=6 part=needs
DOM events reach the vdom through `data-dioxus-id`, and Dioxus skips event kinds with zero handlers. Link 2 reads handlers from `VNode.dynamic_attrs` / `VNode::mounted_dynamic_attribute`, not from the attributes, which over-report. A handler on an owner element may serve sub-targets.
@task_item n=7 part=needs
The driver session's command schema: `VERBS` (twelve verbs), the eight closed causes, `Session::run` validating first, and nothing public running a `Command` that did not come from `validate`. A new verb and outcome for the graph read must join that `'static` schema and its count pins.
@task_item n=8 part=needs
The driver's carriers: the `v3` wire, the `escher-session` command line and its usage table, the MCP server and its tools, and the `command` and `tool_call` spans. Each answer is written by hand in the crate-private `json` module, no serialization crate writes one, and no ending writes back what the caller typed.
@task_item n=9 part=needs
The boundary rule (security rules §Surfaces, the rule of 2026-10-10): a new carrier or exit, or a new open value set on a ratified carrier, widens the boundary and is the founder's fork. Component names, handler event names and registered state names going out are such a question.
@task_item n=10 part=needs
Masking: `is_masked_input` (a password or file input) is the one masked-control test, and `MASKED_VALUE` stands in for its value. Every state or control value the graph carries reads masked as on every carrier.
@task_item n=11 part=needs
No log or event carries the snapshot, its text or a diff, except a diff's three counts on the command span. No log site, span field or print may carry a node, a component name, a handler, a state or a value. escher-driver builds its spans by hand, logs no id, name or value and installs no subscriber.
@task_item n=12 part=needs
escher-telemetry's scrubbing (`SAFE_FIELDS`, `CONTENT_FIELDS` always redacted, targets outside the prefix sets dropped) stands behind the never-logged rule. The graph adds no field it must print.
@task_item n=13 part=needs
The headless stand (`seven_guis::stand`): the four lean tasks, Cells (`boot_cells`) and the load page, in a pinned 800x600 Light viewport. The stand's hook-local signals move onto the registry. It installs no telemetry and reads no env var. `stand_non_lean_tasks` pins the starting «reacts to a click» reading on the four lean tasks, Cells and both layout modes.
@task_item n=14 part=needs
The recording and kept checks: the graph read's recorded step, `Check::replay` running through `Session::run`, a masked value's stand-in, and the two committed kept checks under `tests/blitz-tests/checks/` that `stand_replay` replays.
@task_item n=15 part=needs
The served stand tasks of the `escher-session` command line and the MCP `start` tool's `enum`, both written from one private table. The capability statement (`docs/capabilities.md`) and the root `README.md` give the count of driver commands an MCP client gets as tools, and a pin reads that count.
@task_item n=16 part=needs
Config management and feature gating: the `accessibility` feature gates the dioxus-native-dom snapshot modules. The graph reader and the snapshot field follow that gating, and features are forwarded down the crates.
@task_item n=17 part=needs
Apps and examples conventions for the stand: reactive state through `#[derive(Store)]` and signal and store hooks. The stand's signals, wrapped by the registry, keep those hooks' behaviour.
@task_item n=18 part=needs
Manifests: a dependency the registry or graph reader takes (for example dioxus-core) is declared once in `[workspace.dependencies]`, used as `{ workspace = true }`, and in-repo crates take `default-features = false`.
@task_item n=19 part=needs
Visibility and naming: crate internals stay `pub(crate)` / `pub(super)`. Builder methods follow `with_…` / `without_…`. Every public item of the registry and graph reader carries rustdoc, under the workflow-wide `-D warnings`.
@task_item n=20 part=needs
The direct engine changes for every document and their merge surfaces: the chunk adds none (no blitz-dom change is expected), so that list and the Upstream sync line it owns must stay as they are.
@task_standing n=1
convention: Each new test sits in a `#[cfg(test)] mod tests` at the end of its source file or in a `#[path]` child file. Each integration test file covers one behaviour and opens with a `//!` doc naming it. Scenarios run in both layout modes, tests first assert that the fixture produces the condition, and stand checks share helpers through non-target modules under `tests/`.
home: conventions
@task_standing n=2
convention: Each new module or file opens with a `//!` doc. Public items carry `///`, and constants state their rationale.
home: conventions
@task_standing n=3
convention: Each change passes `cargo fmt --all --check`, `cargo clippy --workspace --locked -- -D warnings` and rustdoc `-D warnings` through `.github/scripts/ci-leg.sh` legs. `expect` / `unwrap` carry a justification.
home: conventions
@task_standing n=4
convention: Tracing in the engine crates is gated per call site with a `cfg(not(feature = "tracing"))` fallback. escher's own crates take `tracing` ungated, with spans built by hand and no `#[instrument]`.
home: conventions
@task_standing n=5
convention: Error types are enums with `Display` and a `From` per wrapped error. Mutex locks recover from poisoning with `into_inner`.
home: conventions
@task_standing n=6
convention: The per-crate coverage floor and the `stand-repeat` leg's zero-failure budget hold for each new crate code and stand check.
home: cross-cutting-patterns
@task_item n=1 part=not_needed
The browser app (history store, URL suggester, about pages, document loader) and rdme: the graph is read on the headless stand and the driver, not in those apps.
@task_item n=2 part=not_needed
The WPT runner, its outputs, bucketing and execution model: the chunk adds no WPT test and changes no runner.
@task_item n=3 part=not_needed
Networking (blitz-net, the `User-Agent`, the per-host semaphore, the HTTP cache) and resource loading: the graph makes no fetch.
@task_item n=4 part=not_needed
Paint and shell (blitz-paint, layer caps, renderer backends, blitz-shell's loop and windowing, keyboard shortcuts): the graph is a read and paints nothing. The canvas renderer is out of scope.
@task_item n=5 part=not_needed
JavaScript on Boa (blitz-vibey-script, timers, the module loader, `document_may_load`): the stand is a Dioxus app, so no script is involved.
@task_item n=6 part=not_needed
Layout, CSS approximations, unsupported features, fonts, inline SVG and HTML parsing decisions: the graph reads already-built nodes and changes no style or layout.
@task_item n=7 part=not_needed
Deployment, Nix packaging, cross-compilation, publish-browser bundles and signed builds: the chunk adds no build target or package.
@task_item n=8 part=not_needed
Licensing exceptions and the HTML fixture conventions under `examples/assets`: the chunk adds no licensed asset and no HTML fixture page.
@task_item n=9 part=not_needed
The stylo threading lock, the thread-local layout context, mimalloc and the WPT panic hook: the graph read spawns no thread and touches no process-wide engine state.
@task_item n=10 part=not_needed
«What can I do here» (v030-06), «Handler coverage» (v030-07), virtual-node production, `drag` and the canvas renderer are out of scope. Only the id form is reserved; nothing produces it.
