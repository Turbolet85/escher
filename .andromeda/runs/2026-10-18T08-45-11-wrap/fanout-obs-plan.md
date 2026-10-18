proposals:
  - detector: D-obs-instrumentation
    severity: warning
    section: "§4. Span / Trace Coverage"
    change: "In the `packages/escher-driver` command-span bullet, `cause` is one of the eight names `Cause::name` returns (`unknown-verb` · `malformed` · `not-found` · `stale` · `disabled` · `covered` · `off-screen` · `fonts-unavailable`), `time-unavailable` retired since 2026-10-18-one-virtual-clock (PROVISIONAL — the delegate operator's answer, for the founder's epoch batch), all eight returned by code today: `unknown-verb` and `malformed` by `validate`, `not-found`, `stale`, `disabled`, `covered`, `off-screen` and `fonts-unavailable` (six) by the executor; the dated history clauses (the causes each earlier chunk returned) stay as history."
    sidecar: "obs-plan §4: the command span's `cause` set closes at eight names, `time-unavailable` retired (all eight returned by code, two by `validate`, six by the executor), PROVISIONAL for the founder's epoch batch (2026-10-18-one-virtual-clock)."
    rationale: "Report Changes → Symbols / APIs (escher-driver): `Cause::TimeUnavailable` is removed, `CAUSES: [Cause; 8]`; Schema / config: the refusal set closes at eight causes; Counts item 1 lists obs-plan.md:113 as changed under both `causes-nine` and `time-unavail`; Addendum inputs#I4 (a): every master site stating the eight causes marks it PROVISIONAL. The span's fields are unchanged (Schema / config), so only the cause set moves."
    basis: "packages/escher-driver/src/refusal.rs:208 (the_eight_causes_are_named_in_order); .andromeda/obs-plan.md:113"
  - detector: D-obs-instrumentation
    severity: warning
    section: "§6. Log Coverage → Logged events (current truth) → escher-driver"
    change: "The command-span line's closed value sets read eight cause names, not nine (nine until 2026-10-18-one-virtual-clock, `time-unavailable` retired, PROVISIONAL), beside the verb table's eleven words, three busy-class words, bools and counts."
    sidecar: "obs-plan §6 escher-driver log line: eight cause names, not nine (2026-10-18-one-virtual-clock, PROVISIONAL)."
    rationale: "The same retired claim restated in §6: its escher-driver bullet lists `nine cause names` among the command line's closed values. The report's Counts item 1 (`causes-nine`, `time-unavail`) retires the ninth cause; the sweep's `causes-nine` row lists obs-plan.md:113 only, so this restatement (worded `nine cause names`) was not reached by the pattern and would survive a single-site apply."
    basis: ".andromeda/obs-plan.md (§6 Logged events, escher-driver, the `span=\"command\"` bullet)"
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: "§4. Span / Trace Coverage"
    change: "Add to the command-span bullet: since 2026-10-18-one-virtual-clock `advance` is accepted on every session (it moves the session's one virtual clock by exactly `ms`, then the app's time step where one was handed) and, accepted, records `verb` and the acting fields (`settled`, `busy`, `passes`, `added` · `removed` · `changed`) like every acting verb, with `advanced_ms` and any time or clock reading still never fielded — measured in process in both layout modes by `stand_act_spans`, whose CRUD `advance` row now reads an accepted line (`Expect::Acts`) holding only the schema fields; the span keeps its eight fields."
    sidecar: "obs-plan §4: an accepted `advance` (every session, the one virtual clock) leaves one `command` line with only the schema fields, no time fielded — `stand_act_spans`' CRUD `advance` row now `Expect::Acts` (2026-10-18-one-virtual-clock, PROVISIONAL)."
    rationale: "Report Changes → escher-driver `advance` (accepted on every session, `advanced_ms = ms`); Schema / config: the `command` and `tool_call` span fields are unchanged, no time, size or image fielded; Coverage of new surfaces: driver `advance` → instrumentation ✓ (`stand_act_spans` reads an accepted `advance` line); Outcome (obs) MET: the CRUD `advance` row now `Expect::Acts`; Expected amendments item 6 (§4: `stand_act_spans`' `advance` line accepted)."
    basis: "tests/blitz-tests/tests/stand_act_spans.rs"
  - detector: D-obs-instrumentation
    severity: warning
    section: "§4. Span / Trace Coverage"
    change: "Add to the `tool_call` bullet: since 2026-10-18-one-virtual-clock an `advance` tool call on a session with no time step ends `accepted` and parents one `command` child whose `verb` is `advance` and which records no `cause`, measured in process by `stand_mcp_tools`; no field added, the span keeps its two."
    sidecar: "obs-plan §4 `tool_call`: an accepted `advance` tool call ends `accepted` with one child `command` (`verb` advance, no `cause`), read by `stand_mcp_tools` (2026-10-18-one-virtual-clock)."
    rationale: "Report Deviations item 10 (on the operator's word): `stand_mcp_tools` now reads the accepted `advance`'s `tool_call` span — `ending` = `accepted`, one child `command` whose `verb` is `advance` and which records no `cause` (plan step 13's F2 A row); Coverage of new surfaces: driver `advance` → instrumentation ✓ (`stand_mcp_tools` reads its `tool_call` ending `accepted` with one child `command`)."
    basis: "tests/blitz-tests/tests/stand_mcp_tools.rs"
note: "D-obs-stack and D-obs-pii read clean — no dependency added (`web-time` already a blitz-dom dependency), no new log site, span field or env read, `DocumentConfig.clock` and `Harness::advance_time` instrumentation n/a by design. The plan's Expected amendments item 6 also asks §3's silent census to name the two new checks `virtual_clock_engine` and `stand_virtual_clock`, but the report records no silence probe over them (no subscriber / env / print reading) and does not say whether `stand_virtual_clock` reads `session_common/mod.rs` (which would move §3's `the module twenty-two checks read` count) — no proposal is made for it; measure before adding."
