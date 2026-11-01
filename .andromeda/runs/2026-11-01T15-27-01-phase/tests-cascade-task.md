# Task — tests · 2026-11-01-upstream-sync-ui-graph

## The chunk
Takes upstream DioxusLabs/blitz into this fork at the measured sha `e42ee1f25` (merge base `23b9b26a9`, 33 commits to take or leave), by the shape the founder chooses at P4: a cherry-pick on the registry pins, that plus winit beta.4, the whole merge, or nothing.
Resolves the conflicts of the take with both sides' intent kept, and runs upstream's new and edited test files under the fork's blitz-tests rule where the take carries them.
Carries eight blocks of folded freight: the left-out delta and the merge surfaces re-read row by row, the matrix job's free-disk step, the key-press panic at `node.rs:1775` (its fix a founder fork), a local font-embolden gate, the orphan comment in blitz-dom's `lib.rs`, and a watch on the free-disk step of `Stand checks repeated`.
Re-proves every escher check on the synced engine: the `fast`, `doc`, `a11y` and `audit` legs, the stand checks in both layout modes, the incremental oracle, and fork CI green with every job settled.
Re-reads the per-OS tallies from the take's own green run, explaining each moved count, and re-checks the sink allowlist against the taken delta, the capture checks staying at 0 needles.

## Reached files
- Cargo.lock · conflict surface
- examples/seven_guis/src/tasks/cells.rs · conflict surface
- packages/blitz-dom/assets/default.css · conflict surface
- packages/blitz-dom/src/layout/construct.rs · conflict surface
- packages/blitz-dom/src/layout/inline/build.rs · conflict surface
- packages/blitz-dom/src/layout/writing_mode.rs · conflict surface
- packages/blitz-dom/src/mutator.rs · conflict surface
- packages/blitz-dom/src/node/node.rs · conflict surface
- packages/blitz-dom/src/scrolling.rs · conflict surface
- packages/blitz-dom/src/stylo.rs · conflict surface
- packages/blitz-paint/src/text.rs · conflict surface
- packages/blitz-vibey-script/src/runtime/bootstrap.rs · conflict surface
- packages/blitz-vibey-script/src/runtime/events.rs · conflict surface
- packages/dioxus-native-dom/src/dioxus_document.rs · conflict surface
- tests/blitz-tests/tests/pseudo_element_flow_classification.rs · conflict surface
- Cargo.toml · edits the coupled pins, where the chosen take moves them
- .github/workflows/ci.yml · edits the free-disk step's `if:` in the `matrix_test` job
- .github/scripts/ci-leg.sh · edits the local gate a font-embolden run joins
- packages/blitz-dom/src/lib.rs · edits the orphan comment «Re-export taffy»
- tests/blitz-tests/tests/select_popup.rs · edits the one row that skips the key press, where the founder's fork takes the fix
- .andromeda/residuals.md · edits the left-out-delta line and the merge-surfaces line, at the wrap
- .andromeda/architecture.md · edits the `Upstream sync:` line, at the wrap
- .github/scripts/test_blitz_tests_targets.py · re-reads the blitz-tests target list pin
- packages/escher-telemetry/src/format.rs · re-reads the sink allowlist
- packages/blitz-test-harness/src/contrast.rs · re-reads `Harness::text_contrast`'s `None` conditions
- tests/blitz-tests/tests/all.rs · re-reads upstream's target, declared and never built
- tests/blitz-tests/tests/select_popup_painted.rs · re-reads the font-embolden witness
- tests/blitz-tests/tests/stand_element_ids.rs · re-reads the stable element id check
- tests/blitz-tests/tests/stand_id_persistence.rs · re-reads the id persistence check
- tests/blitz-tests/tests/stand_id_edits.rs · re-reads the id stability check
- tests/blitz-tests/tests/incremental_oracle.rs · re-reads the incremental/non-incremental oracle
- tests/blitz-tests/tests/stand_keyboard.rs · re-reads the focus order and activation check
- tests/blitz-tests/tests/stand_contrast.rs · re-reads the contrast check
- tests/blitz-tests/tests/inert_subtree.rs · re-reads the inert subtree check

