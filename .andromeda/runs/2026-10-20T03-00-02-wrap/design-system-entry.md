
## 2026-10-20-load-wait — the load wait moves no clock
**Section:** §Motion → Animation runtime
**Change:** a new bullet: the driver's load wait moves no clock — while an acting step waits on a load in flight, for at most `LOAD_WAIT` (5 s of wall time), the harness's `VirtualClock` stands still, so a timer or an animation does not run during the wait.
**Why:** this chunk's own wait (the plan's reviewed expected amendment); settle's own claim beside it stays true.
**Ref:** .andromeda/runs/2026-10-20T03-00-02-wrap/
