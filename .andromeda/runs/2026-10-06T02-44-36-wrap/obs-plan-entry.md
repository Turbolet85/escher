
## 2026-10-06-headless-stand — telemetry installer named; the headless stand installs none
**Section:** §3 Observability Harness Contract → Logging stack
**Change:** was "escher's stand installs `escher_telemetry::init`"; now `seven_guis_native` (the windowed stand binary) installs it, and the headless stand `seven_guis::stand` with its in-process checks installs no subscriber — no `escher_telemetry::init`, no `println!`, no env read — so a headless boot has no escher sink.
**Why:** after the headless stand chunk "the stand" names two surfaces; only the windowed binary installs telemetry, and init stays once per process.
**Ref:** .andromeda/runs/2026-10-06T02-44-36-wrap/
