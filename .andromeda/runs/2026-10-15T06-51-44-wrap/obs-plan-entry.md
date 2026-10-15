
## 2026-10-14-quality-gates — `busy` on a real line, the stderr readings, the `repair` phase
**Section:** §2 Telemetry Strategy → Feature wiring · §4 Span / Trace Coverage (`command`) · §8 PII Scrubbing & Compliance · §9 CI Integration (package install)
**Change:**
- §4: `busy` is driven. `stand_act_spans` runs `press tab` on a fixture that re-keys an element every render, in both layout modes. The line carries `settled=false` and `busy="render"`, no `passes` and no `cause` — six of the span's eight fields. Was "not driven … covered by its unit test only".
- §8 and §2: a client's own stderr, the host beyond three CRUD commands, and the workspace-built windowed `seven_guis_native` on the host's display were read by build and level, 28 runs, 0 needles. Was "not measured there".
- §9: the install diagnostic's phase is `update`, `install` or `repair` — the bounded `dpkg --configure -a` before each retried attempt. The line's shape is unchanged (was "`update` or `install`").
**Why:** measured at this chunk; no scrub, redaction, log field or span changed.
**Ref:** .andromeda/runs/2026-10-15T06-51-44-wrap/
