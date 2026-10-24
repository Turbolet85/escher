# tests · cascade read · group 3 of 4

## Carried
- (.andromeda/test-plan-detail/test-strategy.md:8) Every re-asserted `stand_contrast` row — control, CRUD list, text and colour-state rows — must read its pair through the harness's contrast reader, which moves no state, so the re-read changes nothing it measures.
  > since 2026-10-13-stand-contrast-harness a contrast reader — `Harness::text_contrast(node_id) -> Option<TextContrast>`, reading and changing nothing (no mutation, input, pump or settle; every node by `get_node`): the text-bearing element's computed `color`, font size and weight over the background composited source-over from its ancestors' `background-color`s onto the root-else-`body` canvas over opaque white, `None` with no text or where an element on the walk has `opacity` below 1, a filter or a background image
- (.andromeda/test-plan-detail/unit-test-strategy.md:14) The ratio and the threshold each `stand_contrast` row asserts must be the ones `contrast.rs`'s stated tables pin — 4.5, or 3.0 for large text — so SC 1.4.3 is judged by the pinned reader, not a figure restated in the check.
  > since 2026-10-13-stand-contrast-harness `contrast.rs` pins the contrast reader — `contrast_ratio` as the WCAG 2.x ratio over a stated table of 5 rows (21.0 both ways, 1.0, 4.48 and 4.39 within 0.005), `required_ratio` over a stated table of 5 rows (3.0 for large text — at least 24 px, or 14 pt at weight 700 — else 4.5)
- (.andromeda/test-plan-detail/test-strategy.md:34) `stand_keyboard` and `stand_contrast`, and any row the chunk adds to them, must assert unconditionally under the bundled font with system fonts off, with no skip path.
  > the headless-stand checks have no skip path — they assert unconditionally under the bundled DejaVu Sans `seven_guis::stand::font_ctx()` registers with system fonts off, passed through `HarnessOptions.font_ctx`
- (.andromeda/test-plan-detail/test-data-fixtures.md:31) The keyboard and contrast rows must measure under this one bundled payload, so their readings are the same on every runner the widened leg and `stand-repeat` run on.
  > | Font payload | the embedded bullet font; the bundled DejaVu Sans woff2 `seven_guis::DEJAVU_SANS`, which the headless stand registers for every generic |
- (.andromeda/test-plan.md:94) The broad rule the stand rows inherit: no skip path for a headless-stand check.
  > the headless-stand checks have no skip path — they assert unconditionally under the bundled DejaVu Sans with system fonts off
- (.andromeda/test-plan-detail/test-data-fixtures.md:30) The Tab sequences, activation rows and contrast rows must be read at the stand's pinned viewport, never a size the check sets for itself.
  > | Viewports | fixed sizes (800x600 and 400x300, scale 1.0, Light) |
- (.andromeda/test-plan-detail/test-strategy.md:46) `tests/common/mod.rs` is a reached file: a re-recorded `tab_order` or `ENABLED_TAB_ORDER` row lands in it, and its stated reader count moves only when a check that declares `mod common;` is added.
  > `tests/blitz-tests` holds integration tests, one test-target file per behavior under `tests/`, and three shared modules that are no target — `tests/common/mod.rs`, read through `mod common;` by 21 `stand_*.rs` checks
- (.andromeda/test-plan.md:146) The shared module must stay the one statement of the stand tables, with no test and no target of its own, the Tab sequences living there and nowhere else.
  > `tests/common/mod.rs` is the one statement of the tables and helpers the stand checks share, with no test in it and no test target of its own
- (.andromeda/test-plan.md:147) A re-recorded Tab sequence or a new select row is stated expectation, never a mapping that re-derives roles or names.
  > the tables are stated expectation, never a second role or name mapping
- (.andromeda/test-plan-detail/test-strategy.md:47) `stand_keyboard` and the leg candidate `select_interaction` declare the fixtures module, so a widened leg builds it under the workspace's engine features.
  > `stand_replay`, `stand_keyboard`, `stand_snapshot_state`), `font_face_sniffed_source.rs` and `select_interaction.rs`
- (.andromeda/test-plan-detail/test-strategy.md:48) A new a11y check, if the chunk adds one, must be its own auto-discovered target under `tests/` with no manifest line, and the stated file and target counts move by it.
  > while `[package]` sets no `autotests` and every other `.rs` file directly under `tests/` stays its own auto-discovered target: 145 `.rs` files, 144 test targets and the unbuilt `all.rs`
