## 2026-10-07-settle-detection — the settle rule's one answer for the keeps-animating set
**Section:** §Motion → Animation runtime (the keeps-animating bullet and the animation-clock bullet)
**Change:** a harness settle answers for every member of the keeps-animating set the same way — none holds it open and none is waited on or advanced: it returns `Settled` with `animating` reading the document's animating flag; one member, a CSS animation, is exercised by a check, the others are answered by the same reading and not each exercised. Settle reads animation time only through the harness's controlled clock and never advances it. The harness-clock citation is re-pointed to the lines measured after the chunk (it was stale before it).
**Why:** the set's scrollbar-fade member reads the wall clock and a canvas reads animating for as long as it exists, so waiting on the flag would need a sleep; the caller holds the clock.
**Ref:** .andromeda/runs/2026-10-07T11-06-31-wrap/
