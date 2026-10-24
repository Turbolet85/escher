# tests · Test Strategy — category read

## Carried
- (.andromeda/test-plan-detail/test-strategy.md:8) The contrast re-read must go through the harness's contrast reader as stated: it reads and changes nothing, composites ancestors' backgrounds over opaque white, reads `None` past opacity, a filter or a background image, and grades by the WCAG 2.x ratio.
  > and since 2026-10-13-stand-contrast-harness a contrast reader — `Harness::text_contrast(node_id) -> Option<TextContrast>`, reading and changing nothing (no mutation, input, pump or settle; every node by `get_node`): the text-bearing element's computed `color`, font size and weight over the background composited source-over from its ancestors' `background-color`s onto the root-else-`body` canvas over opaque white, `None` with no text or where an element on the walk has `opacity` below 1, a filter or a background image, and `contrast_ratio` the WCAG 2.x ratio over the `color` crate's relative luminance (packages/blitz-test-harness/src/contrast.rs:13-23; packages/blitz-test-harness/src/contrast.rs:56-91)
- (.andromeda/test-plan-detail/test-strategy.md:34) The re-asserted stand checks (`stand_keyboard`, `stand_contrast`) and any row added to them must assert unconditionally under the bundled font, with no skip path.
  > the headless-stand checks have no skip path — they assert unconditionally under the bundled DejaVu Sans `seven_guis::stand::font_ctx()` registers with system fonts off, passed through `HarnessOptions.font_ctx` (examples/seven_guis/src/stand.rs:52-54; tests/blitz-tests/tests/stand_boot.rs:51-65)
- (.andromeda/test-plan-detail/test-strategy.md:46) A new check stays one target file per behaviour, and a new `stand_*.rs` reader of `tests/common/mod.rs` moves the stated reader count of 21.
  > `tests/blitz-tests` holds integration tests, one test-target file per behavior under `tests/`, and three shared modules that are no target — `tests/common/mod.rs`, read through `mod common;` by 21 `stand_*.rs` checks (19 until 2026-10-14-stand-requirement-sweep added `stand_requirement_sweep.rs` and `stand_non_lean_tasks.rs`, 18 until 2026-10-13-stand-keyboard-harness added `stand_keyboard.rs`, 17 until 2026-10-13-stand-contrast-harness added `stand_contrast.rs`)
