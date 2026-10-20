proposals:
  - detector: D-obs-instrumentation
    severity: warning
    section: '§4. Span / Trace Coverage (the escher-driver `command` span bullet, obs-plan.md:121)'
    change: 'Beside «`snapshot` ... record `verb` alone», state that since 2026-10-20-large-screen-in-parts a part read — `snapshot` with or without `part` answering `Outcome::Part { text, part, parts }` — records `verb` alone too (`record_result` puts `Outcome::Part` in the verb-alone arm), the span keeping its eight fields; add to the never-fielded list a part number, its `parts` count, a part size or its text; and add `Snapshot::text_parts` / `TextPart` (dioxus-native-dom, pure and in process) to the observed-absent list as opening no span, emitting no event and recording no field, escher-driver keeping its 2 `info_span!` sites.'
    sidecar: '§4 command span: a part read (`Outcome::Part`) records `verb` alone; part number, `parts`, size and text never fielded; `Snapshot::text_parts` opens no span (2026-10-20-large-screen-in-parts).'
    rationale: 'The report adds a new operation on the command path, the paged `snapshot` (Symbols / APIs: `Outcome::Part`, execute.rs:32-41, :188-204; `record_result` execute.rs:308-313), and states under Schema / config that the `command` span records `verb` alone for a part read with no field for a part number, count, size or text, 2 `info_span!` sites kept (gate entry 9 → 2); Expected amendments names «obs-plan §4 (a part read records `verb` alone): carried». §4 names `snapshot`, `screenshot` and `lint` but not the part read nor its never-fielded values.'
    basis: 'packages/escher-driver/src/execute.rs:308-313'
  - detector: D-obs-instrumentation
    severity: warning
    section: '§6. Log Coverage → Logged events → escher-driver, the `command` closed-span line (obs-plan.md:195)'
    change: 'After «a lint''s line is `span="command" verb="lint"` ... with 0 needles», add: a part read''s line is `span="command" verb="snapshot"` (with `cause` when refused, e.g. a `part` outside 1 to 1000 refused `malformed`) and holds no part number, `parts` count, size or text — `host_log` (seven calls, 7 command-span lines) and `mcp_log` (ten tool calls, 8 `command` lines) each drive one part read at `trace` with 0 id and 0 name needles, under both builds.'
    sidecar: '§6 escher-driver log lines: the part read''s `command` line (`verb="snapshot"` alone) and its 0-needle reading in `host_log` / `mcp_log` at `trace` (2026-10-20-large-screen-in-parts).'
    rationale: 'The §6 line enumerates each verb''s span line (shot, lint) as §4 does; the report''s Counts / qualifiers moved (logflows) says `host_log` and `mcp_log` each drive one part read, `host_log` sending seven calls and reading 7 command-span lines, `mcp_log` ten tool calls with 8 `command` lines, and Outcome (obs) says a part read''s `command` line holds `verb="snapshot"` and no other field, 0 id and 0 name needles at `trace` under both builds. The sweep''s no-change at :195 is for the «each drive one `lint`» wording, which still holds; the part read is a second enumerated line missing beside it.'
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: '§8. PII Scrubbing & Compliance → Values logged as-is (obs-plan.md:311)'
    change: '«(34 calls per mode, eight verbs read back)» → «(35 calls per mode, eight verbs read back — since 2026-10-20-large-screen-in-parts one `snapshot` with `part` 1 per mode among them, its line `verb` alone)».'
    sidecar: '§8: `stand_act_spans` calls per mode 34 → 35, one part read added (2026-10-20-large-screen-in-parts).'
    rationale: 'Report Counts / qualifiers moved: «`stand_act_spans` calls per mode: 34 → 35 (one `snapshot` with `part` 1 per mode, asserted `run.calls == 35`) — `spancalls`: 2 standing, both changed: test-plan.md:27 @c31178, obs-plan.md:311 @c2334».'
    basis: 'obs-plan.md:311'
  - detector: D-obs-stack
    severity: warning
    section: '§3. Observability Harness Contract → Logging stack (subscriber installation), the `<!-- detail: absence · logs -->` list'
    change: 'Add one bullet: «The parts check is silent: `stand_snapshot_parts` (a `fixtures` and `session_common` reader, both layout modes, in process and through `serve_mcp`) installs no subscriber, reads no env var and prints nothing, its readings going into assertions only — the silence census reading 0 over it and the print/log census 0 over `tests/blitz-tests/tests/session_common/mod.rs`, whose `snapshot_part` and `part` add none; dioxus-native-dom''s `Snapshot::text_parts` / `TextPart` hold no log, print or failure path, and `Outcome::Part { text, part, parts }` is written by `to_json` alone as `{"text":…,"part":n,"parts":m}` — a command''s answer, over MCP its JSON line — never logged or fielded; no log site, span, field, log field or env read added, escher-driver keeping its 2 `info_span!` sites (as measured at escher-0.2.0/chunks/2026-10-20-large-screen-in-parts/report.md).»'
    sidecar: '§3 Logging stack: the parts check `stand_snapshot_parts` silent; `text_parts` and `Outcome::Part` never logged (2026-10-20-large-screen-in-parts).'
    rationale: 'Each new check family and output form gets its silence statement in this list (load wait, select, lint). The report adds the new target `stand_snapshot_parts` (Harness / gate surface), whose gate entries read the silence census on it 0 and the print/log census on session_common/mod.rs 0; `Snapshot::text_parts` «has no document access, failure path, log or print» (snapshot_text.rs:59-81); `Outcome::to_json` writes `Outcome::Part` (json.rs:330-336); escher-driver keeps 2 `info_span!` sites.'
    basis: 'packages/escher-driver/src/json.rs:330-336'
  - detector: D-obs-stack
    severity: warning
    section: '§3. Observability Harness Contract → Logging stack (subscriber installation), the headless-stand bullet (obs-plan.md:70)'
    change: '«the module twenty-eight checks read — `stand_load_wait`, `stand_lint`, …» → «the module twenty-nine checks read — … and, since 2026-10-20-large-screen-in-parts, `stand_snapshot_parts`», the module still installing no subscriber and reading no env var.'
    sidecar: '§3: `session_common` readers 28 → 29 (`stand_snapshot_parts`) (2026-10-20-large-screen-in-parts).'
    rationale: 'Report Counts / qualifiers moved, Shared-module readers: «`tests/session_common/mod.rs` readers go 28 → 29 … since `stand_snapshot_parts` declares both». The readers sweep listed architecture and test-plan sites only; obs-plan states the same count spelled out and is missed by it.'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§3. Observability Harness Contract → Logging stack, the `<!-- detail: absence · logs -->` bullet on the third shared module `tests/blitz-tests/tests/fixtures/mod.rs`'
    change: '«its 23 readers (as measured at escher-0.2.0/chunks/2026-10-20-select-by-id/report.md)» → «its 24 readers (as measured at escher-0.2.0/chunks/2026-10-20-large-screen-in-parts/report.md), `stand_snapshot_parts` the twenty-fourth», the 4 print or sink sites of `stand_act_spans`'' re-run children unchanged.'
    sidecar: '§3: `fixtures` readers 23 → 24 (`stand_snapshot_parts`) (2026-10-20-large-screen-in-parts).'
    rationale: 'Report Counts / qualifiers moved, Shared-module readers: «`tests/fixtures/mod.rs` readers 23 → 24 (`stand_*` 21 → 22), since `stand_snapshot_parts` declares both». The readers sweep did not list this obs-plan site; the report states no line for it.'
    dependent-of: D-obs-stack
note: 'D-obs-pii holds: the report adds no log site or field; a part read logs `verb` alone, a password reads `MASKED_VALUE` in every part and the typed sentinel occurs 0 times across a walk (stand_snapshot_parts), and host_log / mcp_log read 0 id and 0 name needles at trace with a part read driven. No Retired items. The readers sweep missed two obs-plan count sites (session_common «twenty-eight» spelled out, fixtures «23 readers»), proposed above as D-obs-stack dependents.'
