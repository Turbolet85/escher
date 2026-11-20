@history_item n=1 marker=2026-10-16-fast-failing-mutants
**2026-10-16-fast-failing-mutants** — (§10 Mutation tier) wrote the mutation-tier recipe the picked rows mutation-testing-dev-host-only, mutation-runs-under-nextest-profile, mutation-baseline-skipped and mutation-baseline-read-per-unit restate: cargo-mutants `--test-tool nextest` under the root `.config/nextest.toml` `mutants` profile (fail-fast, `priority = 100` on the direct tests, selected by `NEXTEST_PROFILE=mutants`), `--baseline=skip`, each unit's mutant test command read green on the unmutated tree first, dev host only, never in CI, cargo-mutants 27.1.0 and nextest 0.9.146 as host tools. Why: cargo-mutants' baseline tests the mutated package alone yet passes every test argument, and under plain `cargo test` mutants read `timeout`. Matters here: any re-run of the four `capture.rs` mutants, which needs the founder's word, must follow this recipe, and `-p browser`'s mutant test command has to be read green by hand first.

@history_item n=2 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — six dioxus-native-dom unit tests killed the seven mutants the Epoch 2 code audit left surviving. They sit in the `cfg(test)` child module `dioxus_document::bridge_tests`, because `DioxusEventHandler`'s fields are private and it has no constructor (the origin of the picked bridge-tests-without-widening and unit-tests-in-source-file rows). Trap: a sed or grep address that ends at a function's name also selects every item whose name opens with it. Matters here: this is the precedent for catching an audit's survivors with direct unit tests inside the crate's own `cfg(test)` module. `render_scene` is likewise reachable only from inside `capture.rs`, so the test belongs in its own module and must not widen visibility.

