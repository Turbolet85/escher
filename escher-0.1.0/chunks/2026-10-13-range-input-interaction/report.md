# Report — 2026-10-13-range-input-interaction

**Chunk:** Range input interaction — arrow keys step a range input's value, a click sets it by position, an input event fires
**Date:** 2026-10-13T21:11:52Z
**Commits:** since `last_wrap` (2026-10-13T18:31:43Z; `git log --format='%h %s' 97ce760d..HEAD`):
- `dde6b1705` chore(2026-10-13-range-input-interaction): operator pre-CI commit, for the run this chunk verdict reads

That commit is the whole tree: this chunk's code and tests, its phase and implement run dirs, and the phase's master and route promotion. It was pushed by the operator pass.

## Changes (structured — detectors read this)

- **Files:** the 12 the chunk lists; basis: `gate.py scope` at this wrap: `clean — changed 12 · listed 12 · recorded 0`, base `97ce760d`.
  - New:
    - `packages/blitz-dom/src/node/range.rs` (522 lines);
    - `tests/blitz-tests/tests/range_input_interaction.rs` (298 lines).
  - Modified:
    - `packages/blitz-dom/src/node/mod.rs`
    - `packages/blitz-dom/src/node/element.rs`
    - `packages/blitz-dom/src/layout/construct.rs`
    - `packages/blitz-dom/src/mutator.rs`
    - `packages/blitz-dom/src/events/keyboard.rs`
    - `packages/blitz-dom/src/events/pointer.rs`
    - `packages/blitz-dom/src/form.rs`
    - `packages/dioxus-native-dom/src/snapshot.rs`
    - `tests/blitz-tests/tests/stand_act_range.rs` (rewritten)
    - `tests/blitz-tests/tests/stand_act_ids.rs`
  - Chunk folder:
    - `evidence/range-readings.md`
    - `evidence/operator-16.txt`
    - `evidence/ci.md` (uncommitted; written after the push)
    - `inputs/` I1–I5
  - Added-line coordinates are in `## New text, by line` below, from `cites.py added`.

