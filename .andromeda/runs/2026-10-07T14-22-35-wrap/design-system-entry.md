
## 2026-10-07-act-by-id — the driver's `advance` moves app time and leaves the animation clock alone
**Section:** §Motion → Animation runtime
**Change:** the harness-clock bullet gains one clause: the driver's `advance` moves an app's own time — through the step a session's caller hands it, on the stand whole Timer ticks — and leaves the harness's animation clock where it was, as do `click`, `type` and `press`; a driver click beside a running CSS animation returns settled without waiting on it. No style value, token or markup changes, and no cited line of this document moved.
**Why:** the chunk wired `advance`, and the two clocks must not be read as one: the app's time is the caller's to move, the animation clock is the harness's and moves only by `tick`. Measured on the stand's Timer (the clock reads the same before and after every call) and on a fixture with a running animation.
**Ref:** .andromeda/runs/2026-10-07T14-22-35-wrap/
