@task_prose part=session
The layout engine Taffy moved from the crates.io registry version 0.15.1 to upstream Taffy's git source at rev e90751fd (package version 0.15.0, rust-version 1.71, licence MIT, no checksum line), the lock's one git source.
Eight upstream blitz commits were taken by no-commit cherry-pick, the lock the only conflict in each; their layout changes reach every document and land ratified by the founder, not provisional.
The inline layout pass now hands Taffy's float placement a caller-defined float id (Taffy #1303), its one call of that function changed by one line; no public symbol, crate, schema, config or gate surface changed.
Measured: the fast leg, the stand, stand-repeat 3 of 3, a11y, msrv, audit, doc, build, counter and wasm legs green, the smoke up 10 s, and CI 17 of 17 green across the native platforms; every test tally equal to the base's (test 239 · 1464 · 0 · 13, embolden 6, a11y 14 · 115, stand 294 · 0 · 8).
Left out by design for a later entry: Taffy from #1372 on (reversed scroll origins and the five later bumps), which leaves blitz-dom uncompiled.
@task_file n=1 path=Cargo.toml
what: the workspace manifest — the engine dependency list and its version pins; edited, the taffy line now a git dependency at rev e90751fd
@task_file n=2 path=Cargo.lock mark=conflict
what: the dependency lock the locked cargo legs and the dependency audit read; edited, the taffy package now a git source with no checksum, the conflict of each upstream pick resolved in it
@task_file n=3 path=packages/blitz-dom/src/layout/inline/layout.rs mark=merge
what: blitz-dom's inline layout pass — float placement through Taffy's place_floated_box; edited by the cherry-picked upstream commit to pass the float id
@task_item n=1 part=touches
decision/incremental-differential-oracle — the eight taken commits change layout for every document and the float placement call changed; the incremental layout oracle ran in the workspace run and stayed green.
@task_item n=2 part=touches
decision/layout-tests-in-both-layout-modes and decision/stand-checks-in-both-layout-modes — the inline layout pass's float placement changed; the layout assertions and stand checks ran green with their counts unmoved.
@task_item n=3 part=touches
decision/workspace-tests-in-fast-ci-leg — the fast leg ran the changed layout path: 239 · 1464 · 0 · 13, equal to the base.
@task_item n=4 part=touches
decision/tally-move-attributed-by-test-name — no tally moved in any leg, so no move is owed an attribution by test name.
@task_item n=5 part=touches
decision/stand-repeat-three-rounds, decision/zero-failure-stand-repeat and decision/stand-checks-zero-failure-budget — stand-repeat 3 of 3 rounds green, the stand 294 · 0 · 8; the zero-failure budget held.
@task_item n=6 part=touches
decision/msrv-1-91 and decision/msrv-verified-by-build — the git Taffy declares rust-version 1.71, below the workspace minimum; the msrv leg built green and the minimum stands.
@task_item n=7 part=touches
decision/dependency-audit-advisories-and-licenses — the lock now holds one git source (taffy, MIT, no checksum); the audit leg read advisories ok, licenses ok.
@task_item n=8 part=touches
decision/cargo-legs-locked — the lock moved five lines and gained a git source re-resolved by cargo metadata; every locked cargo leg ran green on it.
@task_item n=9 part=touches
decision/ci-shared-test-cache, decision/shared-test-cache and decision/ci-rust-cache — the lock the cache reads moved and a git checkout of Taffy joins the build; indirect, through the dependency change.
@task_item n=10 part=touches
decision/native-platform-ci-matrix — CI 17 of 17 green on the pushed head, the macOS, windows, android and ios test jobs among them.
@task_item n=11 part=touches
decision/a11y-leg-membership — the a11y leg ran green at 14 · 115, unmoved; its membership unchanged.
@task_item n=12 part=touches
decision/embolden-local-witness and decision/embolden-leg-local-only — the embolden witness leg ran at 6, unmoved, over a changed layout engine.
@task_item n=13 part=touches
decision/clippy-legs and decision/blitz-dom-fixtures-built-manually — blitz-dom changed by one line in its inline layout pass; its lint and its manually built fixtures ran unchanged.
@task_item n=14 part=touches
decision/per-crate-coverage-floors, decision/coverage-floor-per-crate and decision/coverage-leg-exits-with-floor — blitz-dom's code changed by one line; the coverage leg was not among this run's gates, so a blitz-dom floor a row holds was not re-read.
@task_item n=15 part=touches
decision/wpt-conformance-report-only, decision/fork-wpt-reading-report-only and decision/wpt-conformance-runner — the layout changes for every document may move WPT readings; no WPT reading was taken this run, the reading report-only.
@task_item n=16 part=touches
decision/local-pre-push-gate and decision/local-baseline — the local gate ran green before the push and its tallies are the baseline CI was read against.
@task_item n=17 part=touches
decision/regression-tests-document-bug and decision/regression-tests-named-for-bug — the Taffy #1303 float id came with no new test; its presence is held by a gate entry, not a committed test.
@task_item n=1 part=untouched
The session wire, session host, session errors, session recording and session state contracts of the session lifecycle: no session code, protocol or bound moved.
@task_item n=2 part=untouched
The escher-session command line, MCP role and CLI contracts (exit codes, help, snapshot budget, tools list, refusals): no command, answer or exit code changed.
@task_item n=3 part=untouched
The driver session, driver verbs, refusal causes and handler coverage rows: no verb, cause or coverage reading changed.
@task_item n=4 part=untouched
The cold-agent pipe rows and the agent-run contract (verbs, selections, events, exit codes): run as before, unchanged.
@task_item n=5 part=untouched
The harness construction, core, input, inspection, settle, contrast and screenshot contracts and the virtual clock: no harness API moved.
@task_item n=6 part=untouched
The mocking and stubbing discipline, the stand's timer and load stand-ins and the test data and fixtures rows: no stand-in, fixture or committed check changed.
@task_item n=7 part=untouched
The CI workflow's own rules — actions pinned to SHAs, read-only permissions, no secrets, triggers, fast/slow split, bounded package installs and their exit codes, the leg script: no workflow or script changed.
@task_item n=8 part=untouched
decision/scrolled-box-reads-where-it-stands and the scrollbar rows: the reversed scroll origins of Taffy #1372 were left out, so scrolled-box reading is unchanged.
@task_item n=9 part=untouched
The mutation testing, complexity ceiling and keyboard_compat coverage exclusion rows, and absence/windowed-end-to-end-tests: the smoke is a gate entry, no committed windowed test.
