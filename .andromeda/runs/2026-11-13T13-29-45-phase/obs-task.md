@task_prose part=chunk
Takes upstream blitz at the pinned sha 563d3db73 into the fork, in the shape the founder picks at P4 (a cherry-pick on the registry pins, that plus upstream's registry moves, the whole merge, or nothing), resolving conflicts so escher's engine behaviour stands.
Folds the CI reds: the clippy needless_bool lint at three engine sites and the dependency-audit advisory on the SQLite crate behind the browser's persistence.
Adds two headless frame tests (blend/isolation paint and an inline box's own decorations), each in both layout modes, pins every action of the remaining workflows to a commit SHA and widens the pinning check to every workflow.
Re-reads the per-OS test tallies of the take's green CI run, re-checks the sink allowlist against every new tracing/log call site, target, print and env read in the taken delta, and re-runs every escher proof: fast, doc, a11y and audit legs, the stand checks in both layout modes and the oracle.
Lists every boundary widening the take carries (DevTools/CDP socket, lossy page decoding, new attribute reads, text-shadow and mask paint, the AccessKit refresh, new env reads) for the operator at P4, never settled silently.
@task_file n=1 path=packages/blitz-dom/src/document.rs mark=edits
what: the engine's base document, holding the anonymous-block deallocation fix that is a recorded merge surface — edits: the needless_bool site flagged by clippy
@task_file n=2 path=packages/blitz-dom/src/layout/damage.rs mark=edits
what: the engine's layout damage and changed-set computation — edits: the two needless_bool sites flagged by clippy
@task_file n=3 path=packages/dioxus-native-dom/src/dioxus_document.rs mark=merge
what: the Dioxus native document bridge holding the buffered render, its two render sites and buffered_mutations field, a recorded merge surface
@task_file n=4 path=packages/dioxus-native-dom/src/mutation_buffer.rs mark=merge
what: escher's own mutation buffer for the buffered render, replaced whole if the Dioxus 0.8 move is taken
@task_file n=5 path=packages/dioxus-native-dom/src/lib.rs mark=merge
what: the dioxus-native-dom crate root, carrying the mutation buffer's module line
@task_file n=6 path=.github/workflows/wpt.yml mark=edits
what: the upstream-only WPT CI workflow — edits: every action reference, pinned to a 40-hex commit SHA with its ref as a trailing comment
@task_file n=7 path=.github/workflows/publish-browser.yml mark=edits
what: the browser publishing CI workflow — edits: every action reference, pinned to a 40-hex commit SHA with its ref as a trailing comment
@task_file n=8 path=.github/workflows/wpt-post-results.yml mark=edits
what: the upstream-only WPT results-posting CI workflow — edits: every action reference, pinned to a 40-hex commit SHA with its ref as a trailing comment
@task_file n=9 path=.github/scripts/test_ci_workflows.py mark=edits
what: the CI workflow guard checks — edits: the action-pinning check, widened from the main CI workflow to every workflow
@task_file n=10 path=.github/scripts/test_blitz_tests_targets.py mark=re-reads
what: the blitz-tests target-list check: one file and one target per behaviour
@task_file n=11 path=.github/scripts/ci-leg.sh mark=re-reads
what: the CI leg runner: the fast, doc, a11y and dependency-audit legs re-run after the take
@task_file n=12 path=packages/escher-telemetry/src/format.rs mark=re-reads
what: escher's telemetry sink line format and its standing capture checks, held at 0 needles
@task_file n=13 path=Cargo.toml
what: the workspace manifest: members and the coupled dependency pins (Stylo, Taffy, Parley, AnyRender/vello, winit, Dioxus)
@task_file n=14 path=Cargo.lock
what: the workspace lock: resolved dependency versions read by the dependency audit
@task_item n=1 part=needs
The sink target allowlist: every new tracing or log call site and target the taken delta brings is read against it.
@task_item n=2 part=needs
Engine and upstream crates take tracing only behind the tracing cargo feature; taken engine code with new tracing calls must stay feature-gated.
@task_item n=3 part=needs
escher-telemetry's sink: its log records, output stream and RUST_LOG handling, with the standing capture checks at 0 needles after the take.
@task_item n=4 part=needs
Engine targets print safe fields only and escher targets redact the content-named set: any new event field in taken code is judged by that rule.
@task_item n=5 part=needs
Scrubbing happens at the sink, not at call sites, and the sink's scrub set (url, error) covers any new field the take carries.
@task_item n=6 part=needs
New env reads in the taken delta are listed and read against the sink configuration (RUST_LOG) and the no-new-env-var boundary.
@task_item n=7 part=needs
New prints in the taken delta (stdout/stderr) are read, since escher-session's stdout carries only its answers and its diagnostics are fixed strings.
@task_item n=8 part=needs
No telemetry backend, OTel export, error-reporting service or metric exporter enters with the take; a new crate such as the DevTools/CDP code is read for any of them.
@task_item n=9 part=needs
DevTools/CDP may open a socket or port: a boundary widening listed for the operator at P4, never a silent new listener.
@task_item n=10 part=needs
The script engine surface: any script API the take adds is read for what it carries out, including the script diagnostics drain and the script error surface.
@task_item n=11 part=needs
The buffered render's mutation buffer contents stay in memory only, whatever the merge resolution of the bridge.
@task_item n=12 part=needs
Conflict resolutions in the select model and list, the snapshot and stable ids, the range and keyboard models, inert and the accessibility state reads keep escher's behaviour, including what the snapshot and select list never log.
@task_item n=13 part=needs
The changed set's write sites and their never-list qualifier for an open select list: P3 reads whether the AccessKit refresh or mutation changes move them.
@task_item n=14 part=needs
The headless stand runs with no sink: the new frame tests and the stand checks in both layout modes run that way.
@task_item n=15 part=needs
Fork CI on GitHub workflows: the take's green run with every job settled, its per-leg log artifacts read to re-count per-OS test tallies, each moved count explained.
@task_item n=16 part=needs
WPT scores and results are published from upstream only; the WPT workflows get action pins but no publishing change.
@task_item n=17 part=needs
The dependency audit leg: the SQLite advisory resolved by a semver-compatible update or an ignore by ID with a written reason, as P4 rules, and any entering crate run through the licence check.
@task_item n=18 part=needs
The WASM panic hook and escher-telemetry's panic hook (previous hook kept) are not displaced by taken upstream code.
@task_item n=19 part=needs
The window error event and the WPT report message shapes stay as recorded if taken engine or runner code touches the script error path.
@task_standing n=1
convention: every new check or test is silent: it prints nothing beyond its verdict and shares modules rather than logging
home: 3-observability-harness-contract
@task_standing n=2
convention: a check that asserts an empty stderr unsets RUST_LOG before it runs
home: 3-observability-harness-contract
@task_standing n=3
convention: a check failure names a kind and a count, never the content it found
home: 3-observability-harness-contract
@task_standing n=4
convention: CI job edits keep the fork's artifact rules: a per-leg log uploaded on failure, coverage on success, no local state uploaded
home: 9-ci-integration · ci-integration
@task_item n=1 part=not_needed
The escher-driver command and tool_call spans and their fields, endings and refusal cause names: the take adds no driver command and changes no span.
@task_item n=2 part=not_needed
The cold-agent pipe and agent-run harness rows (their events, verdict, counted verbs, kept checks, transcripts and record directories): the chunk runs no agent and changes neither harness.
@task_item n=3 part=not_needed
escher-session's roles, exit codes, host wire, error object and coverage line: the take changes no escher-session command or answer.
@task_item n=4 part=not_needed
The session recording file, recording bounds, session lifecycle and session state directory: no recording or session change is in the chunk.
@task_item n=5 part=not_needed
The observability tier and the SLO and telemetry-budget sections: the chunk sets no tier and no budget.
@task_item n=6 part=not_needed
The graph and actions answers and the reached record's contents: adopting new upstream APIs in escher's own code is out of scope, and the take adds no driver answer.
