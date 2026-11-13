@task_prose part=chunk
Takes upstream blitz main at the measured pin 563d3db73 as this fork's tenth sync, in the shape the founder picks at P4 (cherry-pick on registry pins, that plus the registry moves Stylo 0.24 / Parley 0.13 / winit beta.4, a whole merge, or nothing).
Folds the two external CI reds: the clippy needless_bool lint at three engine sites, and the SQLite advisory in the dependency audit (bump or ignore by ID, a P4 fork).
Adds two headless frame tests, one for blend mode / isolation paint and one for an inline box's skipped own decorations, each in both layout modes; pins every action of the remaining workflows to a commit SHA and widens the pinning check to every workflow.
Re-reads the left-out delta and the merge surfaces against the new pin, re-asserts every escher proof (fast, doc, a11y and audit legs, stand checks in both layout modes, the incremental oracle, the embolden witness, fork CI green), re-reads the per-OS test tallies and re-checks the log-sink allowlist.
Any layout, style, text or paint number the take moves (stand bounds, snapshot byte size, lint findings, screenshot pixels) is re-measured and recorded, never loosened.
@task_file n=1 path=packages/blitz-dom/src/document.rs mark=edits
what: the engine document, home of the anonymous-block deallocation fix that is a merge surface — edits: the needless_bool site the clippy leg flags
@task_file n=2 path=packages/blitz-dom/src/layout/damage.rs mark=edits
what: the layout damage computation behind the changed set — edits: the two needless_bool sites the clippy leg flags
@task_file n=3 path=.github/workflows/wpt.yml mark=edits
what: the upstream-only WPT workflow — edits: every action reference, pinned to a 40-hex commit SHA with its ref as a trailing comment
@task_file n=4 path=.github/workflows/publish-browser.yml mark=edits
what: the browser-app publish workflow — edits: every action reference, pinned to a commit SHA
@task_file n=5 path=.github/workflows/wpt-post-results.yml mark=edits
what: the WPT results-posting workflow — edits: every action reference, pinned to a commit SHA
@task_file n=6 path=.github/scripts/test_ci_workflows.py mark=edits
what: the CI workflow guard — edits: the action-pinning check, widened from the main CI workflow to every workflow
@task_file n=7 path=.github/scripts/test_blitz_tests_targets.py mark=edits
what: the blitz-tests rule check, one file and one target per behaviour — edits: the pinned target list, for upstream's new test files and the two new frame tests
@task_file n=8 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: the Dioxus native document — the buffered render's merge surface: its two render sites, the buffered mutations field and the ported task-polled-during-render test
@task_file n=9 path=packages/dioxus-native-dom/src/mutation_buffer.rs
what: escher's own mutation buffer, which upstream's Dioxus 0.8 buffer replaces whole if the 0.8 move is taken
@task_file n=10 path=packages/dioxus-native-dom/src/lib.rs
what: the crate root of the element ids, snapshot, UI graph and buffered render — its one module line for the mutation buffer
@task_file n=11 path=Cargo.toml
what: the workspace manifest — the coupled dependency pins (Stylo, Taffy, Parley, AnyRender/vello, winit, Dioxus) and the members list
@task_file n=12 path=Cargo.lock
what: the dependency lock, read for new packages and the SQLite advisory chain
@task_file n=13 path=packages/escher-telemetry/src/format.rs mark=re-reads
what: the standing log-capture checks that hold the sink allowlist at 0 needles
@task_file n=14 path=.github/scripts/ci-leg.sh mark=re-reads
what: the fast, doc, a11y and audit CI legs re-run after the take
@task_item n=1 part=needs
The stand checks run in both layout modes after the take; a stand check that asserts exact bounds may read a new value when Taffy, Stylo or Parley move, and is re-measured, not loosened.
@task_item n=2 part=needs
The headless stand's pinned viewport: the two new frame tests render on it, each in both layout modes.
@task_item n=3 part=needs
The stand's stable element ids and their persistence across edits must survive the take unchanged, read by the element-id stand checks.
@task_item n=4 part=needs
Stand ids, names, roles and states carried as attributes alone: the accessibility state reads are an engine surface the take must keep.
@task_item n=5 part=needs
The select model and its open list: the open list rows read their own bounds, and the flight-type select reads its two options at boot; a conflict resolution keeps both.
@task_item n=6 part=needs
Scrolled-box bounds and hit stops: the visible region through clipping boxes and into-view scroll moving every holding scroller stay as escher has them through any resolution.
@task_item n=7 part=needs
Keyboard activation and the range and keyboard models, including the CRUD list as one keyboard region, kept through any conflict resolution.
@task_item n=8 part=needs
The stand lints clean at boot: a lint finding may move with the take and is re-measured, never loosened.
@task_item n=9 part=needs
The escher-session snapshot and screenshot answers: a snapshot's byte size or a screenshot's pixels may move with the paint and text pins, and is re-measured and recorded.
@task_item n=10 part=needs
The held stand instance reads as a fresh boot, and session roles share the stand boot; the take must not change boot state the stand checks rely on.
@task_item n=11 part=needs
Only the load page has a network provider; a take carrying lossy page decoding or new content readers is a boundary widening listed for the operator at P4, never settled silently.
@task_item n=12 part=needs
The MCP server stays local over stdio and the chunk adds no port, listener or env var; a DevTools/CDP socket the take carries is a security-plan amendment listed at P4.
@task_item n=13 part=needs
Caller input is never written back: the sink allowlist re-check reads every new log call site, print and env read the take brings.
@task_item n=14 part=needs
The session recording as a replayable check stays passing after the take, under the same escher proofs.
@task_item n=15 part=needs
The shell provider's window chrome controls: a winit move to beta.4 can touch them, and the resolution keeps escher's behaviour.
@task_standing n=1
convention: every new headless frame test renders on the pinned headless stand viewport and runs in both layout modes
home: surface-desktop-native
@task_standing n=2
convention: every new stand-facing test reads elements by their stable ids as attributes, never by position
home: surface-desktop-native
@task_item n=1 part=not_needed
The agent-run test contract and the cold-agent run pipe (their usage, exit codes, stdout and record directories): the chunk changes no script or pipe of the agent tooling.
@task_item n=2 part=not_needed
The multi_doc command line and its environment knobs: the chunk adds no argument or env variable to it.
@task_item n=3 part=not_needed
The escher-session exit codes, refusals, help and command-line form: the take adds no verb or exit path to the session tool.
@task_item n=4 part=not_needed
The served-task list of escher-session and the cold-agent prompt rules: no task is added or renamed.
@task_item n=5 part=not_needed
The absence of arbitrary index.html templates and the fixed document skeleton: no document template work is in scope.
@task_item n=6 part=not_needed
The stand accent colour: no visual design of the stand changes.
@task_item n=7 part=not_needed
The mobile-native and web-spa surfaces and the decisions log: they hold no rows and the chunk reaches neither surface.
