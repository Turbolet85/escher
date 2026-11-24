@task_prose part=chunk
Syncs upstream/main at 29a439e05 into build/escher-0.4.0 as a real two-parent merge (never a rebase, squash or cherry-pick), taking all six upstream commits: a br kept in its flex item, Taffy 0.16.0 from crates.io in place of the git rev, one-axis overflow clip in paint, drops from other applications (a DataTransfer model, drag events, a winit drop source, a dropped file's reader over std::fs), preventDefault ignored on a non-cancelable event, and a file_drop example.
Resolves the one conflict in the Dioxus event conversion's import blocks and makes two adaptations of ours: the layout reason's calc resolution under Taffy's CalcHandle, and the drag event's modifiers through convert_modifiers under the Dioxus 0.7.3 hold.
Re-reads the standing left-out set, the Dioxus hold, the merge-surface residuals and the one-axis clip limit; re-checks the sink allowlist over the merged delta for every new tracing or log call site, target, print and env read.
Re-runs every escher proof after the merge: the CI legs, stand checks in both layout modes, the incremental oracle, the a11y leg, the dependency audit, screenshot and cross-OS readings, and fork CI green on the pushed commit; a moved reading is re-measured, never loosened.
@task_file n=1 path=packages/dioxus-native-dom/src/events.rs mark=conflict
what: the Dioxus event conversion of the dioxus-native-dom document, its two import blocks in conflict, and the drag event's modifiers through convert_modifiers under the Dioxus hold
@task_file n=2 path=packages/blitz-dom/src/layout_reason.rs mark=edits
what: escher's layout reason, the source of the layout read (Session::layout, the layout verb's answer, stand_layout) — edits: its call to the calc-value resolution, taking taffy::CalcHandle under Taffy 0.16
@task_file n=3 path=Cargo.toml mark=merge
what: the root workspace manifest — the Taffy dependency moved from the git rev to the 0.16.0 registry release, the dioxus-native dev features gaining net and html, and the escher crates and seven_guis members of the workspace
@task_file n=4 path=Cargo.lock mark=merge
what: the workspace lock file — taffy 0.15.0 git to 0.16.0 registry with its checksum, and url in blitz-shell's dependency list
@task_file n=5 path=packages/blitz-shell/Cargo.toml mark=merge
what: the windowed shell's manifest, gaining the url dependency
@task_file n=6 path=packages/blitz-shell/src/dnd.rs mark=merge
what: the windowed shell's winit drop source for drag data from other applications
@task_file n=7 path=packages/blitz-traits/src/events/datatransfer.rs mark=merge
what: the DataTransfer model of drag data in the shared event traits
@task_file n=8 path=packages/blitz-dom/src/events/driver.rs mark=merge
what: the engine's event driver — the drag events, and preventDefault ignored on a non-cancelable event
@task_file n=9 path=packages/blitz-dom/src/layout/construct.rs mark=merge
what: the engine's layout tree construction, a br after text kept in the text's anonymous flex item
@task_file n=10 path=packages/blitz-dom/src/layout/mod.rs mark=merge
what: the engine's layout module, moved onto Taffy 0.16
@task_file n=11 path=packages/blitz-dom/src/layout/replaced.rs mark=merge
what: the engine's replaced-element layout, moved onto Taffy 0.16
@task_file n=12 path=packages/blitz-dom/src/layout/table.rs mark=merge
what: the engine's table layout, moved onto Taffy 0.16
@task_file n=13 path=packages/stylo_taffy/src/convert.rs mark=merge
what: the style-to-Taffy conversion, its calc-value resolution taking a CalcHandle
@task_file n=14 path=packages/blitz-paint/src/render.rs mark=merge
what: the paint renderer, a one-axis overflow clip clipping the clipped axis alone
@task_file n=15 path=examples/dnd.rs mark=merge
what: upstream's root drag-and-drop example
@task_file n=16 path=examples/file_drop.rs mark=merge
what: upstream's root file_drop example with previews of dropped content
@task_file n=17 path=tests/blitz-tests/tests/flex_br.rs mark=merge
what: upstream's new engine test of a br in a flex item, run under the blitz-tests one-file-one-target rule
@task_file n=18 path=tests/blitz-tests/tests/drag_and_drop.rs mark=merge
what: upstream's new engine test of drag and drop, run under the blitz-tests one-file-one-target rule
@task_file n=19 path=tests/blitz-tests/tests/all.rs mark=merge
what: the blitz-tests aggregate test file, gaining the new mod lines
@task_file n=20 path=tests/blitz-tests/tests/pointer_leave.rs mark=merge
what: the engine's pointer-leave test, extended for a non-cancelable event
@task_file n=21 path=.github/scripts/test_blitz_tests_targets.py mark=re-reads
what: the check pinning the blitz-tests rule, one file and one target per behaviour
@task_file n=22 path=.github/scripts/ci-leg.sh mark=re-reads
what: the CI legs run after the merge, fast and doc
@task_item n=1 part=needs
The sink target allowlist re-checked over the merged delta: every new tracing or log call site, target, print and env read the six commits bring is read, and the standing capture checks stay at 0 needles.
@task_item n=2 part=needs
Engine tracing stays behind the tracing cargo feature: any tracing call site the take brings into the engine, shell or traits crates compiles in only with that feature, and escher's own crates keep tracing ungated.
@task_item n=3 part=needs
Engine log sites the take brings write fixed messages and print safe fields only: the drag events, the drop source and the dropped file's reader log no dropped content, file name, path or url; a path or url field falls under escher-telemetry's scrub set.
@task_item n=4 part=needs
The OS drop input and its file reads are a new boundary widening: drag data from other applications reaches every windowed document's page as four DOM events, and a Dioxus handler can read a dropped file's bytes through std::fs; no harness input, driver verb, MCP tool or stand task reaches it, and no page content from it enters a session log.
@task_item n=5 part=needs
The layout reason is a surface of this take: its calc resolution moves to Taffy's CalcHandle, and its readings — Session::layout, the layout verb's answer and its content, stand_layout — must read as before; the layout read opens no span, and the command span's layout field and the layout answer's logging rule hold.
@task_item n=6 part=needs
preventDefault ignored on a non-cancelable event changes what the event driver reports to page script; the page script events and the script error surface still read as their contracts state.
@task_item n=7 part=needs
The one-axis overflow clip moves paint alone: the clipped axis alone is clipped, paint now agrees with the visible region, and the hit still stops at the whole padding box; the screenshot and cross-OS readings that move are re-measured and recorded, never loosened.
@task_item n=8 part=needs
The Taffy move retires the workspace's one git-dependency exception; the lock moves as one coupled set with its whole lock diff and a locked workspace build; the dependency audit reads advisories ok and licenses ok; MSRV stays 1.91.
@task_item n=9 part=needs
No new crate, port, env var or listener of ours; no telemetry backend, exporter or metric emission arrives with the take.
@task_item n=10 part=needs
Conflicts keep both sides' intent: the select model and its list, the stable ids and the snapshot, the accessibility state reads, the stamped sub-resource request and the file: refusal for non-file documents hold across the merge.
@task_item n=11 part=needs
Evidence hygiene: no raw log carrying a temp or home path enters the chunk's evidence; the trial worktree's and builds' raw output is read only, never copied in.
@task_item n=12 part=needs
CI workflow edits, if the merge brings any, keep every action pinned to a 40-hex SHA and contents: read; upstream's workflow files stay guarded by github.repository; fork CI's artifacts stay as they are.
@task_item n=13 part=needs
Fork CI green on the pushed merge commit, every job settled, including the a11y leg, the audit and the macOS and Windows tests; the last wrap's session commit verdict is re-read before the push.
@task_item n=14 part=needs
Each commit left out is named with its reason in the left-out re-check: the standing left-out set (parley_code_editor out of the workspace, upstream's AGENTS.md excluded, Dioxus held at 0.7.3) re-read against 29a439e05.
@task_standing n=1
convention: a new test or test file runs silent — no print to stdout or stderr on a pass — and an empty-stderr check unsets RUST_LOG
home: 3-observability-harness-contract
@task_item n=1 part=not_needed
The cold-agent pipe, its events, verdict, counted verbs and kept checks: the take touches no cold-agent run.
@task_item n=2 part=not_needed
The agent-run harness events and its raw log: the chunk adds no agent run.
@task_item n=3 part=not_needed
Session recording, the recording file, the recording and session state directories and the session lifecycle: the merge brings nothing to the session's recording.
@task_item n=4 part=not_needed
escher-session's exit codes, host wire, MCP role stdout, coverage role and error object: no driver verb or session role changes.
@task_item n=5 part=not_needed
The build-directory sweep's stdout line: the chunk runs no sweep and deletes no build directory.
@task_item n=6 part=not_needed
WPT score publishing and the WPT report: upstream's WPT conformance numbers are out of scope.
@task_item n=7 part=not_needed
The WASM console panic hook: the chunk builds no WASM target.
@task_item n=8 part=not_needed
The package-install diagnostic line in CI: the merge adds no package install.
