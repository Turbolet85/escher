# Report — 2026-10-15-cold-agent-test

**Chunk:** cold-agent test: a fresh agent given only the driver's MCP tool completes a stand task and writes a passing check, wrong calls counted (v010-15)
**Date:** 2026-10-15
**Commits:** `539defbae chore(2026-10-15-cold-agent-test): operator pre-CI commit, for the run this chunk verdict reads` (the one commit since `last_wrap` 2026-10-15T07:32:41Z, whose commit is `01297a12`; `git log --format='%h %s' 01297a12..HEAD`)

## Changes (structured — detectors read this)
- **Files:** (basis: `gate.py scope` — `changed 3 · listed 3 · recorded 0`, base `01297a12`; `cites.py added`)
  - `scripts/cold-agent.sh` — modified, 269 lines added in 26 ranges;
  - `scripts/cold_agent_stub.py` — deleted;
  - `.github/scripts/test_cold_agent.py` — modified, 363 lines added in 58 ranges;
  - `escher-0.1.0/chunks/2026-10-15-cold-agent-test/evidence/` — new: `live-run.md`, `live-verdict.json`,
    `live-events.jsonl`, `live-check.json`, `live-transcript.jsonl`, `operator-9-claude-version.txt`,
    `operator-respell.txt`, `operator-12-hygiene.txt`, `operator-13-push.txt`, `ci.md`;
  - `escher-0.1.0/verification-matrix.json` — v010-15 `planned → implemented`, ref
    `escher-0.1.0/chunks/2026-10-15-cold-agent-test/evidence/live-verdict.json` (`matrix.py implement`).
  - No file under `packages`, `examples`, `tests`, `apps`, `wpt`, nor `Cargo.toml` / `Cargo.lock`, moved from
    `01297a12` (gate `git diff --name-only 01297a12… -- packages examples tests apps wpt Cargo.toml Cargo.lock | wc -l`
    read 0).
