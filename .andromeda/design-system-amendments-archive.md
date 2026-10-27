# design-system — archived amendment originals
Writer = wrap P7 only · read by NO loop skill · cold history, never cited for current truth; each run's originals under its own heading.

# Consolidated at the 2026-10-27-readme-and-records wrap — 0 re-worded · 2 pruned

## 2026-10-13-stand-contrast-harness — the seven_guis accent measured under white button text
**Section:** §Color Palette → Core Colors (seven_guis accent row)
**Change:** the row adds the measured reading: `#4a6cf7` is the background of white `#ffffff` button text on the five accent controls (`counter-increment`, `flight-one-way`, `flight-book`, `timer-reset`, `crud-create`; 14–16 px, weight 500–600, at boot), 4.39 against SC 1.4.3's 4.5 — a fail; every measured stand colour equals its declared value. The declared accent, hover and active values are unchanged.
**Why:** the stand contrast check measured the declared pairs; the palette states what its accent reads under the text the stand paints on it. Re-colouring is owned by the route entry "Stand a11y assertions", not by this row.
**Ref:** .andromeda/runs/2026-10-13T18-00-36-wrap/

## 2026-10-14-stand-a11y-assertions — the stand's accent one step darker, SC 1.4.3 met
**Section:** §Color Palette → Core Colors (the `seven_guis accent` row)
**Change:** the accent is `#3a5ce5`, hover `#2a4cd3`, active `#1f3fc1` in every task CSS that declares it, its non-text uses moving with it and its three `rgba` uses at `rgba(58, 92, 229, …)`, alpha kept; was `#4a6cf7` / `#3a5ce5` / `#2a4cd3`. White button text on the five accent controls reads 5.47, a pass (was 4.39, a fail); on the hover shade 6.83 and on the active shade 8.27, reached states of `stand_contrast`.
**Why:** the operator's F5, "One step darker, stand-wide", by question dialog — a stand change and no engine change, the hex literals staying where each task declares them (the token set is NOT YET MEASURED).
**Ref:** .andromeda/runs/2026-10-14T19-50-50-wrap/
