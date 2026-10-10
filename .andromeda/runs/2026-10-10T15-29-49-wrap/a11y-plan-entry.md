
## 2026-10-10-scrolling-box-bounds-and-hit — a click on a plain button leaves nothing focused, measured
**Section:** §5 → Focus restoration
**Change:** gains one line: after an accepted pointer click on a plain `button` no node reads `focused` in the snapshot and the accessibility tree's focus carries no author id, in both layout modes and under both builds. The section held one click line before — a click on a non-interactive area clears focus — and no statement on a button.
**Why:** the chunk's plan took this reading to be recorded here already, as read from code; this section held no such line, and the chunk measured it on a restated stand check. Whether a click should focus the button is not decided here: it is owned by the route entry that works keyboard focus.
**Ref:** .andromeda/runs/2026-10-10T15-29-49-wrap/
