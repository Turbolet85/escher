# P5 review card — 2026-10-31-select-popup, «Select popup»

**Plan:** `escher-0.3.0/chunks/2026-10-31-select-popup/plan.md` · 22 steps · 31 gates · starts from `3af5a9bcf`. Research: `research.md` beside it. Controls: `.andromeda/runs/2026-10-31T16-59-55-phase/p5-controls.md`.

## Goal
A closed single `select` opens a list the engine paints, in every document and through every renderer that paints with blitz-paint. Its rows are the select's own `option` elements, so each keeps its stable id. A person or an agent chooses by pointer and by key through the one write path, closes with no choice, and the select reads the choice. No verb, node key, cause, session error, exit, carrier, listener, env var, crate or dependency is added.

All ten forks are his option A, in his own words (inputs#I4). No `expanded` key and the other leans are your word (inputs#I6).

## How it is built
- **One engine overlay**, shaped like the overlay scrollbar: state in one `BaseDocument` field naming DOM nodes; one geometry that paint, hit and snapshot all read; painted last in `paint_scene`, hit first in `hit_with_scrollbar`. `hit_inner` is not edited, so the recorded `0980cce5f` conflict does not grow.
- **It lays out nothing**, so both layout modes read one layout.
- **The list closes by one rule:** a reader that answers only while the select resolves, holds focus, and is not disabled, inert, hidden, under a singular transform or moved. `resolve` closes whatever that reader stops answering. So `inert`'s silent focus loss closes it too, and page scroll closes it as your lean had it.

## Leans made while planning, for your eye
Each sits in the plan's steps or `Constraints` and is a one-line change now.
- **The fixed colours live in code, not `default.css`.** A `default.css` rule would turn a closed `selected disabled` option grey, no selector can tell an open row, and a config with its own sheets would paint no list.
- **Only the plain pair is readable by `text_contrast`.** The highlight and disabled pairs are measured on frames, within 1 per channel, with `contrast_ratio`. The lint's `contrast` reads a row's plain pair even when it is highlighted, a stated limit.
- **`SelectListView` is one public type** beside `node::select`'s rule of none, because blitz-paint is another crate (the `ScrollbarRef` precedent). Every rule and writer stays crate-private.
- **The border is always `#999`**, even on the stand, whose select has a `#c0c0c0` border. That is how I read his F6 text. The list's background is the select's, composited over white so it is always opaque.
- **Row metrics:** one height for all rows, measured at opening; a row with a larger authored font clips. Width is measured at opening and capped at the viewport.
- **The thumb is painted and not draggable.** Keys, wheel and `scroll` reach the rows, also with the `scrollbars` feature off, as in the harness's build.
- **`type` now focuses any select with no click**, so a `type` on a single select fires no DOM focus events. A `multiple` select already reads so.

## What stays unproven, stated in the plan
- **The windowed list is read, not run.** No test launches a window, and the smoke proves the boot alone.
- **What an assistive technology reads** stays unmeasured, its residual line unmoved. It hears the list is open, not which row is highlighted.
- **`size`'s height is unmeasured on a frame.** The equality is `line-height: 1.2` on the rows and `size × 1.2em` on the box, read from `stylo_to_parley.rs:407-412`; `select_popup`'s `size` row is its first reading.
- **No cold-agent run.** The new `covered` clause and the list could move `wrong_calls`; no entry reads it, and `v030-15` owns the next run.

## Top acceptance criteria
- `select_popup`: opens by click, Enter and Space; chooses by click, Enter, Space and Tab, one `input` then one `change`; closes by Escape, a second click and a click outside with no event.
- `stand_act_popup`: `click flight-type`, then `click flight-type-return`, the next snapshot reading `return`; `press escape` closes with the value unchanged; a closed option stays refused `covered`.
- `select_popup_painted`: rows on frames past `overflow: hidden` and under `opacity`; each pair measured; a closed select's frame byte-equal before opening and after a close.
- Disabled, inert, singular-transform and removal cases open nothing and do not panic; a hostile `size`, 2,000 options and a 5,500-character label stay finite.
- `describe`'s count row stays `[12, 13, 12, 3, 8, 4, 15]`; the a11y leg stays fourteen files; the census reads 0; CI reads `verdict: green`.

## Touchpoints
5 new files · 35 modified: blitz-dom's `document`, `resolve`, `events/{keyboard,pointer}`, `accessibility`, `scrolling`, `stylo`, `node/mod`, `lib`, `assets/default.css`; blitz-paint's `render`; dioxus-native-dom's `snapshot`, `lib`; escher-driver's `execute`, `lint`, `refusal`, `schema`, `schema_tests`, `json`, `lib`; `docs/capabilities.md`; `.gitignore`; `ci-leg.sh` and its pin; 6 standing blitz-tests checks; 4 seven_guis test files. `stand_act_ids`' 68 and 9 are forecast unmoved. Graph impact: `research.md`.

## Test commands
31 gates: 7 inline guards · 3 unit crates · the 3 new targets · 2 standing target sets · `ci-leg.sh fast`, `doc`, `audit`, `a11y` · `agent-run.sh` stand and all · `seven_guis` · windowed build and smoke · 4 operator entries (hygiene, push, CI read, per-OS tally). `gate.py baseline`: 10 seen green · 17 new, none unbaselined. The push also carries the two records-door commits not yet on the remote.

## Concretization preview — `v030-17` «Select popup»
**Now:** "A closed select opens a painted list of its options; a person or an agent chooses one by pointer and by key and the select then reads that choice; an option of the opened list can be acted on through the driver; the list closes without a choice."

**Proposed:** the same sentence, then, in both layout modes through the real dispatch: `select_popup` opens by click, Enter and Space, chooses by click, Enter, Space and Tab, one `input` then one `change` per change, and closes by Escape, a second click and a click outside with no event; `select_popup_painted` reads the rows on frames; `stand_act_popup` accepts `click flight-type`, then `click flight-type-return`, the next snapshot reading value `return`, and closes by `press escape` with the value unchanged.

Never weakened. It is an affordance cap read through real input, not a proxy: every key and click goes through the harness's dispatch. No other markerless entry lists it.

## Validation
- **Mechanical checks:** pass. Sections present, every modify path exists, the fence parses, 31 entries, 226 prose lines. `planlint`: 1 hit, gate 31's per-OS log read, its control fired at this P5 on CI#41208321538 and equal to the recorded rows. Check 12 (d)'s seven claims read; none is contradicted by a step.
- **Controls:** every inline guard read both ways. They caught one wrong gate of mine: entry 2 named `stand_id_persistence.rs`, which prints its child's lines by design. It left by file, the note says why, and the entry was re-fired.
- **Check 10:** `I1` and `I4` unchanged; no `UNPARSED`.
- **Validation-1:** aligned. Three scope premises were corrected at P3: a pressed Space is typeahead today, more checks click a select before a key, and a native select writes no `expanded`.
- **Tree:** no code changed; `git status --short -- packages tests examples docs .github Cargo.lock Cargo.toml .gitignore` prints nothing.

## Sampler
- `sampler: architecture — E carried 189 · the cascade lost 14 · listed 12`
- `sampler: security-plan — E carried 86 · the cascade lost 5 · listed 3`
- `sampler: design-system — E carried 46 · the cascade lost 2 · listed 4`
- `sampler: test-plan — E carried 123 · the cascade lost 15 · listed 13`
- `sampler: obs-plan — E carried 43 · the cascade lost 6 · listed 4`
- `sampler: a11y-plan — E carried 102 · the cascade lost 7 · listed 0`

49 lost in all, ids in `cascade-compare-{doc}.md`. Each stands in the plan marked `(sampler)`, at its step, its gate or in `## Implementation notes`. Most of test-plan's and architecture's are readings the plan's gates already hold, so the sample reads no gap in what the plan builds.

## Beyond the plan
| # | Proposal | Where it lands | By whom |
|---|---|---|---|
| 1 | **Every limit this plan states and leaves for later gets a residual line and a deferred line:** the thumb not draggable · a highlighted row's contrast unread by the lint · a larger authored font clipping · the highlight absent from the snapshot and the tree · a single select's `size` unread. The existing `press` line, Ctrl on `press`, also owes its deferred line | `.andromeda/residuals.md`, one line each for the five; `escher-0.4.0-incubator/deferred-from-0.3.0.md`, one line each for the six | this chunk's wrap, at its P5 |

**At the word (added after the card was shown):** row 1 is the operator's addition, given with the `yes` as a row on this card and changing no plan line, so the word stands with no re-prompt. Snapped in this chunk's inputs.
