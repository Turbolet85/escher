# Fan-out results — 2026-10-15-cold-agent-test (P2, resumed 2026-10-15)

Batch: one parallel batch of seven `general-purpose` doc-agents, inside one snapshot pair —
before `tree 5b8c916e6b9263101407cf9e55e88a5e24827907 index a51a0962900b60be1eaa9cad470e1ec729511eb0 head 539defbae322f8a65bf0ef8ac7b7e3bd9750ef0d`,
after `tree e6e35abc62354e03bbaa3ce8320906a994dc2af4 index a51a0962900b60be1eaa9cad470e1ec729511eb0 head 539defbae322f8a65bf0ef8ac7b7e3bd9750ef0d`;
`git diff-tree` printed exactly the seven `A` lines, index and head unmoved. No file fixed, no twin, no re-spawn; every
receipt's count equals its file's. Detector prompts: 15 detectors over the seven docs (arch 2 · security-plan 3 ·
design-system 1 · layout-templates 1 · test-plan 3 · obs-plan 3 · a11y-plan 2), the drift-base's 15 `doc:` names.

Rejected for a source the report does not carry: 0 (every line coordinate a proposal writes is the report's, or a row,
an `@` line or a row-to-row span of its **New text, by line**; test-plan 10's per-file counts other than
test_cold_agent.py's are test-plan's own standing tally).