- (.andromeda/test-plan-detail/unit-test-strategy.md:36) The blitz-tests manifest must keep exactly its two-line difference from upstream whatever file the chunk adds, `BlitzTestsTargetsTest` green on it.
  > asserts that `[package]` does not set `autotests` to false and that the manifest holds exactly one `[[test]]` table, `all` at `tests/all.rs` with `test = false`: the standing check on the fork's two-line difference from upstream's manifest
- (.andromeda/test-plan.md:213) The body's statement of the same standing check, which a target added to the leg never needs a manifest line to pass.
  > `BlitzTestsTargetsTest` reads `tests/blitz-tests/Cargo.toml` with `tomllib` and asserts that `[package]` does not set `autotests` to false and that the manifest holds exactly one `[[test]]` table
- (.andromeda/test-plan-detail/unit-test-strategy.md:64) The edited blitz-tests files must hold no in-file `#[cfg(test)]` module; a check is a target file, its helpers in the shared modules.
  > In-file `#[cfg(test)]` unit-test modules in tests/blitz-tests · searched: `#\[cfg` over the 61 files of slice s12
- (.andromeda/test-plan.md:234) The body's absence the edited checks keep.
  > or in-file in tests/blitz-tests — each observed absent by its search
- (.andromeda/test-plan-detail/unit-test-strategy.md:27) `test_ci_workflows.py` is a reached merge surface: the a11y job must stay restore-only on the test job's cache and run its leg through `ci-leg.sh`, the leg present in the script's list, however its target list widens.
  > a rust-cache (matched by its `Swatinem/rust-cache@` prefix) with a `main` + `build/` save rule on every compiling job but the cache-less `coverage` and the restore-only `a11y` and, since 2026-10-14-quality-gates, `stand-repeat` (each carries the test job's `shared-key` and `save-if: false`), the fast four with no `needs` and every other job needing all four, each linux job's leg through `ci-leg.sh` (the leg present in the script's list)
- (.andromeda/test-plan-detail/unit-test-strategy.md:27) The a11y job keeps its accessibility name, which the invariant pins.
  > an a11y job named for accessibility
- (.andromeda/test-plan-detail/unit-test-strategy.md:28) The `LegScriptTest` a11y row must pin the widened leg's test files in the leg's own order, moving in the same change as `ci-leg.sh`'s list.
  > the `a11y` row pinning all six test files in order, `accessibility_hidden`, `accessibility_roles`, `focusability_updates`, `stand_keyboard`, `stand_contrast`, `stand_accessibility_ids`, the last three added at 2026-10-14-stand-a11y-assertions
- (.andromeda/test-plan-detail/unit-test-strategy.md:29) The CI-scripts tally must be re-read if the chunk adds a test case to `test_ci_workflows.py`; a re-shaped tuple alone leaves it unmoved.
  > the CI-scripts leg runs 100 tests over seven test files — test_ci_workflows.py's 33
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:43) The edited `test_ci_workflows.py` sits in this search's set and must keep its hand-written `cargo` shim, with no mocking library added.
  > Mocking libraries in CI and build configuration · searched: `mock` over .github/workflows/*.yml, Cargo.toml, Cross.toml, flake.nix, .github/scripts/*.py
- (.andromeda/test-plan-detail/unit-test-strategy.md:49) A select keyboard row in `stand_keyboard` must assert what the select's stated tables pin — a step skipping disabled options and stopping at the ends, typeahead choosing the first enabled option whose label starts with the text.
  > four `node/select.rs` tests pin the select's rules over stated tables — HTML's selectedness setting algorithm (the last chosen, else the first option not disabled), a step skipping disabled options and stopping at the ends, typed text choosing the first enabled option whose label starts with it and nothing for an empty text
- (.andromeda/test-plan-detail/unit-test-strategy.md:49) The enabled-state Tab sequences re-read after 0.2.0 rest on `disabled` read by its presence on a form control, the element state these tests pin.
  > four tests assert `DISABLED`/`ENABLED` element state for a button with `disabled` (empty or `"false"` value), an `<a>` with `disabled`, and a bare button
- (.andromeda/test-plan.md:19) A select row that asserts one of the select's stated limits moves this statement; one that does not leaves it standing.
  > while the select's are asserted by no check
- (.andromeda/test-plan-detail/integration-test-strategy.md:17) If `stand_act_select` is taken into the a11y leg, these assertions must hold there under the workspace's engine features, in both layout modes.
  > a `type` on a single-choice select chooses the first option whose label starts with the text — accepted and settled, its diff naming the select with `value` in `differs` and the two options whose `selected` moved, the select's options read as its children named by their labels, a disabled select skipped by the Tab walk and refused, in both layout modes (`stand_act_select`
- (.andromeda/test-plan-detail/integration-test-strategy.md:15) A typeahead or select row the chunk adds must take no sleep and read no wall clock; the two recorded exceptions are the only ones.
  > with no sleep and no wall-clock read in the checks — bar two recorded exceptions whose subject is wall time
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:26) The typeahead buffer resets after 1 s of the document's clock, which is the harness's `VirtualClock`: a row crossing that second moves it through the harness (or a session's `advance`), never through elapsed wall time.
  > the driver moves that time by `advance`, which moves the session's clock — the harness's `VirtualClock`, the clock every harness-built document reads — by exactly `ms` inside one settled step
- (.andromeda/test-plan.md:124) If `accessibility_label` and `accessibility_description` join the leg, they keep upstream's `verify_that!` form under the standard harness, unrewritten.
  > beside which blitz-tests' merged upstream checks (`accessibility_label`, `accessibility_description`, since 2026-10-17-upstream-sync-engine-debts) return `TestResult<()>` and assert with test-that's `verify_that!` under the same `#[test]` harness
- (.andromeda/test-plan.md:389) A target added to the a11y leg is green only when read under the workspace build; a per-package run of it is not the leg's green.
  > so a check that reads a sink's capture counts lines by target, never in total, a green per-package run is not the workspace leg's green
- (.andromeda/test-plan.md:410) A row added to `stand_keyboard` or `stand_contrast` joins the zero-failure budget, three rounds with no retry.
  > the stand checks hold a zero-failure budget — the `stand-repeat` leg and its job `Stand checks repeated` run the `stand_*.rs` files, the ten named blitz-tests targets and seven_guis' process checks three times with no retry
- (.andromeda/test-plan.md:176) A row added to a `stand_*` check moves `run stand`'s stated total, which is re-counted and recorded, never left at its old figure.
  > re-counted with `stand-repeat`'s ten named targets, `scripts/agent-run.sh` unchanged: `run stand`'s `run.end` reads passed 197 · failed 0 · ignored 7 over 48 files
- (.andromeda/test-plan.md:48) The windowed AT witness stays out: the statement and its owner stand, and no check the chunk adds claims it.
  > no check reads the accessibility states as the platform adapter hands them to an assistive technology — owed, owner: its `.andromeda/residuals.md` line (target: next)

## Bindings
- **strategy (font-dependent tests) ↔ test data (font payload)**: the no-skip rule binds the rows to the one bundled payload, so the Tab and contrast readings are runner-independent — (.andromeda/test-plan-detail/test-strategy.md:34) ↔ (.andromeda/test-plan-detail/test-data-fixtures.md:31)
- **strategy (contrast reader) ↔ unit (contrast.rs tables)**: every `stand_contrast` row reads through the reader whose ratio and thresholds the unit tables pin — (.andromeda/test-plan-detail/test-strategy.md:8) ↔ (.andromeda/test-plan-detail/unit-test-strategy.md:14)
- **strategy (directory pattern) ↔ unit (blitz-tests targets)**: a new a11y check file is an auto-discovered target and the manifest pin holds unchanged — (.andromeda/test-plan-detail/test-strategy.md:48) ↔ (.andromeda/test-plan-detail/unit-test-strategy.md:36)
- **strategy (shared module readers) ↔ the body (stand tables)**: the reached `tests/common/mod.rs` holds the re-recorded sequences as stated expectation — (.andromeda/test-plan-detail/test-strategy.md:46) ↔ (.andromeda/test-plan.md:146)
- **unit (select tables) ↔ integration (`stand_act_select`) ↔ mocking (time)**: a select keyboard row asserts the pinned rules, the driver check states the same choice, and the typeahead reset is crossed on the harness's clock — (.andromeda/test-plan-detail/unit-test-strategy.md:49) ↔ (.andromeda/test-plan-detail/integration-test-strategy.md:17) ↔ (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:26)
- **integration (no wall clock) ↔ mocking (virtual time)**: a typeahead row moves time only through the harness's `VirtualClock` — (.andromeda/test-plan-detail/integration-test-strategy.md:15) ↔ (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:26)
- **unit (CI workflow invariants) ↔ mocking (no mocking library) ↔ the body (engine features by runner)**: the widened a11y tuple moves in `test_ci_workflows.py` under its hand-written shim, and the leg's green is the workspace build's — (.andromeda/test-plan-detail/unit-test-strategy.md:28) ↔ (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:43) ↔ (.andromeda/test-plan.md:389)
- **strategy (fixtures readers) ↔ the body (engine features by runner)**: `stand_keyboard` and `select_interaction` build the fixtures module, which the widened leg compiles under the workspace's features — (.andromeda/test-plan-detail/test-strategy.md:47) ↔ (.andromeda/test-plan.md:389)