## Needs
- The local gates after the take: `ci-leg.sh fast` (fmt, clippy, test, ci-scripts) and `ci-leg.sh doc`, stopping at the first red, and how each leg runs.
- Fork CI on the pushed commit with every job settled: the fast four, the slow jobs behind them, the a11y and stand-repeat jobs, the coverage job and the windows, macOS, iOS and android matrix.
- The per-OS tallies: the method the plan records for reading test-bearing job logs, why linux, macOS and windows differ, iOS and android building only, and an explanation owed for each moved count.
- The stand checks in both layout modes and the incremental/non-incremental oracle as the proofs a conflict resolution in layout, node, scrolling or mutator code must still pass.
- The contracts a resolution keeps and the checks that hold them: the changed set, the clock invariant (`virtual_clock_engine`, `stand_virtual_clock`), the select model and its list (`select_interaction`, `stand_act_select`, `select_popup`, `select_popup_painted`, `stand_act_popup`), the hit stops (`hit_test_dropped_nodes`) and the scrolled-box bounds (`scrolled_box_absolute_position`).
- Upstream's new and edited test files under a take that carries them: one file and one target per behaviour, `tests/all.rs` declared and never built, the target list pinned by `BlitzTestsTargetsTest`, and upstream's test style (`verify_that!`, `TestResult`) under the standard harness.
- Numbers a moved Taffy, Stylo or winit can shift: a stand check's exact bounds, a snapshot's byte size, a screenshot's pixels, a committed check's kept digests. Each is re-measured and recorded, never loosened to pass.
- dioxus-native-dom as the most-modified crate: the element ids, the snapshot with its text, parts and diff, unkeyed actionables, the `accessibility_tree` override, the `stand_snapshot*` checks and the unit tests of the bridge and document events, with `6ef4b532d` editing its mounted-events path.
- The a11y leg and its standing proofs (`stand_keyboard`, `stand_contrast`, `accessibility_*`, `inert_subtree`) for WCAG SC 2.1.1, 1.4.3 and 2.4.3: focus order, Enter/Space activation, contrast and accessibility-tree identity.
- Stand contrast rows and `Harness::text_contrast`'s `None` conditions, re-read where a take carries `mix-blend-mode` and `isolation` (`6fca6308d`), `background-clip: text` (`ac6f9f582`), `-webkit-text-fill-color` (`e5929f9e2`) or the failed-stylesheet change (`f6edb9701`).
- The dependency audit: `ci-leg.sh audit`, its ignored IDs standing as the security rules record them, the licence check and hand review for any crate that enters, read from the lock per shape.
- The sink allowlist: engine and `escher_*` targets admitted, the rest dropped; the taken delta read for each new `tracing` or `log` call site, target, print and env read.
- The standing capture checks at 0 needles (`telemetry_*`, `host_log`, `mcp_log`, `stand_act_spans`, the session quiet checks), and the plan's recorded line counts by level where a take moves engine log lines.
- The free-disk step of `matrix_test` at `ci.yml:392-394`, skipped on every row because its `if:` names `ubuntu-24.04` and `ubuntu-24.04-arm` while android runs `ubuntu-latest`: fixed or removed, this entry's CI read its witness, and the `ci.yml` invariants pinned by `test_ci_workflows.py` must still hold.
- The watch on the free-disk step in `Stand checks repeated`: it retires on a recurrence of the out-of-disk red or on 3 green runs, so this take's CI read counts toward it.
- The key-press panic in `position_in_containing_block`: a controlled run at the base, a read of upstream's `node.rs`, and, if the founder's fork takes the fix, the skipped key press of `select_popup` restored and a regression check of the dropped anonymous block.
- The font-embolden gate: `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted` joins a local gate, where only CI's macOS leg builds that feature today, and `ci-leg.sh`'s legs are pinned under a `cargo` shim by `test_ci_workflows.py`.
- Engine features by runner: `tracing` and `writing-mode` are on in the workspace build only, so a resolution in `writing_mode.rs` or a new log call is proven by the workspace leg, a green `-p blitz-tests` run not standing in for it.
- The merge-surface rows re-read against the take, nothing sent upstream: the eight engine-debts changes, `0980cce5f`'s `hit_inner` hunk against the non-invertible answer, and the select list's sixteen files.
- The left-out delta re-read commit by commit with its measured reasons, and the plan's readings tied to what a take brings: blitz-dom's mutation, invalidation and damage checks (`fe33b778b`, `8b1a0a3ce`), computed sizes (`ae774d7f4`) and culling bounds (`e42ee1f25`).
- Script tests over the script engine as a security surface: `91ef65e21` adds `URLSearchParams`, a page-reachable API, and `bootstrap.rs` and `events.rs` conflict; blitz-vibey-script's `dom.rs` and `selection.rs` run on the result.
- The coverage floors on the coverage job, read per crate (`dioxus-native-dom` 83 %, `escher-driver` 97 %, `blitz-test-harness` 90 %), with dioxus-native-dom's `keyboard_compat.rs` excluded until escher takes upstream's retirement of it, which a take may bring.
- The stand-repeat leg's three rounds with no retry over the stand checks, the named targets and seven_guis' process checks, read on the synced engine.
- MSRV 1.91, proven by build only, and no new crate, port, env var or listener of ours, the manifests and lock re-read for any `members` change or new package.
- The re-counted baselines a take moves: `run stand` 237 · 0 · 7 over 51 files and `run all` 727 · 0 · 12, an upstream file with no `stand_` prefix picked up by `run all` alone.