- **Symbols / APIs:** all in `scripts/cold-agent.sh` (coordinates from the new-text listing below), plus the contract
  test file; no Rust symbol, no driver verb, no schema change.
  - **The task allowlist** is `TASKS=(stand)`; `counter` is no longer a task. The usage line names `stand`, and usage
    is still checked first (exit 2, stdout empty).
  - **A third precondition** (`:459-462`): `target/debug/escher-session` present and executable, else exit 3 with one
    stderr line naming `cargo build --locked -p seven_guis --bin escher-session`. The pipe builds nothing.
  - **The fixed prompt** `STAND_PROMPT` (`:16-27`, a quoted heredoc): the four lean apps one at a time, the four goals,
    a snapshot of each final screen, then the check as the reply's last fenced ```json block. It names no element id
    and no app slug — none of `counter-` `flight-` `timer-` `crud-` `task-header` `id=`
    (`test_the_prompt_names_no_stand_id`).
  - **The per-run `mcp.json`** names one server, `escher`:
    `{"type":"stdio","command":"sh","args":["-c","exec env RUST_LOG=info \"$1\" mcp 2>\"$2\"","sh",<abs target/debug/escher-session>,<abs target/cold-agent/server.log>]}`
    — the 2026-10-11 trial's wrapper without its request `tee`.
  - **The client line:** `timeout 600 claude -p "$STAND_PROMPT" --output-format stream-json --verbose --tools ""
    --strict-mcp-config --mcp-config … --allowedTools mcp__escher --setting-sources "" --disable-slash-commands
    --no-session-persistence --model claude-opus-5-5 --max-budget-usd 3`. Before this chunk it read
    `--allowedTools mcp__stub` and `--max-budget-usd 1`, with no `--model`.
  - **Embedded python** (`EVENTS_PY`):
    - `DONE` (`:52-58`), the done-state atoms:
      - `counter`: `Paragraph "3" id="counter-value"`;
      - `flight-booker`: `Paragraph "Booked return flight: 24.12.2026 → 31.12.2026" id="flight-booked"`;
      - `timer`: `Paragraph "Elapsed: 10.0s" id="timer-elapsed"` + `Unknown "10.0s" id="timer-duration-value"`;
      - `crud`: `ListBoxOption "Lovelace, Ada"` + `ListBoxOption "Tischler, Roman"`, lacking `id="crud-person-0"`.
    - `text_of` (`:108-113`); `span_counts` (`:148-157`), which counts ` span="tool_call" ` lines and their
      ` ending="refused" ` / ` ending="error" `; `done` (`:160-162`).
    - `tasks_met` (`:165-181`): the app of the latest accepted `start`; an accepted `stop` clears it; a `snapshot`
      result counts for the started app only.
    - `extract_check` (`:184-193`): the last fenced ```json block, at most 65,536 bytes, a JSON array.
    - `valid_step` (`:196-204`): only `tool` ∈ the eleven verbs, `arguments` an object, `expect` a list of strings.
    - `Server` (`:207-246`): a stdio child, a reader thread, every reply under `REPLY_BOUND = 30` s, kill and reap.
    - `replay` (`:249-266`), `started_apps` (`:269-278`).
    - `judge_check` (`:281-305`): at most 400 steps; a full replay, then an ablated one (acting steps left out) on a
      second fresh child.
    - `MAX_CHECK_BYTES = 65536`, `MAX_CHECK_STEPS = 400`.
  - **Test file:** `FAKE` (a fake `escher-session`, `:67-126`), `SHIM` (the `claude` stand-in, spawning the server
    exactly as `mcp.json` names it), `flow` (`:198-208`) and `stand_check` (`:211-220`).
  - **Processes per live run:** the client; its `sh` → `escher-session mcp` child; two replay `escher-session mcp`
    children, spawned directly with stderr to `/dev/null`.
  - No listener, port or socket is added. No env var is read by the pipe: `RUST_LOG=info` is set on the server child
    only (census gate `grep -c -E 'socket|TcpListener|bind\(|listen\(|http\.server|urllib|requests|os\.environ|getenv|ANTHROPIC_' scripts/cold-agent.sh`
    read 0).
- **Crates / modules:** none.
- **Dependencies:** none (the pipe's python uses the stdlib only: `datetime json os queue re subprocess sys threading`).
- **Schema / config:**
  - The verdict's fields, every one a count, a boolean or a closed word:
    - kept: `task client_exit isolated session_tools client_version model tool_calls wrong_calls transcript_errors
      counts_agree num_turns duration_ms cost_usd input_tokens output_tokens cache_read_input_tokens
      cache_creation_input_tokens transcript_file outcome reasons ts`;
    - added: `server_tool_calls refused_calls error_calls spacer_calls tasks_met check_steps check_expects
      check_passed check_can_fail`;
    - retired: `final_count`.
  - `isolated` now requires the eleven `mcp__escher__{verb}` tools and one server `escher` connected; skills and
    slash commands empty, `apiKeySource` none, and no cwd or memory path naming the root are unchanged.
  - `counts_agree`: the server's span lines equal the escher tool uses, AND refused + error lines equal the escher
    tool results marked `is_error`.
  - `wrong_calls` = refused + error + foreign tool uses + permission denials.
  - Reasons: `client-exit no-result not-isolated counts-disagree no-tool-call task-unmet` kept; `no-check`,
    `check-failed` and `check-cannot-fail` added.
  - State files: `target/cold-agent/{verdict.json, events.jsonl, transcript.jsonl, client.log, server.log,
    check.json}`. `calls.jsonl` and `stub-state.json` are retired. `server.log` is counted by span name and never
    printed or copied; `check.json` is written as parsed (`json.dump`, `indent=2`) and never printed.
- **Spec-master edits:** none.
- **Counts / qualifiers moved:**
  - `test_cold_agent.py` 27 → 33 tests: `ColdAgentStubTest` (6) retired, `ColdAgentPipeTest` 21 → 33. Basis: the
    unit gate's log `Ran 33 tests … OK`.
  - The `ci-scripts` leg 94 → 100 tests, still over seven files. Basis: `target/ci-logs/ci-scripts.log`
    `Ran 100 tests in 8.555s` / `OK`.
  - Sites stated in the masters (site sweep `report-patterns.toml`):
    - `contract-tests` `test_cold_agent`: standing 4 — test-plan.md:127, :145, :148; architecture.md:141.
    - `ci-tally`: standing 3 — test-plan.md:8 and :145 state 94 and seven files; test-plan.md:161 is no-change (a
      different "seven files", slice s07).
- **Dev-tool versions:** none — `claude` re-read at `2.1.296 (Claude Code)` on the dev host (operator entry 9;
  `evidence/operator-9-claude-version.txt`), the version of the 2026-10-11 trial.
- **Harness / gate surface:** the cold-agent pipe (`scripts/cold-agent.sh`) swapped from the stub to the driver's own
  MCP surface.
  - Its verbs are unchanged: `run` · `status` · `cleanup` · `logs`.
  - `run` takes the task `stand`, and its exit grammar is unchanged (0 · 1 · 2 · 3). The precondition set is now
    `claude`, `python3` and the built `escher-session`.
  - `scripts/cold-agent.ps1` stays a pass-through (unchanged).
  - The stub `scripts/cold_agent_stub.py` is deleted, and the pipe copies nothing into the session dir.
  - The contract tests run under the `claude` shim and a fake `target/debug/escher-session` in the temp root. No live
    model, credential or built binary is used, and no helper is named `test_…`.
  - The 33 cases, one each:
    - a passing four-app run;
    - wrong calls counted from span lines and agreeing with the transcript;
    - a foreign tool, a permission denial and a spacer call each counted;
    - an unmet app, and an app with no final snapshot (a snapshot after its stop counts for none);
    - no check; a check step answered `isError`; a check expectation unmet;
    - a malformed check never replayed (an unknown tool, an extra key, non-object `arguments`, non-string `expect`,
      401 steps);
    - a check that cannot fail; a check missing an app;
    - an empty session; four isolation breaches (an extra tool, a second server, a cwd naming the root,
      `apiKeySource`);
    - a failed client, an unsuccessful result, a missing transcript; counts that disagree;
    - usage exit 2 (incl. `run counter`); a missing client and a missing binary each exit 3 before any state;
    - `status` / `logs` before a run; `status` reporting the last verdict;
    - the client argv and `mcp.json`; the prompt naming no stand id; the session outside the root and removed after;
    - events and verdict holding no scrub-set key and none of four planted markers (tool argument, tool answer,
      reply, check);
    - `cleanup` idempotent.
- **Cross-project / external claims:**
  - **CI:** CI#38713461945 (push) on `539defbae322f8a65bf0ef8ac7b7e3bd9750ef0d`, `verdict: green · checks 17/17 ·
    wall 809 s` (`ci.py conclusion --sha HEAD --wait 1500`, operator entry 14; `evidence/ci.md`). The verdict was
    taken on `539defba`; this wrap's commit adds to that tree.
  - **The model provider, reached once** on the dev host through the operator's own Claude Code login (`apiKeySource`
    none). The live run carried the stand's screen to it — element ids, accessible names and control values in
    snapshot texts and diffs — over 43 tool calls; no `screenshot` was called. This was the founder's «Один запуск,
    до $3 (Рекомендую)» (inputs#I2), fired on the operator's word (inputs#I7).
  - `inputs.py verify` (re-fired at this wrap after I8 · I9 were snapped, and again once this report cited them):
    `inputs: 9 entries — unchanged 1 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 8 ·
    uncited 0 · unparsed 0`. Before this report existed the same call read `uncited 4` (I6–I9).
    - I1 · message · copy · n/a — the operator's arguments at take-up.
    - I2 · `../additional/escher-overseer/relays/cold-agent-founder-answers.md` · copy no-repo · unchanged — the
      founder's four answers: «Один запуск, до $3 (Рекомендую)», «Все четыре в одной сессии», «JSON-проверка с
      повтором (Рекомендую)», «Отложить на 0.2.0, наблюдать (Рекомендую)».
    - I3 · message · copy · n/a — the P4 dialog's answer as it arrived.
    - I4 · message · copy · n/a — the operator's word on `.andromeda/runs/2026-10-15T08-01-57-phase/p5-card-1.md`;
      superseded by I6's card 2.
    - I5 · message · copy · n/a — who fires the live run: inside /implement, on the operator's word.
    - I6 · message · copy · n/a — **cited here (inputs#I6):** the operator's word «yes — apply.» on the governing card
      `.andromeda/runs/2026-10-15T08-01-57-phase/p5-card-2.md`. Its `## Beyond the plan` row, verbatim:
      > | 1 | A `.andromeda/residuals.md` line, target 0.2.0: whether `covered` and its remedy are the right words for a box with no height (`task-header-spacer`, refused `covered` on all four lean tasks; pinned by `stand_act_ids::refused_at_boot`). Its evidence is this chunk's live-run `spacer_calls` and the 2026-10-10 trial's 0 calls on it. CARRY 3 on working-route.md:100 is spent into it. | the wrap's route-resolve writes the line | the founder's «Отложить на 0.2.0, наблюдать (Рекомендую)» (inputs#I2); approved by the operator at card 1 (inputs#I4) |

      Authority: the founder's word (I2), approved by the operator (I4, I6) — not provisional.
    - I7 · message · copy · n/a — **cited here:** the operator's word firing entry 10, the founder's one live run,
      exactly once (/implement P2).
    - I8 · message · copy · n/a — **cited here:** the operator's directive for the operator pass. Respell the phase
      run's `.build.txt:13`, fire hygiene as written, the pre-CI commit, entries 13 and 14, write `evidence/ci.md`.
    - I9 · message · copy · n/a — **cited here:** the operator's word at this wrap's invocation, «run P1 only and
      stop» (the two-window form).
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:** none measured false.
  - The masters' cold-agent claims (the stub, `run counter`, `final_count`, `--max-budget-usd 1`, server `stub`, 27
    tests, 94 CI-script tests) describe the tree this chunk replaced. They are dispositioned under Expected
    amendments, not as disproofs.
  - Two plan forecasts read otherwise, neither a spec claim:
    - plan Implementation notes: «`arrowleft` is refused `malformed` … a likely counted wrong call on the timer» —
      the live agent sent `arrow-left` ×10, with 0 wrong calls;
    - the predicted counts «about 32» / «about 99» measured 33 / 100.
