# Cascade dispositions — 2026-10-07-settle-detection

Written from the listing `cascade.py sweep` printed after every body of this pass was applied (trail
`cascade-2026-10-07-settle-detection.json`; patterns `cascade-patterns.toml`, 20 patterns; baseline `4ed27b53`, the
parent of the pre-CI commit). Every pattern's known-positive control fired on the pre-pass masters. The search was the
amended claims' own wordings: the retired "Not built … settle" clause, every mention of settle, the four-reader claim
about `mod session_common;` in its three wordings, the old coordinates of every citation into the five edited files,
the two count chains' last links, the timer's "next pump", the harness's export, module and capability sentences, the
session's method list, the `SessionError` variant list, the manual net provider, the request-timeout row, the
keeps-animating set and the harness clock, and claims about the changed set on a fresh document. NOT searched: the
bare numbers 142, 72 and 19 (overwhelmingly line numbers inside citations — the report dispositions them hit by hit),
and `session_common` at large (26 occurrences, most of them citations of a module this chunk did not edit).

## Masters and key files

| Pattern | Rows | Disposition |
|---|---|---|
| `notbuilt` | 0 in masters | the clause is gone from architecture; 1 leaf row, below |
| `settle` — new 30 | architecture 115 · 130 · 255 · 257 · 263; security-plan 202; design-system 288 · 289; test-plan 19 · 21 · 26 · 47 · 65 · 87 · 88 · 100 · 115 · 141 · 142 · 179 · 251 · 269 · 270 · 280 · 324; obs-plan 69 · 105; a11y-plan 37; the key file `session-lifecycle.md` 5 · 11 | this pass's own text, each read back after its edit |
| `settle` — standing 3 | architecture 129 (`ResizeSettleCheck`, a shell event) · 150 ("resize settling", the shell's window resize) · 133 (`edited`, the Driver session contract) | 129 and 150: another sense of the word, no change. 133: the amended line; its one pre-pass match was the retired clause, now restated — re-read whole, no second statement of it on the line |
| `readers4` — standing 1 | architecture 263 (`edited`) | amended: "by four of them and by `stand_settle`" — the swept words stand inside the new, true sentence |
| `onlyshare` · `theirmod` | 0 in masters | both wordings are gone (architecture 115, obs-plan 69) |
| `cite-harness` · `cite-hlib` · `cite-driver` | 0 rows | no old coordinate into the five edited files stands anywhere — masters, key files, leaves, curation homes or judgment bases |
| `ws-count` — standing 1 | test-plan 324 (`edited`) | the chain's earlier link (142 · 579 · 8 at sink-target-allowlist) stands as history; the new link follows it |
| `stand72` — standing 1 | test-plan 115 (`edited`, ×2) | the chain's earlier link (72 at driver-session), history; the new link follows it |
| `nextpump` | 0 in masters | restated at test-plan 280 |
| `exports` · `modules` · `pumptick` | architecture 130 · 255, test-plan 47 (each `edited`) | amended: the list continues with the four new exports, the module list with `settle`, the capability sentence with settle |
| `hmut` — standing 2 | architecture 133, the key file line 5 (both `edited`) | amended: `act` follows the two accessors |
| `errlist` — standing 1 | architecture 133 (`edited`) | amended: `NotSettled(Busy)` follows `Io` |
| `manualnet` — standing 1 | test-plan 269 (`edited`) | amended: the second file-private provider named |
| `reqtimeout` — standing 1 · new 1 | security-plan 201 · 202 | 201: the row about blitz-net's absent request timeout — a true claim, untouched. 202: this pass's row |
| `animset` — standing 2 | design-system 288 · 289 (both `edited`) | amended: the settle rule follows the set; the clock citation re-pointed |
| `freshdoc` — standing 1 · new 1 | test-plan 12 · a11y-plan 37 | test-plan 12 states the change flag reads false on a fresh blitz-dom document — a bare `BaseDocument` in a unit test, before anything is written; a11y-plan 37 (this pass) states it reads true on a freshly BOOTED harness document, whose boot wrote into it. Two subjects, both true; no change to line 12 |

## Curation homes and judgment bases

0 rows for every pattern in CLAUDE.md's session learnings, the rule files' `## Session Additions`,
`docs/session-learnings.md`, `playbook.md` and `drift-base.md`.

## Leaves — each row adds its leaf to the re-derivation, none stands in for it

| Leaf row | Pattern | Disposition |
|---|---|---|
| `.claude/docs/services/escher-driver.md` 6 · 15 · 18 | `notbuilt` · `settle` · `hmut` · `errlist` | re-derived: settle is built in process (`act`), the method list and the variant list restated |
| `CLAUDE.md` 103 | `settle` | re-derived: the Architecture paragraph's "still to come — settle detection" |
| `.claude/rules/observability.md` 36 | `settle` | read: "one span per driver command (settle wait, diff size, refusal cause)" owed by "Driver command spans" — still true; re-derived with its section |
| `.claude/docs/services/blitz-shell.md` 14 | `settle` | `ResizeSettleCheck` — another sense, no change |
| `.claude/rules/testing.md` 15 · 18 | `onlyshare` · `manualnet` | 15 re-derived (the session checks no longer the only readers); 18 is the list of hand-written fakes, still true |
| `.claude/docs/tests-summary.md` 13 · 29 | `pumptick` · `ws-count` · `stand72` | re-derived: settle beside pump and tick; the two counts |
| `.claude/docs/services/seven_guis.md` 21 | `nextpump` | re-derived |
| `.claude/docs/services/blitz-test-harness.md` 6 · 14 | `pumptick` · `exports` | re-derived |
| `.claude/rules/security.md` 13 | `reqtimeout` | blitz-net's absent request timeout — a true claim, untouched by this pass; no change (the new settle row adds no rule a contributor must follow; `security-summary.md` carries it) |

Beyond the rows the sweep printed, the leaves re-derived for the amended sections: `CLAUDE.md` (the modules lines of
blitz-test-harness and escher-driver, two pointer-table rows, the Architecture paragraph), `.claude/docs/services/`
`blitz-test-harness.md` · `escher-driver.md` · `seven_guis.md`, `.claude/docs/tests-summary.md`, `obs-summary.md`,
`security-summary.md`, `a11y-summary.md`, `.claude/rules/testing.md` · `verification-harness.md` · `observability.md`.
Read and left: `design-summary.md` (it carries no motion or animation-runtime statement — its motion line is a
not-yet-measured marker about tokens), `.claude/rules/a11y.md` (its tree-refresh line states the shell's refresh, which
this pass did not change), `docs/commands.md`, `conventions.md`, `stack.md`, `gotchas.md`, `workflow.md` (grep for the
harness, the driver and the shared session module: no statement an amendment touched).

## The lateral binds

- test-plan §3 ↔ obs-plan §3 (the harness commands, status shape and log format): neither side's harness surface
  moved — no agent-run verb, status shape or log line changed; both now state that `stand_settle` and the settle loop
  install no subscriber and print nothing.
- a11y-plan schema ↔ obs-plan schema: neither moved.