- **Symbols / APIs:**
  - **blitz-dom, public:**
    - `blitz_dom::node::RangeInputData` (`range.rs:6-14`, its impl `16-48`): a range input's current value, an `f64`, held apart from its `value` attribute.
      - Public method: `value()`.
      - Crate-internal: `new`, `reseed`, `resanitize`, `apply`.
      - Exported from `node/mod.rs:27`. `blitz_dom::node` is a `pub mod`, and the crate root re-exports none of the range types.
    - A new variant `SpecialElementData::RangeInput(RangeInputData)` (`element.rs:368-369`). `SpecialElementData` is not `#[non_exhaustive]`, so this is a source break for an external exhaustive match; none exists in this workspace outside blitz-dom.
    - A new variant `SpecialElementType::RangeInput` (`element.rs:341`). That enum is `#[non_exhaustive]` and has no reader.
    - `Clone` and `Debug` arms for the new variant (`element.rs:390`, `:978`).
    - New public `ElementData` methods: `range_input_data()`, `range_input_data_mut()`, and `range_input_value() -> Option<String>`, which returns the formatted current value (`element.rs:622-644`).
  - **blitz-dom, crate-internal:**
    - `RangeRules` (`range.rs:65-73`, impl `83-204`): `min`, `max`, `step` (`None` for `any`) and step base, read from the four attributes.
    - `RangeMove` (`range.rs:50-63`): `StepUp`, `StepDown`, `Minimum`, `Maximum`, `Fraction(f64)`.
    - `parse_float` (`range.rs:230-294`): the HTML rules for parsing floating-point number values. A parse error, NaN or an infinity reads as absent; `-0` reads as `0`.
    - `ElementData::{range_rules, move_range_input, reseed_range_input, resanitize_range_input}` (`element.rs:646-684`).
    - `BaseDocument::apply_range_move` (`keyboard.rs:157-178`): where the value changed, it marks `changed_nodes`, dispatches `DomEventData::Input(BlitzInputEvent { value })` and requests a redraw. Both the keys and the pointerdown use it.
    - `flush_range_input` in `construct.rs:929-947`.
  - **The value rules** (`range.rs`), written against `https://html.spec.whatwg.org/multipage/input.html#range-state-(type=range)`, cited at the code:
    - `min` defaults to 0 and `max` to 100. A `max` below `min` reads as `min`.
    - `step` defaults to 1. A step ≤ 0 or unparsable reads 1. `any` sets no grid and steps by 1 on a key.
    - The step base is `min` where it parses, else the `value` attribute where it parses, else 0.
    - The default value is the midpoint, computed `min/2 + max/2` so it cannot overflow.
    - Sanitizing clamps to `[min, max]` and moves to the nearest grid value inside them, the greater on a tie. The grid index is computed `value/step − base/step`, so the difference of two finite bounds cannot overflow.
    - The value is then rounded to the decimals of the step and the base, and formatted in its shortest round-trip form.
    - No print, log site, `unwrap`, `expect`, panic or env read outside the tests (the census entry reads 0).
  - **Default actions, for every document with an `input type="range"`:**
    - **Keys** (`keyboard.rs:103-117`). The input must be focused, have no `disabled` attribute (presence) and receive a pressed key event:
      - `ArrowRight` and `ArrowUp` step up;
      - `ArrowLeft` and `ArrowDown` step down;
      - `Home` goes to the minimum and `End` to the maximum;
      - every other key does nothing.
      - The arms carry no `#[cfg]`.
    - **Pointerdown** (`pointer.rs:452-456`, `:463-474`, `:517-525`). A `ClickTarget::RangeInput { fraction }` arm sits after the `disabled` arm. On the main button the fraction is `(hit.x − (padding.left + border.left)) / content_box_width`, clamped to `[0, 1]`; a content box of no width reads 0. The value is set from the fraction.
    - **Click** (`pointer.rs:670-682`). A `handle_click` arm sits after the `disabled` check. It focuses the input through `generate_focus_events` and changes no value, so a label's synthetic click focuses without moving the value.
    - Every change fires one `input` event; a move that changes nothing fires none.
    - Without this chunk, no key or click moved a range input, and a click did not focus it (measured at 2026-10-07-act-by-id).
  - **Construction** (`construct.rs:436` → `929-947`):
    - An `input` whose `type` reads `range` (ASCII case-insensitive) is seeded with `RangeInputData` from its sanitized `value` attribute, only when it holds none, so a re-construction keeps a moved value.
    - An element holding the data that no longer reads `type="range"` is cleared to `None`.
    - Construction then continues as before, with no early `return`.
  - **Attribute writes** (`mutator.rs:352-358`, `:375-377`, `:499-506`):
    - In `set_attribute`, a `value` write re-seeds the current value from the sanitized new attribute, and a `min`, `max` or `step` write re-sanitizes it.
    - `clear_attribute` does the same on removal.
    - The removal walk gains a `RangeInput(_) => {}` arm (`mutator.rs:1118`).
    - This departs from the spec's dirty-value flag, which ignores attribute writes once the user has moved the value. It is the re-seed the founder's Q1 answer took on as a cost (inputs#I3), so that a Dioxus re-render keeps control.
  - **Form submission** (`form.rs:318-319`): `construct_entry_list` submits a range input's formatted current value, before the `value` attribute fallback.
  - **dioxus-native-dom** (`snapshot.rs:55-56`, `:198-200`, `:222-224`):
    - `NodeState.value` for a range input now reads `range_input_value()`, its current value. An element without the data still reads its `value` attribute.
    - The field doc and the `value` reader's doc are restated to say so.
    - The bridge's converter (`dioxus_document.rs:377-380`) is unchanged and carries the event's value to `oninput`.
  - **Unchanged:**
    - no driver verb, key name, cause, help text or span field;
    - no accessibility node value or action (`accessibility.rs` unchanged);
    - no paint or UA-sheet change;
    - no stand markup change.
    - Basis: the preservation probe (entry `git diff --quiet 97ce760d… -- packages/escher-driver … deny.toml Cargo.toml`) reads exit 0.
  - **Remaining callers:** `handle_click`, `handle_pointerdown` and `handle_key_or_input_event` keep their signatures and their callers (`events/mod.rs:192, 215, 218, 232`, research.md Graph impact). `apply_range_move` has two callers, `keyboard.rs` and `pointer.rs`.

- **Crates / modules:**
  - blitz-dom gains the module `node/range` (private `mod range;` at `node/mod.rs:8`).
  - No crate is added or removed.

- **Dependencies:** none. The `Cargo.lock` census entry reads 0 new `[[package]]` blocks, exit 1.

- **Schema / config:** none. No config key, schema, scrub set or env var.

- **Spec-master edits:** none. No master was edited by this chunk.

- **Counts / qualifiers moved:** standing master counts this chunk moved. Basis: the measured totals in `evidence/range-readings.md` (§Totals the gates read). Sites: the site sweep, pattern `counts`.
  - **Workspace tests:** now 181 result lines · 915 passed · 0 failed · 12 ignored. `test-plan.md:330` reads 180 · 900 · 0 · 12 (pattern `counts` ×2 at that line; standing 1 line).
  - **blitz-tests:** now 120 `.rs` files and 119 targets; it was 119 files and 118 targets. The `counts` pattern holds `119 files|118 targets` and found no master row with that wording, so the master site is whatever wording test-plan uses. The detectors' own sweep controls reach.
  - **`run stand`:** passed 141 · failed 0 · ignored 7 over 37 files — unchanged. `stand_act_range` keeps one test.
  - **The 15 added workspace tests** are 6 in `range_input_interaction`, 7 in `range.rs`'s unit tables, 1 form test and 1 added snapshot test. The renamed snapshot test counts once.
  - **`stand_act_ids`' pinned count** of stand controls that change the screen at boot moves from 8 to 9, the Timer slider joining. It is the file's own assertion.
  - **The silent-check census:** obs-plan's census of checks that install no subscriber (pattern `silent`: `obs-plan.md:70` ×7, `:69` ×2, `:106`) names no `range_input_interaction`. The new check installs no subscriber, reads no env var and prints nothing (the silence census entry reads 0).

- **Dev-tool versions:** none.

- **Harness / gate surface:** none. No agent-run, CI step or harness API change. `range_input_interaction` drives `blitz_test_harness::Harness` (`from_html_with`, `dispatch_recorded`, `click_at`-shaped pointer dispatch, `layout_rect_of`, `base_mut().set_focus_to`) as it stands.

- **Cross-project / external claims:**
  - **CI on the pushed sha** (`evidence/ci.md`):
    - sha `dde6b1705ac576d06118352e99cda282d9c4614f`;
    - run CI#38466886504, push, completed/success;
    - `verdict: green`, checks 16/16, wall 797 s.
    - The checks include `MSRV Build [Rust 1.91]`, `Test (macos)`, `Test (windows)`, `Test (ios)` and `Test (android)`, the legs not run locally.
    - The wrap's own commit will add to that tree.
  - **Inputs** (`inputs.py verify` at this wrap, before this report existed; summary `inputs: 5 entries — unchanged 1 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 4 · uncited 2 · unparsed 0`). No DRIFT.
    - I1 · message (the operator, in the /andromeda-phase invocation, 2026-10-13T19:01Z) · copy message · n/a — cited: no mutation runs; pipeline tools bare or redirected.
    - I2 · message (the overseer relaying the founder, the P4 dialog answer, 2026-10-13T20:07Z) · copy message · n/a — cited: Q1 «Отдельное текущее значение», Q2 «Стрелки + Home/End (Рекомендую)», Q3 «Клик даёт фокус (Рекомендую)».
    - I3 · `../additional/escher-overseer/relays/range-input-founder-answers.md` · copy no-repo · unchanged — cited: the overseer's record of the founder's answers, with provenance and Q1's costs.
    - I4 · message (the operator, at the P5 review, 2026-10-13T20:19Z) · copy message · n/a — `verify` printed it UNCITED. This report cites it as inputs#I4, with its card below.
    - I5 · message (the operator, in the implement window, 2026-10-13, citing overseer1, F279(b)) · copy message · n/a — snapped at THIS wrap, cited here as inputs#I5. It is the operator-pass directive:
      - fire entry 16 as written, not through `gate.py run --operator`;
      - fix a tmp row by rewrite, not respell;
      - the pre-CI commit message;
      - fire 17 and 18;
      - write `evidence/ci.md`.
  - **inputs#I4 — the operator's word on the P5 review card**, `.andromeda/runs/2026-10-13T19-01-34-phase/p5-card-1.md` (the one card named; none superseded). Authority: the operator.
    - The word: «yes — apply … All three judgement calls are confirmed as the operator, as consequences of the founder's own answers, not new forks … The Beyond-the-plan CARRY to working-route.md:98 is approved.»
    - The card's `## Beyond the plan` rows, verbatim:

      | # | Proposal | Owner / place |
      |---|---|---|
      | 1 | The merge surface of this engine change for every document (playbook :66 has it "owned by a CARRY on the upstream-sync entry"): `node/element.rs`, `node/mod.rs`, `layout/construct.rs`, `mutator.rs`, `events/keyboard.rs`, `events/pointer.rs`, `form.rs`, against upstream `main` at `434e0661`. No markerless upstream-sync entry remains, because the last one, 2026-10-13-upstream-sync-polish-and-ship, is complete. | a CARRY on "Quality gates" (working-route.md:98), the release-readiness entry ahead of ship — written by this chunk's wrap |

    - The row's owner, approved: a CARRY on "Quality gates", written by this wrap at P5. The merge-surface file list is as built: the seven upstream files the row names, plus our new `node/range.rs`.

- **Reverted / negative API facts:** none.

- **Insufficient fixes (written, kept, not the remedy):** none.

- **Spec claims disproved by measurement:** standing master text this chunk's measured behaviour makes false. Sites are from the site sweep, its `range` pattern (standing 6) and `valueattr` pattern (standing 5), each row read.
  - `architecture.md:263` — the range input's entry in the stand checks' list is "the range input's measured reading". The no-op it named is gone. `stand_act_range` now asserts that the driver moves the slider by click and key (sequences A and B, `evidence/range-readings.md`).
  - `test-plan.md:24` — the dioxus-native-dom snapshot unit tests include "a range input reading its `value` attribute". The test is renamed `a_range_input_reads_its_current_value`, and a new test proves an uncontrolled range input reads its moved value while its attribute keeps the boot value (`snapshot.rs:607-651`).
  - `architecture.md:136` (×3) and `a11y-plan.md:276` (×2) state the snapshot's `value` as the editor's text, else the `value` attribute. A range input now reads its current value. This is the Dioxus DOM bridge's snapshot clause the plan expects amended.
  - Rows read and left standing, true as they read:
    - `test-plan.md:26` ("the range input reading the app's duration") — still true at boot.
    - `a11y-plan.md:224` (`timer-duration` no-text in the contrast record) — still true; nothing is painted differently.
    - `a11y-plan.md:107` (range→Slider) — still true.
    - `architecture.md:136`'s role list naming Slider — still true.
    - `security-plan.md:126` ×4 (the password and file mask's `value` attribute) — about other inputs, still true.
  - **The plan's own implementation note** (`plan.md:485-489`) bases its hypothesis "a centre click on the Timer slider maps to 15" on the default sheet's 2px padding. Measured on the stand at 800 × 600, in both modes, from `final_layout()`: border 1px, padding 0, content box 107 × 0. The conclusion holds by the symmetric border alone. This is a plan claim, not a master one, recorded in `evidence/range-readings.md` §The Timer slider's box.
  - **Leaves the site sweep listed in the same two patterns** (the cascade's to re-derive):
    - `CLAUDE.md:110` ("a range input, which no verb can move, is measured and owned by a route entry");
    - `.claude/docs/services/escher-driver.md:55` ("A click on a range input is accepted and does not focus it"), `:61`;
    - `.claude/docs/tests-summary.md:24`;
    - `.claude/docs/services/dioxus-native-dom.md:17`.

- **Expected amendments (from plan):** sites per the P1 site sweep (`{run_dir}/report-patterns.toml`, `cascade.py sweep --pre-pass`, `site-sweep.txt` in this run dir; every master and registry file swept).
  - **security-plan §Input Validation — a range-input row.** Carried: Symbols / APIs, *Default actions* and *The value rules*; Coverage below.
    - Search: `range` 0 hits in security-plan; `defaction` `security-plan.md:119` ×1 (the selection and click default actions); `engchange` `security-plan.md:126`, `:212`.
    - The row states: the six keys and the click as default actions for every document, the browser app's remote pages included; the four attributes read, with their defaults, bounds and sanitization; no log site, no panic, nothing carried out but the control value inside the ratified carriers.
    - The founder's ruling «Пусть будет а» (2026-10-07) and his three fork answers «Отдельное текущее значение», «Стрелки + Home/End (Рекомендую)», «Клик даёт фокус (Рекомендую)» (2026-10-13), each relayed verbatim by the overseer (inputs#I2, inputs#I3).
    - Playbook pattern: Boundary widening (`playbook.md:36`, escalate). It is the founder's own word, so the wrap halts on it once, as the answer card.
  - **architecture §Established Decisions → [Scrolling and input] — the direct engine change for every document.** Carried: Symbols / APIs, *Default actions*.
    - Search: `engchange` `architecture.md:87` (counts "a third direct engine change … beside those two"), `:95` (the password paint, "a direct engine change for every document beside those under [Scrolling and input]").
    - The plan names this change "the fifth". The ordinal is the P2 detectors' to re-derive from those two rows.
    - Ratified by the founder (2026-10-07; the fork answers 2026-10-13). Upstreamable against upstream `main` at `434e0661`, with no upstream pull request open. Its merge-surface files are as in inputs#I4's row. The measured report is this file. Playbook pattern: `playbook.md:64`, routine on the founder's own word.
  - **architecture §Standard Contracts.** Carried: Symbols / APIs.
    - Nodes and widgets: `SpecialElementData::RangeInput`, `RangeInputData`, the accessors. Search: `special` `architecture.md:126` ×2.
    - Events: the range default actions. Search: `defaction` `architecture.md:134`, `:196`.
    - Dioxus DOM bridge: the snapshot's `value` clause. Search: `valueattr` `architecture.md:136` ×3.
    - §Existing Scopes: blitz-dom's new module. Search: `scopes` `architecture.md:228`.
  - **test-plan §1, §2, §3, §9 — the new target, the rewritten check, the unit tables, the counts as measured.** Carried: Counts / qualifiers moved; Spec claims disproved. Search: `counts` `test-plan.md:330` ×2; `range` `test-plan.md:24`, `:26` ×2; `valueattr` `test-plan.md:24`.
  - **obs-plan §3 — `range_input_interaction` joins the census of silent checks by name.** Carried: Counts / qualifiers moved. Search: `silent` `obs-plan.md:69` ×2, `:70` ×7, `:106` ×1. The census lives at `:70`.
  - **a11y-plan §5 Keyboard handling and §7.** §5: a range input steps by the arrows and Home/End, and a click focuses it. §7: the snapshot reads a range input's current value. Carried: Symbols / APIs; Spec claims disproved. Search: `keys` `a11y-plan.md:162`, `:163` ×2 (`:163` reads "Text inputs handle arrow keys, Home/End, …"); `valueattr` `a11y-plan.md:276` ×2.
  - **Not in the plan's list, carried for P5:** inputs#I4's approved Beyond-the-plan row — the merge-surface CARRY on "Quality gates" (working-route.md:98).

- **Coverage of new surfaces:**
  - `range-input default actions (keys · pointerdown · click), every document` → validation ✓ (every markup number through `parse_float` to a finite value in `[min, max]` on the grid; `disabled` by presence; no panic path, census 0) · instrumentation n/a (no span, event or field added; `stand_act_spans` passes) · PII n/a (no log site; the value crosses only as the snapshot's control value and the `input` event, inside the ratified carriers) · tests unit (`range.rs` tables) + integ (`range_input_interaction`, both modes; `stand_act_range` and `stand_act_ids` through the driver) · a11y ✓ (keyboard-operable by the arrows and Home/End; a click focuses; focus order unchanged — `stand_accessibility_ids` and `focusability_updates` pass; role `Slider` and name unchanged) · tokens n/a (nothing painted differently; no colour literal)
  - `RangeInputData` / `SpecialElementData::RangeInput` (public type) → validation ✓ (sanitized on seed, write and move) · instrumentation n/a · PII n/a · tests unit + integ · a11y n/a · tokens n/a
  - `snapshot value of a range input` → validation n/a · instrumentation n/a · PII n/a (a range value, not masked by design) · tests unit (`a_stepped_uncontrolled_range_input_reads_its_moved_value`) + integ (`stand_act_range` diffs) · a11y ✓ (the driver's diff names `timer-duration` with `differs` `value`) · tokens n/a
  - `form entry of a range input` → validation ✓ · instrumentation n/a · PII n/a · tests unit (`form::tests::a_range_input_submits_its_current_value`) · a11y n/a · tokens n/a

## Deviations from intent
Each is from the implement run's outcome (`.andromeda/runs/2026-10-13T20-31-06-implement/outcome-2026-10-13-range-input-interaction.md`), re-read here against the tree.

1. **`RangeRules` and `RangeMove` stay crate-internal, re-exported `pub(crate)` from `node/mod.rs:28`.** The plan exports `RangeInputData` alone, so the public surface stays the plan's: `RangeInputData` and the `ElementData` accessors.
2. **Crate-internal helpers beyond the plan's named accessors.** `ElementData::{range_rules, move_range_input, reseed_range_input, resanitize_range_input}` and `BaseDocument::apply_range_move` (`keyboard.rs:157-178`) give the keys and the pointerdown one write · mark · `Input` · redraw path, which steps 5 and 6 require.
3. **The seed lives in its own function, `flush_range_input`, called before the input match** (`construct.rs:436`). Inside the match, the borrow of the tag name stays live on the path that does not return. No early `return`, which the plan binds.
4. **The step base follows the spec where `min` is absent:** it is then the `value` attribute.
   - The unit table reads `value="3.4" step="0"` (no `min`) as `3.4`.
   - The degenerate row `value="1e308"` with no `min` has its base at 1e308, and still reads finite, inside `[0, 100]` and on its grid.
5. **The grid index is computed `value/step − base/step`**, so that `min=-1e308 max=1e308` does not overflow. The degenerate unit table holds that row.
6. **`stand_act_ids`' pinned count of stand controls that change the screen at boot moves 8 → 9.** It sits in the listed file, beside the `changes_at_boot` row step 11 names (`stand_act_ids.rs:230-231`).
7. **`stand_act_range` asserts part of each diff's `differs`.** After a value change it asserts that the slider's contains `value` and the display's contains `name`. At steps A7 and A9 the display's text narrows (`0.0s`, 40 px against 42 px) and the slider has `flex: 1`, so both nodes also read `bounds` (measured through the driver; `evidence/range-readings.md` §Sequence A). After step A1 it asserts `differs == [focused]` exactly, as the plan says.
8. **`range_input_interaction`'s boot condition asserts the range is not focused, not that nothing is focused.** A fresh `HtmlDocument` focuses a node at boot (measured red on the first run).
9. **The plan's hypothesis basis did not hold.** Padding is 0, not 2px; the conclusion held. See Spec claims disproved.

Scope record: none — `gate.py scope` clean, 0 recorded (`clean — changed 12 · listed 12 · recorded 0 (companion 0 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 59`).

## Decisions & corrections
- **The operator-pass directive (inputs#I5; the operator, citing overseer1, F279(b)).**
  - Entry 16 (hygiene) is fired as written, on its own command line, never through `gate.py run --operator`, which drops hygiene's head and control lines from the record.
  - A tmp row needing a hand fix is recorded as a rewrite, not a respell.
  - Kept: `evidence/operator-16.txt` holds the head, control and summary lines; the entry was clean on the first firing.
- **The operator at the P5 review (inputs#I4)** confirmed three leans as the operator's, as consequences of the founder's answers, not new forks: submission sends the current value; a value write resets it; pointerdown moves the value and the click focuses. He approved the merge-surface CARRY on "Quality gates" (working-route.md:98).
- **The founder's three fork answers (inputs#I2, inputs#I3),** relayed verbatim by the overseer as his chosen option labels: Q1 «Отдельное текущее значение» (not the plan's recommendation; its costs planned), Q2 «Стрелки + Home/End (Рекомендую)», Q3 «Клик даёт фокус (Рекомендую)».
- **The standing directive (inputs#I1):** no mutation runs; pipeline tools run bare or redirected to a file read whole.
- **Sweep hazard:** obs-plan words its silent-check census as "installs no subscriber" (`obs-plan.md:70`), never "silent census". A site sweep on the plan's word "silent census" refuses as an unfired control (this wrap's first sweep, exit 3); the master's own wording fires.
- **Sweep hazard:** a `.expect(…)` in a new inline test module reddens the added-line census entry (`git diff … | grep -c '…\.expect\('`). The census reads every added line, tests included. The plan's own note is that inline test modules use assertions; the fix is `let … else { panic!(…) }` (implement, gate entry 9, first firing red, one fix).
- **Lint:** clippy `-D warnings` flags `manual_clamp` on `x.max(0.0).min(1.0)` even where the clamp form would panic only on NaN bounds. With constant bounds `clamp` is safe; the variable-bound `value.max(min).min(max)` was not flagged (implement, entry 14, first firing red).
- **Host:** `escher-session` refuses a state directory whose socket path exceeds 94 bytes (`state-dir-too-long`, exit 3). The session scratchpad's path is over it, so a manual driver run uses a relative `target/tmp/…` directory, as the session checks do (implement, the evidence drive, one re-run).
- **Measurement method:** with no listed producer for the slider's content-box width, implement read it with a throwaway probe test in `tests/blitz-tests/tests/`, run once and deleted before the scope read. Recorded as a workaround in the implement step's evolve record.
- **Guard refusals at implement:** bash-guard refused a `cd` that would move the session cwd and a `cat` heredoc with a file target. Each was re-issued once in the passing form (absolute paths; Edit tool).
- **Standing-learning recurrence check:** host-linux.md 2026-10-13 (pipeline tools bare or redirected) — none; every `gate.py`, `matrix.py`, `evolve.py`, `inputs.py`, `cites.py` and `cascade.py` call ran bare or redirected and was read whole.
- **This wrap's invocation:** "run P1 only and stop" — the two-window form. P2–P7 resume in this run dir at Setup 2a.

## Outcome
**Acceptance criteria, re-asserted against the diff:**
- (arch) Keys, click and `input` event in both modes — **met**. `range_input_interaction`: the 7-row keys table and the 5-row clicks table, one `input` per change, none on a no-change Home; the input focused after every click.
- (arch) The current value held apart from the attribute; a `value` write re-seeds and `min`/`max`/`step` re-sanitizes — **met**. Attribute-write rows: `7` → `8`, then `max=5` → `4`. The snapshot unit test: attribute `15`, value `16`.
- (arch) Driver schema unchanged — **met**. The preservation probe reads exit 0 over `packages/escher-driver`.
- (arch) No crate, listener, port, env var or package added — **met**. The lock census reads 0; the preservation probe covers the manifests.
- (security) Degenerate rows: no panic, finite, bounded, on the grid, both modes — **met**. The 7 integration rows and the 8-row unit table.
- (security) No print, panic, log or env read in the rules module outside its tests; no added engine line printing, logging, reading env or adding `unwrap`/`expect` — **met**. Both censuses read 0 on the final firing.
- (security) A disabled range takes neither key nor click — **met** (the disabled rows). The driver still refuses its click `disabled` first; that executor is unchanged.
- (tests) `stand_act_range` rewritten, prefix kept, sequences A and B as stated, diffs equal to `before.diff(&after)` and across modes — **met**.
- (tests) `stand_act_ids` with the Timer row `timer-duration`; the fifteen standing targets green — **met**.
- (tests) Both builds; `run stand` failed 0; `fast` and `doc` green; totals recorded — **met**. `run stand` 141 · 0 · 7 over 37 files; workspace 181 · 915 · 0 · 12.
- (a11y) After two Tabs, the arrow step leaves the slider focused and the tree's focus, role `Slider` and its name unchanged; forward focus sequence unchanged — **met**. `stand_act_range` sequence B; `stand_accessibility_ids` and `focusability_updates` pass.
- (a11y) A stepped or clicked value is named by the diff on `timer-duration` (`value`) and on the display (`name`) — **met**.
- (a11y) `stand_contrast` 18 rows unchanged, `timer-duration` no-text — **met** (the standing entry green).
- (design) No colour literal; paint and UA sheet unchanged; a click settles in one step, no animation — **met**. The preservation probe covers `packages/blitz-paint` and `packages/blitz-dom/assets`.
- (layouts) Timer markup diff empty; the mount reads as fresh after the range calls — **met**. The preservation probe covers `examples`; `stand_session_fresh` is green.
- (obs) No span, event or field added; `stand_act_spans` green; the two checks silent — **met**.
- (ci) The operator's CI read on the pushed sha reads `verdict: green` — **met**. `dde6b1705ac5`, CI#38466886504, 16/16.

**Gates.** The last implement firing (`.andromeda/runs/2026-10-13T20-31-06-implement/gate-run-2.txt`, the tree at `dde6b170`'s content) reads `entries 18 · green 15 · red 0 · not-run 3`. Each entry by its `run`:
- `cargo test -p blitz-dom --locked` — green (exit 0 · `lacks FAILED`).
- `cargo test -p dioxus-native-dom --locked` — green.
- `cargo test -p blitz-tests --locked --test range_input_interaction` — green (6 passed).
- `cargo test -p blitz-tests --locked --test stand_act_range --test stand_act_ids` — green.
- `cargo test --workspace --locked --test range_input_interaction --test stand_act_range --test stand_act_ids` — green.
- `cargo test -p blitz-tests --locked --test stand_snapshot_state … --test checked_attribute` (15 standing targets) — green.
- `git diff 97ce760d… -- Cargo.lock | grep -c '^+\[\[package\]\]'` — green (exit 1 · last line 0).
- `sed '/^#\[cfg(test)\]/,$d' …/range.rs | grep -c -E '…'` — green (exit 1 · 0).
- `git diff 97ce760d… -- packages/blitz-dom packages/dioxus-native-dom | grep -c -E '…'` — green on its re-run (exit 1 · 0). Red on the first firing (count 1, a test-module `.expect`), fixed in this chunk.
- `cat …/range_input_interaction.rs …/stand_act_range.rs | grep -c -E '…'` — green (exit 1 · 0).
- `git diff --quiet 97ce760d… -- packages/escher-driver … Cargo.toml` — green (exit 0).
- `bash scripts/agent-run.sh boot` — green.
- `bash scripts/agent-run.sh run stand` — green (run.end 141 · 0 · 7, outcome passed).
- `bash .github/scripts/ci-leg.sh fast` — green on its re-run. Red on the first firing (clippy `manual_clamp`), fixed in this chunk. The test log: 181 result lines · 915 passed · 0 failed · 12 ignored.
- `bash .github/scripts/ci-leg.sh doc` — green.
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` (leg operator) — fired as written by the operator pass (inputs#I5): exit 0, `hygiene: clean` on the first firing (`evidence/operator-16.txt`).
- `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin build/escher-0.1.0` (leg operator) — green, 120.89 s, `history moved: refs/remotes/origin/build/escher-0.1.0 97ce760d→dde6b170` (the push it exists to make). CI-scripts tail `Ran 79 tests` · `OK` (`evidence/ci.md`).
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800` (leg operator) — green: `dde6b1705ac5 verdict: green · checks 16/16 · wall 797 s · runs CI#38466886504 completed/success` (`evidence/ci.md`).

No `defer`, no deferral; the Rust delta is non-zero and every Rust gate ran.

**Smoke:** skipped. No binary or boot path changed, and the plan lists no smoke or self-verify entry. The evidence drive booted the real `escher-session` host on the Timer: `start`, nine calls and `stop`, every call exit 0.

**Watches:** none folded.

**Outcome basis:**
- The operator pass ran: the pre-CI commit `dde6b170`, parent `97ce760d`, its only commit (`git log 97ce760d..HEAD`). The final HEAD's CI run is CI#38466886504, recorded in `evidence/ci.md`.
- Implement's report, held in this window and persisted at `.andromeda/runs/2026-10-13T20-31-06-implement/outcome-2026-10-13-range-input-interaction.md`, is the basis for the deviations and the gate firings. It is the only `*-implement` run dir of this chunk.
- The operator directive between implement and this report, inputs#I5, changed how entry 16 was fired.
- Post-implement artifacts: `evidence/operator-16.txt` and `evidence/ci.md`.

**Process hygiene** (implement's P4 census plus the operator pass; re-measured at this wrap: `ps -eo pid,comm,args` shows no `escher-session`, `seven_guis`, `cargo`, `rustc`, `gate.py` or `ci.py` process):

| process | started by | final state |
|---|---|---|
| cargo, rustc and test binaries of the implement gate firings and ad-hoc test runs | this chunk's implement run | terminated |
| `agent-run.sh boot` · `run stand` | implement, through the gate tool | terminated |
| `escher-session` host ×2 (evidence drives, state dirs `target/tmp/range-sess`, `range-sess2`) | implement | terminated by `stop`; state dirs removed |
| `ci-leg.sh fast` + push (entry 17); `ci.py conclusion` (entry 18) | the operator pass, through the gate tool | terminated |
| `cargo metadata` (one, seen once in the pass's census, exited before its parent could be read) | not attributable — not started by this chunk's runs, by its arguments (an editor's project scan) | exited |
| `scripts/code-graph.py refresh` | this wrap's Setup | terminated (`.refresh-done` present) |
## Addendum — resumed 2026-10-13
- inputs#I6 · message: the operator relaying the overseer, at this wrap resume, 2026-10-13T21:27Z · copy message · n/a — the founder's answer to this wrap's answer card (`answer-card.md`, security-plan 1–2, architecture 1–3, a11y-plan 1–2): «Подтвердить (Рекомендую)» = 1, Confirm. Relayed verbatim by the overseer as his chosen option label; the English wording is the overseer's. He was told it also covers the three P5 consequences. The directive with it: resume at P2 Escalate then Apply, wrap to the end — commit, push, read CI, report, stop; no next skill.
  - bears on **Expected amendments (from plan)**: adds to it — the security-plan range-input row and the architecture [Scrolling and input] engine change land as ratified by the founder's own word, his card answer of 2026-10-13
  - bears on **Decisions & corrections**: adds to it — the founder's confirmation also covers the three consequences the operator confirmed at the P5 review (inputs#I4)
- inputs#I7 · ../additional/escher-overseer/relays/range-input-founder-answers.md · copy no-repo · unchanged — the overseer's record of the same answer, section "The wrap's answer card": put in Russian with both options whole, the overseer's clock 2026-10-13 21:27:32Z right after, no words of his own.
  - bears on **Expected amendments (from plan)**: adds to it — the record of the card answer
- I3 · unchanged → drifted — DRIFT: the overseer appended the section "The wrap's answer card" to the record; the three fork answers I3 carries stand as copied (inputs#I7 holds the whole file now)
- inputs.py verify, re-fired: inputs: 7 entries — unchanged 1 · drifted 1 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 5 · uncited 2 · unparsed 0; moved since the report: I3 unchanged → drifted; uncited 2 → I6 and I7, cited by this addendum
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 97ce760d (the parent of the oldest pre-CI commit dde6b170) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### packages/blitz-dom/src/events/keyboard.rs — added 43 line(s) in 3 range(s)
added: 2-6 · 103-117 · 156-178
- 2-6 «use crate::{»
  - 157-178 @159 «pub(crate) fn apply_range_move(»
### packages/blitz-dom/src/events/pointer.rs — added 40 line(s) in 5 range(s)
added: 21 · 452-456 · 463-474 · 517-525 · 670-682
### packages/blitz-dom/src/form.rs — added 63 line(s) in 2 range(s)
added: 318-319 · 466-526
- 467-526 @468 «mod tests {»
  - 472-477 «fn attribute(name: &str, value: &str) -> Attribute {»
  - 479-525 @480 «fn a_range_input_submits_its_current_value() {»
### packages/blitz-dom/src/layout/construct.rs — added 23 line(s) in 3 range(s)
added: 27-28 · 436 · 929-948
- 929-947 @932 «fn flush_range_input(doc: &mut BaseDocument, node_id: NodeId) {»
  - 933-935 «let Some(element) = doc.nodes[node_id].data.downcast_element_mut() else {»
  - 936-939 «let is_range = element.name.local == local_name!("input")»
  - 941-946 «if is_range && !holds_range {»
### packages/blitz-dom/src/mutator.rs — added 19 line(s) in 4 range(s)
added: 352-358 · 375-377 · 499-506 · 1118
### packages/blitz-dom/src/node/element.rs — added 70 line(s) in 6 range(s)
added: 34 · 341 · 368-369 · 390 · 622-685 · 978
  - 622-629 @624 «pub fn range_input_data(&self) -> Option<&RangeInputData> {»
  - 631-637 @632 «pub fn range_input_data_mut(&mut self) -> Option<&mut RangeInputData> {»
  - 639-644 @641 «pub fn range_input_value(&self) -> Option<String> {»
  - 646-654 @647 «pub(crate) fn range_rules(&self) -> RangeRules {»
  - 656-665 @658 «pub(crate) fn move_range_input(&mut self, to: RangeMove) -> Option<String> {»
  - 667-675 @669 «pub(crate) fn reseed_range_input(&mut self) {»
  - 677-684 @679 «pub(crate) fn resanitize_range_input(&mut self) {»
### packages/blitz-dom/src/node/mod.rs — added 3 line(s) in 2 range(s)
added: 8 · 27-28
### packages/blitz-dom/src/node/range.rs — new file · 522 line(s)
- 6-14 @12 «pub struct RangeInputData {»
- 16-48 «impl RangeInputData {»
  - 17-23 @19 «pub(crate) fn new(rules: &RangeRules, value_attribute: Option<&str>) -> Self {»
  - 25-28 @26 «pub fn value(&self) -> f64 {»
  - 30-34 @32 «pub(crate) fn reseed(&mut self, rules: &RangeRules, value_attribute: Option<&str>) {»
  - 36-39 @37 «pub(crate) fn resanitize(&mut self, rules: &RangeRules) {»
  - 41-47 @42 «pub(crate) fn apply(&mut self, rules: &RangeRules, to: RangeMove) -> bool {»
- 50-63 @52 «pub(crate) enum RangeMove {»
- 65-73 @67 «pub(crate) struct RangeRules {»
- 83-204 «impl RangeRules {»
  - 84-122 @85 «pub(crate) fn new(»
  - 124-127 @125 «fn default_value(&self) -> f64 {»
  - 129-135 @130 «fn sanitize_attribute(&self, value: Option<&str>) -> f64 {»
  - 137-159 @142 «fn sanitize(&self, value: f64) -> f64 {»
  - 161-165 @163 «fn grid_index(&self, value: f64, step: f64) -> f64 {»
  - 167-169 «fn clamp(&self, value: f64) -> f64 {»
  - 171-190 @172 «fn moved(&self, value: f64, to: RangeMove) -> f64 {»
  - 192-197 @194 «fn decimals(&self) -> Option<usize> {»
  - 199-203 @200 «pub(crate) fn format(&self, value: f64) -> String {»
- 206-211 @207 «fn decimals_of(value: f64) -> usize {»
  - 209-210 «text.split_once('.')»
- 213-224 @215 «fn round_to(value: f64, decimals: Option<usize>) -> f64 {»
  - 216-218 «let Some(decimals) = decimals else {»
  - 219-223 «format!("{value:.decimals$}")»
- 226-228 «fn is_ascii_whitespace(c: char) -> bool {»
- 230-294 @234 «fn parse_float(input: &str) -> Option<f64> {»
  - 235-238 «let mut chars = input»
  - 241-250 «match chars.peek() {»
  - 253-255 «while let Some(digit) = chars.next_if(char::is_ascii_digit) {»
  - 258-262 «if chars.next_if_eq(&'.').is_some() {»
  - 263-266 @264 «if integer.is_empty() && fraction.is_empty() {»
  - 268-271 «if !fraction.is_empty() {»
  - 273-287 «if chars.next_if(|c| matches!(c, 'e' | 'E')).is_some() {»
  - 289-292 «let value = number»
- 296-522 @297 «mod tests {»
  - 300-309 @301 «fn on_grid(rules: &RangeRules, value: f64) -> bool {»
  - 311-316 «type Attributes = (»
  - 318-321 «fn rules(attributes: Attributes) -> RangeRules {»
  - 323-345 @324 «const PARSED: [(&str, Option<f64>); 20] = [»
  - 347-357 @348 «fn the_float_parser_reads_the_html_grammar() {»
  - 359-387 @362 «const RULES: [(Attributes, (f64, f64, Option<f64>, f64)); 9] = [»
  - 389-405 @390 «fn attributes_read_as_bounds_step_and_base() {»
  - 407-421 @408 «const SEEDED: [(Attributes, &str); 12] = [»
  - 423-435 @424 «fn a_value_attribute_seeds_finite_bounded_and_on_the_grid() {»
  - 439-451 @441 «const MOVES: [(RangeMove, &str, bool); 9] = [»
  - 453-462 @454 «fn moves_step_jump_and_place_on_the_grid() {»
  - 464-475 @466 «const DEGENERATE: [Attributes; 8] = [»
  - 477-500 @478 «fn degenerate_attributes_keep_the_value_finite_bounded_and_on_the_grid() {»
  - 502-511 @503 «fn a_decimal_step_writes_its_decimals() {»
  - 513-521 @514 «fn a_reseed_and_a_resanitize_follow_the_attributes() {»
### packages/dioxus-native-dom/src/snapshot.rs — added 54 line(s) in 5 range(s)
added: 55-56 · 198-200 · 222-224 · 594 · 607-651
### tests/blitz-tests/tests/range_input_interaction.rs — new file · 298 line(s)
- 18-23 «fn page(attributes: &str) -> String {»
  - 19-22 «format!(»
- 25-45 @26 «fn open(attributes: &str, incremental: bool) -> (Harness<HtmlDocument>, NodeId) {»
  - 27-33 «let harness = Harness::from_html_with(»
  - 35-43 «assert!(»
- 47-53 «fn value(harness: &Harness<HtmlDocument>, range: NodeId) -> Option<String> {»
  - 48-52 «harness»
- 55-57 «fn focus(harness: &mut Harness<HtmlDocument>, range: NodeId) {»
- 59-71 @60 «fn press(harness: &mut Harness<HtmlDocument>, key: Key) -> usize {»
  - 61-68 «let events = harness.dispatch_recorded([»
- 73-81 @74 «fn click(harness: &mut Harness<HtmlDocument>, x: f32, y: f32) -> usize {»
  - 75-78 «let events = harness.dispatch_recorded([»
- 83-85 «fn inputs(events: &[String]) -> usize {»
- 87-99 @89 «fn along(harness: &Harness<HtmlDocument>, range: NodeId, fraction: f32) -> (f32, f32) {»
  - 95-98 «(»
- 101-111 @103 «const KEYS: [(Key, &str, usize); 7] = [»
- 113-135 @114 «fn the_arrows_home_and_end_step_a_focused_range_input() {»
  - 116-134 «for incremental in [false, true] {»
- 137-145 @139 «const CLICKS: [(Option<f32>, &str); 5] = [»
- 147-175 @148 «fn a_click_sets_the_value_by_position_and_focuses() {»
  - 150-174 «for incremental in [false, true] {»
- 177-188 @180 «const DEGENERATE: [(&str, (f64, f64, Option<f64>, f64)); 7] = [»
- 190-218 @191 «fn degenerate_attributes_keep_the_value_finite_bounded_and_on_the_grid() {»
  - 193-217 «for incremental in [false, true] {»
- 220-234 @221 «fn a_decimal_step_reads_its_decimals() {»
  - 222-233 «for incremental in [false, true] {»
- 236-258 @237 «fn a_disabled_range_input_takes_neither_keys_nor_a_click() {»
  - 238-257 «for incremental in [false, true] {»
- 260-262 «fn attribute(name: &str) -> QualName {»
- 264-298 @265 «fn attribute_writes_reseed_and_resanitize_the_value() {»
  - 266-297 «for incremental in [false, true] {»
### tests/blitz-tests/tests/stand_act_ids.rs — added 7 line(s) in 4 range(s)
added: 11-13 · 36 · 41 · 230-231
### tests/blitz-tests/tests/stand_act_range.rs — added 124 line(s) in 20 range(s)
added: 1-10 · 13 · 20-21 · 23-28 · 34-35 · 43-74 · 76-79 · 83 · 89-93 · 97-98 · 100 · 102-117 · 121-122 · 131 · 138
       141-143 · 147-170 · 173-174 · 176-180 · 182-185
- 23-28 @25 «enum Changes {»
- 43-48 @44 «fn slider_label(session: &Session) -> Option<(Role, String)> {»
- 50-66 @53 «fn names(diff: &SnapshotDiff, changes: Changes) -> bool {»
  - 54-56 «if !diff.added.is_empty() || !diff.removed.is_empty() {»
  - 58-65 «match changes {»
  - 107-117 «[»
  - 182-185 «assert!(»
