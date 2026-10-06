# Fan-out results — 2026-10-06-cold-agent-run-pipe

Seven Explore doc-agents, one parallel batch; 15 detectors (arch 2 · security 3 · design 1 · layouts 1 · tests 3 ·
obs 3 · a11y 2 = the drift-base's 15). Keyed contracts: tests · obs · a11y rendered; arch NOT MIGRATED (body read);
security · design · layouts n/a. Entity probe: no `&lt;` `&gt;` `&amp;` in any return (each read). Totals: 20 proposals
(arch 6 · security 6 · layouts 1 · tests 3 · obs 4 · design 0 · a11y 0).

Report corrections made before Validate (detector-surfaced, re-measured by the orchestrator):
- a11y: the report's a11y-plan `agent-run` hit was a false match ("agent-runnable", a11y-plan.md:57) — bullet fixed.
- tests: the report claimed "no master states the 37"; test-plan.md:127 @1554 states it. The count grep re-run with
  every hit read at its offset: 18 rows — 2 at that master site, 1 leaf (`.claude/docs/tests-summary.md:26`), 15
  `file:line` citations. Counts bullet fixed.

## design-system — 0
`proposals: []`; stripping removed commentary only (every new surface `tokens n/a`).

## a11y-plan — 0
`proposals: []`; stripping removed commentary only (no interactive UI; violation schema unchanged).

## layout-templates — 1
- **L1** D-layout-surface · warning · §Surface: cli → Primary screens — a `scripts/cold-agent.sh` bullet beside
  agent-run.sh (usage `cold-agent.sh <verb> [task]`, verbs, `counter` allowlist, usage → stderr exit 2 before any
  precondition, JSON-line stdout, `target/cold-agent/` state, `.ps1` pass-through; exit grammar + schema in test-plan §3).
  → **apply** (check 1: Accurate this-chunk addition; expected amendment names it).

## architecture — 6
- **A1** D-arch-resources · warning · §Standard Contracts → CI contracts — the cold-agent pipe contract beside agent-run.
  → **apply** (check 1: Accurate this-chunk addition; expected amendment names it). Its client-launch clause records the
  widened crossing → applies under the ratification of escalation E1.
- **A2** D-arch-resources · warning · §Occupied Resources → Filesystem — `target/cold-agent/` + the per-run `mktemp -d`
  session dir. → **apply** (check 1: Accurate this-chunk addition; agent-run precedent; expected amendment names it).
  The committed `evidence/live-*` named per the expected amendment.
- **A3** D-arch-resources · warning · §Occupied Resources → Process-wide state — the spawned `claude` client + its
  stdio stub per run; the client's own local socket. → **escalate E1** (check 1: Boundary widening — a subprocess/IPC
  boundary gains a new crossing).
- **A4** D-arch-resources · warning · §Occupied Resources → Outbound hosts — the model provider via the operator's
  login, live run only. → **escalate E1** (Boundary widening — a new outbound crossing).
- **A5** D-arch-resources · warning · §Occupied Resources → Names — the three scripts, server name `stub`, the test file
  and its classes. → **apply, narrowed** (check 1: the Registry over-reach rule's precondition fails — Names tracks
  named entry points item by item (crates, binaries, features), not a category; the plan's P5-approved expected
  amendment names "Names gains `scripts/cold-agent.{sh,ps1}` and `scripts/cold_agent_stub.py`" — that direction
  settles it). Applied: the three scripts + the MCP server name `stub`; the test file and class names dropped (beyond
  the direction; tests are not registered names).
- **A6** D-arch-decisions · warning · §Stack and Technologies → CI/CD — Claude Code CLI `claude` as the pipe's host-only
  agent client; stdlib python3 stdio MCP stub. → **escalate E1** (Boundary widening — the agent client is the crossing's
  subject; not in the expected list).

## security-plan — 6
Stripping removed two comment lines (D-security-deps no drift; §Logging & Monitoring left to the orchestrator).
- **S1** D-security-input · escalate · Input Validation — row "CLI arguments (cold-agent.sh)". → **reject** (re-derivation
  tell: change/basis cite `scripts/cold-agent.sh:12-21 · :230-235 · :269-278`, locations the report does not carry). Its
  fact enters through check 5 (raised R2).
- **S2** D-security-input · escalate · Input Validation — row "MCP stdio server (cold_agent_stub.py)". → **reject**
  (re-derivation tell: cites `scripts/cold_agent_stub.py:60-84 · :96-113 · :140-159`). Fact raised at check 5 (R2).
