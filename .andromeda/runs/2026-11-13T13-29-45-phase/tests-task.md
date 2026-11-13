@task_prose part=chunk
Takes upstream blitz at the measured pin 563d3db73 into the fork (the tenth sync), in the shape the founder picks at P4 from four measured on trial worktrees: cherry-pick on the registry pins, that plus upstream's registry moves (Stylo 0.24, Parley 0.13, winit beta.4), a whole merge, or nothing.
Fixes the CI reds it inherits: the clippy needless_bool lint under rustc 1.100 at three engine sites, and the dependency-audit advisory on libsqlite3-sys under rusqlite (fixed by bumping or ignored with a written reason, decided at P4).
Adds two headless frame tests, one for mix-blend-mode/isolation and one for an inline box skipping its own decorations, each in both layout modes. Pins every action of the three upstream-only workflows to a commit SHA and widens the pinning check to every workflow.
Re-reads the left-out delta, the merge surfaces (anonymous-block deallocation, hit_inner, the buffered render), the per-OS test tallies of the take's green CI run, and the sink allowlist over the taken delta. Re-asserts every escher proof: the fast, doc, a11y and audit legs, stand checks in both layout modes, the incremental oracle, the embolden witness and fork CI green.
@task_file n=1 path=packages/blitz-dom/src/document.rs mark=edits
what: the engine document, upstream-owned, home of the anonymous-block deallocation fix and a merge surface — edits: the needless_bool lint site, resolutions in BaseDocument::deallocate_anonymous_block where the take reaches it
@task_file n=2 path=packages/blitz-dom/src/layout/damage.rs mark=edits
what: the engine's layout damage and changed-set tracking — edits: two needless_bool lint sites
@task_file n=3 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: the Dioxus document bridge holding the buffered render, a merge surface re-read at the take (the two render sites, the buffered_mutations field, the ported buffered-render test)
@task_file n=4 path=packages/dioxus-native-dom/src/mutation_buffer.rs
what: escher's own mutation buffer for the buffered render on the Dioxus 0.7 bridge, replaced whole only if the Dioxus 0.8 move is taken
@task_file n=5 path=packages/dioxus-native-dom/src/lib.rs
what: the dioxus-native-dom crate root, its one module line for the mutation buffer
@task_file n=6 path=.github/workflows/wpt.yml mark=edits
what: the upstream-only WPT conformance workflow — edits: every action reference pinned to a 40-hex commit SHA with its ref as a trailing comment
@task_file n=7 path=.github/workflows/publish-browser.yml mark=edits
what: the upstream-only browser publishing workflow — edits: every action reference pinned to a commit SHA
@task_file n=8 path=.github/workflows/wpt-post-results.yml mark=edits
what: the upstream-only WPT results posting workflow — edits: every action reference pinned to a commit SHA
@task_file n=9 path=.github/scripts/test_ci_workflows.py mark=edits
what: the CI workflow guard, a Python unit test over the workflows — edits: the action-pinning check, widened from ci.yml to every workflow
@task_file n=10 path=.github/scripts/test_blitz_tests_targets.py mark=edits
what: the check pinning the blitz-tests target list, one file and one target per behaviour — edits: the target list, where upstream's new test files arrive
@task_file n=11 path=Cargo.toml
what: the workspace manifest, its members and the coupled dependency pins (Stylo, Taffy, Parley, AnyRender/vello, winit, Dioxus) moved as one set or not at all
@task_file n=12 path=Cargo.lock
what: the dependency lock, read for new or removed crates and for the advisory's rusqlite/libsqlite3-sys chain
@task_file n=13 path=.github/scripts/ci-leg.sh mark=re-reads
what: the CI leg script's fast, doc, a11y and dependency-audit legs, re-run after the take
@task_file n=14 path=packages/escher-telemetry/src/format.rs mark=re-reads
what: the standing sink capture checks, held at 0 needles
@task_item n=1 part=needs
Coupled dependency pins: a take carries upstream's pin set as one set or none; a set that does not resolve halts, never a one-sided fix; no git dependency enters unless the founder rules the whole-merge shape.
@task_item n=2 part=needs
Numbers moved by a layout, style, text or paint dependency move (stand bounds, snapshot byte size, lint findings, pixels) are re-measured and recorded, never loosened to pass.
@task_item n=3 part=needs
Clippy legs over the workspace and upstream blitz-dom must go green under the toolchain CI runs (rustc 1.100), with fast jobs gating the rest.
@task_item n=4 part=needs
The dependency audit (advisories and licences) must go green: the libsqlite3-sys advisory is cleared by a bump or an ignore by ID with a written reason, and any crate that enters is reviewed and licence-checked.
@task_item n=5 part=needs
MSRV stays 1.91, verified by the MSRV build leg against any dependency the take or the audit fix brings.
@task_item n=6 part=needs
Stand checks run in both layout modes on the bundled font, with a zero-failure budget, and the incremental/non-incremental differential oracle stays green after the take.
@task_item n=7 part=needs
Conflict resolutions keep escher's engine behaviour: the one virtual clock and advance by exactly the ms asked, the changed set and its write sites (and the select-list qualifier), the select model and list painted by the scene painter, hit stops, a scrolled box read where it stands, text extent and the password mask.
@task_item n=8 part=needs
Stable element ids and the snapshot (its text, parts, diff, byte budget and stable serialization), and the UI graph, decided by the id persistence, id edits, snapshot, graph and handler-coverage stand checks.
@task_item n=9 part=needs
The a11y target (keyboard reach, contrast ratio, focus order) re-proven by the a11y leg and its membership, with inert and the accessibility state reads kept through the AccessKit refresh.
@task_item n=10 part=needs
The buffered render on the Dioxus 0.7 bridge, the upstream buffered-render test kept as written, and the upstream mounted-flush test left out unless the chosen shape takes Dioxus 0.8.
@task_item n=11 part=needs
The key press after a dropped anonymous block, read against upstream's transform resolution through anonymous block boxes at the pin.
@task_item n=12 part=needs
Two new headless frame tests (blend mode/isolation, inline box decorations skipped), pixels on a CPU buffer, frames compared whole, in both layout modes; the harness contrast reading over mix-blend-mode where it applies.
@task_item n=13 part=needs
Upstream's new and edited test files run under the blitz-tests rule: one file, one target per behaviour, the target list pinned, the manifest's difference from upstream kept.
@task_item n=14 part=needs
CI actions pinned to commit SHAs in every workflow, with the pinning guard widened past ci.yml; workflow edits keep read-only permissions, no secrets and the upstream-only repository guard on publishing and WPT jobs.
@task_item n=15 part=needs
Per-OS tallies of the take's green CI run re-read by the recorded method (ANSI strip of job logs), each moved count attributed by test name.
@task_item n=16 part=needs
Telemetry sink: the taken delta read for every new tracing/log call site, target, print and env read; the target allowlist holds, third-party targets dropped, sink capture counted by target at 0 needles.
@task_item n=17 part=needs
The embolden witness, a local leg with no CI job, re-run after the take.
@task_item n=18 part=needs
A typed password never leaves the instance and is never returned, kept through any take touching text input (copy firing no input event).
@task_item n=19 part=needs
No new crate, port, env var or listener of ours; boundary widenings the take carries (DevTools/CDP socket, lossy decoding, new attribute reads, text-shadow and mask paint, AccessKit refresh, env reads) are listed for the operator at P4, never settled silently.
@task_item n=20 part=needs
Cargo legs run locked and per-package green is not workspace green: fork CI green on the pushed commit with every job settled.
@task_standing n=1
convention: each new frame or pixel test is driven headless through the harness, renders on a CPU buffer and compares frames whole
home: test-strategy · 2-test-strategy
@task_standing n=2
convention: each new layout or frame assertion runs in both layout modes
home: integration-test-strategy · 5-integration-test-strategy
@task_standing n=3
convention: each new test target is one file holding one behaviour, joining the pinned target list
home: test-scope-summary · 1-test-scope-summary
@task_standing n=4
convention: each new font-dependent test runs on a real pinned font and never skips
home: mocking-stubbing-discipline · 8-mocking-stubbing-discipline
@task_standing n=5
convention: each new regression test is named for and documents its bug
home: 2-test-strategy
@task_standing n=6
convention: each new test states its data in code, reads no wall clock and sleeps never
home: test-data-fixtures · 7-test-data-fixtures
@task_standing n=7
convention: each new test is controlled red-before-green and its failure prints no argument value
home: unit-test-strategy
@task_standing n=8
convention: each new or edited CI workflow line pins its actions to a commit SHA, holds read-only permissions and uses no secrets
home: unit-test-strategy · 4-unit-test-strategy
@task_item n=1 part=not_needed
The session lifecycle: session wire, session host, session errors, session state directory and recording rows; the chunk adds no session behaviour.
@task_item n=2 part=not_needed
The escher-session command line, CLI and MCP role contracts (exit codes, help, tools list, protocol revision, stdout and stderr lines); untouched by an upstream take.
@task_item n=3 part=not_needed
Agent-run verbs, events, selections and exit codes; the agent-run harness is not changed.
@task_item n=4 part=not_needed
The cold-agent pipe: its events, verdict, isolation, tasks and exit codes.
@task_item n=5 part=not_needed
Driver verbs, argument kinds, refusal causes and their order, and the driver's log line; the driver surface gains nothing.
@task_item n=6 part=not_needed
Committed recorded checks and their recording script, replay and freshness; no check is recorded or re-recorded.
@task_item n=7 part=not_needed
Lint defect kinds and finding order as contracts of the lint verb; the chunk adds no lint behaviour.
@task_item n=8 part=not_needed
Mutation testing and cognitive-complexity ceilings read on the dev host; not part of this take's exit.
@task_item n=9 part=not_needed
The apt-install and coverage-floor script exit codes; those scripts are not edited.
@task_item n=10 part=not_needed
The absence of windowed end-to-end tests and of a mocking library; nothing here adds either.
