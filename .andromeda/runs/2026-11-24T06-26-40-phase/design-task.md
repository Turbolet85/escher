@task_prose part=chunk
Syncs this fork with upstream as one real two-parent merge of upstream/main (29a439e05, six commits over the last sync's 165d698c3) into build/escher-0.4.0, never a rebase, squash or cherry-pick series; the founder's answers carry all six commits.
The take brings Taffy 0.16.0 from the registry in place of the git rev, a br after text kept in its anonymous flex item, one-axis overflow clip painting the clipped axis alone, drops from other applications (a DataTransfer model, four drag DOM events, a winit drop source, a dropped file's reader over std::fs), preventDefault ignored on a non-cancelable event, and two root examples.
Two adaptations of ours ride the take: the layout reason's calc resolution under Taffy 0.16's CalcHandle, and the drag event's modifiers through convert_modifiers under the Dioxus 0.7.3 hold; the one conflict, the drop events' import blocks, is resolved keeping both sides.
It re-reads the standing left-out set, the Dioxus hold, the merge-surface residuals, the one-axis clip hit limit and the sink allowlist against the merge, and re-measures the whole lock diff under a --locked workspace build.
Every escher proof is re-run on the merged tree: the fast and doc CI legs, the stand checks in both layout modes, the incremental oracle, the a11y leg, the audit, the embolden witness, the screenshot and cross-OS readings, and fork CI on the pushed commit.
@task_file n=1 path=Cargo.toml mark=merge
what: the workspace root manifest — the workspace members, the Taffy dependency pin and the dioxus-native dev features
@task_file n=2 path=Cargo.lock mark=merge
what: the workspace dependency lock — the taffy pin and blitz-shell's dependency list
@task_file n=3 path=examples/dnd.rs mark=merge
what: a root example of drops from other applications
@task_file n=4 path=examples/file_drop.rs mark=merge
what: a root example of a file drop with previews of the dropped content
@task_file n=5 path=packages/blitz-dom/src/document.rs mark=merge
what: the engine document — its event handling entry for the new drag events
@task_file n=6 path=packages/blitz-dom/src/events/driver.rs mark=merge
what: the engine event driver — drag event dispatch and preventDefault on a non-cancelable event
@task_file n=7 path=packages/blitz-dom/src/events/mod.rs mark=merge
what: the engine events module — the drag events' wiring
@task_file n=8 path=packages/blitz-dom/src/layout/construct.rs mark=merge
what: layout tree construction — the anonymous flex item a br after text stays in
@task_file n=9 path=packages/blitz-dom/src/layout/mod.rs mark=merge
what: the engine layout module — its Taffy calls under Taffy 0.16
@task_file n=10 path=packages/blitz-dom/src/layout/replaced.rs mark=merge
what: replaced-element layout — its Taffy calls under Taffy 0.16
@task_file n=11 path=packages/blitz-dom/src/layout/table.rs mark=merge
what: table layout — its Taffy calls under Taffy 0.16
@task_file n=12 path=packages/blitz-dom/src/node/custom_widget.rs mark=merge
what: the custom widget node — the custom-widget paint and its event entry
@task_file n=13 path=packages/blitz-paint/src/render.rs mark=merge
what: the paint renderer — the overflow clip of a box clipped on one axis
@task_file n=14 path=packages/blitz-shell/Cargo.toml mark=merge
what: the shell crate's manifest — its dependency list, gaining url
@task_file n=15 path=packages/blitz-shell/src/application.rs mark=merge
what: the shell application — the window event loop routing drops
@task_file n=16 path=packages/blitz-shell/src/dnd.rs mark=merge
what: the shell's winit drop source — drag data from other applications
@task_file n=17 path=packages/blitz-shell/src/lib.rs mark=merge
what: the shell crate root — its module list
@task_file n=18 path=packages/blitz-shell/src/window.rs mark=merge
what: the shell's window event handling — the drop source's wiring into the window
@task_file n=19 path=packages/blitz-traits/src/events.rs mark=merge
what: the shared event types — the drag events added to the UI event set
@task_file n=20 path=packages/blitz-traits/src/events/datatransfer.rs mark=merge
what: the DataTransfer model of dragged data
@task_file n=21 path=packages/dioxus-native-dom/src/dioxus_document.rs mark=merge
what: the Dioxus document — drag events handed to a Dioxus app
@task_file n=22 path=packages/dioxus-native-dom/src/events.rs mark=conflict
what: the Dioxus event conversion — the drag event and its modifiers through convert_modifiers, and a dropped file's reader over std::fs
@task_file n=23 path=packages/stylo_taffy/src/convert.rs mark=merge
what: the style-to-Taffy conversion — resolve_calc_value taking a CalcHandle under Taffy 0.16
@task_file n=24 path=packages/blitz-dom/src/layout_reason.rs mark=edits
what: escher's layout reason, read by Session::layout, the layout verb's answer and stand_layout — edits: its call to resolve_calc_value, passing a CalcHandle
@task_file n=25 path=tests/blitz-tests/tests/all.rs mark=merge
what: the blitz-tests target list — its new mod lines
@task_file n=26 path=tests/blitz-tests/tests/drag_and_drop.rs mark=merge
what: upstream's new drag-and-drop behaviour test
@task_file n=27 path=tests/blitz-tests/tests/flex_br.rs mark=merge
what: upstream's new test of a br after text in a flex container
@task_file n=28 path=tests/blitz-tests/tests/pointer_leave.rs mark=merge
what: the pointer-leave behaviour test, extended for preventDefault on a non-cancelable event
@task_file n=29 path=.github/scripts/ci-leg.sh mark=re-reads
what: the CI leg runner — the fast and doc legs re-run on the merged tree
@task_file n=30 path=.github/scripts/test_blitz_tests_targets.py mark=re-reads
what: the blitz-tests rule check — one file and one target per behaviour
@task_item n=1 part=needs
The layout reason and its readings — Session::layout, the escher-session layout verb's arguments and answer, and stand_layout — are a surface of this take: its calc resolution now passes Taffy 0.16's CalcHandle, and what it answers stays as read.
@task_item n=2 part=needs
Drops from other applications are a new behaviour the take brings: drag data reaches every windowed document's page as four DOM events, carried in a DataTransfer model; no harness input, driver verb, MCP tool or stand task reaches it.
@task_item n=3 part=needs
A Dioxus app's drop handler can read a dropped file's bytes through std::fs with no path restriction — a boundary widening the founder ratified on his own word.
@task_item n=4 part=needs
The drag event's modifiers convert through convert_modifiers under the Dioxus 0.7.3 hold; the hold is re-read and stands, with the keyboard_compat coverage exclusion.
@task_item n=5 part=needs
One-axis overflow clip now paints only the clipped axis, so paint agrees with the visible region; the hit on a box clipped on one axis still stops at its whole padding box, the stated limit — whether paint and hit part is measured.
@task_item n=6 part=needs
A br after text is kept in the text's anonymous flex item — a layout construction change the take brings.
@task_item n=7 part=needs
preventDefault on a non-cancelable event is ignored — an event dispatch change the take brings, reached by the pointer frame and click synthesis paths.
@task_item n=8 part=needs
Taffy moves from the pinned git rev to 0.16.0 from crates.io with a checksum, and blitz-shell gains url: the whole lock diff is read, the workspace builds with --locked, the dependency audit stays green and MSRV stays 1.91 unless a taken pin requires more.
@task_item n=9 part=needs
The workspace's one git dependency exception, Taffy's pinned rev, is retired by the take; no other git dependency enters.
@task_item n=10 part=needs
A take carries upstream's pins as one coupled set; a pin held where upstream moved it is an undo named with its reason.
@task_item n=11 part=needs
Each commit left out is named with its reason and undone in the merge's tree; the standing left-out set — parley_code_editor out of the workspace, upstream's AGENTS.md excluded, Dioxus held at 0.7.3 — is re-checked against the new upstream tip.
@task_item n=12 part=needs
The conflict resolution keeps both sides' intent: the changed set's contract, the clock invariant, the select model and its list, the hit stops and scrolled-box bounds, the into-view readings, the stable ids and the snapshot, the range and keyboard models, inert, the accessibility state reads.
@task_item n=13 part=needs
The resolution also keeps the custom-widget paint, the pointer frame and click synthesis, the stamped sub-resource request and the file: refusal, and the escher crates and seven_guis members of the workspace.
@task_item n=14 part=needs
Every escher proof passes on the merged tree: the stand checks in both layout modes, the incremental/non-incremental oracle, the a11y leg, the embolden witness, and fork CI green on the pushed commit with every job settled.
@task_item n=15 part=needs
The screenshot and cross-OS readings are re-read; a reading the take moves is re-measured and recorded, never loosened.
@task_item n=16 part=needs
Upstream's new test files run under this fork's blitz-tests rule: one file and one target per behaviour.
@task_item n=17 part=needs
The sink allowlist is re-checked over the merged delta: every new tracing or log call site, target, print and env read; the standing capture checks stay at 0 needles.
@task_item n=18 part=needs
A CI workflow edit keeps every action pinned to a 40-hex SHA and the workflow's contents: read; upstream's workflow files stay guarded by github.repository.
@task_item n=19 part=needs
The chunk adds no crate, port, env var or listener of its own.
@task_item n=20 part=needs
The merge-surface residuals are re-read against the merge; one a taken move resolves is re-measured, never assumed.
@task_item n=21 part=needs
No raw log carrying a temp or home path enters evidence; a probe worktree stands outside target/.
@task_standing n=1
convention: each new blitz test file is built and run with system fonts
home: typography
@task_item n=1 part=not_needed
Areas with no recorded rows — brand identity, border radius, anti-patterns, self-validation, the design decisions log: the take adds nothing they could rule.
@task_item n=2 part=not_needed
Colour rows — engine colours, select list colours, the color-scheme recascade, base color, the Circle Drawer canvas colours: the take changes no colour.
@task_item n=3 part=not_needed
Body rightmargin and bottommargin, and the open select list's placement in the viewport: no taken commit reaches them.
@task_item n=4 part=not_needed
The WPT runner's output and result colour contracts: upstream's WPT numbers are out of scope and the runner is not touched.
@task_item n=5 part=not_needed
The shell provider's minimize, maximize, decorations toggle and window drag contracts: a drop from another application is not a window gesture of the provider.
@task_item n=6 part=not_needed
Cursor icons, cursor resolution and the favicon: the take touches no cursor or icon.
@task_item n=7 part=not_needed
The escher-session task list — counter, crud, timer, flight-booker, cells, order, load — and its unknown-task answer: no task is added and no stand task reaches drops.
@task_item n=8 part=not_needed
The web-spa surface, WASM font context, the JS window sizes and mobile targets: the take reaches no WASM or mobile behaviour.
@task_item n=9 part=not_needed
Canvas draw commands and every later entry: out of this chunk.
