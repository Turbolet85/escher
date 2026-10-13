# P5 review, card 2: 2026-10-13-stand-keyboard-harness ("Stand keyboard harness")

This card folds your directive on `.andromeda/runs/2026-10-13T22-07-35-phase/p5-card-1.md` (inputs#I5). Everything on
card 1 stands except what is listed here. After the fold, the whole mechanical set re-ran clean:
- the fence parses
- planlint reads 0 hits
- the lists parse (18 items)
- no entry reads NOT SEEN, so the baselines stand
- 5 inputs are cited and none is unparsed

The plan now reads 387 lines, still 12 steps, 12 acceptance criteria and 17 gate entries.

## What the fold changed in `escher-0.1.0/chunks/2026-10-13-stand-keyboard-harness/plan.md`
- **Leans:** 1 and 3 are recorded as confirmed by you (Provenance). Lean 2 is recorded as confirmed on your
  condition.
- **Step 1 (F3):** keeps today's reach, every element. It now names the known limit: a `<div disabled tabindex=0>` is
  no longer focusable, whereas browsers apply `disabled` to form controls only and would focus it.
- **Step 8 (disabled-skip test):** adds one row pinning that limit. A `div disabled tabindex="0"` placed between the
  fixture's buttons is skipped by Tab. This is the measured-and-pinned shape: red by design when the narrowing lands.
- **Constraints & rejected approaches:** narrowing F3 to `button`, `input`, `select` and `textarea` is not this chunk.
  It is a CARRY on "Stand requirement sweep".
- **Expected amendments (wrap), a11y-plan §5 Focus order:** the text states presence on every element and the
  `<div disabled tabindex=0>` limit, with its owner.

## Beyond the plan
Rows 1–4 are the ones you approved on card 1. Row 5 is new from your directive.

| # | proposal | where |
|---|---|---|
| 1 | New CARRY: the CRUD list takes focus but the tree's focus is the list, not the row. The accessibility builder reads no `aria-activedescendant`, and a click on a row or on the list still leaves nothing focused (F2 covers buttons only). This is your F5 condition, inputs#I4. | "Stand a11y assertions" (escher-0.1.0/working-route.md:94), beside the listbox-name CARRY |
| 2 | The CARRY "the second position reader reads a scrolled box shifted" moves off this entry, by the founder's F4 word «Отложить (Рекомендую)». Its hypothesis is restated: an activation key reaches the self-scrolled node, and only the click event's coordinates are off. | "Stand requirement sweep" (escher-0.1.0/working-route.md:96) |
| 3 | New CARRY: the engine honours no `inert`. A truthy `inert` neither removes focusability nor stops a hit. Pinned by `stand_keyboard`; a fix is an engine change for every document. | "Stand a11y assertions" (escher-0.1.0/working-route.md:94) |
| 4 | New CARRY: the merge surface of the three ratified engine changes, against 434e0661 (`events/keyboard.rs`, `events/mod.rs`, `events/pointer.rs`, `node/element.rs`). This follows the range model's precedent. | "Quality gates" (escher-0.1.0/working-route.md:98) |
| 5 | New CARRY: narrow F3's presence reading to the form controls `disabled` applies to (`button`, `input`, `select`, `textarea`). Until then a `<div disabled tabindex=0>` is unfocusable, unlike in browsers, which is the known limit pinned by `stand_keyboard`. This is your condition, inputs#I5. | "Stand requirement sweep" (escher-0.1.0/working-route.md:96) |

Apply? (yes / review / cancel)
