
## 2026-10-13-stand-keyboard-harness — `stand_keyboard` joins the silent census
**Section:** §3 Observability Harness Contract → Logging stack (subscriber installation)
**Change:**
- The stand checks reading the shared module are 19 (was 18), `stand_keyboard` the nineteenth.
- `stand_keyboard` joins the silent census: it drives the four lean tasks and its own fixtures in process in both layout modes, installs no subscriber, reads no env var, prints nothing and binds no listener, its readings going into assertions only. The silence probe reads 0 over it and over the shared module whose `tab_order` and `focused` it reads.
**Why:** this chunk's new check and its count. The shared module's probe was run at this wrap, the report's probe having read the check file alone.
**Ref:** .andromeda/runs/2026-10-13T23-23-45-wrap/
