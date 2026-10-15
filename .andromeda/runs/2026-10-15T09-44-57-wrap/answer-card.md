# Answer card — 2026-10-15-cold-agent-test: the swap crossing (P2 Validate)

**Class.** Playbook `.andromeda/playbook.md:36`, Boundary widening — never routine. The chunk swapped the cold-agent
pipe from its toy stub to the driver's own MCP surface, so:
- the live run carries the stand's real screen to the model provider — element ids, accessible names and control
  values in snapshot texts and diffs, over 43 tool calls, no `screenshot` called;
- the model's reply comes back as a new input class, the agent-written JSON check, which the pipe parses and replays
  on two fresh `escher-session mcp` children it spawns.

**His answers, on record before this Validate.** These are the founder's answers to the P4 forks of this chunk
(`plan.md`, Goal and Implementation notes). He answered by question dialog in the overseer session on 2026-10-15,
and the overseer relayed them verbatim as his chosen option labels; the English wording is the overseer's
(inputs#I2, `escher-0.1.0/chunks/2026-10-15-cold-agent-test/inputs/I2-cold-agent-founder-answers.md.txt`; as the
answer arrived, inputs#I3):
- Q1, how and where the live cold agent runs, and its cost — «Один запуск, до $3 (Рекомендую)»: one run, on the dev
  host, with the operator's own login, at most $3.
- Q3, the form of the agent's check — «JSON-проверка с повтором (Рекомендую)»: a JSON list of tool calls the pipe
  replays on a fresh session with no model.
- (Q2 «Все четыре в одной сессии» and Q4 «Отложить на 0.2.0, наблюдать (Рекомендую)» shape the run and the residuals
  line; neither is a crossing.)

**What it reads, measured.** The one live run, fired once on the operator's word (inputs#I7), read `passed`, with 0
wrong calls, 0.3197 USD and 48.8 s (`evidence/live-run.md`, `live-verdict.json`). The check is bounded at 65,536 bytes
and 400 steps, with closed keys and the eleven verbs, and each step is validated again by the server on replay. The
committed transcript copy is host-path-masked, and the evidence holds no credential (the report's Outcome).

## The proposals this card covers (numbered as in `fanout-results.md`)
Each one lies wholly inside the answers above:
- **security-plan 2** · §Input Validation — the deleted stub's row is replaced by the agent-written check row: the
  last fenced json block, at most 65,536 B, a JSON array, at most 400 steps, closed keys and verbs, replayed on fresh
  `escher-session mcp` children that validate each step again. *(Q3)*
  - with **security-plan 3** · §Logging & Monitoring — its group (dependent-of): the stub's call log is retired;
    `server.log` is counted by span name and never printed; `check.json` is never printed. *(Q3)*
- **security-plan 4** · §Secret Management, Storage → Development — the 2026-10-15 live run, one run, on the operator's
  own login (`apiKeySource` none); no new secret source; the env census is over `cold-agent.sh` alone. *(Q1)*
  - with **security-plan 5** · §Threat Model Summary — its group (dependent-of): the MCP-server vector names the
    live run as a second carriage of the stand's screen to the model provider, and the reply's check as the input that
    returns. *(Q1, Q3)*
- **architecture 2** · §Occupied Resources, Process-wide state and threads — the client's `sh` → `escher-session mcp`
  child, plus the replay's two children spawned directly (stderr to `/dev/null`, 30 s per reply, killed and reaped).
  The `python3 cold_agent_stub.py` child is replaced. *(Q1, Q3)*
- **architecture 7** (raised by main) · §Occupied Resources, Outbound hosts — the model-provider row gains the
  2026-10-15 run: it now carries the stand's screen, one run, on «Один запуск, до $3 (Рекомендую)». *(Q1)*
- **test-plan 4** · §3 Cold-agent run pipe — the stub bullet is replaced by the escher server and the check's judge
  (validated, replayed in full and ablated on two fresh children, each step validated again by the server). *(Q3)*
- **obs-plan 4** · §8 Scrubbing — the cold-agent bullet: the transcript now carries the stand's screen (raw by design,
  never printed, and the committed copy masked); `server.log` and `check.json` are never printed. *(Q1)*

The other 21 applied proposals are routine (playbook :28, and :52 for security-plan 1) and ride no card. Among them are
the run's own description and its Proof line in test-plan §3: test-plan 1 and 8 restate the client line and the
measured run, and the provider crossing they describe is this card's.

**On confirm:** each covered amendment is applied. Its sidecar entry's **Why:** names the founder's answer as his
(«Один запуск, до $3 (Рекомендую)» / «JSON-проверка с повтором (Рекомендую)», 2026-10-15, relayed verbatim by the
overseer as his chosen option labels, inputs#I2) and this card's confirm. The ratification stays his answers.

## The answer
_(appended here as given — verbatim when relayed, a host prefix respelled)_

The operator's message, 2026-10-15, verbatim (inputs#I11):

> The founder's answer to the card, relayed verbatim by the overseer as his chosen option label; the English wording is the overseer's: «Подтвердить (Рекомендую)» = Confirm, so all eight apply. Record: ~/dev/projects/additional/escher-overseer/relays/cold-agent-founder-answers.md, section 'The wraps answer card'. Append it under the card, resume at P2 Escalate and wrap to the end: commit, push, read CI, report, and stop. No next skill, no version close, no route run.

The record's section, verbatim (inputs#I12, `../additional/escher-overseer/relays/cold-agent-founder-answers.md`, «## The wrap's answer card (answered 2026-10-15)»):

> The founder answered the one question of `.andromeda/runs/2026-10-15T09-44-57-wrap/answer-card.md` by question dialog in the overseer session; the overseer's clock read 2026-10-15T10:01:40Z right after. The question asked whether the eight proposals (security-plan 2–5, architecture 2 and 7, test-plan 4, obs-plan 4) lie inside his answers Q1 and Q3. It was put in Russian with both options whole, and the crossing and the check's bounds were stated plainly. He chose «Подтвердить (Рекомендую)» = Confirm: all eight apply as his ratification. He added no words of his own. Relayed verbatim by the overseer as his chosen option label; the English wording is the overseer's.

**Disposition:** confirmed whole — the founder's own word, relayed verbatim (not provisional). All eight proposals apply;
the ratification stays his answers «Один запуск, до $3 (Рекомендую)» and «JSON-проверка с повтором (Рекомендую)»
(inputs#I2), confirmed by «Подтвердить (Рекомендую)» (inputs#I12).
