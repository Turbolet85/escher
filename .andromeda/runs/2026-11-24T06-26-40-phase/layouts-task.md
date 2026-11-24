@task_prose part=chunk
Syncs this fork with upstream as one real two-parent merge of upstream/main at 29a439e05 into build/escher-0.4.0 (merge base 165d698c3, six commits) — never a rebase, squash or cherry-pick series.
The take brings Taffy 0.16.0 from the registry in place of the pinned git rev, a br after text kept in its anonymous flex item, one-axis overflow clip painting the clipped axis alone, drops from other applications (a drag data model, drag events, an OS drop source, a dropped file's reader), preventDefault ignored on non-cancelable events, and two root examples.
It resolves one conflict (the native Dioxus document's event import blocks) and makes two adaptations of ours: the layout reason under Taffy's calc handle and the drag event's modifiers under the Dioxus hold.
It re-reads the left-out set, the Dioxus hold, the merge-surface residuals, the one-axis clip hit limit and the sink allowlist, and re-runs every escher proof (fast and doc legs, stand checks in both layout modes, the incremental oracle, the a11y leg, the audit, the embolden witness, screenshot and cross-OS readings, fork CI).
@task_file n=1 path=Cargo.toml mark=merge
what: the workspace root manifest — the coupled dependency pins (Taffy moved from a git rev to the 0.16.0 registry release), the escher crates and seven_guis members of the workspace, and dioxus-native's dev features gaining net and html for the root examples
@task_file n=2 path=Cargo.lock mark=merge
what: the workspace lock — taffy 0.15.0 git to 0.16.0 registry with its checksum, and url in blitz-shell's dependency list; read whole against ours and built --locked
@task_file n=3 path=packages/blitz-shell/Cargo.toml mark=merge
what: the windowed shell crate's manifest — gains the url dependency for the OS drop source
@task_file n=4 path=packages/dioxus-native-dom/src/events.rs mark=conflict
what: the native Dioxus document's event conversion — the conflict in its two import blocks, and the drag event's modifiers through convert_modifiers under the Dioxus 0.7.3 hold
@task_file n=5 path=packages/blitz-dom/src/layout_reason.rs mark=edits
what: escher's layout reason, read by Session::layout, the layout verb's answer and stand_layout — edits: its call to resolve_calc_value, which under Taffy 0.16 takes Taffy's CalcHandle
@task_file n=6 path=packages/blitz-dom/src/layout/construct.rs mark=merge
what: layout tree construction — a br after text kept in the text's anonymous flex item
@task_file n=7 path=packages/blitz-dom/src/layout/mod.rs mark=merge
what: the document's layout over Taffy — moved to Taffy 0.16.0
@task_file n=8 path=packages/blitz-dom/src/layout/replaced.rs mark=merge
what: replaced-element layout over Taffy — moved to Taffy 0.16.0
@task_file n=9 path=packages/blitz-dom/src/layout/table.rs mark=merge
what: table layout over Taffy — moved to Taffy 0.16.0
@task_file n=10 path=packages/stylo_taffy/src/convert.rs mark=merge
what: the style-to-Taffy conversion, home of resolve_calc_value — moved to Taffy 0.16.0 and its calc handle
@task_file n=11 path=packages/blitz-paint/src/render.rs mark=merge
what: the paint of a document — a one-axis overflow clip clips the clipped axis alone
@task_file n=12 path=packages/blitz-dom/src/events/driver.rs mark=merge
what: the document's event driver — drag events from other applications, and preventDefault ignored on a non-cancelable event
@task_file n=13 path=packages/blitz-traits/src/events/datatransfer.rs mark=merge
what: the DataTransfer model of dragged data from other applications
@task_file n=14 path=packages/blitz-shell/src/dnd.rs mark=merge
what: the winit drop source feeding drops from other applications into a windowed document
@task_file n=15 path=examples/dnd.rs mark=merge
what: upstream's root drag-and-drop example
@task_file n=16 path=examples/file_drop.rs mark=merge
what: upstream's root file_drop example with previews of dropped content
@task_file n=17 path=tests/blitz-tests/tests/flex_br.rs mark=merge
what: upstream's new blitz test of a br in a flex item, under the one-file-one-target-per-behaviour rule
@task_file n=18 path=tests/blitz-tests/tests/drag_and_drop.rs mark=merge
what: upstream's new blitz test of drag and drop, under the one-file-one-target-per-behaviour rule
@task_file n=19 path=tests/blitz-tests/tests/pointer_leave.rs mark=merge
what: the blitz test of pointer leave, extended upstream for preventDefault on a non-cancelable event
@task_file n=20 path=tests/blitz-tests/tests/all.rs mark=merge
what: the blitz tests' module list — gains upstream's new mod lines
@task_file n=21 path=.github/scripts/test_blitz_tests_targets.py mark=re-reads
what: the check pinning one file and one target per behaviour in blitz-tests
@task_file n=22 path=.github/scripts/ci-leg.sh mark=re-reads
what: the CI legs re-run after the merge — fast and doc
@task_item n=1 part=needs
The take itself: a two-parent merge whose parents are this branch's tip and upstream's 29a439e05; each commit left out named with its reason, a left-out change being undone in the merge's tree; the standing left-out set (parley_code_editor out of the workspace, upstream's AGENTS.md excluded, Dioxus held at 0.7.3) re-read against the new upstream, a reason that no longer holds putting the commit back in.
@task_item n=2 part=needs
The Taffy move: upstream's pins taken as one coupled set, Taffy 0.16.0 from the registry replacing the pinned git rev, so the workspace's one git-dependency exception is retired; the whole lock diff enters the plan, a --locked workspace build runs on the merged tree, the dependency audit and the MSRV leg beside it, MSRV staying 1.91 unless a taken pin requires more.
@task_item n=3 part=needs
The layout reason under Taffy 0.16: its calc resolution adapted to Taffy's calc handle, and its readings — Session::layout, the session's layout verb answer and the stand_layout check — kept as they read.
@task_item n=4 part=needs
Drops from other applications, a new input boundary: drag data reaches every windowed document's page as four DOM events through the OS drop source, the drag data model and the driver's drag events; no harness input, driver verb, MCP tool or stand task reaches it; ratified on the founder's own word.
@task_item n=5 part=needs
A dropped file's reader in the native Dioxus document reads a dropped file's bytes through std::fs (metadata, read, read_to_string, open) with no path restriction — a file-read widening the take carries, ratified by the founder, beside the stamped sub-resource request and the file: refusal it must keep.
@task_item n=6 part=needs
The drag event's modifiers converted through convert_modifiers under the Dioxus 0.7.3 hold; the hold stands (no measured reason re-reads it), with its residuals and the keyboard_compat coverage exclusion read standing.
@task_item n=7 part=needs
One-axis overflow clip: paint now clips only the clipped axis, so paint agrees with the visible region through clipping boxes; the hit still stops at the whole padding box as the one-axis clip readings decision states; whether the merge moves the hit, the paint alone or opens a gap is measured, and the obstructed-act stand check still passes.
@task_item n=8 part=needs
A br after text stays in the text's anonymous flex item — a layout change the take brings to every document, read back by the stand checks in both layout modes and the incremental/non-incremental oracle.
@task_item n=9 part=needs
preventDefault is ignored on a non-cancelable event in the event driver — an event-model change the take brings, beside the pointer frame and click synthesis it must keep.
@task_item n=10 part=needs
Conflicts keep both sides' intent: the changed set's contract, the clock invariant, the select model and its list, hit stops and scrolled-box bounds, the into-view readings, stable ids and the snapshot, the range and keyboard models, inert, the accessibility state reads, custom-widget paint, pointer frame and click synthesis, the stamped sub-resource request and the file: refusal, and the escher crates and seven_guis members of the workspace.
@task_item n=11 part=needs
Upstream's two root examples and the root manifest's dioxus-native dev features gaining net and html — taken as upstream's own set, no dependency bumped beyond it.
@task_item n=12 part=needs
Upstream's new blitz tests (flex_br, drag_and_drop, and the module list's new mod lines) run under this fork's blitz-tests rule: one file and one target per behaviour, pinned by the targets check.
@task_item n=13 part=needs
Every escher proof still passes after the merge: the fast and doc CI legs, the stand checks in both layout modes, the incremental/non-incremental oracle, the a11y leg and the dependency audit, the embolden witness, the screenshot and cross-OS readings (a moved reading re-measured and recorded, never loosened), and fork CI green on the pushed commit with every job settled.
@task_item n=14 part=needs
The sink allowlist re-checked over the merged delta: every new tracing or log call site, target, print and env read; the standing capture checks stay at 0 needles.
@task_item n=15 part=needs
The merge-surface residuals re-read against the merge, one a taken move resolves re-measured, never assumed.
@task_item n=16 part=needs
CI workflow edits, where the merge brings any: every action pinned to a 40-hex SHA, ci.yml keeping contents: read, upstream's workflow files staying guarded by github.repository.
@task_item n=17 part=needs
No new crate, port, env var or listener of ours; the url dependency enters only as upstream's.
@task_item n=18 part=needs
Evidence hygiene and probing: no raw log carrying a temp or home path enters evidence; the trial worktree stands outside target/ and conflicts are measured with rerere disabled; a guard hook's refusal of a write is a stop.
@task_item n=1 part=not_needed
The cold-agent pipe and its command line (usage, exit codes, record dir, Windows pass-through) and the cold-agent decisions: the take adds no agent run and changes no prompt.
@task_item n=2 part=not_needed
The agent-run.sh test contract: the take changes neither its usage, its log nor its exit codes.
@task_item n=3 part=not_needed
The multi_doc and paint_bench command lines: no argument, env knob, backend or output of theirs moves.
@task_item n=4 part=not_needed
The MCP server's transport, protocol revisions, tools list, result bound and screenshot result: the drop input reaches no MCP tool and the take adds none.
@task_item n=5 part=not_needed
The ShellProvider window chrome controls (window drag, maximize, minimize, close, decorations): the OS drop of other applications' data is an input, not window chrome.
@task_item n=6 part=not_needed
The session's served tasks, exit codes, refusals and recording files: no verb, task, refusal or exit code is added or changed by the take.
@task_item n=7 part=not_needed
Stand page design decisions — the accent colour, the fixed document skeleton with its main mount, the order page built to cause, the CRUD list's keyboard region, a held instance reading as a fresh boot, the load page's network provider: no stand app is changed.
@task_item n=8 part=not_needed
The mobile-native and web-spa surfaces: not measured, and the take reaches neither.
@task_item n=9 part=not_needed
The absence of arbitrary index.html templates and the decisions log: the take adds no document template and records no new design intent.
