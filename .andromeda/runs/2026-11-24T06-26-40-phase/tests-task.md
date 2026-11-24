@task_prose part=chunk
The upstream sync that opens Epoch 4: a real two-parent merge of upstream/main at 29a439e05 into the build branch, taking all six upstream commits (a br in a flex text item, Taffy 0.16.0 from the registry, one-axis overflow clip in paint, drops from other applications, preventDefault ignored on a non-cancelable event, a file-drop example).
It resolves one conflict (the dioxus-native-dom event bridge's two import blocks) and makes two adaptations of ours: the layout reason's calc resolution under Taffy 0.16's CalcHandle, and the drag event's modifiers through convert_modifiers under the Dioxus 0.7.3 hold.
It measures the whole lock move and builds the workspace locked; runs upstream's new blitz-tests files under the one-target-per-behaviour rule.
It re-runs every escher proof on the merged tree (fast and doc legs, stand checks in both layout modes, the incremental oracle, the a11y leg, the audit, the embolden witness, the screenshot and cross-OS readings) and fork CI green.
It re-reads the merge-surface residuals, the left-out set, the Dioxus hold, the one-axis-clip hit limit and the sink allowlist against the merge.
@task_file n=1 path=packages/dioxus-native-dom/src/events.rs mark=conflict
what: the Dioxus bridge's event conversion — the one conflict of the merge, its two import blocks; the drag events and a dropped file's reader land here, and the drag event's modifiers pass through convert_modifiers under the Dioxus 0.7.3 hold
@task_file n=2 path=packages/blitz-dom/src/layout_reason.rs mark=edits
what: escher's layout reason, the reader behind Session::layout, the layout verb's answer and stand_layout — edits: its call to stylo_taffy's resolve_calc_value, which now takes taffy::CalcHandle
@task_file n=3 path=Cargo.toml mark=merge
what: the root workspace manifest — the Taffy dependency moving from a pinned git rev to 0.16.0 on crates.io, the dioxus-native dev features gaining net and html, the escher crates and seven_guis members kept
@task_file n=4 path=Cargo.lock mark=merge
what: the workspace lock — taffy 0.15.0 git to 0.16.0 registry with its checksum, and url in blitz-shell's dependency list
@task_file n=5 path=packages/blitz-shell/Cargo.toml mark=merge
what: the windowed shell's manifest — gains the url dependency for the drop source
@task_file n=6 path=packages/blitz-shell/src/dnd.rs mark=merge
what: the shell's winit drop source for drags from other applications
@task_file n=7 path=packages/blitz-traits/src/events/datatransfer.rs mark=merge
what: the DataTransfer model of dragged data in blitz-traits
@task_file n=8 path=packages/blitz-dom/src/events/driver.rs mark=merge
what: blitz-dom's event driver — drag events dispatched to the document, and preventDefault ignored on a non-cancelable event
@task_file n=9 path=packages/blitz-dom/src/layout/construct.rs mark=merge
what: layout tree construction — a br after text kept inside the text's anonymous flex item
@task_file n=10 path=packages/blitz-dom/src/layout/mod.rs mark=merge
what: the layout module's Taffy bindings, moved to Taffy 0.16
@task_file n=11 path=packages/blitz-dom/src/layout/replaced.rs mark=merge
what: replaced-element layout, moved to Taffy 0.16
@task_file n=12 path=packages/blitz-dom/src/layout/table.rs mark=merge
what: table layout, moved to Taffy 0.16
@task_file n=13 path=packages/stylo_taffy/src/convert.rs mark=merge
what: the style-to-Taffy conversion, its resolve_calc_value now taking a CalcHandle
@task_file n=14 path=packages/blitz-paint/src/render.rs mark=merge
what: the scene painter — a one-axis overflow clip clips only the clipped axis
@task_file n=15 path=examples/dnd.rs mark=merge
what: upstream's root drag-and-drop example
@task_file n=16 path=examples/file_drop.rs mark=merge
what: upstream's root file-drop example with previews of dropped content
@task_file n=17 path=tests/blitz-tests/tests/flex_br.rs mark=merge
what: upstream's new blitz-tests file for a br inside a flex text item
@task_file n=18 path=tests/blitz-tests/tests/drag_and_drop.rs mark=merge
what: upstream's new blitz-tests file for drops from other applications
@task_file n=19 path=tests/blitz-tests/tests/all.rs mark=merge
what: the blitz-tests module list, gaining upstream's two new mod lines
@task_file n=20 path=tests/blitz-tests/tests/pointer_leave.rs mark=merge
what: upstream's pointer-leave blitz-tests file, extended for a non-cancelable event
@task_file n=21 path=.github/scripts/test_blitz_tests_targets.py mark=re-reads
what: the check pinning one file and one target per behaviour in blitz-tests
@task_file n=22 path=.github/scripts/ci-leg.sh mark=re-reads
what: the CI leg script's fast and doc legs, re-run on the merged tree
@task_item n=1 part=needs
The sync is a real two-parent merge — never a rebase, squash or cherry-pick series — whose parents are the branch tip and upstream's 29a439e05; every one of the six commits is taken, and a left-out commit is one whose change is undone in the merge's tree.
@task_item n=2 part=needs
The left-out set (parley_code_editor out of the workspace, upstream's AGENTS.md excluded, Dioxus held at 0.7.3) is re-read against the new upstream commit, each with its reason; a reason that no longer holds puts the commit back in.
@task_item n=3 part=needs
The lock move: Taffy moves from the pinned git rev to 0.16.0 on the registry, so the workspace's one git-dependency exception is retired; the whole lock diff is measured and the workspace is built with --locked; upstream's pins are carried as one coupled set, and nothing is bumped beyond upstream's own set.
@task_item n=4 part=needs
The dependency audit (advisories and licenses) and the MSRV leg are read on the merged tree; MSRV stays 1.91 unless a taken pin requires more.
@task_item n=5 part=needs
The layout reason's adaptation to Taffy 0.16's CalcHandle: the layout reason and its readings — Session::layout, the layout verb's answer and stand_layout — are a surface of this take; every layout reader word stays read on a fixture, and layout assertions hold in both layout modes.
@task_item n=6 part=needs
The Dioxus bridge's event conflict resolved keeping both sides' intent, and the drag event's modifiers converted through convert_modifiers under the Dioxus 0.7.3 hold; the hold stands, with res-xpkmxnvl, res-xchmbfjr and the keyboard_compat coverage exclusion re-read as standing.
@task_item n=7 part=needs
A new boundary widening the take brings: drag data from other applications reaches every windowed document's page as four DOM drag events through a DataTransfer model and the shell's drop source, and a Dioxus app's handler can read a dropped file's bytes through std::fs with no path restriction; no harness input, driver verb, MCP tool or stand task reaches it, and no new crate, port, env var or listener of ours comes with it.
@task_item n=8 part=needs
A changed layout behaviour: a br after text stays in the text's anonymous flex item; the incremental and non-incremental layout oracle and the stand checks in both layout modes hold on it.
@task_item n=9 part=needs
A changed paint behaviour: a one-axis overflow clip clips only the clipped axis in paint, so paint agrees with the visible region, while a hit still stops at the whole padding box as the one-axis-clip readings state; whether the hit moves or a gap opens is measured, and stand_act_obstructed passes.
@task_item n=10 part=needs
A changed event behaviour: preventDefault is ignored on a non-cancelable event, carried by upstream's extended pointer-leave test.
@task_item n=11 part=needs
Upstream's new test files (the flex br and drag-and-drop files and the module list's two new lines) run under this fork's blitz-tests rule — one file and one target per behaviour — and the blitz-tests manifest keeps its stated difference from upstream; they are upstream-authored and kept as upstream wrote them.
@task_item n=12 part=needs
Upstream's two root examples join as example fixtures read by a human, carrying no automated assertion, with the dioxus-native dev features gaining net and html.
@task_item n=13 part=needs
Conflicts keep both sides' intent: the changed set's contract, the clock invariant, the select model and its list, the hit stops and scrolled-box bounds, the into-view readings, the stable ids and the snapshot, the range and keyboard models, inert, the accessibility state reads, the custom-widget paint, pointer frame and click synthesis, the stamped sub-resource request and the file: refusal.
@task_item n=14 part=needs
Every escher proof passes on the merged tree: the fast and doc CI legs, the stand checks in both layout modes under the zero-failure stand-repeat budget, the incremental oracle, the a11y leg, the dependency audit and the embolden local witness; fork CI green on the pushed commit with every job settled.
@task_item n=15 part=needs
The screenshot and cross-OS readings are re-measured; a moved reading is recorded, never loosened; the canvas shot's cross-OS digest is observed in CI only.
@task_item n=16 part=needs
The merge-surface residuals res-7qgqygtj, res-xpkmxnvl, res-xchmbfjr, res-vuvpa25q, res-q4s3p5ga, res-kk4tzmi5, res-derughab and res-ipcah32k are re-read against the merge; one the take resolves is re-measured, never assumed.
@task_item n=17 part=needs
The sink allowlist is re-checked over the merged delta — every new tracing or log call site, target, print and env read — and the standing capture checks stay at 0 needles; third-party targets stay dropped by the sink.
@task_item n=18 part=needs
Any CI workflow edit keeps every action pinned to a 40-hex SHA and the workflow's read-only contents permission; upstream's publishing and WPT workflows stay guarded to the upstream repository.
@task_item n=19 part=needs
The trial and probe worktrees stand outside the build directory and conflicts are measured with rerere disabled; no raw log carrying a temp or home path enters evidence; a guard refusal of a write is a stop.
@task_standing n=1
convention: one integration test target per behaviour and per file in blitz-tests, pinned by the targets check
home: 1-test-scope-summary · test-scope-summary
@task_standing n=2
convention: a regression test documents its bug and is named for it
home: 2-test-strategy
@task_standing n=3
convention: a new test is controlled red-before-green, pins its expected value as a literal and prints no argument value on failure
home: unit-test-strategy
@task_standing n=4
convention: layout assertions hold in both layout modes, and no check sleeps or reads the wall clock
home: integration-test-strategy
@task_standing n=5
convention: test data is stated in code, nothing seeded
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=6
convention: cargo legs run locked, and stand checks repeat three rounds under a zero-failure budget
home: 9-ci-integration · 10-quality-gates-coverage-targets
@task_item n=1 part=not_needed
The cold-agent pipe — its prompt, isolation, events, verdict, exit codes and replay — is not reached: the take adds no task, tool or recording an agent run reads.
@task_item n=2 part=not_needed
The agent-run five-command contract — its verbs, selections, events, state and exit codes — is untouched by an upstream merge.
@task_item n=3 part=not_needed
The session wire, the session host's lifecycle, the session errors, the session recording and the escher-session command line and MCP role are not reached: no driver verb, MCP tool or harness input carries the new drop input.
@task_item n=4 part=not_needed
The WPT runner's command line, outputs and status set, and upstream's WPT conformance numbers, are out of this chunk.
@task_item n=5 part=not_needed
The contrast readers and the WCAG contrast formula are not reached: no taken commit touches text colour or contrast.
@task_item n=6 part=not_needed
The committed recorded checks' recording script and its byte-equality to the binary's recording are not reached: no recorded check changes.
@task_item n=7 part=not_needed
Mutation baselines, scoped mutation filters and the cognitive-complexity ceiling are dev-host readings no step of this sync runs.
@task_item n=8 part=not_needed
Session idle expiry, owner-only session state and the socket's unix gating are not reached by the merged delta.
