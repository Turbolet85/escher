@task_prose part=chunk
Takes upstream/main at the pinned sha 563d3db73 into the fork, in the shape the founder picks at P4: a cherry-pick on the registry pins, that plus the registry moves (Stylo 0.24, Parley 0.13, winit beta.4), the whole merge, or nothing; conflicts are resolved keeping both sides' intent.
Folds two external CI reds: the clippy needless_bool lint under rustc 1.100 and the dependency-audit advisory on the SQLite crate behind the browser's persistence.
Carries the folded freight: the 21 commits left out last time re-judged, the merge surfaces and the buffered render re-read, two headless frame tests (blend/isolation and an inline box's own decorations) in both layout modes, and every action in the three remaining workflows pinned to a commit SHA, with the pinning check widened to every workflow.
Re-runs every escher proof after the take (fast, doc, a11y and audit legs, stand checks in both layout modes, the incremental oracle, fork CI green), re-reads the per-OS test tallies, and re-checks the log sink allowlist against the taken delta.
Re-measures any layout, text, paint or contrast number the take moves, and lists every boundary widening (new attribute reads, the AccessKit refresh, new paint, DevTools) for the operator at P4.
@task_file n=1 path=packages/blitz-dom/src/document.rs mark=edits
what: the engine document: node allocation, anonymous block boxes and their deallocation, a merge surface of the last take — edits: the needless_bool site, deallocate_anonymous_block's fix as the take reaches it
@task_file n=2 path=packages/blitz-dom/src/layout/damage.rs mark=edits
what: the engine's layout damage tracking — edits: the two needless_bool sites
@task_file n=3 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: the Dioxus document: element ids, the snapshot, the reached-handler record and the buffered render's two render sites and its buffered mutations field
@task_file n=4 path=packages/dioxus-native-dom/src/mutation_buffer.rs
what: the mutation buffer the Dioxus render writes into, replaced whole if the Dioxus 0.8 move is taken
@task_file n=5 path=packages/dioxus-native-dom/src/lib.rs
what: the Dioxus native document crate's module list, one mod line for the mutation buffer
@task_file n=6 path=.github/workflows/wpt.yml mark=edits
what: the upstream-only WPT workflow — edits: every action pinned to a 40-hex commit SHA with its ref as a trailing comment
@task_file n=7 path=.github/workflows/publish-browser.yml mark=edits
what: the browser publishing workflow — edits: every action pinned to a 40-hex commit SHA with its ref as a trailing comment
@task_file n=8 path=.github/workflows/wpt-post-results.yml mark=edits
what: the WPT results posting workflow — edits: every action pinned to a 40-hex commit SHA with its ref as a trailing comment
@task_file n=9 path=.github/scripts/test_ci_workflows.py mark=edits
what: the CI workflow guard checking actions are pinned to SHAs — edits: the pinning check, widened from ci.yml to every workflow
@task_file n=10 path=.github/scripts/test_blitz_tests_targets.py mark=edits
what: the check pinning the blitz-tests target list, one file and one target per behaviour — edits: the target list, for upstream's new test files and the two new frame tests
@task_file n=11 path=.github/scripts/ci-leg.sh mark=re-reads
what: the CI legs re-run after the take: fast, doc, the a11y leg and the dependency audit
@task_file n=12 path=packages/escher-telemetry/src/format.rs mark=re-reads
what: the log sink's target allowlist and its standing capture checks, held at 0 needles
@task_file n=13 path=Cargo.toml
what: the workspace manifest: members and the coupled dependency pins (Stylo, Taffy, Parley, AnyRender/vello, winit, Dioxus)
@task_file n=14 path=Cargo.lock
what: the dependency lock, read for new crates and the SQLite advisory chain
@task_item n=1 part=needs
The accessibility tree built through the document's own method and refreshed on change: upstream's AccessKit refresh may reach how and when the tree is rebuilt, and the resolution keeps escher's build and refresh.
@task_item n=2 part=needs
The accessibility state reads (ARIA states on every element, checked, pressed, selected, invalid, expanded, the native checkbox and select option states) kept through any conflict the take raises.
@task_item n=3 part=needs
Role mapping and accessible name rules: upstream's empty alt read as decorative is a new attribute read that may change a node's role or name; it is a boundary widening listed for the operator at P4.
@task_item n=4 part=needs
Hidden and inert content kept out of the tree, inert elements taking no hit and focus fixup reading inertness alone: inert is an engine surface escher changed and the resolution keeps it.
@task_item n=5 part=needs
The select model and its engine overlay list: closed- and open-list keys, the list's rows, colours, row height, no motion, focus staying on the select, and the expanded reading, all kept through the take.
@task_item n=6 part=needs
The range input and keyboard activation models (Enter and Space as a click, range keys, pointerdown and click), and Tab focus movement, kept through the take; upstream's text-input copy firing no input event is read against the form event rules.
@task_item n=7 part=needs
The changed set the engine marks and what the harness settle leaves in it, with the qualifier that a scroll closing an open list marks its select, landing as a wrap amendment; P3 reads whether the AccessKit refresh or mutation changes move its write sites.
@task_item n=8 part=needs
Hit stops and scrolled-box bounds, and driver clicks in a shown scrollbar band, kept through any conflict; 0980cce5f's hit-testing hunk stays a textual conflict.
@task_item n=9 part=needs
The document clock: double click read on the document's clock and its threshold kept under the clock invariant.
@task_item n=10 part=needs
Stable element ids and the snapshot model with its readers (snapshot node, text form, parts, state tokens, diff, select value and row bounds, UI graph, reached-handler record), which live in the Dioxus document crate and are decided by its stand checks.
@task_item n=11 part=needs
The Dioxus render into a mutation buffer: a merge surface flagged not upstreamable, replaced whole if the Dioxus 0.8 move is taken; the mounted-event test returns only with that move.
@task_item n=12 part=needs
Password value masked and never revealed, with its caret placed by bullets, kept through the take.
@task_item n=13 part=needs
Contrast and lint numbers: Stylo, Parley or the default-font fallback can move computed styles and text, so the stand contrast measurement, the lint's contrast finding (bounds, ratio) and the lint-clean-at-boot reading are re-measured and recorded, never loosened.
@task_item n=14 part=needs
The a11y target, WCAG SC 2.1.1, 1.4.3 and 2.4.3, re-proven by its standing checks and the a11y leg, which must be green and gates CI.
@task_item n=15 part=needs
The a11y CI leg and stand checks run in both layout modes and repeated three rounds, green on the pushed commit with every job settled.
@task_item n=16 part=needs
The accessibility feature wiring: default-on, the Dioxus accessibility feature taken by name, the session library always enabling it, browser off by default, no platform adapter on stand binaries; a Dioxus or AccessKit move must keep each.
@task_item n=17 part=needs
AccessKit and the accesskit_xplat adapter: any version the take brings keeps the adapter reading the document's own tree, creation order, and requests reaching one event handler.
@task_item n=18 part=needs
The log sink allowlist: any new tracing or log call site the take brings in blitz or accesskit_xplat targets prints only the allowed fields, other targets dropped whole, and the capture checks stay at 0 needles.
@task_item n=19 part=needs
Accessibility assertions stay Rust tests over AccessKit; the take's new upstream tests and the two frame tests do not change that harness.
@task_standing n=1
convention: every new accessibility assertion is a Rust test over the AccessKit tree, run in the a11y leg
home: 3-a11y-assertion-harness-contract
@task_standing n=2
convention: every new stand check runs in the slow-tier accessibility leg, gates CI, and is repeated three rounds
home: ci-integration · 9-ci-integration
@task_standing n=3
convention: every new log line passes the sink's target allowlist, printing only allowed fields and redacting values
home: 3-a11y-assertion-harness-contract
@task_item n=1 part=not_needed
Cognitive accessibility rows (invalid state beyond colour or class, the shared Tab order table): the take changes no stand form or its invalid styling.
@task_item n=2 part=not_needed
The driver's verbs, the actions answer, MCP tool results and driver refusal causes: the chunk adds or changes no driver verb or tool, and adopting new upstream APIs is out of scope.
@task_item n=3 part=not_needed
UI graph state registration (use_graph_state, unconfirmed, undeclared): no app or stand registers new state in this chunk.
@task_item n=4 part=not_needed
The stand's own design choices (accent colour, near-white author thumb): no stand styling is chosen or changed.
@task_item n=5 part=not_needed
Areas with no recorded intent (SLO invariants and budgets, anti-patterns, decisions log): no row to bind.
@task_item n=6 part=not_needed
The not-yet-measured strategy items (live regions, heading hierarchy, skip links, modal focus traps, timeout extensions): the chunk defines none of them.
