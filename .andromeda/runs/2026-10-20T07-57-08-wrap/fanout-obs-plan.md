proposals:
  - detector: D-retired
    severity: warning
    section: §4 Span / Trace Coverage (escher-driver command span, its «Still observed absent» list)
    change: 'Replace «the in-process `Session::lint` — no verb, never handed to `Session::run` —» with: `Session::lint` itself opens no span, emits no event and records no field, while since 2026-10-19-visual-lint (gated) its `lint` verb, the eighth instance verb, is handed to `Session::run` and runs inside that call''s `command` span (no span of its own); `BaseDocument::text_extent` and `Harness::text_contrast` stay unspanned as stated.'
    sidecar: '§4 — the lint is no longer «no verb, never handed to `Session::run`»: the `lint` verb runs through `Session::run` inside its `command` span; `Session::lint` itself still opens none (2026-10-19-visual-lint gated, founder F1 option 1).'
    rationale: 'Report Retired: «Every master statement "in process only — no verb, `Command`, `Outcome` variant … carries it" is now false»; Counts/qualifiers `lintverb` names obs-plan.md:121 (@c5941 «the in-process `Session::lint` — no verb, never handed to `Session::run`») stale. Symbols: `Outcome::Linted`''s arm calls `Session::lint` (execute.rs:261-263), so it runs no step.'
    basis: packages/escher-driver/src/execute.rs:261-263
  - detector: D-obs-instrumentation
    severity: warning
    section: §4 Span / Trace Coverage (escher-driver command span, its recorded fields)
    change: 'In the `command` span bullet add: an accepted `lint` (since 2026-10-19-visual-lint) records `verb` alone, like `snapshot` and an accepted `screenshot`, and a refused one `verb` and `cause` — `malformed` for a call naming an argument, `fonts-unavailable` returned by the executor for `lint` too, before anything is read; add to «never fielded» a lint''s findings, their count and any finding''s id, bounds, defect or reason (F2 «Без поля (Рекомендую)», no new field — the span keeps its eight).'
    sidecar: '§4 — the `command` span''s reading for the new `lint` verb stated: `verb` alone accepted, `verb` + `cause` refused, no findings field (2026-10-19-visual-lint gated, founder F2 option 1).'
    rationale: 'Report Symbols: `Outcome::Linted` — `record_result` returns early for it as for `Screen` and `Shot` (execute.rs:276); «Its `command` span records `verb` and, refused, `cause` only — F2 option 1»; refused `fonts-unavailable` exactly as `Session::lint` is, before anything is read; an argument refused `malformed`. Schema/config: «No new … span field». Coverage: instrumentation the `command` span✓ (`verb` and `cause` only, F2).'
    basis: packages/escher-driver/src/execute.rs:276
  - detector: D-obs-instrumentation
    severity: warning
    section: §4 Span / Trace Coverage (escher-driver `tool_call` span)
    change: '«`tool`, the verb table''s word — one of its eleven, `help` among them … and `screenshot` since 2026-10-12-headless-screenshot» becomes one of its twelve, `lint` among them since 2026-10-19-visual-lint; add: a `lint` tool call ends `accepted` (its reply the JSON line `{"findings":[…]}` through the default arm) or `refused`, and parents one `command` child whose `verb` is `lint`, measured in process in both layout modes (`stand_mcp_tools::a_lint_tool_call_answers_the_readers_findings_and_parents_its_command`, tests/blitz-tests/tests/stand_mcp_tools.rs:706-766); no field added, the span keeps its two.'
    sidecar: '§4 — `tool_call`''s `tool` is one of twelve words, `lint` added; the lint tool call''s parentage of its `command` child measured (2026-10-19-visual-lint gated).'
    rationale: 'Report Symbols: MCP — a tool, its result the JSON line through the existing default arm (mcp.rs:58 rustdoc «the eight instance verbs»); Deviations: the added test `stand_mcp_tools::a_lint_tool_call_answers_the_readers_findings_and_parents_its_command` (stand_mcp_tools.rs:706-766); Counts `mcpsrv`: «obs-plan.md:122 describes the `tool_call` spans measured for `screenshot` and `help`, and adding the lint''s is new fact»; Coverage: the `tool_call` span✓ (`tool` and `ending`).'
    basis: tests/blitz-tests/tests/stand_mcp_tools.rs:706-766
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: §6 Log Coverage → Logged events (escher-driver, the `command` closed-span line)
    change: '«the verb table''s eleven words, ten until 2026-10-12-headless-screenshot and nine until 2026-10-12-self-description (seven instance-level; …)» becomes «the verb table''s twelve words, eleven until 2026-10-19-visual-lint, ten until 2026-10-12-headless-screenshot and nine until 2026-10-12-self-description (eight instance-level; …)»; add: since 2026-10-19-visual-lint a lint''s line is `span="command" verb="lint"` (with `cause` when refused) and holds no finding, id, bound or count — `host_log` and `mcp_log` each drive one `lint` at `trace` with 0 needles.'
    sidecar: '§6 — the `command` line''s verb words eleven → twelve (eight instance-level); a lint''s line stated, holding no finding (2026-10-19-visual-lint gated).'
    rationale: 'Report Counts `verbs12`: «One site outside the pattern''s wording, found by the window read: obs-plan.md:195, «the verb table''s eleven words» (stale)»; the clause''s parenthesis states the current eleven''s split (7 instance + 3 session + `help`), so with `lint` it reads eight instance-level. `logflows`: `host_log` and `mcp_log` each drive one `lint`; Coverage: «`host_log` and `mcp_log` read 0 needles with a lint on their flows».'
    basis: packages/escher-driver/src/schema.rs:310-335
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: §6 Log Coverage → Logged events (escher-driver, the `tool_call` closed-span line)
    change: '«`tool` (the table''s word, one of its eleven, `help` among them since 2026-10-12-self-description and `screenshot` since 2026-10-12-headless-screenshot)» becomes one of its twelve, adding «and `lint` since 2026-10-19-visual-lint».'
    sidecar: '§6 — the `tool_call` line''s `tool` word set eleven → twelve, `lint` added (2026-10-19-visual-lint gated).'
    rationale: 'Report Symbols: `lint` is the twelfth verb in `VERBS` (schema.rs:458-461) and an MCP tool through the table; `help`''s `verbs` 11 → 12. The `tool_call` line''s word set is the verb table''s, so its count moves with it.'
    basis: packages/escher-driver/src/schema.rs:310-335
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: §8 PII Scrubbing & Compliance → Values logged as-is (the in-process capture reading)
    change: '«the verb table holds seven since 2026-10-12-headless-screenshot, when `stand_act_spans` also began checking a refused shot''s `fonts-unavailable` cause on the span» gains: eight since 2026-10-19-visual-lint, when `stand_act_spans` also began reading an accepted lint''s line and a refused no-font lint''s line (34 calls per mode, eight verbs read back).'
    sidecar: '§8 — the instance-verb count under the in-process capture seven → eight; `stand_act_spans` reads the lint''s span lines (2026-10-19-visual-lint gated).'
    rationale: 'Report Counts `instance8` (seven → eight) and `spancalls`: «`stand_act_spans`'' calls per mode, 32 → 34, with eight verbs read back where it read seven»; «that the check also reads a lint''s line and a refused no-font lint''s line is new fact». The obs-plan.md:311 sentence states the current instance count (seven), apart from its dated «six instance-level verbs of that day», which the report keeps no-change.'
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: §3 Observability Harness Contract → Logging stack (the session library escher-driver)
    change: 'In the enumeration «`Session::run`, with the refusal detection …, its sixth verb `scroll` and, since 2026-10-12-headless-screenshot, its seventh, `screenshot`» add «and, since 2026-10-19-visual-lint, its eighth, `lint`»; beside `Outcome::Shot` state that `Outcome::Linted { findings }` is written by `to_json` alone as `{"findings":[…]}` — a command''s answer, over MCP its JSON line, kept whole in a recorded check — never logged or fielded (packages/escher-driver/src/json.rs:360-364).'
    sidecar: '§3 — the eighth instance verb `lint` under the command span; `Outcome::Linted` written by `to_json` alone, never logged or fielded (2026-10-19-visual-lint gated).'
    rationale: 'Report Symbols: `Outcome::Linted` a sixth `Outcome` variant (execute.rs:75-79); `Outcome::to_json` writes `{"findings":[…]}` (json.rs:360-364); a finding holds stable ids, closed words and numbers only; recording keeps a `lint` step''s answer whole. Counts `outcome` keeps obs-plan.md:70 @c3743 (naming `Outcome::Shot`) no-change; the verb enumeration and the Linted written form are the new fact.'
    basis: packages/escher-driver/src/json.rs:360-364
    dependent-of: D-obs-instrumentation
note: 'D-obs-stack and D-obs-pii read no drift — Dependencies none, no new log site, span or field, findings ids/closed words/numbers, host_log and mcp_log 0 needles with a lint on their flows. obs-plan.md:80 («The visual lint is silent») stands, as the report says. The report keeps obs-plan.md:195''s «(seven instance-level» no-change; this detector reads it as the current eleven''s split and moves it to eight inside the :195 proposal. The §4/§6 execute.rs and stand_act_spans line citations past the inserted lines are the citation sweep''s.'
