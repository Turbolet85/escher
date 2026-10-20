proposals:
  - detector: D-obs-instrumentation
    severity: warning
    section: "§3 Observability Harness Contract → Logging stack (detail: absence · logs)"
    change: "Add a bullet after the load-wait check's: The select checks are silent — `select_interaction` (a `fixtures` reader, both layout modes) and `stand_act_select` (a `fixtures` and `session_common` reader, both layout modes) install no subscriber, read no env var and print nothing, their readings going into assertions only, the two checks' silence probe reading 0 (gate entry 9); the select model they pin (blitz-dom's `pub(crate)` module `node::select`, the keyboard arm `press_select_key` / `type_into_select` / `apply_select_choice`, the pointer arm, the mutator's `select_nodes` settle, `DomEventKind::Change`) adds no log site, span, field, log field or env read — select.rs 0 print / log / env / panic tokens (gate entry 8), escher-driver keeping its 2 `info_span!` sites (gate entry 10), no wall-clock read (gate entry 7: typeahead reads the document's clock) (as measured at escher-0.2.0/chunks/2026-10-20-select-by-id/report.md)."
    sidecar: "obs-plan §3: the select checks `select_interaction` and `stand_act_select` join the silence census; the select model adds no log site, span or field (2026-10-20-select-by-id)."
    rationale: "The report's Expected amendments name «obs-plan §3 — the two new checks in the silence census» (site `silence`, obs-plan.md:70 re-derived; obs-plan.md:74-80 no-change, «this chunk's is an addition»); Coverage of new surfaces states instrumentation n/a — gate entry 8 reads 0 over select.rs, entry 10 two `info_span!`; Outcome «(obs) two span sites, silent checks, no wall read — met: entries 10 (2), 9 (0), 7 (0)»."
    basis: "escher-0.2.0/chunks/2026-10-20-select-by-id/report.md (Counts / qualifiers moved → the silence census; Outcome (obs))"
  - detector: D-obs-instrumentation
    severity: warning
    section: "§3 Observability Harness Contract → Logging stack (the headless stand `seven_guis::stand` bullet)"
    change: "«for the fourteen driver-action checks `stand_act_*` (… the fourteenth, `stand_act_transformed`)» → «for the fifteen driver-action checks `stand_act_*` (… the fourteenth, `stand_act_transformed`; the fifteenth, `stand_act_select`, since 2026-10-20-select-by-id)», and the census gains the engine check `select_interaction` (installs no subscriber, reads no env var, prints nothing, declares no shared module but `fixtures`)."
    sidecar: "obs-plan §3: the stand census counts fifteen `stand_act_*` checks, `stand_act_select` the fifteenth, and gains `select_interaction` (2026-10-20-select-by-id)."
    rationale: "Report, Counts / qualifiers moved: «`stand_act_*` files fourteen → fifteen (`stand_act_select` the fifteenth) — pattern `stand-act`: … stale (… obs-plan.md:70 @c5360, @c6267 …)»; and the `silence` row «obs-plan.md:70 re-derived (its census gains `select_interaction` and `stand_act_select`, gate entry 9 reading 0 over them)»."
    basis: "obs-plan.md:70 @c5360"
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: "§3 Observability Harness Contract → Logging stack (the headless stand `seven_guis::stand` bullet, the `session_common` reader list)"
    change: "«… `stand_settle`, the fourteen `stand_act_*`, since 2026-10-10-mcp-surface `stand_mcp_tools` …» → «… `stand_settle`, the fifteen `stand_act_*`, …» (`stand_act_select` the fifteenth, since 2026-10-20-select-by-id)."
    sidecar: "obs-plan §3: the `session_common` reader list counts fifteen `stand_act_*` (2026-10-20-select-by-id)."
    rationale: "Report, Counts / qualifiers moved, pattern `stand-act`: obs-plan.md:70 @c6267 stale, fourteen → fifteen."
    basis: "obs-plan.md:70 @c6267"
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: "§3 Observability Harness Contract → Logging stack (the headless stand `seven_guis::stand` bullet, the `session_common` reader count)"
    change: "«and for the module twenty-seven checks read — …» → «and for the module twenty-eight checks read — …», `stand_act_select` the twenty-eighth reader of `tests/blitz-tests/tests/session_common/mod.rs` (since 2026-10-20-select-by-id)."
    sidecar: "obs-plan §3: `tests/session_common/mod.rs` is read by twenty-eight checks, `stand_act_select` added (2026-10-20-select-by-id)."
    rationale: "Report, Counts / qualifiers moved: «`tests/session_common/mod.rs` readers 27 → 28 (`stand_act_select`)». The sweep's `session-common` row lists only architecture.md:363 and test-plan.md:68 / :103 as stale and marks obs-plan.md:70 @c6844 no-change, but the same line states the count in words («the module twenty-seven checks read»), which a numeral pattern does not match — a missed form of the moved count."
    basis: "obs-plan.md:70"
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: "§3 Observability Harness Contract → Logging stack (detail: the `tests/blitz-tests/tests/fixtures/mod.rs` bullet)"
    change: "«its 21 readers (as measured at escher-0.2.0/chunks/2026-10-20-load-wait/report.md)» → «its 23 readers (as measured at escher-0.2.0/chunks/2026-10-20-select-by-id/report.md; `select_interaction` and `stand_act_select` added, both silent)», the 4 print or sink sites of `stand_act_spans`' re-run children still none added."
    sidecar: "obs-plan §3: the fixtures module's readers 21 → 23, `select_interaction` and `stand_act_select` added (2026-10-20-select-by-id)."
    rationale: "Report, Counts / qualifiers moved: «`tests/fixtures/mod.rs` readers 21 → 23 (20 → 21 `stand_*` files, `font_face_sniffed_source`, and `select_interaction`, a non-`stand_*` reader)»; the sweep's `fixtures` row lists six standing sites, none in obs-plan, and the report marks obs-plan.md:74-80 no-change — but the fixtures bullet inside that range states the reader count as standing («its 21 readers»), now stale."
    basis: "obs-plan.md:74-80"
    dependent-of: D-obs-instrumentation
note: "D-obs-stack and D-obs-pii read no drift (Dependencies none; no log site, span or field added; the `change` event and the snapshot's select value cross only inside ratified carriers, never a log). No D-retired (Retired: none). Citations in obs-plan into files this chunk grew — packages/blitz-dom/src/events/pointer.rs (the link-href log sites, after 857-870), document.rs (after 717-776), mutator.rs (after 1017-1020 · 1097-1101 · 1211-1242) and node/element.rs (after 455-461) — move and are the P2 citation sweep's, not proposed here."
