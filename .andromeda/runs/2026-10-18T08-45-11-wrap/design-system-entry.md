## 2026-10-18-one-virtual-clock — §Motion: the harness drives animation from the documents' clock, PROVISIONAL
**Section:** §Motion (the settle-and-animating bullet; the animation-clock bullet)
**Change:**
- Settle: `animating` reads the document's animating flag, the scrollbar fade's membership read off the document's clock — after a driver `scroll` it reads true until 700 ms of that clock pass.
- Animation clock: was «the test harness drives animation from a controlled clock advanced by tick … the driver's `advance` moves an app's own time … and leaves that clock where it was»; now the harness drives animation from the `VirtualClock` its documents read — the overlay scrollbar fade, smooth scrolling and the fling read it headlessly — moved by `tick`, `advance_time` and the driver's `advance`, which moves it by exactly its `ms` and the app's own time through the session's step together; `click`, `type` and `press` leave it where it was. `FADE_DELAY`, `FADE_DURATION` and the smooth and fling constants are unchanged. PROVISIONAL.
**Why:** the founder's forks F1 B and F2 A, answered by the operator as the founder's delegate (delegate the operator, 2026-10-18), PROVISIONAL, for the founder's epoch-boundary batch (the operator, 2026-10-18, at the wrap resume).
**Ref:** .andromeda/runs/2026-10-18T08-45-11-wrap/
