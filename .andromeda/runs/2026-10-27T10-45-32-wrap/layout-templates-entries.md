## 2026-10-27-upstream-sync-engine-debts — citations into the files the take moved
**Section:** §Surface: desktop-native → Primary screens (the browser frame) · Component — Primary navigation (browser tabs) · IA notes (the viewport and safe area, pointer coordinates, the scrolling element, the root element's client size)
**Change:**
- 4 `file:line` citations into `apps/browser/src/{main,tab}.rs` and blitz-shell's `window.rs` re-pointed by the take's line shifts; claim text unchanged, the browser frame's column still holding TabStrip, Toolbar, one TabWebView per tab, the FPS overlay and StatusBar.
- 2 citations into blitz-vibey-script re-pointed to new paths: the scroll of the root element by `window.scrollTo`/`scrollBy` was `src/runtime.rs`, now `src/runtime/window.rs`; the root element's `clientWidth`/`clientHeight` was `src/dom/element.rs`, now `src/dom/element/geometry.rs`. Claim text unchanged.
**Why:** the take split blitz-vibey-script's runtime into `runtime/` modules and its element bindings into `dom/element/` modules, moving that code between files and removing none, and its upstream commits shifted lines in the browser app and the shell. The two old paths no longer held the cited code, so their paths are rewritten and not only their digits.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — the ten further stand lines on the desktop-native screens
**Section:** §Surface: desktop-native → Primary screens (its `values · desktop-native-screens` block: the served-screens bullet, the flight booker's mode-region bullet, two new lines)
**Change:**
- Before, the block held the nine stand rows alone, and the ten further stand lines waited on the founder in design-system, per 2026-10-27-readme-and-records — the stand's colour, depth, spacing and radius rows on the desktop-native screens (PROVISIONAL). Now the stand-only lines among them land here, values unchanged:
  - the served-screens bullet: the load page holds two keyed elements, `load-fetch` and `load-done`, and `LoadNet` answers from a thread of its own, so a driver `click` on `load-fetch` waits it out and returns with the message shown; the bullet now reads as measured at 2026-10-25-cold-agent-bar.
  - the mode-region bullet: the select's styling is 15px, weight 400 (before, 15px with no weight given), and its two options show the chosen one alone by the engine's select rule.
  - a new line, the stand's invalid state: `#e53e3e` / `#fff5f5` / `#c53030`.
  - a new line, the stand's type: sans-serif, the home title 700 at 36px over a 14px base.
- The stand's text `#1a1a1a` gets no line of its own, because the colours line already states it.
**Why:** the founder chose by phone that all ten lines move, the shared ones split, and the operator relayed his word for this wrap. The two rows shared with todomvc and the subjects line are design-system's to change. At the P2 Validate card dialog the operator ruled that the stand's text colour is a duplicate and folds into the colours line.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — F2 ratified: the stand rows' gap marker drops
**Section:** §Surface: desktop-native → Primary screens (its `values · desktop-native-screens` block, the closing line)
**Change:**
- Before, the block closed on an `OPEN — PROVISIONAL` line: the stand's colour, shadow, spacing and radius held on this surface, out of design-system's product surface, owner F2, per 2026-10-27-readme-and-records — the stand's colour, depth, spacing and radius rows on the desktop-native screens (PROVISIONAL). Now that line is removed and the block ends on its value rows, which stand as moved with their values unchanged.
**Why:** the founder ratified F2 by phone, and the operator relayed his word for this wrap. A ratification replaces the delegate's PROVISIONAL answer by rule. At the P2 Validate card dialog the operator confirmed the discharge, which follows the founder's word: the gap marker drops, and this entry carries the ratification.
**Kept:** the 2026-10-27-readme-and-records PROVISIONAL entry keeps its words and is not superseded, because a sidecar is append-only history and the rows it moved still stand in the body.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/
