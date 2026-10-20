- line: .andromeda/architecture.md:192
old: «the crate re-exports 55 names under `#![deny(missing_docs)]` — the session's `Session`, `SessionError`, `serve`, `IDLE_EXPIRY`, `start`»
new: «the crate re-exports 56 names under `#![deny(missing_docs)]` — the session's `Session`, `SessionError`, `serve`, `IDLE_EXPIRY`, `LOAD_WAIT`, `start`»

- line: .andromeda/architecture.md:192
old: «which runs the step on the held instance, settles it (`Harness::settle`, the Test harness contract above) and maps»
new: «which runs the step on the held instance, settles it through the load wait (below: `Harness::settle`, the Test harness contract above, settled again while a load is in flight, for at most `LOAD_WAIT`) and maps»

- line: .andromeda/architecture.md:192
old: «its private `image` encoder print nothing and read no clock or environment.»
new: «its private `image` encoder print nothing and read no environment, and none of them reads a clock but the session's load wait (below).»

- line: .andromeda/architecture.md:192
old: «the snapshot before, the step, `Harness::settle`, the snapshot after and `before.diff(&after)`; settling moves no time and waits on no load.»
new: «the snapshot before, the step, the load wait over `Harness::settle`, the snapshot after and `before.diff(&after)`; settling moves no time, and a load in flight is waited for, for at most `LOAD_WAIT` (the load wait, below).»