## Standing conventions
- S1 · a new blitz-tests check is one file and one target per behaviour under `tests/blitz-tests/tests/`, with no new `[[test]]` table · .andromeda/test-plan-detail/test-strategy.md
- S2 · a regression test is named for the bug it guards and states what exposed it · .andromeda/test-plan-detail/test-strategy.md
- S3 · a stand check runs in both layout modes under the bundled font with no skip path, and holds no sleep or wall-clock read unless wall time is its subject · .andromeda/test-plan-detail/integration-test-strategy.md
- S4 · a check compared byte for byte across OSes holds no shot · .andromeda/test-plan-detail/test-strategy.md
- S5 · a fixture more than one check reads is stated once in `tests/blitz-tests/tests/fixtures/mod.rs`, a one-reader page stays in its check's file, and test data is stated in code · .andromeda/test-plan-detail/test-data-fixtures.md
- S6 · a new unit test sits in a `#[cfg(test)]` module beside its code, or a `#[path]` file of its own, adding no constructor, `pub` or test shim to the code under test · .andromeda/test-plan-detail/unit-test-strategy.md
- S7 · a new test uses hand-written stand-ins, with no mocking library or new test framework · .andromeda/test-plan-detail/mocking-stubbing-discipline.md
- S8 · a new CI step or leg runs through `ci-leg.sh`, every cargo leg `--locked` but `fmt`, actions pinned to SHAs, no `secrets.`, read-only permissions and every package install bounded · .andromeda/test-plan-detail/ci-integration.md
- S9 · a check reading a sink's capture counts lines by target, never in total · .andromeda/test-plan-detail/ci-integration.md
- S10 · a new test holds the zero-failure budget, with no retry · .andromeda/test-plan-detail/quality-gates-coverage-targets.md
- S11 · a new test moves the recorded `run stand` and `run all` readings, re-counted with where they were measured · §3 Test Harness Contract, Agent-run contract, Proof

## Not needed
- WPT conformance numbers, scoring and diffs, the WPT runner's own tests and ports, and the repository-guarded WPT workflows: they are upstream-only and out of this chunk's scope.
- The browser crates' test cases and helpers (about pages, history, favicon, suggestions, persistence, tabs): the scope changes nothing there, and they reach this chunk only inside the workspace leg's total.
- The cold-agent pipe's live run, its verdict fields and the operator pass: never part of CI, and the scope does not run them.
- The mutation tier: a dev-host run, never in CI, which the scope does not ask for.
- The capability statement's pin over `docs/capabilities.md` and README.md: no driver verb, refusal cause or tool count moves, and the chunk adopts no new upstream API in escher's code.
- The driver's wire grammar, session label rule, session host and socket lifecycle, MCP wire form and recording format: escher-owned code outside every predicted conflict, run by the legs and rebuilt by nothing here.
- The harness's construction, pump, settle and input-helper contracts: the chunk drives them only through the standing checks.
- Human-read example fixtures, the manual tools `paint_bench` and `screenshot`, the ignored benchmarks and the commented-out `tests/stylo_usage.rs`: they assert nothing a take must re-prove.
- Tests of the UI graph and later entries: this chunk builds nothing of the graph, and those entries land on the synced engine afterwards.
