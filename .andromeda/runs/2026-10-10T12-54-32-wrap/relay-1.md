# Route adaptation — the two engine fixes get their own entry ahead of "MCP surface" (2026-10-10)

No chunk is pending. This is a 0-pending wrap whose work is the route-resolve below.

## Context correction

- CI on the wrap commit `177d1652` is green 16/16 (CI#38047227075, read by the operator at 11:19Z). The handoff's
  tests line still reads `e144b44d` only.
- The Driver CLI wrap pinned two CARRYs on "MCP surface" as its own placement, each marked "move it if it reads
  wrong": a scrolled box's own bounds read shifted, and the hit walk reaching content scrolled out of its box.

## The founder's ruling

Asked on 2026-10-10, by question dialog in the overseer session, where the two defects are fixed — three options put
to him whole: their own entry before "MCP surface" · their own entry after it and before "Headless screenshot" ·
left on "MCP surface" as the wrap placed them — the founder chose «Отдельный чанк до MCP». That label is the
overseer's Russian wording for "a separate chunk before MCP surface"; the ruling is his, relayed by the overseer.

His earlier words on the same subject stand beside it: 2026-10-07, the engine function itself is fixed for nested
scrolling; 2026-10-10, fork 9A — measure and state in the Driver CLI chunk, the fixes stay owned on the route.

The reasoning that was put to him: every later surface carries `bounds` and a diff out of the process — an MCP
tool's result, an element's screenshot by id — and his standing direction is that what a feature rests on is built
before the feature; and "MCP surface" already carries the protocol, a new crossing question and the JSON question,
after a wrap whose second half filled a cleared window to 81 %.

## Adaptation items — for your route-resolve dialogue

1. A new markerless entry directly ahead of "MCP surface" that fixes both defects in the engine. Its name, its one
   line, and whether it is one entry or two are the dialogue's. What it has to carry, from the Driver CLI chunk's own
   record: each fix with a regression check in both layout modes; the two readings that chunk pinned "as measured",
   which turn red by design when the fix lands; and the three sentences of the tool that state the limits today
   (`covered`'s meaning, the help of `changed` and of `snapshot`'s `text`), restated to what is then true.
2. The two CARRYs leave "MCP surface" for that entry.
3. Both fixes are upstreamable and are flagged so in that chunk's report. No upstream PR is opened: the founder's
   standing ruling is that none is until DioxusLabs/blitz#1083 gets a reaction.

## Anchors — what stays

- "MCP surface" is next after the new entry, with its other four CARRYs and its WATCH unchanged. The rest of
  Epoch 5 and all of Epoch 6 keep their order.
- The new entry claims no capability unless the dialogue finds a requirement whose wording it proves.
- The fix is a direct change in the engine's files. Additivity is a preference, never a goal: where staying additive
  would bend the logic, the upstream code is changed and the next sync's conflict is the accepted price.
