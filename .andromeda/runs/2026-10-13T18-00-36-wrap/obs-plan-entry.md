
## 2026-10-13-stand-contrast-harness — the contrast reader and `stand_contrast` silent
**Section:** §3 Observability Harness Contract → Logging stack (subscriber installation) · §4 Span / Trace Coverage
**Change:**
- §3: the headless stand's `stand_*` checks reading the shared module are 18 (was 17), `stand_contrast` the eighteenth; the census names `stand_contrast` — no subscriber, no env read, no print, no listener, its readings into assertions only; the silent harness listing adds `Harness::text_contrast` (with `contrast_ratio` and `TextContrast`), which prints, panics and logs nothing and reads and changes nothing in the document.
- §4: "Still observed absent" adds blitz-test-harness's contrast reader — `Harness::text_contrast` and the root re-exports, the private sixth module `contrast` — which opens no span, emits no event, reads no env var and mutates nothing; the crate still takes no `tracing`.
**Why:** a test-harness reader and a stand check, no hot path; no instrumentation is owed, and the record of what is uninstrumented names them. The "17 of which" wording carried the readers' count without the `mod common;` token.
**Ref:** .andromeda/runs/2026-10-13T18-00-36-wrap/
