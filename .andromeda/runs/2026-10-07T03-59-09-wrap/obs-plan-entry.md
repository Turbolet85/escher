
## 2026-10-07-audit-corrections — the headless-stand census covers the checks' shared module
**Section:** §3 Observability Harness Contract → Logging stack (the headless-stand bullet)
**Change:** the headless stand installs no subscriber — no `escher_telemetry::init` and no env read in it, its checks or their shared module `tests/blitz-tests/tests/common/mod.rs`, and no `println!` in any of them save `stand_id_persistence`'s re-executed child (was: "in it or its checks"). The shared module's head is cited; the two existing citations keep their numbers.
**Why:** six stand checks now read their tables and helpers, `boot` among them, from a module that is not itself a `stand_*` check, so the census had to name it; it reads 0 there. The chunk added no log site, print, subscriber, env read or file write anywhere.
**Kept:** the chunk's other new file, dioxus-native-dom's `dioxus_document_tests.rs`, is named in no obs-plan sentence — the plan has none about that crate's unit tests; its census of 0 is in the chunk's report.
**Ref:** .andromeda/runs/2026-10-07T03-59-09-wrap/
