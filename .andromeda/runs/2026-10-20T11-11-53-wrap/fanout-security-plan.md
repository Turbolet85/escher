proposals:
  - detector: D-security-input
    severity: escalate
    section: §Input Validation → Driver command schema (escher-driver)
    change: "The row's «six argument kinds, each with its bound» becomes seven: after `name` add «and, since 2026-10-20-large-screen-in-parts, the seventh, `part`, a whole number from 1 to 1000 (`MAX_PART`), taken through `u32::try_from` — `snapshot`'s one optional argument, any other number `malformed` (`OutOfBound`), a text or flag value `malformed` (`WrongKind`)», the argument count beside it gaining `snapshot`'s `part`; `snapshot` (among the seven asked of the held instance) takes that optional `part` and answers `text`, then `part` and `parts` (counts, `always: false`) — a screen over `SNAPSHOT_TEXT_BUDGET` read in parts, from part 1 with no `part`, a part past the last the empty text with no new cause, a part read recording the whole screen's ids as `snapshot` does — and `Outcome` gains `Outcome::Part { text, part, parts }`, written by `to_json` alone as `{\"text\":…,\"part\":n,\"parts\":m}`; in the never-fielded list add «a part's number or count» beside «the screen text or its length» (a part read records `verb` alone) — a widening of this validated surface on every ratified carrier, ratified by the founder (2026-10-20, F1 «Аргумент part, части по умолчанию (Рекомендую)» · F2 «Пустая часть (Рекомендую)»)."
    sidecar: "security-plan §Input Validation (Driver command schema): the seventh argument kind `part` (1 to `MAX_PART` 1000) on `snapshot`, its paged answer `Outcome::Part` `{text,part,parts}`, the span fielding no part number or count — 2026-10-20-large-screen-in-parts, founder F1/F2."
    rationale: "The report's Changes add a new external-input value on every carrier (socket `call`, command line `--part`, MCP `{\"part\":n}`, a kept check's `arguments`): `ArgKind::Part` the seventh kind, `MAX_PART` 1000, `validate` admitting `1..=MAX_PART` via `u32::try_from`, else `OutOfBound`/`WrongKind` → `malformed` (Symbols / APIs; Coverage of new surfaces). It is validated, so no unvalidated boundary — but the §Input Validation row that mandates and records the mechanism still says «six argument kinds» (Counts moved: `kinds` security-plan.md:83 @c2129 changed) and lists no `part`; the span list «no field for a part's number, count, size or text» (Schema / config). Founder's word inputs#I2 (Decisions & corrections)."
    basis: .andromeda/security-plan.md:83
  - detector: D-security-input
    severity: escalate
    section: §Input Validation → Markup attributes (`aria-selected` · `aria-pressed` · `aria-checked` · `aria-invalid`)
    change: "«a `snapshot` result is `to_text`'s output and an acting call's result its JSON line» becomes «a whole-screen `snapshot` result is `to_text`'s output, a paged one (since 2026-10-20-large-screen-in-parts) its JSON line `{\"text\",\"part\",\"parts\"}` holding the same lines, and an acting call's result its JSON line» — the states crossing as the same closed words and booleans either way."
    sidecar: "security-plan §Input Validation (Markup attributes, states row): the MCP `snapshot` result is two-formed — a whole screen `to_text`, a paged one its JSON line — 2026-10-20-large-screen-in-parts."
    rationale: "Report Counts / qualifiers moved, `mcpsnap`: security-plan.md:129 @c4031 «a `snapshot` result is `to_text`'s output», changed — a paged result reaches mcp.rs's generic arm and is the JSON line `{\"text\",\"part\",\"parts\"}` (Files; founder F1 «the paged MCP answer as its JSON line», inputs#I2)."
    basis: .andromeda/security-plan.md:129
  - detector: D-security-input
    severity: escalate
    section: §Input Validation → Markup attributes (`id`, stable element id)
    change: "«an accepted `snapshot` the screen text itself, any other accepted call its JSON line» becomes «an accepted `snapshot` the screen text itself — a paged one, since 2026-10-20-large-screen-in-parts, its JSON line `{\"text\",\"part\",\"parts\"}` holding the same text — any other accepted call its JSON line»."
    sidecar: "security-plan §Input Validation (stable element id row): the MCP `snapshot` result's paged form, its JSON line — 2026-10-20-large-screen-in-parts."
    rationale: "The same claim the `mcpsnap` amendment retires at :129, restated in the `id` row's list of the id's exits (MCP tool results); the report's mcpsnap pattern caught only :129, while the report states the MCP `snapshot` result is now two-formed, a paged one its JSON line (Counts / qualifiers moved, `mcpsnap`)."
    basis: .andromeda/security-plan.md:125
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: §Logging & Monitoring
    change: "«`host_log` and `mcp_log` each driving one `lint` with 0 needles» becomes «`host_log` and `mcp_log` each driving one `lint` and, since 2026-10-20-large-screen-in-parts, one `snapshot` part read with 0 needles», so a part's number, count and text reach no log either (a part read's `command` line holds `verb` alone, `stand_act_spans` 35 calls per mode)."
    sidecar: "security-plan §Logging & Monitoring: the sink's needle census drives one part read in `host_log` and `mcp_log`, 0 needles; a part read's span records `verb` alone — 2026-10-20-large-screen-in-parts."
    rationale: "The new `part` input surface's instrumentation witness. Report Counts / qualifiers moved, `logflows`: security-plan.md:410 @c7450/@c7465 «each driving one `lint` with 0 needles», changed to add the part read; Outcome (obs): `host_log` and `mcp_log` drive one part read at `trace` with 0 id and 0 name needles under both builds, and the part read's `command` line holds `verb=\"snapshot\"` and no other field."
    basis: .andromeda/security-plan.md:410
note: "D-security-auth and D-security-deps hold (no identity/session-token/key change; Dependencies: none). Retired: none. The :83 row's «(eight arguments across the table)» count: the report says the span unit test's argument-count pin moved 9 → 10 («the new `part` is the tenth argument name in VERBS», execute.rs:600) — the orchestrator should measure the table's argument count before writing the new figure, since eight + one and the pin's ten disagree."
