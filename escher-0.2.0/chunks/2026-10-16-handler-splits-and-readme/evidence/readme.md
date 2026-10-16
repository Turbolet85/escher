# README — before and after (step 6)

The subject: `README.md`, rewritten to the current state on the founder's word (inputs#I2, «чтоб эшер там с большой
буквы писался») and the operator's directive (inputs#I1). Before: the file at `d236e9186e4ff6ca2610ddfbc9f54f590e8dd01b`
(74 lines). After: the working tree at this run.

## Removed

- `# escher` and `## What escher is` — the title and heading now read `# Escher` and `## What Escher is`.
- Lowercase «escher» in prose at lines 1, 7, 9, 17 (twice), 59, 65, 69 and 73 — each now «Escher». Code spans
  (`escher-session`), the badge and repository URLs and the e-mail address are unchanged.
- "The screen snapshot, the agent driver and its command-line and MCP tools are being built (see Plans)" (line 9) —
  replaced by the present-tense statement of what 0.1.0 built: the stable ids, the snapshot and its diff, acting by
  id, the command line and MCP tools, `help`, and screenshots with no display.
- `## Plans` with "Version 0.1.0 is built in six stages:" (line 52) — replaced by `## Status`.
- "Beyond 0.1.0: a graph of components, handlers and state; virtual time; fuller accessibility names and states;
  visual lint; turning traces into tests; automatic exploration; handler coverage; component isolation." (line 61) —
  it named items 0.2.0 now holds (virtual time, visual lint, turning traces into tests); replaced by a `Later:` line.

## Added, each with its source

- `### 0.1.0 — done and verified`, its six stages kept as the 0.1.0 page listed them (past tense; «Escher» in the
  sixth), then three recorded facts:
  - "All 17 of its capabilities are verified headlessly on the 7GUIs stand" — `matrix.py coverage --dir escher-0.1.0`,
    re-read at this run: `verified 17/17 · deferred 0 · planned 0 · implemented 0 · unclaimed 0`, `done-test: YES`.
  - "A fresh agent given only the MCP tools drove the four lean tasks in 43 calls, with 0 wrong" —
    `escher-0.1.0/chunks/2026-10-15-cold-agent-test/report.md:263-264`, re-read at this run ("43 tool calls with 0
    wrong"). No other figure of that run is named.
  - "The version-end audit's mutation testing left 0 surviving mutants of 1,558" —
    `.andromeda/runs/2026-10-15T10-44-59-code-audit/proposals.md:73-88`, re-read at this run: six units of 33 · 228 ·
    239 · 403 · 525 · 130 mutants (sum 1,558), 0 missed, "Survivors — 0, complete".
- `### 0.2.0 — planned, in five stages` — one line per epoch of `escher-0.2.0/working-route.md`, the names verbatim:
  Foundation · Engine debts · Determinism and traces · Visual lint and real-app readiness · Polish & ship, each with
  what its entries build in plain words.
- `Later:` — from `escher-0.2.0/intent.md` §Out of this version (line 89-90): the UI graph, layout explained,
  auto-exploration, handler coverage, component isolation.

## Kept

Why it exists · Built on Blitz · Try it (every command unchanged; each `escher-session` line fits `<verb> [arguments]
--session <dir>`, `help` or `mcp`) · the CI badge, the one image · License · Author / Contact.

## What it names

No secret, login or credential word; no path outside the repository; nothing of the cold-agent run but its 43 calls
and 0 wrong.
