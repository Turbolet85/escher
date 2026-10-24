# Codebase Research — 2026-10-24-a11y-re-read

## Scope
- **Depth:** moderate · **Reads:** 9 · **Globs/Greps:** 8 (plus one code-graph query and one live leg run)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, 1 Session Addition (the
  `timeout` exit-124 note; no bounded smoke in this chunk) · `.claude/rules/testing.md` — read in full, 9 Session
  Additions (the 2026-10-10 "search tests/ and the owning crate for a pinned table" rule applied: the a11y leg's word
  list is pinned in `.github/scripts/test_ci_workflows.py:298-299` alone) · `.claude/rules/a11y.md` — read in full, 1
  Session Addition (the AT-SPI bus note; no windowed boot in this chunk)
- **Platform issues consulted:** none — no runner-only bullet; CI#40125511019 read green at P3
- **External inputs:** none — every fact this chunk turns on lives in this repository

## Files inspected
- `.github/scripts/ci-leg.sh` (1-80) — line 39 is the `a11y` leg: `cargo test --workspace --locked` with six `--test`
  words; lines 48-62 the `stand-repeat` leg, its ten named non-stand targets on lines 55-58
- `.github/scripts/test_ci_workflows.py` (270-376) — `test_gate_legs_dispatch_to_cargo_and_write_their_log`
  (lines 295-305) pins the a11y leg by `assertIn` per word over the leg log a `cargo` shim writes: `--workspace`,
  `--locked` and the six file names, in no order; `test_stand_repeat_leg_runs_three_rounds` (351-365) pins
  stand-repeat's words
- `.github/workflows/ci.yml` (247-268) — the `a11y` job "Accessibility (a11y) tests": `needs: [fmt, clippy,
  test-features-default, ci-scripts]`, `shared-key: workspace-test`, `save-if: false`, runs `bash
  .github/scripts/ci-leg.sh a11y`, uploads `target/ci-logs/a11y.log` on failure only; no file name appears in it
- `tests/blitz-tests/tests/common/mod.rs` (full) — `controls` (17-46), `tab_order` (50-70), `ENABLED_TAB_ORDER`
  (74-99): the flight booker holds `flight-one-way` · `flight-return` buttons and `flight-start` ·
  `flight-return-date` text inputs, no select
- `tests/blitz-tests/tests/stand_requirement_sweep.rs` (815-1069) — its leg check reads `ci-leg.sh` by
  `include_str!` (line 952) and asserts only the `stand_*.rs` glob and each `STAND_REPEAT_NAMED` /
  `STAND_REPEAT_SEVEN_GUIS` word (lines 1011-1024); the a11y line is not read
- `tests/blitz-tests/tests/stand_keyboard.rs` (test list) — 14 tests, among them 0.2.0's
  `an_inert_subtree_is_skipped_by_tab` (753), `a_disabled_attribute_skips_a_form_control_and_not_a_div` (615) and
  `a_button_disabled_after_it_took_focus_takes_no_key` (517)
- the doc headers of the candidate witnesses (`accessibility_states`, `inert_subtree`,
  `accessibility_active_descendant_unread`, `accessibility_label`, `accessibility_description`,
  `accessibility_merged_states`, `accessibility_names`, `select_interaction`, `stand_act_select`, `focus_redraw`,
  `focus_within`) — what each asserts, below
- `target/ci-logs/a11y.log` via the leg run — `bash .github/scripts/ci-leg.sh a11y` at HEAD db585b711, exit 0:
  accessibility_hidden 6 · accessibility_roles 6 · focusability_updates 4 · stand_accessibility_ids 8 · stand_contrast
  5 · stand_keyboard 14 = 43 tests, 0 failed

## Graph impact (from the code-graph query)
- **tab_order** — read at `tests/blitz-tests/tests/stand_accessibility_ids.rs:24`, `:247`, `:368`,
  `tests/blitz-tests/tests/stand_keyboard.rs:38`, `:95`, `:129`, `:159`, `:345`, `:580` and
  `tests/blitz-tests/tests/stand_requirement_sweep.rs:27`, `:148` — the chunk changes no table, so no reader moves
- **ENABLED_TAB_ORDER** — read at `tests/blitz-tests/tests/stand_keyboard.rs:38`, `:252`, `:338` alone
- **controls** — read in eight stand files (`stand_accessibility_ids`, `stand_act_ids`, `stand_act_obstructed`,
  `stand_contrast`, `stand_keyboard`, `stand_session_fresh`, `stand_session_ids`, `stand_snapshot`) — unchanged