- **S3** D-security-auth · warning · Secret Management → Storage — a development credential source: the operator's Claude
  Code claude.ai login held by the CLI, `apiKeySource none`, no env read, no CI secret. → **escalate E1** (Boundary
  widening — a new credential path; security rules §Surfaces: a credential path is a security-plan amendment first).
- **S4** dependent-of S3 · Secret Management → What counts as secret — the operator's Claude Code login. → with S3 (E1).
- **S5** dependent-of S3 · Secret Management → closing NOT YET MEASURED — narrowed: the login recorded, its at-rest
  location still unmeasured. → with S3 (E1).
- **S6** D-security-auth · warning · Authentication & Authorization — table row for the session's tool-permission grant.
  → **reject** (re-derivation tell: cites `scripts/cold-agent.sh:257 · :96-110`; not in the expected list; the isolation
  flags land in arch CI contracts and test-plan §3 from the report).

## test-plan — 3
- **T1** D-tests-coverage · warning · §4 → What unit tests cover — a `test_cold_agent.py` bullet (Stub 6 · Pipe 21, shim,
  no live model). → **apply** (Accurate this-chunk addition; expected amendment names §4).
- **T2** D-tests-coverage · warning · §4 → CI workflows and leg script bullet — count 37 → 64 (23 + 14 + 27) at
  CI#37413576977. → **apply** (Accurate this-chunk addition; expected amendment: "the ci-scripts count re-derived").
- **T3** D-tests-obs-harness · warning · §3 — a cold-agent run pipe block beside the agent-run contract. → **apply**
  (Accurate this-chunk addition; expected amendment names it; the test §3 ↔ obs §3 bind applied with O1/O2).

## obs-plan — 4
- **O1** D-obs-stack · warning · §3 — a cold-agent pipe log paragraph beside the agent-run one. → **apply**.
- **O2** D-obs-instrumentation · warning · §6 — "Log format (the cold-agent pipe, §3)" block. → **apply**.
- **O3** D-obs-instrumentation · warning · §9 — a cold-agent pipe state row. → **apply**.
- **O4** D-obs-pii · escalate · §8 Scrubbing — the pipe outside escher's scrub, no content-named keys; the raw-by-design
  transcript (host paths, local socket, rate-limit line), never printed, gitignored; the committed copy host-path-masked.
  → **escalate E2** (the detector's own severity: escalate).
(O1–O3: check 1 Accurate this-chunk addition; expected amendment obs §3/§6/§9 names them.)

## Check 5 — expected amendments raised by the orchestrator
- **R1** arch §Infrastructure Patterns → Directory structure: a `scripts/` line (the project scripts, the cold-agent
  three among them). No detector proposed it; the report substantiates it (Files) → **apply** (routine, plan-directed).
- **R2** security-plan §Input Validation: rows for `cold-agent.sh`'s verb/task allowlist (usage exit 2 before any
  precondition) and the stub's argument checks (JSON-RPC −32700/−32601; four refusal causes, no state change; no
  argument value logged; argv only), re-derived from the report. → **escalate E1** (the stub is a new input surface
  over the new IPC crossing — Boundary widening; the cold-agent.sh row rides with it).
- **R3** security-plan §Logging & Monitoring: a cold-agent bullet beside agent-run's — events metadata only (no
  content-named key, no transcript text), the transcript a raw local artifact never printed. → **apply** (routine;
  report Schema / config + Coverage substantiate; consistent with E2's resolution).
- arch Network ports and listeners stays none — no edit (the report: none).

## Checks
- 1 playbook: dispositions above. 2 cross-contradiction: none (test §3 ↔ obs §3/§6 carry one event set and one state
  dir). 3 intent: the report matches the entry + acceptance; the one deviation (masked transcript) carries the
  operator's word; scope record empty. 4 absence: the 37-count claim re-measured (above). 5 expected amendments: all six
  entries matched (A1–A5, R1 · S3–S5, R2, R3 · T1–T3 · O1–O4 · L1). 6 disproved claims: none in the report.

## Escalations
- **E1 — Boundary widening** (A3 · A4 · A6 · S3+S4+S5 · R2, and A1's client-launch clause): the P4 forks were answered
  by the overseer delegate, PROVISIONAL; the class needs the operator's own ratification.
- **E2 — D-obs-pii** (O4).
