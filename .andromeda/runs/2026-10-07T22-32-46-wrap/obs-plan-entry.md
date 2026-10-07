
## 2026-10-07-driver-command-spans — the driver's command span as built; `tracing` ungated in escher-driver
**Section:** §1 (escher-telemetry, one citation) · §2 Telemetry mechanism, Feature wiring · §3 → Logging stack, Service identity (one citation), Agent-run harness log · §4 Span / Trace Coverage · §6 → Logged events
**Change:**
- §4: the `escher-driver` bullet was "observed absent … one span per driver command … is owed by the route entry"; now the span as built — target `escher_driver`, name `command`, INFO, `tracing::info_span!`, entered before `validate`; eight fields each recorded only where it applies (`verb` · `cause` · `settled` · `busy` · `passes` · `added` · `removed` · `changed`); never fielded: `in_view`, `advanced_ms`, the screen text or its length, the label, the record of ids, any argument, any `Command`, `Call`, `ArgValue`, `Outcome`, `Refusal` or `Fault`. Still observed absent: a span, event or `tracing` dependency in `Harness::settle`, and a span of their own on `Session::act` and the lifecycle. `busy` is covered by its unit test only.
- §2: escher-driver is the second escher crate to take `tracing` ungated; its span is compiled into both seven_guis binaries and reached by no code path there. "The engine `tracing` features stay off" (two sites, with §8) now reads of a package-alone build: a workspace-wide build turns `blitz-dom/tracing` on, so a test child with the sink at `info` also receives engine events. Not measured: the two binaries' stderr by level under that build.
- §3: the driver has a `tracing` dependency (was none) and its executor opens the span (was "print or log nothing"). The census of stand checks that install no subscriber has one exception: `stand_act_spans`'s two re-run children, the sink over an in-memory capture. Counts: 15 `common` readers (was 14), ten `stand_act_*` (was nine), fifteen of sixteen session-holding checks read `session_common` (was fourteen of fifteen).
- §6 Logged events: a new group for the driver's one closed-span line per call.
- Citations re-pointed.
**Why:** the chunk delivered the route entry "Driver command spans".
**Kept:** "nothing in the crate prints, logs or fields any of the four" and the same of the record of ids — both still true.
**Ref:** .andromeda/runs/2026-10-07T22-32-46-wrap/
