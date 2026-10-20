proposals:
  - detector: D-obs-stack
    severity: warning
    section: '§3 Observability Harness Contract (Logging stack, the session library clause)'
    change: 'Extend the session library clock clause «its `host` reads a clock for the idle expiry, and its MCP server one when a request arrives with a session held» with: and, since 2026-10-20-load-wait, its `session` module one for the load wait and nothing else — `settle_waiting` / `settle_within` read `std::time::Instant` and call `std::thread::sleep` (every `LOAD_POLL`, 10 ms) up to `LOAD_WAIT` (5 s of wall time), moving no time on the held instance; `execute`, `schema` and every other module stay clock-free (packages/escher-driver/src/session.rs:16-53; as measured at escher-0.2.0/chunks/2026-10-20-load-wait/report.md).'
    sidecar: 'obs-plan §3: escher-driver session.rs joins the wall-clock readers (host, MCP server) for the bounded load wait alone (2026-10-20-load-wait, fact clockrd).'
    rationale: 'Report Counts / qualifiers moved, fact clockrd: session.rs now reads Instant::now and sleeps for the load wait and nothing else (probe lists client.rs host.rs mcp.rs session.rs); site obs-plan.md:70@c2807 marked changed; Expected amendments name obs-plan §3.'
    basis: '.andromeda/obs-plan.md:70'
  - detector: D-obs-instrumentation
    severity: warning
    section: '§4 Span / Trace Coverage (escher-driver command span, the «Still observed absent» list)'
    change: 'Add to the absent list beside the settle wait itself (`Harness::settle`, unchanged): the driver load wait — `settle_waiting` / `settle_within` and the constant `LOAD_WAIT` (5 s), through which `Session::act` and `execute::settled_step` (all five acting verbs on every carrier) now settle — opens no span, emits no event and records no field of its own; it runs inside the call''s `command` span, which keeps its eight fields, and escher-driver keeps its 2 `info_span!` sites and 0 events (packages/escher-driver/src/session.rs:24-29; packages/escher-driver/src/session.rs:31-52; packages/escher-driver/src/execute.rs:462; as measured at escher-0.2.0/chunks/2026-10-20-load-wait/report.md).'
    sidecar: 'obs-plan §4: the driver load wait (settle_waiting, LOAD_WAIT) recorded as opening no span, event or field, inside the unchanged command span (2026-10-20-load-wait).'
    rationale: 'Report Symbols / APIs: settle_within makes no tracing call; Schema / config: no span field added; Coverage of new surfaces: instrumentation `command` span (unchanged eight fields, no new field, by obs-plan §4); Outcome (obs): info_span! probe 2, event probe 0; Expected amendments: obs-plan §4 (the «settle wait» absent line).'
    basis: 'packages/escher-driver/src/session.rs:31-52'
  - detector: D-obs-instrumentation
    severity: warning
    section: '§4 Span / Trace Coverage (escher-driver command span, the `passes` field)'
    change: '`passes` (settle''s pass count, when settle answered quiet) now adds: on a step that waited on a load in flight, the passes of the settle that went quiet — recorded on `Ok` only, as before; a step whose wait ends at `LOAD_WAIT` still loading records `settled=false` and `busy="loads"` and no `passes`.'
    sidecar: 'obs-plan §4: command span `passes` on a load-waited step is the passes of the settle that went quiet (2026-10-20-load-wait).'
    rationale: 'Report Symbols / APIs: «On a waited step the `command` span''s `passes` is the passes of the settle that went quiet (recorded on `Ok` only, as before)»; Outcome arm 2 reads settled false and busy Loads at the bound; Expected amendments: obs-plan §4 (`passes` on a waited step).'
    basis: 'packages/escher-driver/src/execute.rs:462'
  - detector: D-obs-instrumentation
    severity: warning
    section: '§3 Observability Harness Contract (detail: absence · logs, the fixtures module bullet)'
    change: '«its 20 readers (as measured at escher-0.2.0/chunks/2026-10-19-visual-lint/report.md)» becomes «its 21 readers (as measured at escher-0.2.0/chunks/2026-10-20-load-wait/report.md, `stand_load_wait` the twenty-first, through `HeldNet`)», the 4 print or sink sites of `stand_act_spans`'' re-run children still the only ones, none added.'
    sidecar: 'obs-plan §3 detail: fixtures module readers 20 → 21 (stand_load_wait), silence census unchanged (2026-10-20-load-wait, fact fxreaders).'
    rationale: 'Report Counts / qualifiers moved, fact fxreaders: fixtures readers 20 → 21, stand_load_wait reads HeldNet; site obs-plan.md:79 («its 20 readers») marked changed.'
    basis: '.andromeda/obs-plan.md:79'
  - detector: D-obs-instrumentation
    severity: warning
    section: '§3 Observability Harness Contract (Logging stack, the session_common readers clause)'
    change: '«for the module twenty-six checks read — `stand_lint`, …» becomes «twenty-seven», adding `stand_load_wait` (since 2026-10-20-load-wait, reading `Acted`, `act`, `click` and `snapshot`) to the list of readers of `tests/blitz-tests/tests/session_common/mod.rs`.'
    sidecar: 'obs-plan §3: session_common readers 26 → 27 (stand_load_wait) — an occurrence of fact screaders the report''s sweep did not list (2026-10-20-load-wait).'
    rationale: 'Report Counts / qualifiers moved, fact screaders: session_common readers 26 → 27 (stand_load_wait declares `mod session_common;`); the report lists only the two test-plan sites, but obs-plan.md:70 states the same count in words («the module twenty-six checks read»), so a digit-pattern sweep missed it.'
    basis: '.andromeda/obs-plan.md:70'
  - detector: D-obs-instrumentation
    severity: warning
    section: '§3 Observability Harness Contract (detail: absence · logs)'
    change: 'New detail bullet: The load-wait check is silent: `stand_load_wait` (a `fixtures` and `session_common` reader, both layout modes) and its in-file self-answering stand-in `LateNet` hold no print, log site, subscriber, sink, env read or `blitz_net`, their readings going into assertions only — the census probe reading 0 over the file; the load wait it pins (`settle_waiting`, `LOAD_WAIT`) adds no log site, span, field, log field or env read, escher-driver still holding its 2 `info_span!` sites and 0 events (tests/blitz-tests/tests/stand_load_wait.rs:52-60; as measured at escher-0.2.0/chunks/2026-10-20-load-wait/report.md).'
    sidecar: 'obs-plan §3 detail: stand_load_wait and its LateNet stand-in recorded silent; the load wait adds no log site, span or field (2026-10-20-load-wait).'
    rationale: 'Report Outcome (obs): «the new check and its stand-in are silent — met (census probe over stand_load_wait.rs 0)» and «the `command` span keeps its eight fields and escher-driver its two span sites, no event — met (info_span! probe 2, event probe 0)»; each prior chunk''s new checks carry such a silence bullet in this detail block.'
    basis: 'tests/blitz-tests/tests/stand_load_wait.rs:52-60'
note: 'D-obs-pii: no drift — the chunk adds no log site, field or print (report Coverage of new surfaces: PII n/a). D-retired: report Retired reads none; the two statements it says stop being true («settling … waits on no load», «no driver command waits on a load») hold no site in obs-plan (its «settle wait» absent line names `Harness::settle`, still true). The `command` span''s `time.busy` on a waited step now includes up to 5 s of wait — no obs-plan claim contradicts it, so no proposal.'