## architecture: 6 proposals · fanout-architecture.md
note: architecture.md:147 (Outbound hosts, the model provider on the live `cold-agent.sh run`) stays true as a reach claim; its basis cites only the 2026-10-06 run — the 2026-10-15 run may be added; `RUST_LOG` at :153 already registered; no crate, dependency or Established Decision moved.
1. §Occupied Resources (Names) — server `stub` → `escher`, stub dropped → **apply** (check 1: playbook :28 Accurate this-chunk addition; the plan's Expected amendments name it; the `escher` server name is the MCP surface's ratified 2026-10-10 crossing)
2. §Occupied Resources (Process-wide state and threads) — the client's `sh` → `escher-session mcp` child and the replay's two directly spawned children → **escalate** (check 1: playbook :36 Boundary widening by subject — the pipe gains a subprocess crossing into two replay children it feeds the agent's check; on the answer card, inside «JSON-проверка с повтором (Рекомендую)» and «Один запуск, до $3 (Рекомендую)»)
3. §Occupied Resources (Filesystem) — `target/cold-agent/` gains `server.log`, `check.json`, loses the stub's two files; the 2026-10-15 live copies registered → **apply** (check 1: :28)
4. §Standard Contracts (CI contracts) — the pipe restated on the escher server: `stand`, the built-binary precondition, `mcp__escher`, model and $3 budget, the replayed check in the verdict; the stub retired → **apply** (check 1: :28; the plan names §Standard Contracts, CI contracts)
5. §Stack and Technologies (CI/CD row) — the stub clause retired, the pipe's server `escher-session mcp`, its judge stdlib python, `claude` 2.1.296 → **apply** (check 1: :28; check 5: citation row arch:66 `claim false` settled here)
6. §Infrastructure Patterns (directory tree) — `cold_agent_stub.py` removed from `scripts/` (dependent-of D-arch-resources) → **apply** (check 1: :28)
7. (raised by main, check 1/5 — from the agent's note) §Occupied Resources (Outbound hosts) — the model-provider row adds the 2026-10-15 live run: since this chunk the run carries the stand's screen (ids, names, values in snapshot texts and diffs), one run on «Один запуск, до $3 (Рекомендую)» → **escalate** (playbook :36 — the registry row of the crossing the run widened; on the answer card)

## security-plan: 5 proposals · fanout-security-plan.md
note: D-security-deps holds (Dependencies: none); the input boundary is validated as built; the screen-to-provider carriage is a boundary widening answered by the founder (inputs#I2), Validate's to card.
1. §Input Validation, CLI arguments (cold-agent.sh) row — allowlist `counter` → `stand`, the built `escher-session` a third precondition (exit 3), `STAND_PROMPT` → **apply** (check 1: playbook :52 — D-security-input's `escalate` grade with its invariant reported holding and the plan's reviewed list naming «the CLI arguments row's allowlist (`stand`)»; a closed one-task allowlist, no new input class — not :36)
2. §Input Validation — the stub row replaced by the agent-written check row (last fenced json block ≤ 65,536 B, a JSON array, ≤ 400 steps, closed keys and verbs, replayed on fresh `escher-session mcp` children that validate each step again) → **escalate** (check 1: :36 — a validated surface admits a new input class; on the answer card, inside «JSON-проверка с повтором (Рекомендую)»)
3. §Logging & Monitoring — the cold-agent bullet: stub call log retired, `server.log` counted by span name and `check.json` never printed (dependent-of D-security-input) → **escalate** with 2 (check 6: its group; on the answer card)
4. §Secret Management, Storage → Development — the 2026-10-15 live run on the operator's own login, one run, the env census over `cold-agent.sh` alone → **escalate** (check 1: :36 by subject — the run that carries the stand's screen to the model provider; on the answer card, inside «Один запуск, до $3 (Рекомендую)»)
5. §Threat Model Summary — the MCP-server vector names the cold-agent live run as a second carriage of the stand's screen to the model provider, the reply's check as the input that returns (dependent-of D-security-auth) → **escalate** with 4 (check 1: :36 — the widening itself; on the answer card, inside «Один запуск, до $3 (Рекомендую)» and «JSON-проверка с повтором (Рекомендую)»)

## design-system: 0 proposals · fanout-design-system.md
note: D-design-tokens holds — every Coverage row reads `tokens n/a`; no UI rendered; the site sweep read 0 in design-system.md.

## layout-templates: 1 proposal · fanout-layout-templates.md
1. §Surface: cli → Primary screens, the `scripts/cold-agent.sh` entry — task `stand`, the `escher-session` precondition, server `escher`; `counter` and the stub retired → **apply** (check 1: :28; the plan names it)

## test-plan: 11 proposals · fanout-test-plan.md
note: D-tests-framework holds; :118, :161, :201 re-read, no change; §8 Mocking never listed the cold-agent fakes; obs-plan §3 must move with §3 here.
1. §3 Cold-agent run pipe (`run`) — `run stand`, `mcp.json` with server `escher`, the client line, the check extracted, `STAND_PROMPT` → **apply** (check 1: :28; the plan names «the escher server behind the wrapper, the `stand` task and prompt»; the client's MCP crossing is the ratified 2026-10-10 surface, the provider crossing carded under security-plan 4–5)
2. §3 Cold-agent run pipe (Invocation) — the one task `stand`; `RUST_LOG=info` on the server child only (dependent-of D-tests-obs-harness) → **apply** (check 1: :28)
3. §3 Cold-agent run pipe (Exit grammar) — the third precondition (dependent) → **apply** (check 1: :28)
4. §3 Cold-agent run pipe (the stub bullet, replaced) — the escher server, `server.log` counted by span name, the check validated and replayed in full and ablated on two fresh children (dependent) → **escalate** (check 1: :36 by subject — the replay crossing and the check as input; on the answer card, inside «JSON-проверка с повтором (Рекомендую)»)
5. §3 Cold-agent run pipe (Verdict) — nine fields added, `final_count` retired, `passed` on all four apps and the check (dependent) → **apply** (check 1: :28; the plan names «the restated verdict fields and reasons»)
6. §3 Cold-agent run pipe (Events) — the check joins what never reaches an event, a fourth planted marker (dependent) → **apply** (check 1: :28)
7. §3 Cold-agent run pipe (State) — `server.log`, `check.json` added, the stub's two files retired (dependent) → **apply** (check 1: :28)
8. §3 Cold-agent run pipe (Proof) — the 2026-10-15 live run leads, the 2026-10-06 stub run kept as history (dependent) → **apply** (check 1: :28; the plan names «the live run's Proof line»; a measurement, its crossing carded under security-plan 4–5)
9. §4 Cold-agent pipe — test_cold_agent.py 27 → 33, `ColdAgentStubTest` retired, the case list restated → **apply** (check 1: :28; check 5: the plan's «`test_cold_agent.py`'s classes and count»)
10. §4 the CI-scripts tally — 94 → 100 over seven files (dependent-of D-tests-coverage) → **apply** (check 1: :28; the plan's «the CI-scripts tally»)
11. §1 Coverage scope (CI Python scripts) — 94 → 100 in seven files (dependent) → **apply** (check 1: :28; the report's site sweep names test-plan.md:8)

## obs-plan: 6 proposals · fanout-obs-plan.md
note: D-obs-pii holds (no new log touches user data unredacted); the bootstrap-phases key file untouched; the pipe is a new reader of the `tool_call` span line, not a new emitter.
1. §3 Cold-agent pipe log — restated on the escher server: `server.log` from escher's sink at `info` on the child, counted by span name, never printed; task `stand`; state files moved → **apply** (check 1: :28; the plan names obs-plan §3; the test-plan §3 ↔ obs-plan §3 bind moves with test-plan 1–8)
2. §6 Log format — the stub's call-log bullet replaced by `server.log` (dependent-of D-obs-stack) → **apply** (check 1: :28)
3. §6 Log format — `server.log` and `check.json` join the never-printed files (dependent) → **apply** (check 1: :28)
4. §8 Scrubbing — the cold-agent bullet: stub call log retired, `server.log` and `check.json` never printed, the transcript now carrying the stand's screen, four planted markers (dependent) → **escalate** (check 1: :36 by subject — it records what the widened run carries; on the answer card, inside «Один запуск, до $3 (Рекомендую)»)
5. §9 Cold-agent pipe state row — `server.log`, `check.json` added, the stub's two files retired (dependent) → **apply** (check 1: :28)
6. §6 Log format — the verdict fields: nine added, `final_count` retired, `counts_agree` on the server's span lines → **apply** (check 1: :28; the plan names «the pipe's verdict fields»)

## a11y-plan: 0 proposals · fanout-a11y-plan.md
note: D-a11y-surface and D-a11y-obs-schema hold — no interactive element added; no violation schema; the sink's log format unchanged.

## Checks over the set
- Check 2 (cross-contradiction): none — test-plan §3 and obs-plan §3 move in one direction (the bind holds after apply).
- Check 3 (intent-consistency): the scope record is `none` (`gate.py scope` clean, 0 recorded); the report's six deviations are within plan steps 4–7 or the operator's directive (inputs#I8); no divergence.
- Check 4 (absence needs evidence): the absence claims carry their basis — the census `grep -c` 0, the scope gate's 0, the site sweep's zero rows by its controls.
- Check 5 (expected amendments): every plan entry is proposed — test-plan §3 (1–8) · §4 (9, 10) · security-plan §Input Validation (1, 2) · §Threat Model Summary (5) · §Secret Management (4) · §Logging & Monitoring (3) · obs-plan §3 (1) · §6 (2, 3, 6) · §8 (4) · §9 (5) · arch §Occupied Resources (1, 2, 3, 6) · §Standard Contracts (4) · layout-templates (1). Every `claim false` row of `citation-dispositions.md` is settled by a proposal: arch:66 → arch 5 · arch:141 → arch 4 · arch:149 → arch 3 · arch:150 → arch 2 · arch:154 → arch 1 · security-plan:78 → security-plan 2 · :403 → security-plan 3 · test-plan:120 → 2 · :121 → 3 · :122 → 1 · :123 → 4 · :124 → 5 · :148 → 9 · obs-plan:87 → 1 · :159 → 6 · :160 → 2 · :332 → 5.
- Check 6 (disproved claims): «none measured false»; the two plan forecasts (the `arrowleft` wrong call, the «about 32» / «about 99» counts) are plan notes, not spec claims — no master owns them; carried to P3 through *Decisions & corrections*.

## Escalations
One class, one halt: the swap crossing — playbook :36 Boundary widening, answered by the founder at the P4 forks
(inputs#I2, relayed verbatim by the overseer as his chosen option labels). Eight proposals, on one answer card:
`answer-card.md`. No other escalation, no playbook-rule proposal.

**Resolved 2026-10-15:** the founder's «Подтвердить (Рекомендую)» — confirmed whole, his own word relayed verbatim by the
overseer as his chosen option label (inputs#I11, inputs#I12; appended under the card). All 30 proposals applied
(29 the detectors' + architecture 7 raised by main); 0 rejected; 1 escalation resolved.
