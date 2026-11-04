@carried n=1 label=architecture/standard-contracts fact=fct-kllkxz3d
why: link (2)'s verdict, the `click` reading among it, must name what the bridge already writes for a listener and how an event finds its vdom element, read without changing that path.
quote: DOM events route to the vdom via the nearest `data-dioxus-id` attribute parsed as `usize` (packages/dioxus-native-dom/src/dioxus_document.rs:47-55; packages/dioxus-native-dom/src/dioxus_document.rs:415-421); listener registration sets a `"<rust func>"` placeholder and `data-dioxus-id` (packages/dioxus-native-dom/src/mutation_writer.rs:338-345)
@carried n=1 run=2 label=architecture fact=fct-srb7slak
quote: DOM events reach the vdom through `data-dioxus-id`;
@carried n=2 label=architecture/standard-contracts fact=fct-kllkxz3d
why: the hazard's re-verification reaches `events.rs`, where `NodeHandle` backs `MountedData`, and links (1), (3) and (4) must name the public `vdom` and element mapping the bridge already exposes before any cost is claimed.
quote: event converter for form, mouse, keyboard, focus, mounted, pointer, scroll, touch, wheel (packages/dioxus-native-dom/src/events.rs:42-138); `NodeHandle` backs `MountedData` (packages/dioxus-native-dom/src/events.rs:195-307); `DioxusDocument` exposes `vdom`, `inner`, `vdom_state.try_element_to_node_id(ElementId)`, which the guard of `flush_queued_mounted_events` (below) also reads (tests/blitz-tests/tests/stale_node_mapping.rs:66; tests/blitz-tests/tests/stale_node_mapping.rs:76-80)
@carried n=3 label=architecture/standard-contracts fact=fct-kllkxz3d
why: link (1) reads through the stable-id readers, which must stay read-only — the probe writes nothing to DOM or vdom.
quote: **stable element ids** — `DioxusDocument::element_id(NodeId) -> Option<String>` (`None` for a non-element, stale or detached node, never panics) and `element_ids() -> Vec<(NodeId, String)>` (every element reachable from the document root, document pre-order, ids pairwise distinct) compute the id on demand from a vdom walk joined with a DOM walk and write it to neither the DOM nor the vdom (packages/dioxus-native-dom/src/dioxus_document.rs:198-230; packages/dioxus-native-dom/src/element_id.rs:186-282)
@carried n=3 run=2 label=architecture fact=fct-srb7slak
quote: `DioxusDocument::element_id` and `element_ids` compute a stable id on demand in four tiers — author key, anchored path, component path, document path — written to neither DOM nor vdom;
@carried n=4 label=architecture/standard-contracts fact=fct-kllkxz3d
why: the component path tier already joins a node to the component that rendered it through `VComponent.name`, so link (1)'s verdict must weigh this existing bridge walk as its named API.
quote: else (3) the **component path** — `/`-joined component segments below the app root, where dioxus-core's four framework scopes (`ScopeId::ROOT` · `ROOT_SUSPENSE_BOUNDARY` · `ROOT_ERROR_BOUNDARY` · `APP`, the user root dioxus-core names `"root"`) add none, a segment is the last `::` segment of `VComponent.name` (dioxus-core's full type path) with generics stripped, and an owner's 2nd and later instance of one name reads `{name}:{k}`
@carried n=5 label=architecture/standard-contracts fact=fct-kllkxz3d
why: nothing crosses — the probe's readings must put no process-local id into any id it reads or reports.
quote: no `NodeId`, `ElementId`, `ScopeId` or pointer enters an id
@carried n=6 label=architecture/standard-contracts fact=fct-kllkxz3d
why: link (4)'s readers across a re-render and link (1)'s element mapping are read through these bridge-held mappings, which a re-render clears for a removed node.
quote: a node a re-render removes is dropped with its descendants by the bridge's crate-private `remove_and_drop_node` / `replace_and_drop_node`, their `element_id_mapping` / `node_id_mapping` entries cleared, so it no longer resolves and reads no id
@carried n=7 label=architecture/standard-contracts fact=fct-kllkxz3d
why: a probe check that reads the snapshot gets its modules only by naming or inheriting the `accessibility` feature.
quote: dioxus-native-dom's `accessibility` feature reads `["blitz-dom/accessibility", "dep:accesskit"]` and dioxus-native's `accessibility` forwards to it, so a crate taking either Dioxus crate with `default-features = false` (the workspace pin) gets the override, the snapshot model, its text form, its diff and the actionable-key check only by naming the feature or by depending on a crate that names it — blitz-tests and, since 2026-10-07-act-by-id, escher-driver name it on dioxus-native-dom;
@carried n=8 label=architecture/standard-contracts fact=fct-kllkxz3d
why: a probe check reading the snapshot reads it read-only, so a link reading never mutates the screen it measures.
quote: under the same feature `DioxusDocument::snapshot(&self) -> Snapshot` reads the screen as a tree, read-only: it builds from that `accessibility_tree` override and walks its `children` lists from `TreeInfo.root`;
@carried n=9 label=architecture/standard-contracts fact=fct-kllkxz3d
why: the probe's output must log no snapshot, its text or a diff — values masked, never logged.
quote: no log or event carries the snapshot or its text, while a kept check holds it at rest
@carried n=9 run=2 label=architecture fact=fct-tev7ieys
quote: No log or event carries the snapshot, its text or a diff, except a diff's three counts — added, removed and changed — on escher-driver's command span.
@carried n=10 label=architecture/standard-contracts fact=fct-kllkxz3d
why: link (2)'s `click` reading must account that the bridge's `data-dioxus-id` marks every listener but `onmounted` without naming the event, so event names come from the listener map itself.
quote: or the `data-dioxus-id` attribute the bridge writes for every listener but `onmounted`
@carried n=11 label=architecture/standard-contracts fact=fct-2ylcpw3b
why: Cells is the served task whose key handler writes `current` that other nodes read and calls `MountedData` synchronously, the case beside the hazard's spawned-task path that the probe runs and the reading contrasts.
quote: `cells-grid` takes `id`, `tabindex` 0, `aria_label` Cells and an `onkeydown`, with no role, so it is one Tab stop: arrows move `current`, clamped, into view through a synchronous `MountedData::scroll_to(ScrollBehavior::Instant)`, one `onmounted` per data cell holding the handles;
@carried n=12 label=architecture/standard-contracts fact=fct-h4l33tnz
why: the probe adds no node key, so the snapshot's key set stays at `NODE_FIELDS`' thirteen.
quote: The snapshot carries no `expanded` node key and `NodeState` holds no field for a list, `NODE_FIELDS` keeping the thirteen keys §Driver session names.
@carried n=13 label=architecture/standard-contracts fact=fct-c5e4c7ao
why: the served task the probe runs beside a lean one is one of these six, booted through the stand and unchanged by the probe.
quote: over the stand's six served tasks — the four lean ones, Cells and the load page: `main` installs the telemetry sink first, then hands its argv, the six slugs `counter` · `flight-booker` · `timer` · `crud` · `cells` · `load`
@carried n=13 run=2 label=architecture fact=fct-fukxpbvx
quote: `escher-session <verb> [arguments] --session <dir>`, the driver's command line over six served stand tasks — the four lean ones, Cells and the load page,
@carried n=14 label=architecture/standard-contracts fact=fct-c5e4c7ao
why: nothing crosses — the probe adds no verb or argument, so the twelve of `VERBS` stand as they are.
quote: the verbs are the twelve of `VERBS` — `snapshot`, `click --id`, `type --id --text`, `press --key [--shift]`, `advance --ms`, `scroll --id`, `screenshot [--id <id>]`, `lint`, `start <task> [--record <dir>]`, `status`, `stop` and, since 2026-10-12-self-description, `help`
@carried n=14 run=2 label=architecture fact=fct-ksgwc2yv
quote: the command and refusal schema stated once as `'static` data — twelve verbs, eight closed causes, no refusal holding what a call supplied; the `v3` wire, the command line, the MCP server and the `command` and `tool_call` spans carry it; no `NodeId` crosses, and nothing logs an id, a name or a value.
@carried n=15 label=architecture/standard-contracts fact=fct-c5e4c7ao
why: nothing crosses — the probe adds no exit, so the four exit statuses stay as stated.
quote: a command answers one uncoloured line of JSON on stdout and exits `0` accepted · `1` refused · `2` usage error, a fixed line on stderr and stdout empty, nothing booted and no state created · `3` session error, the error object on stdout and its fixed message on stderr — the four read from `EXIT_STATUSES` and served by `help` since 2026-10-12-self-description;
@carried n=16 label=architecture/standard-contracts fact=fct-jelmr2gx
why: a probe check the plan adds under `tests/blitz-tests/tests/` joins `stand-repeat` by its `stand_*.rs` name or else stands outside it unless named, so its file name decides its membership.
quote: `stand-repeat` (since 2026-10-14-quality-gates, not a fast leg: three rounds, each printing `stand-repeat: round R of 3` and running `cargo test -p blitz-tests --locked` with one `--test` per `tests/blitz-tests/tests/stand_*.rs` and, on the same line, seventeen named blitz-tests targets outside that prefix as literal `--test` words
@carried n=16 run=2 label=architecture fact=fct-scmruv42
quote: `.github/scripts/ci-leg.sh {leg}` is the one leg runner of CI and the dev host;
@carried n=17 label=architecture/standard-contracts fact=fct-jelmr2gx
why: a probe check running on the stand must pass three rounds with no retry, so its measured verdicts must be deterministic.
quote: the first red command ends the leg with its status — a zero-failure budget, no retry, no `--ignored`, no env var — and three green rounds end `stand-repeat: 3 of 3 rounds green`
@carried n=18 label=architecture/standard-contracts fact=fct-jelmr2gx
why: the task names the per-crate coverage floors among the gates probe code must pass, dioxus-native-dom's floor among them.
quote: the `coverage` leg calls it with `packages/escher-driver=97 packages/dioxus-native-dom=83 packages/blitz-test-harness=90`
@binding n=1
**Dioxus DOM bridge ↔ the chunk's link (1) and link (2) verdicts**: an «exposed» verdict names the bridge-held API it reads — the component path's `VComponent.name` walk for node → component, the listener registration and `data-dioxus-id` for node → handlers — and reads it without writing DOM or vdom (architecture/standard-contracts fct-kllkxz3d)
@binding n=2
**Dioxus DOM bridge ↔ Event processing (cross-cutting)**: what the listener map shows for `click` is read beside «Dioxus skips event kinds with zero handlers», and `data-dioxus-id` marks every listener but `onmounted` without its event name (architecture/standard-contracts fct-kllkxz3d) (architecture fct-p37qjcmu)
@binding n=3
**Dioxus DOM bridge `MountedData` ↔ the MountedData hazard's re-verification**: `NodeHandle` backing `MountedData` in `events.rs` is where the panic coordinate is re-read, and Cells' synchronous `MountedData` call from `onkeydown` is the served contrast to the spawned-task path (architecture/standard-contracts fct-kllkxz3d) (architecture/standard-contracts fct-2ylcpw3b)
@binding n=4
**Dioxus DOM bridge snapshot ↔ Telemetry bootstrap**: no probe output logs a snapshot, its text, a diff or a value it reads, and escher-telemetry's `SAFE_FIELDS` / `CONTENT_FIELDS` rules hold over any line it prints (architecture/standard-contracts fct-kllkxz3d) (architecture fct-dwg4fa67)
@binding n=5
**CLIs ↔ Driver session schema ↔ capability-statement pin**: the twelve verbs, the four exits and `NODE_FIELDS`' thirteen keys stay as they are, so the counts `docs/capabilities.md` states and `CapabilityStatementTest` pins hold unchanged (architecture/standard-contracts fct-c5e4c7ao) (architecture/standard-contracts fct-h4l33tnz) (architecture fct-ksgwc2yv) (architecture/standard-contracts fct-jelmr2gx)
@binding n=6
**CI contracts ↔ Conventions (Tests)**: a probe check the plan keeps in blitz-tests is one test target per file, run in both layout modes, and its `stand_*` name or a named `--test` word places it in `stand-repeat`'s zero-failure rounds (architecture/standard-contracts fct-jelmr2gx) (architecture fct-wqndq65v)
@binding n=7
**Dioxus DOM bridge `accessibility` feature ↔ Config management (cross-cutting)**: a probe check reading the snapshot builds under the `accessibility` feature, which gates the dioxus-native-dom snapshot modules (architecture/standard-contracts fct-kllkxz3d) (architecture fct-nmnb5xux)
