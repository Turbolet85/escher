# Report — 2026-10-06-cold-agent-run-pipe

**Chunk:** Cold-agent run pipe — fresh agent session with only a stub tool; transcript, wrong-call count and verdict recorded green
**Date:** 2026-10-06T04:40Z
**Commits:** `021668fa chore(2026-10-06-cold-agent-run-pipe): operator pre-CI commit, for the run this chunk's verdict reads` (the only commit since last_wrap 2026-10-06T03:51:51Z, `git log d73df1a4..HEAD`)

## Changes (structured — detectors read this)
- **Files:** new `scripts/cold-agent.sh` (mode 100755) · `scripts/cold-agent.ps1` (4 lines, pass-through) ·
  `scripts/cold_agent_stub.py` (mode 100755) · `.github/scripts/test_cold_agent.py` · chunk evidence
  `escher-0.1.0/chunks/2026-10-06-cold-agent-run-pipe/evidence/{live-verdict.json, live-transcript.jsonl,
  live-calls.jsonl, live-events.jsonl, live-run.md, operator-15-hygiene.txt, operator-16-push.txt, operator-17-ci.txt}`.
  Modified by /andromeda-phase (ledgers, not this chunk's code): `.andromeda/master-route.md` (the pending record),
  `escher-0.1.0/working-route.md` (the entry frozen with its marker), `escher-0.1.0/verification-matrix.json` (v010-15
  `notes` only, phase P5: not claimed). No existing source file changed — `git diff --quiet d73df1a4 -- Cargo.lock
  Cargo.toml packages examples tests apps wpt scripts/agent-run.sh scripts/agent-run.ps1 .github/scripts/test_agent_run.py
  .github/scripts/ci-leg.sh .github/workflows` exit 0 (gate).
- **Symbols / APIs:**
  - **`scripts/cold-agent.sh <verb> [task]`** — verbs `run <task>` · `status` · `cleanup` · `logs`; task allowlist
    `counter` (the only task). Exit grammar 0 success · 1 the verb ran and failed (a `failed` verdict) · 2 usage (bad
    verb, wrong argument count, task outside the allowlist; stderr usage, empty stdout; checked before any
    precondition) · 3 a precondition (`claude` or `python3` not on PATH; `status` with no verdict; `logs` with no
    `events.jsonl`). Run from the repository root.
  - `run counter`: recreates `target/cold-agent/`; makes a per-run `mktemp -d` session directory (outside the repo; its
    path names nothing), copies the stub there and writes `mcp.json` (one stdio server `stub`: `python3 stub.py --state
    … --calls …`); runs `timeout 600 claude -p "{task prompt}" --output-format stream-json --verbose --tools ""
    --strict-mcp-config --mcp-config {session}/mcp.json --allowedTools mcp__stub --setting-sources ""
    --disable-slash-commands --no-session-persistence --max-budget-usd 1` from the session dir, stdin `/dev/null`,
    stdout → `transcript.jsonl`, stderr → `client.log`; a non-zero or timed-out client is `client-exit`, never a
    propagated 124; copies the stub's `calls.jsonl` + `stub-state.json`; removes the session dir (also on exit, trap).
    Task prompt names the goal only (display reads 3), no element id.
  - **Events** (python3 `json`, printed and appended to `target/cold-agent/events.jsonl`; `status`/`cleanup` printed
    only): `run.start {ts, task}` · `run.end {…every verdict field}` · `status {…verdict}` or `status {outcome: "none"}`
    (exit 3) · `cleanup {outcome: "done"}`.
  - **Verdict** `target/cold-agent/verdict.json`: `task, client_exit, isolated, session_tools, client_version, model,
    tool_calls, wrong_calls, transcript_errors, counts_agree, final_count, num_turns, duration_ms, cost_usd,
    input_tokens, output_tokens, cache_read_input_tokens, cache_creation_input_tokens, transcript_file, outcome,
    reasons, ts`. `outcome` passed only on positive evidence (client exit 0 · a `result` with subtype success and
    is_error false · isolated · counts_agree · tool_calls ≥ 1 · final_count 3); `reasons` tokens `client-exit ·
    no-result · not-isolated · counts-disagree · no-tool-call · task-unmet`. `isolated` = init tools exactly the three
    stub tools · one MCP server `stub` connected · skills and slash_commands `[]` · `apiKeySource` none · no cwd or
    memory_paths string contains the repo root's basename (case-folded). `wrong_calls` = stub-log refusals + non-stub
    tool_use blocks + `permission_denials`; it does not decide the outcome. `final_count` is the stub's state, never
    the reply.
  - **`scripts/cold_agent_stub.py --state F --calls F`** — stdlib MCP server, newline-delimited JSON-RPC 2.0 over
    stdin/stdout: `initialize` (echoes the client's protocolVersion, `capabilities.tools`, serverInfo `stub`) ·
    `notifications/initialized` · `tools/list` (exactly `list`, `read`, `press`, each with an inputSchema) ·
    `tools/call`; any other method → JSON-RPC −32601, unparsable line → −32700. Model: display `count` (0), buttons
    `inc` · `dec` (disabled at 0) · `reset`. Refusals are tool results `isError: true`, first word the cause, no state
    change: `not-found` (unknown id; also an unknown tool name, logged with `tool: null`) · `disabled` ·
    `not-pressable` · `malformed` (missing, extra or non-string argument; any argument to `list`). Call log line
    `{seq, tool, outcome: ok|refused, cause}` — never an argument value; state file `{count}` rewritten per call.
    Reads no env var (argv only).
  - Ports / sockets / listeners: none. Env vars read: none (census gate: `socket|TcpListener|bind\(|listen\(|
    http\.server|urllib|requests|os\.environ|getenv|ANTHROPIC_` over the three scripts → 0). Processes: the spawned
    `claude` client and the stub it spawns over stdio, both per run, both ended by the run.
- **Crates / modules:** none.
- **Dependencies:** none (stdlib python; nothing in `Cargo.lock` or a requirements file). The pipe's `run` needs the
  host tool `claude` (Claude Code CLI, the agent client) — a precondition (exit 3), not a CI requirement: CI runs only
  the shim tests.
- **Schema / config:** per-run `mcp.json` (in the session dir only) · the verdict schema above · the stub call-log
  line. Scrub: no event or verdict key is in the scrub set `url href src html text value attrs path request error
  panic.payload` (contract tests + the live-evidence gate); no transcript text, tool argument, tool result or reply
  enters an event or the verdict (marker-string test).
- **Spec-master edits:** none.
- **Counts / qualifiers moved:** CI-scripts tests 37 → 64 (basis: `bash .github/scripts/ci-leg.sh fast` log `Ran 64
  tests … OK`; CI#37413576977 job "Test CI scripts" log `Ran 64 tests … OK`; `test_cold_agent.py` alone `Ran 27 tests`
  — 2 classes: `ColdAgentStubTest` 6 · `ColdAgentPipeTest` 21). Stated at test-plan.md:127 (§4, CI workflows and leg
  script bullet, offset 1554): "the CI-scripts leg runs 37 tests — the 23 of the files above and test_agent_run.py's
  14, as measured at CI run 37409303977" — now 64 = 23 + 14 + 27 at CI#37413576977. Basis: grep `\b37\b` near
  test/case over `.andromeda/*.md` (sidecars out), CLAUDE.md, `.claude/{docs,rules}` → 18 rows, each read at its match
  offset: 2 at the one master count site (test-plan.md:127 @1554 "runs 37 tests" · @1678 `Ran 37 tests`; the first pass
  had characterised it from a 220-char clip — the test-plan detector named it) · 1 leaf (`.claude/docs/tests-summary.md:26`
  "37 CI-script tests", re-derived by the cascade) · 15 `file:line` citations holding `37`/`23` (no count). Workspace tests unchanged 430 · 0 · 4 (fast-leg log sum).
- **Dev-tool versions:** none — `claude` (Claude Code CLI, the agent client) re-read at 2.1.288 on the dev host
  (research and entry 8, `claude --version`, 2026-10-06T04:19:45Z), unchanged.
- **Harness / gate surface:** the new pipe `scripts/cold-agent.{sh,ps1}` + stub (above), state
  `target/cold-agent/{verdict.json, events.jsonl, transcript.jsonl, client.log, calls.jsonl, stub-state.json}`
  (gitignored under `target/`; `cleanup` removes only it; `target/agent-run/` survives — test); contract tests in the
  existing `ci-scripts` leg (`python3 -m unittest discover -s .github/scripts`). `agent-run.sh`, its 14 tests,
  `ci-leg.sh` and `ci.yml` unchanged (gate).
- **Cross-project / external claims:**
  - CI: CI#37413576977 on `021668fa` — verdict green, checks 16/16, wall 429 s (`ci.py conclusion`, entry 17).
  - The agent client (Claude Code 2.1.288, dev host, live run 2026-10-06T04:19:48Z) measured: `--allowedTools
    mcp__stub` (server-wide) let the stub's calls run in `-p` mode (`permission_denials` empty); `--tools ""` removes
    the built-in tools but keeps the MCP server's (init tools = the three stub tools); init `mcp_servers` entries carry
    `source: "dynamic"`; init also carries `messaging_socket_path` (a local client socket) and the stream carries a
    `rate_limit_event` line (the account's rate-limit utilisation); `apiKeySource` `none` under the operator's
    claude.ai login. Model provider reached via that login on the live run only.
  - inputs: `inputs: absent` — none snapshotted (`inputs.py verify`).
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:** none in the seven masters. (research.md read `--tools ""` from `claude
  --help` as "disable all tools"; the live init shows MCP tools kept — research is not a spec, recorded above.)
- **Expected amendments (from plan):** sites located by `grep -c 'agent-run'` per master (the sibling contract each
  lands beside): arch 2 · security-plan 2 · layout-templates 1 · test-plan 10 · obs-plan 6 · a11y-plan 1 (a false
  match: "agent-runnable invariants" at a11y-plan.md:57, not the contract — no a11y site) · design-system 0;
  `grep -ci 'cold-agent'` → 0 in every master.
  - arch §Occupied Resources (Filesystem `target/cold-agent/` · the per-run `mktemp -d` session dir · `evidence/live-*`;
    Process-wide state gains the spawned `claude` client + stub, stdio only; Outbound hosts gains the model provider
    via the operator's login, live run only; Names gains the three scripts; Network ports stays none) — **carried**:
    Symbols / APIs, Harness / gate surface, Cross-project. Sites: architecture.md:144 (ports), :145 (outbound),
    :147 (Filesystem, `target/agent-run` hit), :148 (Process-wide), :152 (Names).
  - arch §Standard Contracts → CI contracts (the pipe contract) + §Infrastructure Patterns → Directory structure —
    **carried**: Symbols / APIs, Harness / gate surface. Site: architecture.md:139 (`agent-run` hit).
  - security-plan §Secret Management (operator's own Claude Code login, `apiKeySource none`, no CI secret) ·
    §Input Validation row (stub argument checks + task allowlist) · §Logging & Monitoring cold-agent row — **carried**:
    Symbols / APIs, Schema / config, Cross-project. Sites: security-plan.md:66 (agent-run.sh Input Validation row),
    :362 (agent-run logging bullet).
  - test-plan §3 cold-agent pipe block beside the agent-run contract + §4 `test_cold_agent.py` with the ci-scripts
    count re-derived — **carried**: Symbols / APIs, Counts. Sites: test-plan.md:102-110 (§3 agent-run block), :128
    (§4 agent-run contract tests).
  - obs-plan §3/§6 cold-agent events · §8 the pipe outside escher's scrub, no content-named keys · §9
    `target/cold-agent/` artifact row — **carried**: Symbols / APIs, Schema / config. Sites: obs-plan.md:83 (§3), :144-147
    (§6 log format), :285 (§8), :312 (§9 row).
  - layout-templates §Surface: cli → Primary screens `scripts/cold-agent.sh <verb> [task]` — **carried**: Symbols /
    APIs. Site: layout-templates.md:71.
- **Coverage of new surfaces:**
  - `scripts/cold-agent.sh` (CLI) → validation verb + argument-count + task allowlist✓ (usage exit 2, tested) ·
    instrumentation JSON-line events✓ · PII redacted✓ (counts/identities only; marker test) · tests integ (21 pipe
    contract tests under a `claude` shim) + e2e (one live run) · a11y n/a · tokens n/a
  - `scripts/cold_agent_stub.py` (MCP stdio server, input from the agent session) → validation argument shape + id
    checks✓ (4 refusal causes, tested) · instrumentation call log✓ · PII redacted✓ (no argument values logged, tested)
    · tests unit (6 stub tests over a pipe) + e2e · a11y n/a · tokens n/a
  - `target/cold-agent/transcript.jsonl` (raw artifact) → validation n/a · instrumentation n/a · PII raw✗ by design
    (a local gitignored artifact holding model output and tool I/O; never printed or logged; the committed evidence
    copy masks two host paths) · tests integ · a11y n/a · tokens n/a

## Deviations from intent
- **The committed transcript is a masked copy, not a byte copy** (plan step 6 says copy). The live transcript held
  two absolute host paths — the session dir under the temp dir (init `cwd`) and the operator's home prefix
  (`memory_paths.auto`) — which the hygiene rule on committed chunk evidence refuses (P1). Each was replaced once
  (`/tmp/<session-dir>`, `/home/<user>/`); same 17 lines, each parses; `live-run.md` records the masking. Operator's
  word: "The masked transcript is fine: masking host paths to pass hygiene is the hygiene rule's own remedy, recorded
  in live-run.md." The unmasked original was removed by the light block's `cleanup` entry; the masked copy is the only
  copy. The other three evidence files are byte copies.
- **Details the plan left open, settled from the agent-run precedent:** an unknown tool name → `not-found`, logged
  `tool: null` (agent-supplied text kept out of the log); the call-log result key is `outcome`; `status` with no
  verdict prints `{"event":"status","outcome":"none"}` (agent-run's "none" run state); the client's stdin is
  `/dev/null` (so a piped stdin can never hold the session); the verdict carries `task`, `client_exit`, `ts` beside
  the plan's fields.
- **Tests beyond the plan's case list:** a permission denial counted as a wrong call · an `apiKeySource` other than
  none reads not-isolated · an unsuccessful `result` subtype reads `no-result` · a missing transcript reads `no-result`
  · `status` after a run echoes the verdict · extra refusal shapes (extra argument, non-string id, argument to `list`,
  unknown tool).
- scope record: none — `gate.py scope` clean, 0 recorded (changed 4 · listed 4).

## Decisions & corrections
- P4 forks (overseer under the founder's standing delegation, PROVISIONAL, FOR DISCUSSION 7): agent client = `claude
  -p` on the operator's claude.ai login, no API key; stub = stdlib python MCP over stdio, no port; live run on the
  operator host only, ONCE, token usage recorded, no CI secret.
- Operator ruling this wrap: the masked transcript stands (quoted above).
- Sweep hazard: the census gate's alternation is fixed-string-ish over whole files, so the word `requests` (and
  `listen(`, `bind(`) is forbidden even in a comment or docstring of the three scripts.
- Sweep hazard: the hygiene P1 temp-dir form fires on illustrative prose — `/tmp/tmp.XXXXXXXXXX` in a `.md` reads as
  a host path; only a token holding `<` (`/tmp/<session-dir>`) is a placeholder. Caught before commit while drafting
  `live-run.md`.
- The evolve tool refuses a hand-typed future `ts` (`REFUSED ts: … later than the clock`) — the stamp is read from
  `date -u` immediately before the append, never anticipated.

## Outcome
- Acceptance criteria, against the diff:
  - (tests) contract tests with no live model and no credential — MET: `test_cold_agent.py` Ran 27 OK; cases cover
    pass, counted wrong calls, task unmet, empty session, isolation breach (cwd, extra tool), client failure, usage
    (2), precondition (3).
  - (tests) agent-run contract untouched — MET: `test_agent_run.py` Ran 14 OK (14 re-derived at `d73df1a4`); the
    `git diff --quiet d73df1a4 -- …` guard exit 0.
  - (tests) one live run reads `passed`, isolated, counts_agree, final_count 3, evidence committed with usage — MET:
    `evidence/live-verdict.json` outcome passed · isolated true · counts_agree true · final_count 3 · tool_calls 5 ·
    wrong_calls 0 · input 12 · output 337 · cache read 17611 · cache creation 3932 · cost_usd 0.0417662;
    `live-run.md` records them.
  - (security) no env read, no socket, no API-key name; evidence holds no key/auth/credential/token/e-mail;
    `apiKeySource` none — MET (census 0; evidence census 0; init `apiKeySource: "none"`).
  - (security) four refusals name their cause, no side effect, tested — MET (`ColdAgentStubTest`).
  - (obs) every stdout and `logs` line one JSON object, no scrub key, no transcript text, marker never reaches events
    — MET (pipe tests; live-events gate `True 0`).
  - (obs) state only in `target/cold-agent/`, cleanup idempotent, `target/agent-run/` intact — MET (cleanup test;
    gate entry).
  - (arch) stub over stdio only; session in a removed per-run `mktemp -d` dir whose init path does not name the repo —
    MET (pipe test; live init cwd under the temp dir, `isolated` true).
  - (layouts) malformed invocation → usage to stderr, empty stdout, exit 2, `<script> <verb> [task]` shape — MET.
  - (ci) pushed sha's CI green, `ci-scripts` runs `test_cold_agent.py` — MET: CI#37413576977 green 16/16; job "Test
    CI scripts" `Ran 64 tests … OK` = 37 + 27 (the job runs unittest without `-v`, so the file is evidenced by count,
    not by name).
  - No matrix capability claimed — MET (`matrix.py show --chunk` → claimed 0).
- Gates (/implement P2 block, then the operator pass):
  - `python3 -m unittest discover -s .github/scripts -p 'test_cold_agent.py' -v` — green · exit 0 · contains OK
  - `python3 -m unittest discover -s .github/scripts -p 'test_agent_run.py' ` — green · exit 0 · Ran 14 tests · OK
  - `bash scripts/cold-agent.sh cleanup && bash scripts/cold-agent.sh cleanup && test ! -e target/cold-agent` — green · exit 0
  - `bash scripts/cold-agent.sh status` — green · exit 3
  - `bash scripts/cold-agent.sh run nonesuch` — green · exit 2
  - the no-listener/no-env census over the three scripts — green · exit 1 · last line 0
  - `git diff --quiet d73df1a4… -- Cargo.lock … .github/workflows` — green · exit 0
  - `claude --version` (leg operator) — driven by hand: exit 0 · `2.1.288 (Claude Code)` (evidence/live-run.md)
  - `bash scripts/cold-agent.sh run counter` (leg operator, THE ONE LIVE RUN) — driven by hand once: exit 0 · all
    four atoms held · verdict fresh (ts 04:20:00Z after start 04:19:48Z) (evidence/live-run.md)
  - the live-verdict re-read probe — green · last line `passed True True 3 True True`
  - the evidence secret census — green · exit 1 · last line 0
  - the live-events/verdict scrub-key probe — green · last line `True 0`
  - `bash .github/scripts/ci-leg.sh fast` — green · exit 0 (fmt · clippy · workspace 430/0/4 · CI scripts 64 OK)
  - `bash .github/scripts/ci-leg.sh doc` — `not run — defer`: zero Rust delta; implement's delta check (uncommitted
    basenames as Rust string-literal path ends → 0 hits; walk-class 0 marked) held it
  - `gate.py hygiene` (leg operator) — exit 0 · `hygiene: clean` (evidence/operator-15-hygiene.txt)
  - `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin
    build/escher-0.1.0` (leg operator) — exit 0 · pushed `d73df1a4..021668fa` (evidence/operator-16-push.txt)
  - `ci.py conclusion --sha HEAD --wait 1800` (leg operator) — exit 0 · `verdict: green` · CI#37413576977 16/16 · 429 s
    (evidence/operator-17-ci.txt)
  - smoke: skipped — no boot-path / UI-surface change (the chunk ships scripts only).
- Watches: none folded.
- Outcome basis: the operator pass ran — commit list `021668fa` (the pre-CI commit; no fix commits); the final HEAD's
  CI run CI#37413576977 recorded in `evidence/operator-17-ci.txt`; implement's P4 report (this conversation) for the
  P2 block and the live run; post-implement artifacts `evidence/operator-{15,16,17}-*.txt`.
- Process hygiene: implement's census — the `claude -p` client and its stub (entry 9): terminated, client exit 0 ·
  the gate blocks: terminated; the operator pass's fast leg, push and CI wait: terminated (each a bounded foreground
  or awaited background call that exited). Re-measured at this report (2026-10-06T04:36Z): `pgrep -af
  'cold_agent_stub|cold-agent\.sh|claude -p|ci\.py conclusion'` → none; no `tmp.*` session dir left under the temp dir.
