# Cascade dispositions — 2026-10-10-mcp-surface (step 2 sweep)

Listing: `cascade.py sweep --patterns-file cascade-patterns.toml` (cascade v1.2), baseline 69a399da (the parent of the
oldest pre-CI commit 36877a23), run after the last body amendment of the pass. Copied from its last two lines:
`total (17 patterns) · 23 rows over 12 files` · `per class · new 0/0 · standing 9/4 · leaf 14/8 · curation 0/0 · base 0/0`.

## The search
Seventeen patterns (`cascade-patterns.toml`), each the retired claim's own wording and its verbs, every control fired on
the pre-pass masters (the first run refused `host-log-one`'s first spelling, its control unfired — respelled to the
master's own `the check that reads a sink-installing`, then fired):
`one-cmd-span` (its / the one (command) span) · `no-mcp` (no MCP command / tool, MCP tool does yet, still no MCP) ·
`re-exports-40` · `eleven-modules` · `three-crates` (three crates, third dependency) · `five-targets` (five integration
targets / seven_guis, the five share, by all five, three more seven_guis) · `unit-50` · `two-exits` · `one-writer`
(writes one thing, one writer, in one place only) · `no-serde` · `cold-agent-only` · `span-is-run` · `one-span-macro` ·
`mcp-to-come` (the MCP surface still to come) · `one-exception` (the stand checks' one subscriber exception) ·
`host-log-one` · `one-listener-mcp` (the MCP surface). Read beside the listing, every amended line re-read for an
intra-line duplicate: none left (the two `edited` rows of architecture.md:133 `no-serde` are a true claim — the three
schema modules use no serialization crate — and the pass's own new text, "no serialization crate writes one").
Zero-row patterns (control fired): `re-exports-40`, `eleven-modules`, `cold-agent-only`, `one-span-macro`,
`one-exception` — each retired wording gone from every master and leaf.

## Rows — masters (standing 9 over 4 files)
- architecture.md:263 `one-cmd-span` — "the one command span a call leaves", `stand_act_spans`' reading of one span per
  call → no change: a true claim sharing the token (one `command` span per call stands).
- obs-plan.md:70 `one-cmd-span` — "`execute` opens the one span of a command" → no change: true (`execute` opens the
  `command` span; `tool_call` is `mcp`'s).
- a11y-plan.md:14 `three-crates` — "the library's third dependency, `tracing`, taken at 2026-10-07-driver-command-spans"
  → no change: a dated history clause, true; the fourth is the pass's own sentence after it (R4).
- obs-plan.md:70 `five-targets` — "since that chunk three more seven_guis targets spawn" → no change: driver-cli's dated
  clause, true; the pass's own "two more … the seven share" follows it (O9).
- security-plan.md:125 `two-exits` — the control-state readings' "two exits, … the platform accessibility adapter … and
  the snapshot" → no change: MCP results carry the snapshot exit's own text (the row's S5 sentence says so); no third
  exit of the readings.
- architecture.md:133 `no-serde` ×2 — "the three schema modules … use no serialization crate" (true) and the pass's own
  "no serialization crate writes one" (true) → no change.
- obs-plan.md:106 `span-is-run` — "the span is `run`'s, beside the MCP server's `tool_call`" → amended this pass (O1);
  no change.
- obs-plan.md:70 `host-log-one` — "it is the check that reads a sink-installing host at `trace`" (the host role) →
  no change: true for the host role; the pass's O9 sentence adds `mcp_log` and retires "the one check".
- architecture.md:87 `one-listener-mcp` — "ahead of the MCP surface" (the founder's 2026-10-10 ruling's dated
  sequencing) → no change: history, true.

## Rows — leaves (14 over 8 files) → cascade step 3 re-derives each
- `.claude/docs/conventions.md:26` one-cmd-span · `.claude/docs/services/escher-driver.md:11` one-cmd-span + :11
  three-crates + :12 no-serde + :12 one-listener-mcp + :58 unit-50 · `.claude/docs/services/escher-telemetry.md:32`
  one-cmd-span · `.claude/docs/security-summary.md:22` and `:31` no-mcp · `.claude/docs/tests-summary.md:24` unit-50 ·
  `.claude/rules/security.md:22` one-writer · `.claude/docs/obs-summary.md:49` one-writer · `CLAUDE.md:108`
  mcp-to-come + one-listener-mcp → **re-derived** (step 3).

## Curation homes and judgment bases
0 rows in `USER:session-learnings`, the rule files' `## Session Additions`, `docs/session-learnings.md`, `playbook.md`,
`drift-base.md`.

## Step 3's leaf set
The floor table, by the amended masters: CLAUDE.md `GENERATED:setup:*` (architecture) · `docs/stack.md` (§Stack) ·
`docs/conventions.md` (§Conventions) · `docs/commands.md` (§Standard Contracts) · `docs/services/{escher-driver,
seven_guis,escher-telemetry,dioxus-native-dom}.md` (the modules amended) · `docs/security-summary.md` +
`rules/security.md` (security-plan) · `docs/tests-summary.md` + `rules/testing.md` (test-plan and its key file) ·
`docs/obs-summary.md` + `rules/observability.md` (obs-plan) · `docs/a11y-summary.md` + `rules/a11y.md` (a11y-plan) ·
`docs/design-summary.md` (design-system, layout-templates) — and the sweep's rows above, all inside it.
