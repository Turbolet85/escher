# Boundary widening — answer card (2026-10-19-session-recording)

## The answers it rests on
- **inputs#I4** — the founder's word, from the dialog in the overseer session at 05:02Z on 2026-10-19, relayed by the operator:
  «Поставить (Рекомендую)» (the windows-gnu target), and on record for this wrap: Session recording **F1 A** and **F2 B**
  are ratified as his, and **F3 widens to B in «Recorded-check replay», not in this chunk**.
  - F1 A — recording is off unless the launcher names a directory (`--record <dir>` on `start`, the host role, the `mcp`
    role): created `0700` or refused when open, one new `0600` check per session; no tool call, verb, argument or
    result field names it.
  - F2 B — text typed into a masked control is replaced by one fixed stand-in (`MASKED_STAND_IN`, `"masked"`) and the
    step flagged `masked:true`; no byte, length or digest of the secret is kept.
  - F3 C (his word: it stands for this chunk, its widening to B owed at «Recorded-check replay» by a CARRY) — each
    answer kept as a fixed FNV-1a 64 digest only; no screen reading at rest.
- **inputs#I3** (the operator at P5) — the calls kept as they crossed are the F1/F2 widening itself, listed by name:
  **call target ids · the `press` key · the `advance` ms · text typed into unmasked inputs**.
- **inputs#I5** (the operator's resume directive) — the card cites I4; F1 A, F2 B and the kept call content are his and
  F3 C stands as the interim form on his word; no PROVISIONAL mark for these.

## The proposals read as lying wholly inside those answers
(`{doc} {n}` as `fanout-results.md` numbers them)
- architecture 1 — §Occupied Resources → Filesystem: the recording directory and its `check-{n}.json`
- architecture 3 — §Standard Contracts → Driver session: `recording_into` · `recording_failed` · `Check` · `Step` · `answer_digest` · `MASKED_STAND_IN`, the kept-check form
- architecture 9 — §Standard Contracts → Driver session: `--record <dir>` in the command line reader, the `start` spawn, the usage line
- architecture 10 — §Standard Contracts → CLIs: `[--record <dir>]` on `start`, the host role, the `mcp` role
- architecture 11 — §Standard Contracts → Driver session: what a session holds gains its recording
- architecture 12 — §Standard Contracts → Driver session: «the one thing the session keeps between calls» restated beside the kept check
- architecture 16 — §Standard Contracts → Dioxus DOM bridge: «no log, event or file carries the snapshot or its text» → a kept check carries call content, no screen reading
- architecture 22 — §Established Decisions → [Driver session]: the owner-only replayable check, call content and answer digests, never logged
- security-plan 1 — §Input Validation: the kept-check reader `Check::from_bytes` row (bounded closed grammar, `malformed`)
- security-plan 2 — §Input Validation: the recording directory row (`0700`, `record-dir-not-private`, `0600` checks, never replaced)
- security-plan 3 — §Input Validation: CLI arguments row, `--record <dir>`
- security-plan 4 — §Threat Model Summary: the host role's shape gains `[--record <dir>]`
- security-plan 9 — §Input Validation: password row, a masked or unresolved `type` kept as the stand-in
- security-plan 10 — §Input Validation: id row, «no log, event or file…» restated for the kept check
- security-plan 11 — §Input Validation: id row, the kept check as an exit of call target ids
- security-plan 12 — §Input Validation: id row, the second exit list
- security-plan 13 — §Data Protection → At rest: the kept check as a medium
- layout-templates 1–5 — §Surface: cli and §Surface: desktop-native → Primary screens: `--record` on `start`, the host role and the `mcp` role, its endings
- test-plan 3 — §3 → Session lifecycle `session-binary`: `--record` and its edges
- test-plan 4 — §3 → Session lifecycle `session-start`: `recording_into` and the kept check
- obs-plan 9 — §9 Telemetry artifact handling: the recording-directory row

25 proposals. None is only partly inside; no delegate's answer or direction covers any of them; no playbook rule rides
this card.

## The answer
_(appended as given, verbatim when relayed)_

Confirm all 25 (Recommended) — «The founder confirmed by dialog in the overseer session, relayed by the overseer:
«Подтвердить все 25 (Рекомендую)».» (the operator's answer to the card's question, 2026-10-19)
