@task_prose part=chunk
Syncs upstream Blitz into escher's 0.4.0 build branch as a real two-parent merge of upstream commit 29a439e05 (merge base 165d698c3, six commits), never a rebase, squash or cherry-pick series; the take carries all six commits.
The take brings Taffy 0.16.0 from crates.io in place of the pinned git rev, a br kept in its text's anonymous flex item, one-axis overflow clip painted on the clipped axis alone, drops from other applications (a DataTransfer model, drag events, a winit drop source, a dropped file's reader over std::fs), preventDefault ignored on non-cancelable events, and two root examples.
It adapts two parts of ours: the layout reason's calc resolution to Taffy's CalcHandle, and the drag event's modifiers through convert_modifiers under the Dioxus 0.7.3 hold; it resolves the one conflict, in dioxus-native-dom's event conversion imports.
It re-reads the standing left-out set, the Dioxus hold, eight merge-surface residuals, the one-axis clip limit and the sink allowlist, measures the whole lock diff with a locked workspace build, and re-runs every escher proof (CI legs, stand checks in both layout modes, a11y leg, audit, MSRV, embolden witness, screenshot and cross-OS readings) through fork CI green.
@task_file n=1 path=Cargo.toml mark=merge
what: the workspace root manifest — the workspace dependency set with Taffy's pin and its members, and the root package's dev dependencies and examples
@task_file n=2 path=Cargo.lock mark=merge
what: the workspace lock file — the locked dependency graph, Taffy's source and checksum among its entries
@task_file n=3 path=packages/blitz-shell/Cargo.toml mark=merge
what: blitz-shell's manifest — the winit window shell's dependency list
@task_file n=4 path=packages/dioxus-native-dom/src/events.rs mark=conflict
what: dioxus-native-dom's event conversion — how engine events become Dioxus events, the drag event and its modifiers, and a dropped file's reader
@task_file n=5 path=packages/blitz-dom/src/layout_reason.rs mark=edits
what: blitz-dom's layout reason — why a box is placed, shifted and sized, the reader behind Session::layout and the layout verb's answer — edits: its calc-value resolution through stylo_taffy's resolve_calc_value, now taking Taffy's CalcHandle
@task_file n=6 path=packages/blitz-dom/src/layout/construct.rs mark=merge
what: blitz-dom's layout box construction — anonymous blocks and inline layout construction
@task_file n=7 path=packages/blitz-dom/src/layout/mod.rs mark=merge
what: blitz-dom's layout entry — the Taffy traversal and layout traits over the document
@task_file n=8 path=packages/blitz-dom/src/layout/replaced.rs mark=merge
what: blitz-dom's sizing of replaced elements under Taffy
@task_file n=9 path=packages/blitz-dom/src/layout/table.rs mark=merge
what: blitz-dom's table layout, HTML tables laid out as a Taffy grid
@task_file n=10 path=packages/stylo_taffy/src/convert.rs mark=merge
what: stylo_taffy's conversion of Stylo computed values into Taffy styles, calc-value resolution among its exports
@task_file n=11 path=packages/blitz-paint/src/render.rs mark=merge
what: blitz-paint's scene rendering — the paint of boxes, clips and overflow
@task_file n=12 path=packages/blitz-dom/src/events/driver.rs mark=merge
what: blitz-dom's event driver — DOM event dispatch, default actions and preventDefault, drag events
@task_file n=13 path=packages/blitz-dom/src/events/mod.rs mark=merge
what: blitz-dom's event module — the document's event handling entry
@task_file n=14 path=packages/blitz-dom/src/document.rs mark=merge
what: blitz-dom's BaseDocument — the core document and its state
@task_file n=15 path=packages/blitz-dom/src/node/custom_widget.rs mark=merge
what: blitz-dom's custom widget node — the custom content embedded as an object element
@task_file n=16 path=packages/blitz-traits/src/events.rs mark=merge
what: blitz-traits' UI event types shared between shell, document and Dioxus
@task_file n=17 path=packages/blitz-traits/src/events/datatransfer.rs mark=merge
what: blitz-traits' DataTransfer model — the data a drop from another application carries
@task_file n=18 path=packages/blitz-shell/src/dnd.rs mark=merge
what: blitz-shell's winit drop source — drops from other applications into a window
@task_file n=19 path=packages/blitz-shell/src/window.rs mark=merge
what: blitz-shell's window — winit window events forwarded to the document
@task_file n=20 path=packages/blitz-shell/src/application.rs mark=merge
what: blitz-shell's application — the winit event loop handler
@task_file n=21 path=packages/blitz-shell/src/lib.rs mark=merge
what: blitz-shell's crate root and its module list
@task_file n=22 path=packages/dioxus-native-dom/src/dioxus_document.rs mark=merge
what: the Dioxus document — the Dioxus DOM bridge over BaseDocument, handle_ui_event among its entries
@task_file n=23 path=examples/dnd.rs mark=merge
what: a root example of drops from other applications
@task_file n=24 path=examples/file_drop.rs mark=merge
what: a root example of file drop with previews of dropped content
@task_file n=25 path=tests/blitz-tests/tests/flex_br.rs mark=merge
what: a blitz-tests integration test file — a br after text kept in its anonymous flex item
@task_file n=26 path=tests/blitz-tests/tests/drag_and_drop.rs mark=merge
what: a blitz-tests integration test file — drag and drop events
@task_file n=27 path=tests/blitz-tests/tests/pointer_leave.rs mark=merge
what: a blitz-tests integration test file — pointer leave, extended with preventDefault on a non-cancelable event
@task_file n=28 path=tests/blitz-tests/tests/all.rs mark=merge
what: the blitz-tests aggregate target's mod list — its new mod lines
@task_file n=29 path=.github/scripts/test_blitz_tests_targets.py mark=re-reads
what: the blitz-tests target pin — one file and one target per behaviour
@task_file n=30 path=.github/scripts/ci-leg.sh mark=re-reads
what: the one CI leg runner — the fast, doc, a11y, audit, msrv and embolden legs re-run on the merged tree
@task_file n=31 path=tests/blitz-tests/tests/stand_layout.rs mark=re-reads
what: the stand's layout reason readings check — the layout verb's answer on the stand
@task_file n=32 path=tests/blitz-tests/tests/stand_act_obstructed.rs mark=re-reads
what: the stand's obstructed-act check — a hit stopped at a clipping box's padding box
@task_item n=1 part=needs
The sync is a real two-parent merge whose parents are the branch tip and the upstream commit taken; each left-out commit is named with its reason and its change undone in the merged tree; the standing left-out set (parley_code_editor out of the workspace, upstream AGENTS.md kept out of the tree, the Dioxus 0.8 move held at 0.7.3) is re-read and a reason that no longer holds puts the commit back in.
@task_item n=2 part=needs
Taffy moves from 0.15.0 at a pinned git rev to 0.16.0 from crates.io with a checksum, and blitz-shell gains a url dependency: upstream's pins are taken as one coupled set, the whole lock diff enters the plan, and the workspace builds with cargo --locked; the no-git-dependency rule's Taffy exception is retired in every row that states it.
@task_item n=3 part=needs
The layout reason's calc resolution through stylo_taffy's resolve_calc_value takes Taffy 0.16's CalcHandle; the layout reason and its readings — the layout reason types, Session::layout, the layout verb's answer and its words, the stand's layout readings — are a surface of this take and read the same after it; the adaptation stays in escher's own function.
@task_item n=4 part=needs
The drag event reaches Dioxus with its modifiers through convert_modifiers under the Dioxus 0.7.3 hold, and the one merge conflict, in the event conversion's import blocks, is resolved keeping both sides.
@task_item n=5 part=needs
Drops from other applications are a boundary widening landed on the founder's ratification: a DataTransfer model, drag data reaching every windowed document as four DOM events, a winit drop source in the shell; no harness input, driver verb, MCP tool or stand task reaches it.
@task_item n=6 part=needs
A Dioxus app's handler can read a dropped file's bytes through std::fs (metadata, read, read_to_string, File::open) with no path restriction — a widening of file reads, ratified, set beside the file URL refusal and file script-load refusal that stay in force.
@task_item n=7 part=needs
Layout box construction keeps a br after text in the text's anonymous flex item — an upstream engine behaviour change ratified with the take, held against the anonymous-block and both-layout-modes rules.
@task_item n=8 part=needs
Paint clips only the clipped axis for a one-axis overflow clip, so paint now agrees with visible_region while the hit still stops at the whole padding box; the one-axis clip readings kept as measured and the hit-stop rule are re-read against the merge.
@task_item n=9 part=needs
The event driver ignores preventDefault on a non-cancelable event — a change to default-action order and DOM event dispatch brought by the take.
@task_item n=10 part=needs
Two root examples (drop, file drop) arrive and the root package's dioxus-native dev features gain net and html; the root package stays virtual and unpublished, the examples unpublished.
@task_item n=11 part=needs
Conflicts keep both sides' intent: the changed set's contract, the clock invariant, the select model and its list, hit stops and scrolled-box bounds, into-view readings, stable ids and the snapshot, range and keyboard models, inert, accessibility state reads, custom-widget paint, pointer frame and click synthesis, the stamped sub-resource request and the file refusal, and the escher crates and seven_guis members of the workspace.
@task_item n=12 part=needs
Upstream's new test files and the aggregate target's new mod lines run under the fork's blitz-tests rule — one file and one target per behaviour, pinned by the blitz-tests target pin.
@task_item n=13 part=needs
Every escher proof passes after the merge: the fast and doc CI legs, the stand checks in both layout modes, the incremental versus non-incremental oracle, the a11y leg, the embolden witness, and the screenshot and cross-OS readings — a moved reading re-measured and recorded, never loosened; fork CI green on the pushed commit with every job settled.
@task_item n=14 part=needs
The dependency and licence audit reads advisories and licences ok on the merged lock, and MSRV stays 1.91 unless a taken pin requires more, then named; the MSRV leg builds beside the audit.
@task_item n=15 part=needs
The sink allowlist is re-checked: the merged delta is read for every new tracing or log call site, target, print and env read, and the standing capture checks stay at zero needles.
@task_item n=16 part=needs
The Dioxus 0.8 move stays held: the hold, its residuals and the keyboard_compat coverage exclusion are re-read and stand; the eight merge-surface residuals are re-read against the merge, one a taken move resolves re-measured.
@task_item n=17 part=needs
Upstream behaviour changes are ratified per take and taken as engine behaviour changes; the boundary widenings, forks and left-out commits are collected in one founder forks file, none answered by a default.
@task_item n=18 part=needs
A CI workflow edit keeps every action pinned to a 40-hex SHA and the workflow's read-only contents permission, and upstream's workflow files stay guarded to the upstream repository.
@task_item n=19 part=needs
The take adds no new crate, port, env var or listener of escher's own.
@task_standing n=1
convention: each integration test file holds one behaviour and is its own blitz-tests target, joined to the target pin
home: conventions
@task_standing n=2
convention: integration tests live in blitz-tests
home: design-philosophy
@task_item n=1 part=not_needed
The browser app — browser history, about pages, toolbar, persistence and its SQLite store — is not reached by the take.
@task_item n=2 part=not_needed
The WPT runner, its results, outputs, status codes and PR-body diff tooling: upstream's WPT conformance numbers are out of scope.
@task_item n=3 part=not_needed
The JavaScript runtime — Boa, blitz-vibey-script, script globals, the script fetcher, CSSOM and selector query APIs — is untouched by the delta.
@task_item n=4 part=not_needed
The cold-agent and agent-run tooling, their events, exit codes, state directories and verdict fields carry nothing of this take.
@task_item n=5 part=not_needed
The session socket wire, MCP server protocol and tool listing, session errors and recording-directory formats: no verb, tool or socket call is added or changed.
@task_item n=6 part=not_needed
The UI graph, the graph and actions verbs, and the graph registry are not touched by the merge.
@task_item n=7 part=not_needed
Kept check files, check replay and the handler coverage reader take no change from the sync.
@task_item n=8 part=not_needed
rdme, wasm_hello, Trunk browser-target builds, and the Android and iOS entry points are outside the delta.
@task_item n=9 part=not_needed
The HTTP cache and the favicon cache are not reached.
@task_item n=10 part=not_needed
Canvas draw commands and the later Epoch 4 entries' work, and sending changes upstream, are out of scope.