- **text_contrast** — defined `packages/blitz-test-harness/src/contrast.rs`, its one non-test caller
  `packages/escher-driver/src/lint.rs:518`, read by `stand_contrast.rs:181`, `:525` and `stand_lint.rs:216` — the
  contrast re-read and the lint share one exempt rule; neither changes here
- query trace: `.andromeda/runs/2026-10-24T19-44-05-phase/tree-query-2026-10-24-a11y-re-read.json` (plane rust;
  every name had rows)

## Patterns detected
- **One leg line, one pin** (`.github/scripts/ci-leg.sh:39`, `.github/scripts/test_ci_workflows.py:298-299`): a
  leg's selection is literal `--test` words on one cargo line, and its pin asserts each word in the shimmed log;
  widening it adds words to both, nothing else.
- **Named non-stand targets** (`.github/scripts/ci-leg.sh:54-58`): stand-repeat names its non-`stand_` targets one
  by one with a comment naming who chose them (the founder's word, 2026-10-24) — the precedent for a named list.
- **Fixture rows beside stand rows** (`tests/blitz-tests/tests/stand_keyboard.rs:400`, `:605`, `:739`): an engine
  reading the stand's markup never exercises is witnessed on an in-file fixture inside the same check.

## Conventions to follow
- **The leg stays one cargo line under `--workspace --locked`** (`.github/scripts/ci-leg.sh:39`): every named
  target must exist as an auto-discovered file target (`tests/blitz-tests/tests/{name}.rs`); `BlitzTestsTargetsTest`
  pins the manifest's lone `all` target.
- **A pin moves with its leg in the same change** (`.github/scripts/test_ci_workflows.py:298-299`).
- **The a11y job keeps its tier** (`.github/workflows/ci.yml:247-268`): no new job, no required status check, no
  cache save — the widening is the leg script and its pin only.

## New files to create
- none

## Files to modify
- `.github/scripts/ci-leg.sh` — the `a11y` leg's `--test` list widened by the files the P4 fork chooses
- `.github/scripts/test_ci_workflows.py` — the `a11y` tuple of `test_gate_legs_dispatch_to_cargo_and_write_their_log` names the same words

## Sweeps
- `select` over examples/seven_guis/src/ (`git grep -n 'select\b\|select {'`): 3 hits · 0 changed · 3 no-change
  (Home's `on_select` handler in app.rs, no select element)
- `inert|aria-labelledby|aria-describedby|aria-checked|aria-expanded|aria-disabled|transform|contain:` over
  examples/seven_guis/src/ (`git grep -n -E`): 0 hits · 0 changed · 0 no-change
- `disabled` over examples/seven_guis/src/tasks/ (`git grep -n`): 9 hits · 0 changed · 9 no-change (on `button` and
  `input` elements and their `:disabled` rules — form controls, so 0.2.0's narrowing moves none)
- the obs silence probe `println!|eprintln!|print!|dbg!|std::env|env::var|escher_telemetry|tracing_subscriber|UnixListener|TcpListener|bind\(`
  over the eleven candidate files (`grep -cE` per file): 0 hits in each · 0 changed
- added files over tests/blitz-tests/tests/ across 0.2.0 (`git diff --name-status --diff-filter=A
  c71f7ed14d5b^ HEAD`, filtered to access|inert|select|focus|keyboard|contrast): 9 hits · 0 changed · 9 no-change
  (the candidates listed in scope.md)
- `a11y` over .github/workflows/ci.yml (`grep -n`): 4 hits · 0 changed · 4 no-change (the job, its name, its run
  line and its artifact name)

## Open questions
- Which files join the `a11y` leg: (a) the tier-0 witnesses 0.2.0 added — `inert_subtree` (SC 2.4.3 · 2.1.1, Tab
  skips an inert subtree), `select_interaction` and `stand_act_select` (SC 2.1.1, a select by keys and through the
  driver) — or (b) those three plus 0.2.0's tree-reading witnesses outside tier 0 — `accessibility_states`,
  `accessibility_label`, `accessibility_description`, `accessibility_merged_states`,
  `accessibility_active_descendant_unread` (SC 4.1.2 reads, the last a pinned gap) — as the leg already runs
  `accessibility_hidden` and `accessibility_roles`, which are not tier-0 either → blocks: plan-decision (P4 fork;
  the leg's membership was set by the founder's F4 of 2026-10-14 and stand-repeat's named list by his word of
  2026-10-24, so it is his fork)
</content>
</invoke>
