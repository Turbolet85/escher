## 2026-10-27-readme-and-records — the stand's colour, depth, spacing and radius rows on the desktop-native screens (PROVISIONAL)
**Section:** §Surface: desktop-native → Primary screens (its `values · desktop-native-screens` block)
**Change:**
- The block held the load page's and the flight booker's select rows alone; it now also holds four stand value rows, values unchanged:
  - colours — the accent `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`) in every task CSS that declares it, on the page `#f5f5f5` with text `#1a1a1a`, its non-text uses the same shade and its three `rgba` uses `rgba(58, 92, 229, …)`; white `#ffffff` text on the four accent controls (`counter-increment`, `flight-book`, `timer-reset`, `crud-create`, 14–16 px, weight 500, at boot) reads 5.47 against SC 1.4.3's 4.5, 6.83 on hover and 8.27 on active, each a reached state of `stand_contrast`, while `#4a6cf7` reads 4.39, a fail; every measured stand colour equals its declared value; `load-done` is `#1a1a1a` on `#f5f5f5`, and no contrast row reads the load page; the flight booker's success message `#ebf8ee` / `#68d391` / `#276749`.
  - depth — the cards' `0 2px 8px rgba(0, 0, 0, 0.08)`, the circle dialog's 0.10.
  - spacing — the cards' 24px 32px padding and 16px gap, Home's 48px 32px 64px.
  - radius, untokenized — cards 8px, buttons 6px, inputs 4px, task cards 6px, tags 3px.
- A closing `OPEN — PROVISIONAL` line: the stand's colour, shadow, spacing and radius held on this surface, out of design-system's product surface, owner F2.
**Why:** the stand's rows leave design-system's product surface and land here with their values as they stood (the report's third expected amendment). F2 is the founder's delegate answer, taken at the operator's question dialog at phase P4 and marked PROVISIONAL at the operator's direction; it goes to the founder, whose own word supersedes it by rule. The operator, by question dialog at this wrap's P2 halt, held the move to the nine rows the plan names, so the ten further stand lines wait for the founder.
**Kept:** the load page row (`load-fetch`'s three accent shades) and the select row (`#1a1a1a` on `#ffffff`, `1px solid #c0c0c0`, radius 6, padding 8/10, the `#3a5ce5` focus border) stand as written; the new rows fold them and do not restate them.
**Ref:** .andromeda/runs/2026-10-27T05-59-26-wrap/