@history_item n=3 marker=2026-10-09-audit-corrections-agent-surfaces
**2026-10-09-audit-corrections-agent-surfaces** — (a textarea's value read) one mutant the Epoch 4 code audit left alive was caught by a test added to the existing `stand_snapshot_state`, controlled red with the `textarea` arm deleted and green on the restored tree. It went there because the audit runs that unit against a fixed test set that holds that file, and a new file would sit outside the set the next audit runs. Matters here: the next boundary audit's `mutation:browser` run reads the browser package's tests, so a catching test has to sit where that run's test set reaches it, inside the browser crate.

@history_item n=4 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (the browser's tab test module) recorded that `apps/browser/src/tab.rs`, in the upstream-only browser app, gained a test module with one `#[test]`, the browser's sixth tested file beside about_pages, browser_history, favicon, url_suggestions and persistence, run by the workspace `test` leg. The s04 search now hits six browser files. Matters here: a `capture.rs` `#[cfg(test)]` module would be the browser's seventh tested file and must be added to the §1 and §4 apps/browser inventory. The same entry's tally sub-entry counts `apps/browser/src/tab.rs` 1 in the workspace re-count.

@history_item n=5 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (the per-OS and local workspace tallies re-read after the take) the workspace tally moved 1117 → 1125 passed, `apps/browser/src/tab.rs` 1 among the named moves. Matters here: a browser unit test added by this chunk moves the workspace tally inside the browser's existing result line, and that move must be attributed by name the same way.

@history_item n=6 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — (the per-OS CI tallies re-read, and attributing a tally move) set the rule that a tally move is attributed by grepping the test's own name. Stderr `Running …` lines and stdout `test result:` lines do not interleave in order in a GitHub Actions job log. Matters here: if this chunk adds browser tests, its CI re-read must attribute the +N passed by name, not by the preceding `Running` line.

@history_item n=7 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — opened §9's "Engine features by runner". `cargo test --workspace` builds blitz-dom with `tracing` on, `-p` builds do not, and a check read green per package and red in the workspace leg. This is the origin of the picked per-package-green-not-workspace-green row. Matters here: a browser test proven with `cargo test -p browser` is not the workspace `test` leg's green. The mutation run's per-package build and the workspace leg compile different engine features.

@history_item n=8 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — (readings on the merged tree) sharpened the feature split: the workspace build resolves blitz-dom with 16 features and the per-package build with 10, `writing-mode` on only in the first, so the two runners compile different code. Matters here: it backs the picked per-package-green-not-workspace-green row's why. A `render_scene` test must read green under both the browser's per-package run and the workspace leg.

@history_item n=9 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** — (CI on the build branch) made the workspace tests the fast `test` leg, `cargo test --workspace --locked` on ubuntu with default features, run through `ci-leg.sh`, with host-reproducible legs and a local pre-push gate. This is the basis of the picked workspace-tests-in-fast-ci-leg row. Matters here: a new `capture.rs` unit test runs in that leg under the browser's default features, and the plan's P3 check of building a `BaseDocument` and an `impl PaintScene` under default features rests on it.

@history_item n=10 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** — (4. Unit Test Strategy) + decision/expected-value-pinned-as-literal: a check never derives its expected value from the constant its mutation control changes; it pins the measured value as a literal. Why: a cut control did not hold until the flood check pinned its measured value. Matters here: the size assertion for `render_scene` must pin the measured `(u32, u32)` as literals for each input, not compute it from the viewport or scale the code reads, or a mutant can read green.

@history_item n=11 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** — (4. Unit Test Strategy) ~ decision/mutation-controlled-tests: a new check is controlled (a named mutation turns it red and the restored tree reads green), and a control that removes a bound loosens it to twice the budget instead. Matters here: each new `render_scene` test needs its named control. The four cargo-mutants replacements `(0, 0)`, `(0, 1)`, `(1, 0)` and `(1, 1)` are the natural controls, applied by hand on the dev host and restored.

@history_item n=12 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — (4. Unit Test Strategy) + decision/no-change-read-control-needs-pending-work: the plan's pump control was measured not to hold on a settled screen, so a control of a no-change read needs pending work. Matters here: this is a controlled-tests convention in the same area. A control must be shown able to turn the check red before it counts, so the test's inputs have to make each of the four constants differ from the real size.

@history_item n=13 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** — set the standing practice of counting unit tests per crate and per file, each count appended as a dated link with earlier links kept as history. Matters here: a browser test added by this chunk moves the apps/browser per-file inventory (§1 and §4) and the workspace baseline chain. Those are the sites the wrap re-counts.

@history_item n=14 marker=2026-10-16-file-splits
**2026-10-16-file-splits** — (moved unit tests counted by their new files) brought the out-of-line `#[path]` `cfg(test)` modules from one to five, each moved out whole with test paths unchanged. This is the history behind the picked inline-cfg-test-modules, inline-unit-test-modules and unit-tests-in-source-file rows' exceptions. Matters here: it sets the form a `capture.rs` test module may take, inline by default or a `_tests` sibling declared by path.

@history_item n=15 marker=2026-11-14-start-in-a-state
**2026-11-14-start-in-a-state** — (2. Test Strategy) ~ decision/inline-cfg-test-modules except: the stated count of out-of-line modules ("Eight …") was dropped, leaving the form alone. Matters here: it is the prior state of the picked inline-cfg-test-modules row, which a browser test module either keeps inline or meets through the `_tests` exception.

@history_item n=16 marker=2026-11-20-codebase-health
**2026-11-20-codebase-health** — (2. Test Strategy, ~ decision/inline-cfg-test-modules and ~ decision/inline-unit-test-modules) rewrote both twin rows' exceptions to their current wording: a source file's unit tests may sit out of line in a sibling `_tests` file declared as a `cfg(test)` module by path, with fixture builders in a child module. Why: the chunk moved the snapshot reader's tests out of line and gave the driver's JSON tests a fixtures child. Matters here: this is the picked rows' exact current text. A `capture.rs` test with a fixture builder for a `BaseDocument` and a `PaintScene` stand-in may use the fixtures-child form.

@history_item n=17 marker=2026-11-20-codebase-health
**2026-11-20-codebase-health** — (10. Quality Gates, + upstream-oversize-files-kept, + oversize-test-targets-kept, + upstream-test-clone-pairs-kept, ~ complexity-ceiling-host-only why) recorded the founder's rulings F1 to F4 on v040-08's other legs. Upstream-authored oversize files, upstream's functions over the complexity ceiling and upstream's duplicated test pairs are kept as upstream wrote them, and escher's seven oversize test targets are kept whole. Matters here: these are the precedents for this chunk's fork, a test pinning an unused value against removing it in upstream `apps/browser` code (more divergence) against keeping the four by the founder's word. They come from the same capability v040-08, whose last open leg this is.

@history_item n=18 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** — (the dev-host complexity ceiling) recorded the Epoch 5 audit's cognitive-complexity ceiling of 15, read on the dev host only with rust-code-analysis-cli. The engine's functions over it were left as they are, its scope being escher's code. It was rejected as a project-wide convention. Matters here: like the mutation tier, it is a dev-host-only boundary-audit metric, and upstream app code was left outside the fix scope. That is the stance the remove-the-return-value option would break.

@history_item n=19 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — (the coverage leg excludes `keyboard_compat.rs` by filename) took the operator's answer to a P4 fork: a named exclusion of upstream's temporary bridge was chosen over lowering the floor or testing its 530 arms. Matters here: it is a precedent for an audit metric on upstream code settled by a ruled keep or exclusion rather than tests written to satisfy the metric. That is the fork this chunk puts to the founder.

@history_item n=20 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (dioxus-native-dom's unit scope and its one test-only shim) ~ the bridge tests' harness: no function under test gained a constructor or a `pub`, and the one `cfg(test)` shim `DioxusDocument::dispatch_to_vdom` was allowed. This is the source of the picked bridge-tests-without-widening row's exception. Matters here: a direct `render_scene` test should likewise add no `pub` or constructor. A `#[cfg(test)]` child module of `capture` reaches the private function without widening anything.

@history_item n=21 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (the event-loop stand-in and the redraw route's check) gave blitz-shell its first unit test, an inline `#[cfg(test)]` module in `event.rs` with a test-local `CountedWakes` implementing `EventLoopProxyProvider`. Matters here: it is the precedent for a crate's first-of-its-kind inline unit test built on a hand-written trait stand-in, under the picked hand-written-stand-ins and inline-unit-test-modules rows. A `render_scene` test may need a test-local `impl PaintScene`.

@history_item n=22 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (`select_popup`'s `CountingShell`) added a hand-written `ShellProvider` that counts redraw requests, stated as a new §8 Shell row. Matters here: a test-local trait implementer written for a check is recorded as a §8 row. A hand-written `PaintScene` stand-in for `render_scene` would owe the same row, with no mocking library allowed.

@history_item n=23 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** — (8. Mocking & Stubbing Discipline) + decision/real-provider-inside-stand-in: a real provider was put behind a hand-written stand-in. Matters here: it is a §8 discipline row beside the picked hand-written-stand-ins row. A `render_scene` test may pass a real anyrender scene instead of a stand-in, and that choice falls under these rows.

@history_item n=24 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — the harness paints with no display through `VelloCpuImageRenderer`, frames compared whole, and a no-font shot is refused, not skipped. Matters here: `capture.rs`'s `render_scene` paints a `BaseDocument` into an `impl PaintScene`, the path a headless test must drive without a window (picked ui-driven-through-session-binary row, absence/windowed-end-to-end-tests). This entry is the existing precedent for painting a document headlessly in a test.

@history_item n=25 marker=2026-11-15-session-binary-smoke
**2026-11-15-session-binary-smoke** — (3. Test Harness Contract: 4 rows, ~ decision/test-data-is-code) changed the picked test-data-is-code row to its current wording: fixtures are compiled into their targets, by boot for blitz-tests and by the run's own cargo call for seven_guis. The same entry also widened the run selections. Matters here: a browser unit test's fixtures, such as a `BaseDocument` with a set viewport, are code under this row. No selection of the agent-run contract builds the browser crate.

@history_item n=26 marker=2026-11-15-session-binary-smoke
**2026-11-15-session-binary-smoke** — (3. Test Harness Contract: 1 row) ~ overview/session-lifecycle: the session is no verb of the five-command contract, and its run session selection runs the session binary's cli_flow flows. Matters here: it is the current wording of the overview listed in the rows file. A browser test is outside every agent-run selection and is reached only by cargo runs of the browser package and by the workspace leg.

@history_item n=27 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** — measured the test-data bootstrap: nothing is seeded, and the fixtures are code compiled into each target. This is the origin of the picked test-data-is-code and test-data-in-code rows. Matters here: a `render_scene` test states its document and viewport in code, with no fixture file.

@history_item n=28 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — (the harness re-exports `NamedKey`; test-that named as the assertion crate) named test-that 0.6.0 as a blitz-tests dev-dependency whose `verify_that!` and `TestResult<()>` the merged checks use under the built-in harness. Matters here: it is the origin of the picked rust-tests-on-built-in-harness row's exception. An escher-written browser test uses plain `#[test]` with `assert_eq!`.

@history_item n=29 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** — (3. Test Harness Contract) ~ decision/rust-tests-on-built-in-harness except: widened from two named checks to upstream's merged checks that return `TestResult<()>` with `verify_that!`. Matters here: it is the picked row's current wording. Only upstream's merged checks take the test-that form, so a fork-written `capture.rs` test asserts with `assert_eq!` under `#[test]`.