- **Expected amendments (from plan):** site sweep `cascade.py sweep --pre-pass --patterns-file
  {run_dir}/report-patterns.toml` (baseline HEAD `539defba`), nine patterns. `.andromeda/registries/**`: 0 hits for
  every pattern but `rust-log` (2 registry rows, both no-change — they read `RUST_LOG=trace` checks).
  design-system.md and a11y-plan.md: 0 hits for every pattern (no owner there).
  - **test-plan §3, Cold-agent run pipe** — the escher server behind the wrapper, the `stand` task and prompt, the
    restated verdict fields and reasons, the state list, the live run's Proof line. Carried: **Harness / gate
    surface**, **Schema / config**, **Symbols / APIs**, Outcome (the live verdict). Sites:
    - `stub` test-plan 6 rows (:122 :123 :124 :126 :127 :148);
    - `task-counter` :122; `budget` :122; `server-name` :122 :124;
    - `state-files` :122 :124 :126;
    - `cold-agent` 12 rows (:118 :120 :121 :122 :123 :124 :125 :126 :127 :145 :148 :201).
    - :201 already names `--allowedTools mcp__escher` (the 2026-10-11 trial) — re-read at P2, likely no-change.
  - **test-plan §4** — `test_cold_agent.py`'s classes and count, the CI-scripts tally. Carried: **Counts /
    qualifiers moved**. Sites: `contract-tests` :127 :145 :148; `ci-tally` :8 :145 (:161 no-change).
  - **security-plan §Input Validation** — the CLI arguments row's allowlist (`stand`), and the stub row replaced by
    the agent's check as a new input. The check is parsed by python `json`, bounded at 65,536 bytes and 400 steps,
    its verbs a closed set, and each step validated again by the server on the replay; it rests on the founder's
    «JSON-проверка с повтором (Рекомендую)» (inputs#I2).
    **§Threat Model Summary and §Secret Management, Storage → Development** — the live run now carries the stand's
    screen to the model provider, one run on «Один запуск, до $3 (Рекомендую)».
    **§Logging & Monitoring** — the cold-agent bullet's files.
    - Carried: **Coverage of new surfaces**, **Cross-project / external claims**, **Schema / config**.
    - Sites: `cold-agent` 6 rows (:75 :78 :292 :319 :321 :403); `stub` :78 :403; `state-files` :403; `server-name`
      :292 (already `mcp__escher`, from the trial).
    - A boundary widening (playbook :36), answered by the founder (inputs#I2) — Validate's to card.
  - **obs-plan §3, §6, §8, §9** — the verdict fields, `server.log` (counted by span name, never printed),
    `check.json`, the retired stub files, `RUST_LOG` set on the child. Carried: **Schema / config**, **Symbols /
    APIs**. Sites:
    - `stub` :87 :159 :160 :303 :332; `task-counter` :87;
    - `state-files` :87 :158 :161 :303 :332;
    - `cold-agent` 8 rows (:87 :156 :158 :159 :160 :161 :303 :332);
    - `rust-log` obs-plan :68 :70 :106 :149 :175 :296 :300 — :70 and :296 already read `RUST_LOG=info` set on stdio
      children; the rest are the sink's filter, no-change.
  - **architecture §Occupied Resources** — Names: the server `escher` replaces `stub`. Process-wide state: the live
    client's escher child and the replay's two children. Filesystem: `target/cold-agent/`'s new files. The directory
    tree without `cold_agent_stub.py`. Also **§Standard Contracts, CI contracts**.
    - Carried: **Symbols / APIs** (processes), **Schema / config** (state files), **Files**.
    - Sites: `stub` :66 :141 :149 :150 :154 :175; `server-name` :154; `budget` :141; `contract-tests` :141;
      `state-files` :141 :149; `cold-agent` 7 rows (:66 :141 :147 :149 :150 :154 :175).
    - `rust-log` :153 (§Environment variables) already registers `RUST_LOG` as escher-telemetry's filter: no new env
      var, a value set on a child the sink reads; :132 :150 :193 no-change.
  - **layout-templates §Surface: cli → Primary screens** — `cold-agent.sh`'s task `stand` and its server. Carried:
    **Harness / gate surface**. Sites: `cold-agent` / `state-files` :73.
  - Leaves (P2's cascade): `stub` 8 · `task-counter` 4 · `contract-tests` 6 · `state-files` 12 · `cold-agent` 31 ·
    `ci-tally` 1 · `rust-log` 19 · `server-name` 2 leaf rows. Among them are
    `.claude/rules/verification-harness.md:4-5` (its `paths:` glob names `scripts/cold_agent_stub.py`, deleted) and
    `:24-28`, `CLAUDE.md:17` and `:68`, `.claude/rules/testing.md:41`, and `.claude/docs/{commands,tests-summary,
    obs-summary,security-summary}.md`. Curation rows: `cold-agent` 2 (`.claude/docs/session-learnings.md:180-181`,
    historical).
  - A listing reaches its patterns' wording only; the detectors' own sweeps remain the control for reach.
- **Coverage of new surfaces:**
  - `the agent-written check (target/cold-agent/check.json, replayed)` → validation {mechanism✓: last fenced block,
    ≤ 65,536 B, JSON array, ≤ 400 steps, closed keys and verbs, each step validated again by the server} ·
    instrumentation {verdict counts `check_steps` / `check_expects`, booleans✓} · PII {n/a — never printed; the
    committed `live-check.json` holds the stand's fixture names and ids} · tests {unit✓ (shim cases), e2e✓ (the live
    run)} · a11y n/a · tokens n/a
  - `server.log (the escher server's stderr at RUST_LOG=info)` → validation n/a · instrumentation {span✓ — the
    `tool_call` span, counted by name} · PII {redacted✓ — escher's sink; never printed or copied} · tests {unit✓ (the
    fake writes span lines; `counts-disagree` case), e2e✓} · a11y n/a · tokens n/a
  - `the task argument stand (CLI)` → validation {allowlist✓, usage exit 2 first} · instrumentation {`run.start.task`✓}
    · PII n/a · tests {unit✓} · a11y n/a · tokens n/a
  - `the replay's two escher-session mcp children` → validation {the server's own `validate` per step✓} ·
    instrumentation {none — stderr to /dev/null by design} · PII n/a · tests {unit✓ (spawn count 3 per run, 1 when
    the check is malformed), e2e✓} · a11y n/a · tokens n/a
  - `the stand's screen to the model provider (the live run)` → validation n/a · instrumentation {verdict counts✓} ·
    PII {n/a — stand fixtures only; the transcript copy host-path-masked✓} · tests {e2e✓} · a11y n/a · tokens n/a

## Deviations from intent
1. **Reasons beside a missing or invalid check** (plan steps 5 and 7 leave this open):
   - `no-check` stands alone;
   - an invalid check reads `check-failed` alone (nothing replayed);
   - `check-cannot-fail` is added only after a replay.
   - Why: each reason names one fault; a check that never ran is not also "cannot fail".
2. **An accepted `stop` clears "the app started"** for `tasks_met`, so a snapshot after an app is stopped counts for
   no app (pinned by `test_an_app_without_a_final_snapshot_is_unmet`). Why: after `stop` no app is started, and the
   server answers such a snapshot `no-session`.
3. **`check.json` is written with `indent=2`** — the plan's "as parsed (`json.dump`)" named no layout. It is never
   printed.
4. **The prompt adds two clauses within plan step 4's content**: "starting and stopping each app in turn" and
   "before that app is stopped". It still names no id and no slug (`test_the_prompt_names_no_stand_id`).
5. **A replay child that fails** — no reply within 30 s, EOF, or an MCP error frame — reads `check_passed` false,
   and `check_can_fail` false for an ablated replay that fails.
6. **The operator pass ran inside the session** on the operator's directive (inputs#I8), before this wrap:
   - the respell of `.andromeda/runs/2026-10-15T08-01-57-phase/.build.txt:13`, a cargo `Compiling …` line holding the
     root path — the phase run's capture, respelled ×1 by `gate.py respell`, recorded in
     `evidence/operator-respell.txt`;
   - entry 12 fired as written (clean);
   - the pre-CI commit `539defba`; entries 13 and 14.
   - The two operator records written after the push (`operator-13-push.txt`, `ci.md`) are uncommitted and ride this
     wrap's commit.

scope record: none — `gate.py scope` clean, 0 recorded (`scope: clean — changed 3 · listed 3 · recorded 0`, base `01297a12`).

## Decisions & corrections
- **The founder's four answers** (inputs#I2): «Один запуск, до $3 (Рекомендую)» · «Все четыре в одной сессии» (not
  the recommendation) · «JSON-проверка с повтором (Рекомендую)» · «Отложить на 0.2.0, наблюдать (Рекомендую)». They
  shaped the run, the check and the `covered` deferral; the residuals row (I6's card) is P5's to write, its evidence
  `spacer_calls` = 0.
- **The operator's word that fired the live run** (inputs#I7): «The operator's word, on the founder's own ratification
  «Один запуск, до $3 (Рекомендую)» (inputs, 2026-10-15). Fire exactly once. Whatever it reads stands; no re-fire.»
  It was fired once; the result is `passed`.
- **The operator's directive for the operator pass** (inputs#I8) and «run P1 only and stop» (inputs#I9) — the
  two-window form. P2–P7 resume in this run dir (`.andromeda/runs/2026-10-15T09-44-57-wrap/`).
- **A measured baseline for 0.2.0** (`.andromeda/residuals.md:9`): the first cold run over the four lean tasks made
  43 tool calls with 0 wrong. By verb: start 4 · snapshot 8 · click 10 · type 6 · press 10 · advance 1 · stop 4; no
  `help`, `status`, `scroll` or `screenshot`. It cost 0.3197 USD and 48.8 s. The agent used `arrow-left`, not
  `arrowleft`, and never read `help`'s 18,354-byte answer. Evidence: `evidence/live-run.md`, `live-verdict.json`.
- **The agent's check holds expectations beyond the done atoms** (`value="24.12.2026"`, `value="10"`,
  `ListBoxOption "Mustermann, Max"`); it replayed green, and the ablated replay missed at least one expectation per
  app (`live-check.json`).
- **A test asserted a wrong count** (implement fix-loop, 1 iteration): `check_expects: 7` was hard-coded where the
  test's own EXPECT table sums to 6. The pipe was right; the literal became the table's sum. Evidence: unit gate
  firing 1, `gate.py show --n 2`.
- **The red-before-green control:** the 33 new contract tests run against HEAD `01297a12`'s pipe in a scratch copy
  read `FAILED (failures=33, errors=3)` — they bind to the swap (`outcome-2026-10-15-cold-agent-test.md`).
- **A sweep hazard — a cargo build capture holds the root path.** A phase run's saved `cargo build` output
  (`Compiling seven_guis v0.1.0 ({root}/examples/seven_guis)`) is an in-root host path. `gate.py hygiene` refused it
  until `respell` ran (I8).
- **A sweep hazard — a filter over a saved sweep capture drops its count lines.** At this wrap's P1 a `grep -v '
  leaf'` over the redirected `cascade.py sweep` capture also dropped the per-pattern count lines, which carry
  `leaf {n}`. The capture was then read whole. This is a recurrence of host-linux.md Session Additions 2026-10-13
  (a pipeline tool's output read whole, never filtered) — it recurred here, at wrap P1.
- **A guard refusal:** the Bash guard's `cd` arm refused a `cd` into the chunk's `inputs/` at /implement Setup. It
  was re-issued with absolute paths and ran.
- **A credential-scan reading of the transcript copy:** one hit for `api[_-]?key`, which is the init record's
  `"apiKeySource":"none"`. That is a field naming no key; the 2026-10-06 copy carries the same.
- **A recurring warning:** the per-package build prints blitz-paint's `variant Intrinsic is never constructed`
  warning. It recurred in the live entry's build log (`10.log`); it is the handoff's deferred learning, untouched.

## Outcome
**Acceptance criteria**, each re-asserted against the diff:
- **(tests) v010-15 — met.** The founder's one live run reads `passed`:
  - `isolated` true (the eleven `mcp__escher__*` tools, one server `escher`); `counts_agree` true;
  - `tasks_met` all four, each from the last `snapshot` answer of its app;
  - `check_passed` true and `check_can_fail` true;
  - `wrong_calls` 0, recorded and deciding nothing.
  - Recorded at `escher-0.1.0/chunks/2026-10-15-cold-agent-test/evidence/live-verdict.json`, a byte copy (`cmp`) of
    the entry's fresh artifact.
- **(tests) The contract tests and the CI-scripts leg — met.** `python3 -m unittest discover -s .github/scripts -p
  test_cold_agent.py` is green (`Ran 33 tests`) and `bash .github/scripts/ci-leg.sh ci-scripts` is green
  (`Ran 100 tests`); the step-9 cases each present.
- **(security) The pipe's CLI and prompt — met.**
  - Usage exits 2 with empty stdout for no verb, an unknown verb, `run counter` and an extra argument
    (`test_usage_errors_exit_2_with_empty_stdout`).
  - The prompt names no stand id (`test_the_prompt_names_no_stand_id`).
  - The census reads 0.
- **(obs) Events and verdict — met.**
  - Under the shim, events and verdict hold no scrub-set key and none of four planted markers (two tests).
  - The live `live-events.jsonl` and `live-verdict.json` hold counts, booleans, closed words and the eleven tool names.
  - `server.log` is counted by span name, never printed or copied.
- **(security) The committed evidence — met.**
  - The transcript copy masks the session dir and the home prefix; 0 e-mail addresses across the 10 evidence files.
  - The credential scan's 1 hit is `apiKeySource: none`.
  - `gate.py hygiene` read clean twice (I8; `evidence/operator-12-hygiene.txt`).
- **(arch) The pipe's footprint — met.**
  - The pipe writes under `target/cold-agent/` and its per-run dir only.
  - `cleanup` leaves `target/agent-run/` untouched (`test_cleanup_is_idempotent_and_touches_only_its_own_state`).
  - The stub is gone, and nothing under `packages examples tests apps wpt` or the manifests moved (gate read 0).
- **(tests) The pre-push gate, rustdoc and CI — met.** `ci-leg.sh fast` and `doc` are green; the pushed sha's CI
  reads `verdict: green` (CI#38713461945).
- **(tests) The observation the deferral reads — met.** `spacer_calls` 0 and `wrong_calls` 0 (`refused_calls` 0,
  `error_calls` 0) are recorded in `evidence/live-run.md`.

**Gates**, by `run`:
- `bash -n scripts/cold-agent.sh` — green.
- `python3 -m unittest discover -s .github/scripts -p test_cold_agent.py` — red at its first firing (the test's own
  wrong literal), then green (`exit 0` · `contains OK`; `Ran 33 tests`).
- `test ! -e scripts/cold_agent_stub.py` — green.
- `grep -c -E 'socket|…|ANTHROPIC_' scripts/cold-agent.sh` — green (`exit 1` · `last line 0`).
- `git diff --name-only 01297a12… -- packages examples tests apps wpt Cargo.toml Cargo.lock | wc -l` — green
  (`last line 0`).
- `bash .github/scripts/ci-leg.sh ci-scripts` — green (`Ran 100 tests`).
- `bash .github/scripts/ci-leg.sh fast` — green (138.98 s at /implement; 186 `test result: ok`, 0 FAILED).
- `bash .github/scripts/ci-leg.sh doc` — green (a cached rebuild, no Rust delta).
- `claude --version` — `leg = 'operator'`: green, `2.1.296 (Claude Code)`, history unmoved
  (`evidence/operator-9-claude-version.txt`).
- `cargo build --locked -p seven_guis --bin escher-session && bash scripts/cold-agent.sh run stand` —
  `leg = 'operator'`, fired once on inputs#I7.
  - green: every atom held (`exit 0`, `"outcome": "passed"`, `"isolated": true`, `"counts_agree": true`,
    `"check_passed": true`, `"check_can_fail": true`).
  - artifact fresh; 53.18 s; history unmoved (`evidence/live-run.md`).
  - Never re-run (the founder's one run).
- `grep -c '"outcome": "passed"' escher-0.1.0/chunks/2026-10-15-cold-agent-test/evidence/live-verdict.json` — red
  (exit 2) before step 11 wrote its input, as the plan states; then green (`last line 1`).
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` — `leg = 'operator'`, fired as written:
  `hygiene: clean` (`evidence/operator-12-hygiene.txt`).
- `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin
  build/escher-0.1.0` — `leg = 'operator'`: green, 134.54 s, `history moved:
  refs/remotes/origin/build/escher-0.1.0 01297a12→539defba` — the intended push (`evidence/operator-13-push.txt`).
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1500` —
  `leg = 'operator'`, fired as written: `verdict: green · checks 17/17` on `539defba`, CI#38713461945
  (`evidence/ci.md`).
- No `defer` entry, no deferral.
- Smoke: no boot smoke in the plan. The pipe's boot path ran end to end as the live entry.

**Watches:** none folded.

**Outcome basis:** the operator pass ran (inputs#I8).
- Commits: `539defbae chore(2026-10-15-cold-agent-test): operator pre-CI commit, for the run this chunk verdict
  reads`. The final HEAD's CI run is CI#38713461945, green 17/17 (`evidence/ci.md`).
- /implement's run (`.andromeda/runs/2026-10-15T09-13-03-implement/`) is held whole in this window. Its
  `outcome-2026-10-15-cold-agent-test.md` agrees: green + live. No other implement run exists for this marker.
- Post-implement artifacts: `evidence/operator-respell.txt`, `operator-12-hygiene.txt`, `operator-13-push.txt`,
  `ci.md`.

**Process hygiene:**

| process | started by | final state |
|---|---|---|
| the P1 shape probe's `escher-session mcp` | /implement | terminated |
| gate entries 1–8, 11 | /implement, through gate.py | terminated |
| the live client, its `sh` → `escher-session mcp` child and the two replay children (entry 10) | a leg (operator entry 10) | terminated |
| the operator pass's `fast` leg, `git push` and the `ci.py` poll | the operator pass, in this session | terminated |
| the code-graph refresh | this wrap's Setup | exited 0 |

Re-measured at this wrap: `ps -eo pid,comm,args` held no `escher-session`, `claude -p`, `cold-agent`, `seven_guis`
or `ci-leg` process.
## Addendum — resumed 2026-10-15
- inputs#I10 · message: the operator, at this wrap resume · copy · n/a — the operator's resume directive: P2 resumes in this run dir from this report, the wrap stops at the founder's answer card for the swap crossing; no mutation tier, no next skill, no version close or route run (the version-end boundary is the founder's call)
  - bears on **Cross-project / external claims**: adds to it (I9's «run P1 only and stop» is followed by this resume)
- inputs.py verify, re-fired: `inputs: 10 entries — unchanged 1 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 9 · uncited 1 · unparsed 0` (before this addendum was spliced, I10 its one uncited row); moved since the report: I10 added, uncited 1 → 0 with this addendum; no verdict moved for I1–I9

## Addendum — resumed 2026-10-15 (after the answer card)
- inputs#I11 · message: the operator, relaying the overseer, after the answer-card halt · copy · n/a — the founder's answer to this wrap's one answer card, «Подтвердить (Рекомендую)» = Confirm (all eight carded proposals apply), relayed verbatim by the overseer as his chosen option label; and the operator's directive to resume at P2 Escalate and wrap to the end — commit, push, read CI, report, stop; no next skill, no version close, no route run
  - bears on **Cross-project / external claims**: adds to it (the card at `.andromeda/runs/2026-10-15T09-44-57-wrap/answer-card.md` is answered)
- inputs#I12 · `../additional/escher-overseer/relays/cold-agent-founder-answers.md` · copy no-repo · unchanged — the founder's answers record, now with its section «The wrap's answer card»: his chosen label «Подтвердить (Рекомендую)», no words of his own
  - bears on **Cross-project / external claims**: adds to it (I2's record gained the card's answer)
- I2 · unchanged → drifted — DRIFT: the source's bytes moved; the record gained the section «The wrap's answer card» (snapped whole as I12); the four P4 answers I2 holds stand unchanged in it, read at I12
- inputs.py verify, re-fired: `inputs: 12 entries — unchanged 1 · drifted 1 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 10 · uncited 2 · unparsed 0` (before this addendum was spliced, I11 and I12 its two uncited rows); moved since the first addendum: I11 and I12 added, uncited 2 → 0 with this addendum, I2 unchanged → drifted

## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 01297a12 (the parent of the oldest pre-CI commit 539defba) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .github/scripts/test_cold_agent.py — added 363 line(s) in 58 range(s)
added: 2-3 · 5 · 19-26 · 28-131 · 137-138 · 140 · 143-144 · 148-149 · 154 · 159 · 162-164 · 168 · 173-174 · 180-184
       186 · 188 · 198-208 · 211-220 · 233 · 235-243 · 266-275 · 286-289 · 292 · 295 · 298-307 · 309-315 · 317 · 325
       327-330 · 332-335 · 337-339 · 342-348 · 351-352 · 354-404 · 407-415 · 419 · 422 · 425-426 · 429 · 433-436
       439-442 · 444-446 · 453 · 457 · 460 · 463-472 · 481 · 487-491 · 493-495 · 497-512 · 515 · 521-530 · 532-538
       548-549 · 551-552 · 554-555 · 562 · 569
- 29-43 «SCREENS = {»
  - 31-34 «"flight-booker": [»
  - 35-38 «"timer": [»
  - 39-42 «"crud": [»
- 44-49 «ACTS = {»
- 50-55 «EXPECT = {»
- 67-68 «with open(__SCENARIO__, encoding="utf-8") as f:»
- 69-70 «with open(__SPAWNS__, "a", encoding="utf-8") as f:»
- 80-103 «def call(name, arguments):»
  - 82-88 «if name == "start":»
  - 89-90 «if name in ("help", "status"):»
  - 91-92 «if name not in VERBS:»
  - 93-94 «if started is None:»
  - 95-97 «if name == "stop":»
  - 98-99 «if name in ("snapshot", "screenshot"):»
  - 100-101 «if any(value in refuse for value in arguments.values()):»
- 106-126 «for line in sys.stdin:»
  - 109-110 «if method == "initialize":»
  - 111-112 «elif method == "tools/list":»
  - 113-122 «elif method == "tools/call":»
  - 123-124 «else:»
- 137-138 «with open(args[args.index("--mcp-config") + 1], encoding="utf-8") as f:»
- 183-184 «if "check" in scenario:»
- 198-208 «def flow(counter_clicks=3, snapshot_apps=APPS):»
  - 201-207 «for app in APPS:»
- 211-220 «def stand_check(expect=None, apps=APPS):»
  - 215-219 «for app in apps:»
  - 266-275 «def run_scenario(self, calls=None, **knobs):»
  - 286-287 «def spawned(self):»
  - 342-348 «def test_a_tool_outside_the_escher_server_is_a_wrong_call(self):»
  - 354-357 «def test_a_call_on_the_header_spacer_is_counted(self):»
  - 359-361 «def test_an_unmet_app_fails(self):»
  - 363-366 «def test_an_app_without_a_final_snapshot_is_unmet(self):»
  - 368-372 «def test_a_reply_without_a_check_fails(self):»
  - 374-377 «def test_a_check_step_answered_is_error_fails(self):»
  - 379-381 «def test_a_check_expectation_not_met_fails(self):»
  - 383-395 «def test_a_malformed_check_fails_and_is_never_replayed(self):»
  - 397-400 «def test_a_check_that_cannot_fail_fails(self):»
  - 402-404 «def test_a_check_missing_an_app_fails(self):»
  - 410-411 «def test_a_tool_beside_the_escher_tools_is_not_isolated(self):»
  - 413-415 «def test_a_second_server_is_not_isolated(self):»
  - 439-441 «def test_counts_that_disagree_fail_the_run(self):»
  - 463-471 «def test_a_missing_server_binary_exits_3_before_any_state(self):»
  - 504-512 «def test_the_prompt_names_no_stand_id(self):»
### scripts/cold-agent.sh — added 269 line(s) in 26 range(s)
added: 2-10 · 14 · 16-27 · 36-37 · 39 · 45-63 · 108-115 · 133-134 · 138 · 148-308 · 316-317 · 323-326 · 329-337 · 340
       349-352 · 354-361 · 382 · 384-390 · 398-401 · 409-410 · 459-462 · 468 · 471 · 476-478 · 481 · 495
- 52-58 @53 «DONE = {»
- 108-113 «def text_of(content):»
  - 109-110 «if isinstance(content, str):»
  - 111-112 «if isinstance(content, list):»
- 148-157 «def span_counts():»
  - 150-152 «try:»
  - 153-154 «except OSError:»
- 160-162 «def done(app, text):»
- 165-181 «def tasks_met(transcript, uses):»
  - 168-180 «for record in transcript:»
- 184-193 «def extract_check(result):»
  - 187-188 «if not found or len(found[-1].encode("utf-8")) > MAX_CHECK_BYTES:»
  - 189-190 «try:»
  - 191-192 «except ValueError:»
- 196-204 «def valid_step(step):»
  - 197-204 «return (»
- 207-246 «class Server:»
  - 210-215 «def __init__(self, binary):»
  - 217-220 «def pump(self):»
  - 222-224 «def send(self, frame):»
  - 226-238 «def rpc(self, method, params):»
  - 240-246 «def close(self):»
- 249-266 «def replay(binary, steps):»
  - 252-262 «try:»
  - 263-264 «except (OSError, ValueError, EOFError, queue.Empty):»
  - 265-266 «finally:»
- 269-278 «def started_apps(check):»
  - 271-277 «for step in check:»
- 281-305 «def judge_check(check, binary):»
  - 282-284 @283 «if len(check) > MAX_CHECK_STEPS or not all(valid_step(step) for step in check):»
  - 287-294 «passed = (»
  - 298-304 «if ablated is not None:»
  - 330-334 «if check is not None:»
  - 335-336 «else:»
  - 384-385 «if check is None:»
  - 386-390 «else:»
  - 459-462 «if [[ ! -f "$BINARY" || ! -x "$BINARY" ]]; then»
### scripts/cold_agent_stub.py — deleted