- (.andromeda/test-plan-detail/test-strategy.md:47) A new reader of the shared fixtures module (a select fixture for a keyboard row, say) moves the stated count of 24 readers; `stand_keyboard` and `select_interaction` already read it.
  > since 2026-10-19-shared-fixtures, `tests/fixtures/mod.rs`, read through `mod fixtures;` by 24 checks — 22 `stand_*.rs` (
- (.andromeda/test-plan-detail/test-strategy.md:48) A new blitz-tests target file is auto-discovered with no manifest line and moves the stated file and target counts.
  > every other `.rs` file directly under `tests/` stays its own auto-discovered target: 145 `.rs` files, 144 test targets and the unbuilt `all.rs`
- (.andromeda/test-plan.md:9) The workspace `test` leg must stay green on the re-read.
  > `cargo test --workspace --locked` runs on ubuntu with default features as the fast `test` leg, `bash .github/scripts/ci-leg.sh test` (.github/workflows/ci.yml:78-98; .github/scripts/ci-leg.sh:26)
- (.andromeda/test-plan.md:19) The select's stated limits are asserted by no check — a keyboard row for the flight booker's select must not be read as closing that gap unless it asserts them, and if it does the line moves.
  > the text extent's stated limits are asserted by blitz-tests' `text_extent`, while the select's are asserted by no check, nor is the process-wide `Parallel` traversal lock
- (.andromeda/test-plan.md:48) The adapter-side AT witness stays owed by its residuals line; the re-read must not claim it.
  > no check reads the accessibility states as the platform adapter hands them to an assistive technology — owed, owner: its `.andromeda/residuals.md` line (target: next)
- (.andromeda/test-plan.md:124) If `accessibility_label` or `accessibility_description` join the a11y leg, they run under the standard `#[test]` harness through test-that's `verify_that!`, with no new framework.
  > beside which blitz-tests' merged upstream checks (`accessibility_label`, `accessibility_description`, since 2026-10-17-upstream-sync-engine-debts) return `TestResult<()>` and assert with test-that's `verify_that!` under the same `#[test]` harness
- (.andromeda/test-plan.md:132) A typeahead row (its 1 s buffer on the document's clock) must move time through the harness's `VirtualClock` (`advance_time` / `tick`), never wall time; `text_contrast` moves no time.
  > The harness's time is a `VirtualClock` its documents read: `time` reads it in seconds and `pump` resolves at it; `tick(dt)` advances it, a negative or non-finite `dt` moving nothing, and `advance_time(Duration)` advances it by exactly the duration, each then pumping; `screenshot`, `screenshot_rect`, `text_contrast` and `settle` never move it (ratified by the founder, 2026-10-18)
- (.andromeda/test-plan.md:136) `stand_keyboard`'s re-read keeps its every-OS Apple delete-binding test green on linux, windows and macOS.
  > since 2026-10-13-stand-keyboard-harness `stand_keyboard::the_apple_delete_binding_deletes_on_every_os` calls `apple_keybinding(deleteBackward:)` and reads the deletion on every OS
- (.andromeda/test-plan.md:146) The Tab-order re-assertion reads these shared tables, and a sequence 0.2.0 moved is re-recorded in them.
  > and, since 2026-10-13-stand-keyboard-harness, `tab_order(task)` (each lean task's Tab sequence, CRUD six stops with `crud-list` third, moved in from `stand_accessibility_ids`), and, since 2026-10-14-stand-a11y-assertions, `ENABLED_TAB_ORDER` (each lean task's Tab sequence in an enabled state — the flight booker after `flight-return`, six stops, and CRUD after a row is selected, eight stops, `crud-update` and `crud-delete` among them — read by `stand_keyboard`) and `REGION_NAMES` (`crud-list` → `People`, read by `stand_accessibility_ids`)
- (.andromeda/test-plan.md:147) Any table the chunk edits or adds must hold stated expectation, never a mapping derived from the code.
  > the tables are stated expectation, never a second role or name mapping
- (.andromeda/test-plan.md:164) The agent harness selects `stand_*` files by prefix; an a11y file with no `stand_` prefix is reached only by `run {name}` or `run all`, and an empty run is never a pass.
  > `stand` = one `--test {stem}` per `tests/blitz-tests/tests/stand_*.rs` present · `all` = the whole package, no `--test` · `{name}` = one file, matching `^[a-z0-9_]+$` with `tests/blitz-tests/tests/{name}.rs` present
- (.andromeda/test-plan.md:164) The run outcome rule the re-read's runs are judged by.
  > The outcome is `passed` only when cargo exited 0, no test failed and at least one test line parsed — otherwise `failed`, exit 1: an empty run is never a pass.
- (.andromeda/test-plan.md:175) The select's checks lie on either side of the prefix: `stand_act_select` in `run stand`, `select_interaction` by name and `run all` only.
  > `select_interaction`, no `stand_` prefix, is read by name and by `run all`
- (.andromeda/test-plan.md:176) The baseline `run stand` count a row added to `stand_keyboard` or `stand_contrast` moves; the re-count is stated with the script unchanged.
  > re-counted with `stand-repeat`'s ten named targets, `scripts/agent-run.sh` unchanged: `run stand`'s `run.end` reads passed 197 · failed 0 · ignored 7 over 48 files
- (.andromeda/test-plan.md:211) Widening the a11y leg moves `test_ci_workflows.py`'s pin of `ci-leg.sh`'s legs, which must stay green in the `ci-scripts` leg.
  > test_ci_workflows.py pins ci.yml's invariants — the triggers, the caches, the fast four and their `needs`, each leg through `ci-leg.sh`, the failure-log uploads, no `secrets.`, no perl rewrite, actions pinned to commit SHAs, read-only permissions, every package install bounded — the upstream repository guard, `ci-leg.sh`'s legs under a `cargo` shim
- (.andromeda/test-plan.md:259) A session check the chunk touches (`stand_act_select`) takes no sleep and no wall-clock read.
  > no sleep and no wall-clock read in the checks, except the two whose subject is wall time
- (.andromeda/test-plan.md:374) The widened a11y leg runs through `ci-leg.sh`, `--locked`, host-reproducible; the `stand-repeat` leg's set of ten named targets is a separate list the chunk must not confuse with the a11y leg's.
  > each linux ci.yml job runs `bash .github/scripts/ci-leg.sh {leg}`, every cargo leg `--locked` but `examples/wasm_hello` (no `Cargo.lock`); the dev host runs the same script, so every linux leg is host-reproducible; the `stand-repeat` leg runs the stand checks, ten named blitz-tests targets outside the `stand_` prefix and seven_guis' process checks for three rounds, the first red command ending the leg with its status — no retry, no `--ignored`, no env var
- (.andromeda/test-plan.md:376) The a11y job keeps its place behind the fast four.
  > `fmt`, `clippy`, `test-features-default` and `ci-scripts` carry no `needs`; every other job, the windows/macos/ios/android matrix included, `needs` all four
- (.andromeda/test-plan.md:378) The a11y job's cache posture is kept: it restores the test job's cache and saves none.
  > the test job saves under `shared-key: workspace-test`, which the `a11y` and `stand-repeat` jobs restore with `save-if: false`
- (.andromeda/test-plan.md:380) The widened leg's failure output lands in its own log under the fixed path and upload rule.
  > each leg's merged output goes to `target/ci-logs/{leg}.log` (truncated at the leg's start) and is uploaded only on failure as `ci-log-{job id}`, kept 7 days
- (.andromeda/test-plan.md:384) The local pre-push gate the chunk must pass.
  > `bash .github/scripts/ci-leg.sh fast` — fmt → clippy → test → ci-scripts, stopping at the first red
- (.andromeda/test-plan.md:387) The a11y leg's stand checks read the same on any runner under the bundled font, the package-alone build included.
  > except the headless-stand checks, which use only the bundled DejaVu Sans with system fonts off and so measure the same on any runner, in the `-p blitz-tests` package-alone build included
- (.andromeda/test-plan.md:389) A green a11y leg run per package is not the workspace leg's green; both builds must be read.
  > `cargo test --workspace --locked` and `cargo test -p blitz-tests --locked` build the same test file against different engine features — `tracing` and `writing-mode` on in the workspace build only — so a check that reads a sink's capture counts lines by target, never in total, a green per-package run is not the workspace leg's green
- (.andromeda/test-plan.md:410) The re-asserted stand checks sit under the zero-failure budget of `stand-repeat`, three rounds with no retry.
  > the stand checks hold a zero-failure budget — the `stand-repeat` leg and its job `Stand checks repeated` run the `stand_*.rs` files, the ten named blitz-tests targets and seven_guis' process checks three times with no retry (§9 Legs)

## Bindings
- **The a11y leg's widened file list ↔ a11y-plan §9 / §Bootstrap**: the leg runs through `ci-leg.sh` and its `ci-scripts` pin moves with it (.andromeda/test-plan.md:211; .andromeda/test-plan.md:374); a11y-plan §9 names which files witness an a11y requirement — the two lists must agree.
- **`tab_order` · `ENABLED_TAB_ORDER` ↔ a11y-plan (SC 2.4.3, SC 2.1.1) and CLAUDE.md's Critical Warnings**: a focus-order change moves the shared tables, re-recorded with the change named (.andromeda/test-plan.md:146; .andromeda/test-plan.md:147).
- **The contrast reader ↔ a11y-plan (SC 1.4.3) and design-system §Color Palette**: `stand_contrast`'s rows are graded by `text_contrast`'s rule and `required_ratio` (.andromeda/test-plan-detail/test-strategy.md:8).
- **The `run stand` count ↔ the agent-run contract (body §3)**: rows added in `stand_*` files move the 197 · 0 · 7 over 48 baseline; a11y files outside the prefix stay reachable by name or `run all` only (.andromeda/test-plan.md:164; .andromeda/test-plan.md:176).
- **The a11y leg ↔ the `stand-repeat` leg's ten named targets (body §9, §10)**: two separate lists; widening one does not widen the other (.andromeda/test-plan.md:374; .andromeda/test-plan.md:410).
- **The adapter AT witness ↔ residuals and security rules (out of scope)**: stays owed by its residuals line (.andromeda/test-plan.md:48).
- **Typeahead timing ↔ blitz-dom's document clock (architecture)**: the 1 s buffer is moved by the harness's `VirtualClock`, never by a sleep (.andromeda/test-plan.md:132; .andromeda/test-plan.md:259).
