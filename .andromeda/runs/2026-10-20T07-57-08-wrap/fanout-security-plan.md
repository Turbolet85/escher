proposals:
  - detector: D-security-input
    severity: escalate
    section: §Input Validation → Driver command schema (escher-driver)
    change: '`validate` checks a call against a closed `''static` table of twelve verbs in three levels — the eight asked of the held instance (`snapshot` … `screenshot`, and, since 2026-10-19-visual-lint, `lint`, the eighth: no argument, any argument refused `malformed` (`Fault::Unnamed`), one result field `findings` (`FieldKind::Entries`, always present), refused `fonts-unavailable` before anything is read exactly as `Session::lint` is, running no step — a widening of this validated surface onto every ratified carrier, ratified by the founder, 2026-10-20, «Новая команда lint (Рекомендую)», relayed verbatim by the overseer); `fonts-unavailable` now refuses a `screenshot` or a `lint`; no new argument kind, cause, field kind or session error'
    sidecar: 'security-plan §Input Validation driver schema row: the verb table twelve, eight instance-level, `lint` the eighth (no argument, `findings`), validated by `validate`, ratified by the founder 2026-10-20 (F1 option 1)'
    rationale: 'Report Changes → Symbols / APIs: `lint` is the eighth instance verb and the twelfth verb, `schema::LINT` in `VERBS` after `SCREENSHOT`, no argument (an argument refused `malformed`, `Unnamed`), one result field `findings` `FieldKind::Entries`, refused `fonts-unavailable` as `Session::lint` is; Schema / config: no new cause, session error, exit status, argument kind, node key or span field; Coverage: validation `validate`✓; Counts `verbs12` · `instance8` name security-plan.md:83 stale; founder''s word inputs#I7'
    basis: packages/escher-driver/src/schema.rs:310-335; packages/escher-driver/src/command.rs:267; .andromeda/security-plan.md:83
  - detector: D-security-input
    severity: escalate
    section: §Input Validation → Driver command schema (escher-driver)
    change: '`Outcome` derives `Debug`, printing … a shot''s PNG bytes (`Outcome::Shot`) and, since 2026-10-19-visual-lint, a lint''s findings (`Outcome::Linted { findings }`: stable ids, closed defect words and numbers only — no `NodeId`, page text, colour or control value), written by `to_json` alone as `{"findings":[…]}` (a finding `id` · `defect` · `bounds` then its reason''s keys); never fielded: … a shot''s image, its size or its dimensions and a lint''s findings or their count (a `screenshot` or a `lint` records `verb` and, refused, `cause` — the founder''s F2 «Без поля (Рекомендую)», 2026-10-20)'
    sidecar: 'security-plan §Input Validation driver schema row: `Outcome::Linted` added to what an outcome holds and prints, its written form `{"findings":[…]}`, the command span of a `lint` holding `verb` and `cause` only (F2 option 1)'
    rationale: 'Report Counts `outcome`: security-plan.md:83 @c7093 stale — an `Outcome` now also holds a lint''s findings; Symbols / APIs: `Outcome::Linted` a sixth variant, `record_result` returns early for it, its span records `verb` and, refused, `cause` only (F2 option 1); the finding''s hand-written form at json.rs:255-309 and `{"findings":[…]}` at json.rs:360-364; founder''s word inputs#I7'
    basis: packages/escher-driver/src/execute.rs:75-79; packages/escher-driver/src/execute.rs:276; packages/escher-driver/src/json.rs:255-309; packages/escher-driver/src/json.rs:360-364
  - detector: D-security-input
    severity: escalate
    section: §Input Validation → Cold-agent check (cold-agent.sh, the agent-written check)
    change: '`valid_step` takes `tool` one of the twelve verbs (`lint` after `screenshot` since 2026-10-19-visual-lint, scripts/cold-agent.sh:48), the pipe''s `isolated` reading requiring twelve tools'
    sidecar: 'security-plan §Input Validation cold-agent check row: `tool` one of the twelve verbs'
    rationale: 'Report Schema / config and Harness / gate surface: the cold-agent pipe''s verb names are twelve, `lint` after `screenshot`, at scripts/cold-agent.sh:48; its `isolated` reading now requires twelve tools; Counts `verbs12` names security-plan.md:77 stale'
    basis: scripts/cold-agent.sh:48; .andromeda/security-plan.md:77
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: §Input Validation → Driver recording directory (escher-driver)
    change: 'a call no record can spell (a verb outside the eight instance verbs — `lint` among them since 2026-10-19-visual-lint, its step kept like every instance verb, its answer, the findings, kept whole — an argument its schema does not name or passes twice) is not recorded'
    sidecar: 'security-plan §Input Validation recording-directory row: the eight instance verbs, a `lint` step recorded with its findings kept whole'
    rationale: 'Report Symbols / APIs → The carriers → Recording: a `lint` step is kept like every instance verb, its answer kept whole; `recording.rs` unchanged since it reads `Level::Instance` from `VERBS`; Counts `instance8` names security-plan.md:80 stale'
    basis: .andromeda/security-plan.md:80
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: §Input Validation → Recorded check (escher-driver `Check::from_bytes`)
    change: '`tool` one of the eight instance verbs (`lint` the eighth since 2026-10-19-visual-lint)'
    sidecar: 'security-plan §Input Validation recorded-check row: `tool` one of the eight instance verbs'
    rationale: 'Report Counts `instance8` / `verbs12` name security-plan.md:81 stale; the reader takes the instance level from `VERBS`, which now holds eight instance verbs (Symbols / APIs, Recording)'
    basis: .andromeda/security-plan.md:81
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: §Input Validation → MCP server (escher-driver `serve_mcp`, the `escher-session mcp` role)
    change: '`tools/list` the twelve verbs, `help` last (`screenshot` among them since 2026-10-12-headless-screenshot, `lint` after it since 2026-10-19-visual-lint), a `lint` tool call''s result its JSON line `{"findings":[…]}` through the existing default arm'
    sidecar: 'security-plan §Input Validation MCP server row: `tools/list` the twelve verbs, `lint` answered as its JSON line'
    rationale: 'Report Symbols / APIs → The carriers → MCP: a tool, its result the JSON line through the existing default arm (mcp.rs — only the rustdoc changed, mcp.rs:58); `mcp::tests::the_listing_is_the_twelve_verbs_of_the_table`; Counts `verbs12` names security-plan.md:85 @c1482 stale'
    basis: packages/escher-driver/src/mcp.rs:58; .andromeda/security-plan.md:85
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: §API Security
    change: 'the `escher-session mcp` role serves the driver''s verbs as MCP tools — eleven since 2026-10-12-headless-screenshot and twelve since 2026-10-19-visual-lint: the tenth, `help`, …, the eleventh, `screenshot`, and the twelfth, `lint` (the founder, 2026-10-20, «Новая команда lint (Рекомендую)»)'
    sidecar: 'security-plan §API Security: the MCP role serves twelve verbs, `lint` the twelfth'
    rationale: 'Report Counts `verbs12` names security-plan.md:214 («eleven since 2026-10-12-…») stale; Symbols / APIs: `lint` the twelfth verb, an MCP tool; founder''s word inputs#I7'
    basis: .andromeda/security-plan.md:214
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: §API Security
    change: 'none of its eight instance-level verbs has an argument or a result field of a wait — the seventh, `screenshot`, …, and the eighth, `lint`, since 2026-10-19-visual-lint, taking no argument and answering `findings`, running no step and settling nothing, so it moves no time'
    sidecar: 'security-plan §API Security: the no-wait claim holds over eight instance-level verbs, `lint` included'
    rationale: 'Report Counts `instance8`: security-plan.md:223 @c1316 stale as a count — the claim holds for `lint`, which has no argument and no wait; Symbols / APIs: `Outcome::Linted`''s arm calls `Session::lint`, so it runs no step and settles nothing (execute.rs:261-263)'
    basis: packages/escher-driver/src/execute.rs:261-263; .andromeda/security-plan.md:223
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: §Logging & Monitoring
    change: 'since 2026-10-19-visual-lint the verb table holds eight instance-level verbs and eight causes; `stand_act_spans` also reads a lint''s line and a refused no-font lint''s line (`cause=fonts-unavailable`), the schema fields only, and `host_log` and `mcp_log` each drive one `lint` with 0 needles — a lint''s findings reach no log'
    sidecar: 'security-plan §Logging & Monitoring: the verb table eight instance-level verbs; the lint''s span lines read by `stand_act_spans`, its flows in `host_log` / `mcp_log` needle-free'
    rationale: 'Report Counts `verbs12` / `instance8` name security-plan.md:410 @c7002 / @c6981 stale («the verb table holds seven instance-level verbs and eight causes»); `spancalls`: `stand_act_spans` reads a lint''s line and a refused no-font lint''s line; `logflows`: `host_log` and `mcp_log` each drive one `lint`; Coverage: PII redacted✓, 0 needles with a lint on their flows'
    basis: .andromeda/security-plan.md:410
    dependent-of: D-security-input
  - detector: D-retired
    severity: escalate
    section: §Input Validation → id row (stable element id, its in-process readers)
    change: 'since 2026-10-19-visual-lint escher-driver''s `Session::lint` is a fifth reader of the id — it takes no argument, reads the snapshot and returns `Finding`s holding ids, closed defect words and numbers only (no `NodeId`, page text, colour or control value; a text control''s value never measured) — and, since its gated part, its findings cross as the `lint` verb''s answer (`Command::Lint` → `Outcome::Linted`, written `{"findings":[…]}`) on every ratified carrier — the socket''s `call`, the command line, MCP and a kept check, recorded whole — a boundary widening ratified by the founder, 2026-10-20, «Новая команда lint (Рекомендую)», relayed verbatim by the overseer; no log site, print or span field carries a finding (its `command` span `verb` and, refused, `cause` only, F2 «Без поля (Рекомендую)»), and `VERBS` holds twelve'
    sidecar: 'security-plan §Input Validation id row: the lint''s «in process only — crosses nothing … `VERBS` staying eleven» retired; its findings cross every ratified carrier as the `lint` verb''s answer, on the founder''s F1 option 1'
    rationale: 'Report Retired: the in-process-only carriage of the lint is superseded by the founder''s own word, option 1 (inputs#I7), and every master statement «in process only — no verb, `Command`, `Outcome` variant, written form, MCP tool, recorded step … carries it, `VERBS` staying eleven» is now false; Counts `lintverb` · `outcome` · `verbs12` name security-plan.md:125 @c10052/10107 and @c10372 / @c10479 stale'
    basis: .andromeda/security-plan.md:125
  - detector: D-retired
    severity: escalate
    section: §Input Validation → id row (the session''s record of ids)
    change: 'the record is fed only from snapshots the session itself takes — once `validate` has admitted a call, and, since 2026-10-19-visual-lint, by `Session::lint`, which takes no argument, whether called in process or reached through the `lint` verb after `validate`'
    sidecar: 'security-plan §Input Validation id row: the record of ids fed by `Session::lint` in process or through the `lint` verb — the «in-process» qualifier retired'
    rationale: 'Report Retired (the in-process-only carriage); Counts `lintverb` names security-plan.md:125 @c14546 («the in-process `Session::lint`») stale; Symbols / APIs: the verb''s arm calls `Session::lint` (execute.rs:261-263)'
    basis: packages/escher-driver/src/execute.rs:261-263; .andromeda/security-plan.md:125
    dependent-of: D-retired
  - detector: D-retired
    severity: escalate
    section: §Input Validation → Layout values · Text extent (`BaseDocument::text_extent`)
    change: 'it returns geometry only, and its one non-test caller is `Session::lint` — called in process and, since 2026-10-19-visual-lint''s gated part, through the `lint` verb''s arm on every carrier'
    sidecar: 'security-plan §Input Validation text-extent row: its caller `Session::lint`, the «in-process» qualifier retired'
    rationale: 'Report Counts `lintverb`: security-plan.md:137 «its one non-test caller is the in-process `Session::lint`» — the caller stands, its in-process qualifier is stale; Expected amendments: security-plan.md:137 stale (`lintverb`); Retired names the in-process-only carriage'
    basis: .andromeda/security-plan.md:137
    dependent-of: D-retired
note: 'D-security-auth and D-security-deps hold — the report touches no identity, token, key or secret and its Dependencies bullet reads none. The report lists security-plan.md:125 @c9112 «meaningful in process only» under `lintverb`, but that clause qualifies `DioxusDocument::unkeyed_actionable`''s process-local `NodeId` field, not the lint, and still holds — no proposal. The MCP row''s listing size (2,984 → 8,844 bytes) likely moved with `lint`''s help text; the report states no new figure, so none is proposed.'
