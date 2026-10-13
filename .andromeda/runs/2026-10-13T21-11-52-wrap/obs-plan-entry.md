
## 2026-10-13-range-input-interaction — `range_input_interaction` joins the silent census
**Section:** §3 Observability Harness Contract → Logging stack (the census of checks that install no subscriber)
**Change:** the census names the engine check `range_input_interaction`, which drives `blitz_test_harness::Harness` on its own parsed pages in both layout modes. It installs no subscriber, reads no env var and prints nothing; its readings go into assertions only. The bullet's measured-at list cites this chunk's report. The rewritten `stand_act_range` stays one of the named `stand_act_*` checks.
**Why:** a new check was added. The chunk adds no span, event, field or log site; the range value leaves only through the ratified carriers.
**Ref:** .andromeda/runs/2026-10-13T21-11-52-wrap/
