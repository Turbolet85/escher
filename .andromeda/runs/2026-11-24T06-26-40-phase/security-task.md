@task_prose part=chunk
Syncs upstream into this fork as a real two-parent merge of upstream commit 29a439e05 into build/escher-0.4.0 — never a rebase, a squash or a cherry-pick series — taking all six upstream commits: a line break kept in a flex text item, Taffy 0.16.0 from crates.io in place of the git rev, a one-axis overflow clip painted on its clipped axis alone, drops from other applications (a DataTransfer model, drag events, a winit drop source, a dropped file's reader over std::fs), preventDefault ignored on a non-cancelable event, and two root examples with net and html dev features.
Resolves the one conflict (the Dioxus native event bridge's import blocks) and makes two adaptations of ours: the layout reason's calc resolution under Taffy's CalcHandle, and the drag event's modifiers through convert_modifiers under the Dioxus 0.7.3 hold.
Measures the whole lock diff with a locked workspace build, the audit and the MSRV leg; collects every widening and the left-out set in the founder forks evidence; the left-out set stays out and its re-check goes back to the founder.
Re-reads the Dioxus hold, the merge-surface residuals, the one-axis clip stated limit and the sink allowlist over the merged delta; re-runs every escher proof and fork CI on the pushed commit.
Deletes nothing: the trial worktree and its build directories stay in place, and no plan step removes them.
@task_file n=1 path=packages/dioxus-native-dom/src/events.rs mark=conflict
what: the Dioxus native event bridge — the conflict in its two import blocks, and the drag event's modifiers converted through convert_modifiers under the Dioxus hold
@task_file n=2 path=packages/blitz-dom/src/layout_reason.rs mark=edits
what: escher's layout reason, behind the in-process layout read and the layout verb's answer — edits: its call to the calc resolver, now passing Taffy's CalcHandle
@task_file n=3 path=Cargo.toml mark=merge
what: the root manifest and its workspace dependency pins — Taffy moved from a git rev to the 0.16.0 registry release; dioxus-native dev features gain net and html
@task_file n=4 path=Cargo.lock mark=merge
what: the workspace lockfile — taffy 0.15.0 git to 0.16.0 registry with its checksum, and url in the shell's dependency list
@task_file n=5 path=packages/blitz-shell/Cargo.toml mark=merge
what: the windowed shell's manifest — gains the url dependency
@task_file n=6 path=packages/blitz-shell/src/dnd.rs mark=merge
what: the shell's winit drop source — drag data and dropped files from other applications enter the document
@task_file n=7 path=packages/blitz-traits/src/events/datatransfer.rs mark=merge
what: the DataTransfer model of a drag's data in the shared event types
@task_file n=8 path=packages/blitz-dom/src/events/driver.rs mark=merge
what: the engine's event driver — drag events dispatched, and preventDefault ignored on a non-cancelable event
@task_file n=9 path=packages/blitz-dom/src/layout/construct.rs mark=merge
what: layout tree construction — a line break after text kept in the text's anonymous flex item
@task_file n=10 path=packages/blitz-dom/src/layout/mod.rs mark=merge
what: the engine's layout bridge to Taffy, moved to Taffy 0.16
@task_file n=11 path=packages/blitz-dom/src/layout/replaced.rs mark=merge
what: replaced-element layout, moved to Taffy 0.16
@task_file n=12 path=packages/blitz-dom/src/layout/table.rs mark=merge
what: table layout, moved to Taffy 0.16
@task_file n=13 path=packages/stylo_taffy/src/convert.rs mark=merge
what: the style-to-Taffy conversion, holding the calc resolver the layout reason calls
@task_file n=14 path=packages/blitz-paint/src/render.rs mark=merge
what: the paint renderer — a one-axis overflow clip clips the clipped axis alone
@task_file n=15 path=examples/dnd.rs mark=merge
what: a root example of drops from other applications
@task_file n=16 path=examples/file_drop.rs mark=merge
what: a root example previewing dropped file content
@task_file n=17 path=tests/blitz-tests/tests/flex_br.rs mark=merge
what: upstream's new engine test of a line break in a flex item, under the blitz-tests one-file-one-target rule
@task_file n=18 path=tests/blitz-tests/tests/drag_and_drop.rs mark=merge
what: upstream's new engine test of drag and drop, under the blitz-tests one-file-one-target rule
@task_file n=19 path=tests/blitz-tests/tests/pointer_leave.rs mark=merge
what: the engine's pointer-leave test, extended for preventDefault on a non-cancelable event
@task_file n=20 path=tests/blitz-tests/tests/all.rs mark=merge
what: the blitz-tests module list — gains the new test modules
@task_file n=21 path=.github/scripts/test_blitz_tests_targets.py mark=re-reads
what: the check pinning one test file and one target per behaviour in blitz-tests
@task_file n=22 path=.github/scripts/ci-leg.sh mark=re-reads
what: the CI legs re-run after the merge — fast and doc
@task_item n=1 part=needs
The upstream sync itself: a two-parent merge whose parents are this branch's tip and the upstream commit taken; each commit left out is undone in the merge's tree and named with its reason; the standing left-out set (a code-editor package out of the workspace, upstream's agent guide excluded, Dioxus held at 0.7.3) is re-read against the new upstream, and a reason that no longer holds puts the commit back in.
@task_item n=2 part=needs
The dependency set the take carries as one coupled set: Taffy from a git rev to the 0.16.0 registry release with a checksum, and url added to the windowed shell — no dependency moved beyond upstream's own set, the git-dependency exception retired, the root manifest left with no git dependency.
@task_item n=3 part=needs
The lock move: the whole merged lock diff measured against ours, the workspace built with --locked, the dependency audit (advisories and licences) green on the merged lock, and MSRV staying 1.91 unless a taken pin requires more, then named.
@task_item n=4 part=needs
The boundary widening the merge carries: drag data from other applications reaches every windowed document's page as four DOM events through a winit drop source and a DataTransfer model — ratified on the founder's word, recorded in the founder forks evidence with what crosses, the rule it meets, its options, measured cost and dependences.
@task_item n=5 part=needs
A dropped file's bytes readable by a Dioxus app's handler through std::fs with no path restriction — a new file-read input of the Dioxus native bridge; no harness input, driver verb, MCP tool or stand task reaches it.
@task_item n=6 part=needs
The layout reason under Taffy 0.16: its calc resolution now takes Taffy's CalcHandle, so the layout reason and its readings — the in-process layout read on a session, the layout verb's answer, the kept check's layout answer and stand_layout — are a surface of this take and read as before.
@task_item n=7 part=needs
The Dioxus hold at 0.7.3 re-read and standing (no measured reason to move): the drag event's modifiers converted through convert_modifiers under the hold, and the residuals res-xpkmxnvl, res-xchmbfjr and the keyboard_compat coverage exclusion reading standing.
@task_item n=8 part=needs
The conflict in the Dioxus native event bridge resolved keeping both sides' intent — the changed set's contract, the clock invariant, the select model and its list, the hit stops and scrolled-box bounds, the into-view readings, the stable ids and the snapshot, the range and keyboard models, inert, the accessibility state reads, the custom-widget paint, pointer frame and click synthesis, the stamped sub-resource request and the file: refusal, and the escher crates and seven_guis workspace members.
@task_item n=9 part=needs
The event driver's change: preventDefault ignored on a non-cancelable event, beside the existing click, focus and pointer dispatch rules the driver keeps.
@task_item n=10 part=needs
The one-axis overflow clip: paint now clips only the clipped axis and agrees with the visible region, while a hit still stops at the whole padding box — the stated limit re-read and measured, stand_act_obstructed passing.
@task_item n=11 part=needs
Layout behaviour the take changes — a line break after text kept in its anonymous flex item, and Taffy 0.16 under flex, replaced and table layout — with every moved screenshot or cross-OS reading re-measured and recorded, never loosened.
@task_item n=12 part=needs
The root examples gain net and html dioxus-native dev features — outbound fetching through the engine's network provider enters example builds only.
@task_item n=13 part=needs
The sink allowlist re-checked over the merged delta: every new tracing or log call site, target, print and env read read; the escher telemetry sink's target allowlist and content-field scrub hold, and the standing capture checks stay at 0 needles.
@task_item n=14 part=needs
Upstream's new tests run under this fork's blitz-tests rule — one file and one target per behaviour — pinned by the blitz-tests target check.
@task_item n=15 part=needs
Every escher proof passes after the merge: the fast and doc CI legs, the stand checks in both layout modes, the incremental and non-incremental oracle, the a11y leg, the dependency audit, the embolden witness, the screenshot and cross-OS readings, and fork CI green on the pushed commit with every job settled.
@task_item n=16 part=needs
CI workflow edits, where the merge carries any: every action pinned to a 40-hex SHA, ci.yml's token kept at contents: read, upstream's workflow files kept guarded by the repository check.
@task_item n=17 part=needs
No new crate, port, env var or listener of ours: the driver session socket stays the one listener and RUST_LOG the one env read of the driver.
@task_item n=18 part=needs
The merge-surface residuals res-7qgqygtj, res-xpkmxnvl, res-xchmbfjr, res-vuvpa25q, res-q4s3p5ga, res-kk4tzmi5, res-derughab and res-ipcah32k re-read against the merge; one a taken move resolves is re-measured, never assumed.
@task_item n=19 part=needs
Evidence hygiene and the trial: no raw log carrying a temp or home path enters evidence; a probe worktree stands outside target/ and its merge runs with rerere disabled; a guard hook's refusal of a write is a stop; the trial worktree and its build directories stay in place, removed by no step.
@task_item n=20 part=needs
The last wrap commit's CI verdict re-read before this chunk's push, never folded as green unread.
@task_standing n=1
convention: every new log call site or target the merge brings passes the telemetry sink's target allowlist, with content-named fields withheld
home: logging-monitoring
@task_standing n=2
convention: every dependency a take brings passes the cargo-deny advisory and licence audit under a locked build, with no git dependency
home: dependency-security
@task_standing n=3
convention: every evidence file holds no credentials and no raw log with a temp or home path
home: secret-management
@task_standing n=4
convention: every CI job keeps a read-only token and publishing guarded to upstream
home: authentication-authorization
@task_item n=1 part=not_needed
Cold-agent runs — their call allowlist, fixed prompt, recordings, transcripts and replay isolation: the sync touches no cold-agent script or its records.
@task_item n=2 part=not_needed
Recorded check files and the recording directory — their strict reader, owner-only directory and failure endings: no recording path changes in the take.
@task_item n=3 part=not_needed
The WPT runner and the WPT post-results workflow — its results, quarantine entries and PR lookup: upstream's WPT conformance is out of scope.
@task_item n=4 part=not_needed
The browser app's history store in sqlite, its SQL placeholders and database encryption: no browser app code is in the delta.
@task_item n=5 part=not_needed
Helper scripts outside the merge — the apt install script, the agent-run selection and the coverage floor checker: none is in the delta.
@task_item n=6 part=not_needed
The opt-in CDP devtools server and rdme's raw HTML passthrough: neither is touched by the take.
@task_item n=7 part=not_needed
The JavaScript engine's rows — script fetcher schemes, JS fetch methods, CSSOM script API, module loading and the URLSearchParams bootstrap: the take carries no script engine change.
@task_item n=8 part=not_needed
MCP protocol errors, the MCP initialize revision and MCP tool result bounds: no MCP tool, verb or result shape changes.
@task_item n=9 part=not_needed
Compliance controls, security anti-patterns and the security decisions log: those areas carry no recorded intent.
