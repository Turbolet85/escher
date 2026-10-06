# Report — 2026-10-06-stand-test-contract

**Chunk:** Stand test contract — agent-invocable boot/run/status/cleanup + JSON-line logs for stand checks and blitz-tests
**Date:** 2026-10-06T03:41:22Z
**Commits:** `73fd624c chore(2026-10-06-stand-test-contract): operator pre-CI commit, for the run this chunk's verdict reads` (since last_wrap 2026-10-06T03:00:50Z; base `913f5918`)

## Changes (structured — detectors read this)
- **Files:** new `scripts/agent-run.sh` (mode 100755, `git ls-files -s`) · new `scripts/agent-run.ps1` · new `.github/scripts/test_agent_run.py` · modified `tests/blitz-tests/tests/dioxus_falsy_disabled.rs` (the `//!` doc lines 1-6 only; body byte-identical — gate `git diff -U0 913f5918 -- … | grep -c -v '^[+-](//!|$)'` read 0).
- **Symbols / APIs:** a new shell-level contract, `scripts/agent-run.sh {boot|run {selection}|status|cleanup|logs}` — invoked from the repository root; no Rust symbol created or changed (the only `.rs` edit is a doc comment).
  - **Exit grammar, every verb:** `0` success · `1` the verb ran and failed (cargo build failure, a failing run, an empty run) · `2` usage (missing/unknown verb, wrong arg count, unknown selection) · `3` a precondition is unmet (not booted; `logs` with no events file).
  - **`boot`** — calls the no-op `ensure_fresh_artifacts` first, recreates `target/agent-run/` (fresh `events.jsonl`), runs `cargo test -p blitz-tests --locked --no-run` (merged output → `run.log`), writes `status.json` (`booted`, `boot_ts`, `head` = `git rev-parse HEAD` or null), prints + appends `{"event":"boot","ts","outcome":"ready"|"failed","cargo_exit","head"}`; exit 0 ready / 1 failed (`booted` false).
  - **`run {selection}`** — `stand` = one `--test {stem}` per `tests/blitz-tests/tests/stand_*.rs` present · `all` = the whole package, no `--test` · `{name}` = one file, `^[a-z0-9_]+$` AND `tests/blitz-tests/tests/{name}.rs` exists; anything else exit 2. Order of checks (operator ruling, this session): usage/selection (2) BEFORE not-booted (3). Runs `cargo test -p blitz-tests --locked [--test …]`, merged output → `run.log` (truncated at the run's start); `status.json` run state `running` → `passed`|`failed`. Events, printed and appended: `{"event":"run.start","ts","selection","files":[…]}` (`files` = the `--test` stems passed — an EMPTY list under `all`, operator ruling) · one `{"event":"test","file","test","outcome":"ok"|"failed"|"ignored"}` per libtest line (`file` = the target binary's stem with its hash stripped, e.g. `stand_boot`, `blitz_tests` for the lib unittests, or `doc:{crate}` for doc-tests) · `{"event":"run.end","ts","selection","passed","failed","ignored","cargo_exit","outcome":"passed"|"failed"}`. `outcome` is `passed` only when cargo exited 0, `failed` = 0 and at least one test event parsed — otherwise `failed`, exit 1 (the empty-run rule). `run stand` with NO `stand_*.rs` present is an empty run: no cargo call, `cargo_exit: null`, outcome failed, exit 1 (operator ruling).
  - **`status`** — prints `{"event":"status","booted","boot_ts","head","run":{"selection","state":"none"|"running"|"passed"|"failed","passed","failed","ignored"}}` from `status.json`; exit 0 booted / 3 not (absent file or `booted` false); builds and runs nothing. Not appended to `events.jsonl`.
  - **`cleanup`** — removes `target/agent-run/` if present, prints `{"event":"cleanup","outcome":"done"}`; idempotent, exit 0; touches nothing else under `target/`.
  - **`logs`** — prints `events.jsonl` verbatim; exit 3 when absent. Raw cargo output stays in `run.log`, never printed.
  - Parser: libtest pretty lines (`Running … (target/…/{stem}-{hash})`, `Doc-tests {crate}`, `test {name} ... ok|FAILED|ignored[, reason]`); nothing between a `failures:` line and the next `test result:` is read, so captured stdout / panic text never reaches an event. JSON encoded by python3's `json` module (embedded in the script), never bash concatenation.
  - `scripts/agent-run.ps1` — Windows entry: `& bash "$PSScriptRoot/agent-run.sh" @args; exit $LASTEXITCODE`, no contract logic; untested on this host (`pwsh` absent — research.md Open questions).
  - Ports/sockets: none. Env vars read: none of its own (census gate `grep -c -E 'TcpListener|bind\(|listen\(|socat|HARNESS_|TMPDIR'` over both scripts read 0).
- **Crates / modules:** none (no workspace crate added or changed).
- **Dependencies:** none — `Cargo.lock` / `Cargo.toml` unchanged against `913f5918` (gate `git diff --quiet 913f5918 -- Cargo.lock Cargo.toml examples/seven_guis packages/blitz-test-harness .github/workflows .github/scripts/ci-leg.sh` exit 0). Host tools the script needs: `bash`, `python3` (already required by the `ci-scripts` leg), `cargo`, optional `git`.
- **Schema / config:** the harness event schema above (`boot` · `run.start` · `test` · `run.end` · `status` · `cleanup`) and `status.json`'s shape. Field names used: `event ts outcome cargo_exit head selection files file test passed failed ignored booted boot_ts run state` — none in the scrub's content-named set (`url href src html text value attrs path request error`; gate over the live `logs` output read 0 keyed lines). It is harness metadata, not an escher-telemetry sink; the stand installs none.
- **Spec-master edits:** none (the masters are untouched by implement; P2 amends).
- **Counts / qualifiers moved:** CI-scripts leg 23 → 37 tests (+14 `AgentRunTest` cases) — basis: `target/ci-logs/ci-scripts.log` "Ran 37 tests … OK" at implement's `ci-leg.sh fast`, the operator pass's `fast` (entry 22) and the runner's "Test CI scripts" job of run 37409303977 (evidence/operator-23-ci.txt); stated at test-plan §4 "the CI-scripts leg runs 23 tests" (`grep -n 'runs 23 tests' test-plan.md` → 1 hit, :114). Workspace tests 430 · 0 · 4 UNCHANGED (basis: summed `test result:` lines of `target/ci-logs/test.log` at implement's gate run) — test-plan §9 Local baseline's 430 · 0 · 4 stays true.
- **Dev-tool versions:** none — no host tool installed or changed (`python3` 3.14.7, bash 5.3.15 read on the dev host, not new).
- **Harness / gate surface:** the 5-command agent-run contract above is NEW (test-plan §3's NOT YET MEASURED 5-command marker; health check 13 now reads ✓ — gate entry `health.py check … --style agent-driven` contains `check 13 · ✓`). State area `target/agent-run/{status.json,events.jsonl,run.log}` (gitignored under `target/`). New CI-scripts test file `.github/scripts/test_agent_run.py`, discovered by the existing `ci-scripts` leg (`python3 -m unittest discover -s .github/scripts`); no CI workflow or leg changed. The test-data bootstrap part of §3's marker remains unmeasured.
- **Cross-project / external claims:** CI run **37409303977** on `73fd624c` (the pre-CI commit): `verdict: green · checks 16/16 · wall 472 s`, all 16 jobs success, its "Test CI scripts" job `Ran 37 tests … OK` (evidence/operator-23-ci.txt). Wall 472 s against 1104 s for the previous pipeline (run 37404017734) — this push changed no `Cargo.lock`; the cause of either figure is not measured. `inputs.py verify`: `inputs: absent` — none — no external input snapshotted.
- **Reverted / negative API facts:** a `status` branch carrying a stray `TMPDIR_FALLBACK` reference was written and removed before any gate ran (it would have tripped the census gate); a helper named `test_files` was collected by unittest as a 15th test and renamed `add_test_files` before the gate run.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:** none new. (The PREREQ's false claim lived in the test's own doc, not a master — `grep -F 'focus order'` over the seven masters: 0 hits; a11y-plan §5 :145 already states `disabled="false"` "matches `:disabled` yet stays focusable" correctly.)
- **Expected amendments (from plan):**
  - test-plan §3 — the 5-command contract measured: **carried** (Symbols / APIs + Harness / gate surface; site `grep -n '5-command' test-plan.md` → 1 hit :100, the NOT YET MEASURED marker; the marker narrows to the test-data bootstrap). Its leaf `.claude/rules/verification-harness.md` "The 5-command contract — NOT YET MEASURED" re-derives by cascade.
  - test-plan §4 — `test_agent_run.py` joins the CI-scripts tests: **carried** (Counts / qualifiers moved; site `grep -n 'LegScriptTest' test-plan.md` → 1 hit :114).
  - obs-plan §3/§6 — the harness JSON-line event schema, location and reach: **carried** (Schema / config; sites `grep -n 'NOT YET MEASURED' obs-plan.md` → :83 "a JSON log schema, log file location" and :142 "a log JSON schema, a log-file sink"; §6 artifact table beside `target/ci-logs` :302-303 — `grep -c 'target/ci-logs' obs-plan.md` → 2).
  - architecture §Occupied Resources → Filesystem + §Infrastructure Patterns / §Standard Contracts: **carried** (Harness / gate surface; sites `grep -n 'target/ci-logs' architecture.md` → 3 hits, :147 Filesystem among them; `CI contracts` :139).
  - security-plan §Logging & Monitoring — the harness log carries no content-named field and no captured output; `run.log` unscrubbed under `target/`: **carried** (Schema / config + Coverage; site `## Logging & Monitoring` :342 — `grep -c 'agent-run' security-plan.md` → 0, a new row).
- **Coverage of new surfaces:**
  - `scripts/agent-run.sh` verbs → validation {selection `^[a-z0-9_]+$` + file-exists; verb allowlist; arg count ✓} · instrumentation {JSON-line events ✓} · PII {n/a — no user content; captured test output kept out of events ✓} · tests {unit: 14 shim cases; integ: live gates cleanup→status→boot→status→run stand→logs→run all→cleanup ✓} · a11y {n/a} · tokens {n/a}
  - `target/agent-run/run.log` → validation n/a · instrumentation n/a · PII {raw✗ by design — unscrubbed cargo/libtest output, like `target/ci-logs/`; never printed by `logs`, gitignored} · tests {unit: `test_run_log_holds_the_raw_cargo_output`} · a11y n/a · tokens n/a
  - `scripts/agent-run.ps1` → tests {unrunnable-here — `pwsh` absent on the dev host; Windows CI runs `cargo test` directly, not this wrapper} · others n/a

## Deviations from intent
- Three points step 2 left open, settled in the impl and ruled by the operator this session ("your three choices stand … the wrap writes them into test-plan §3 as built"): (1) `run.start.files` is `[]` under `all`; (2) `run stand` with no `stand_*.rs` files is an empty run — no cargo call, exit 1; (3) a usage error (2) is checked before not-booted (3).
- `cleanup` and `status` events carry no `ts` — the plan's shapes, kept exactly.
- scope record: none — `gate.py scope` clean, changed 4 · listed 4 · recorded 0.

## Decisions & corrections
- Operator ruling (2026-10-06, this session): the three settled points above stand as built.
- Operator directive: the session drives the operator pass (entries 21-23) itself, then stops for the wrap — done: hygiene clean, pre-CI commit `73fd624c`, push `913f5918..73fd624c`, CI green.
- Sweep hazard: a Python `unittest` helper whose name starts with `test_` (`test_files`) is collected as a test — caught only because the run's count (15) disagreed with the methods written (14).
- Sweep hazard: the gate census pattern `TMPDIR` matches any substring (a `TMPDIR_FALLBACK` variable would have counted 1).

## Outcome
- Acceptance, re-asserted against the diff:
  - (tests) contract tests: MET — `python3 -m unittest discover -s .github/scripts -p 'test_agent_run.py' -v` green, 14 cases.
  - (tests) live block order: MET — status 3 after cleanup, boot 0 `"ready"`, status 0, `run stand` 0 with `run.end`, 13 `ok` stand events, every line JSON.
  - (a11y) `run all` 15 `ok` over the three a11y-leg files; `ci-leg.sh a11y` green, selection unchanged: MET.
  - (tests) cleanup ×2 + `test ! -e target/agent-run`: MET.
  - (security/obs) no scrub-set key, no captured output in events: MET (live probe 0; unit case asserts the panic marker absent).
  - (arch/security) no listener/socket/`HARNESS_`/`TMPDIR`, state under `target/agent-run/` only: MET (census 0).
  - (arch) the guarded paths unchanged against `913f5918`: MET.
  - (tests) health check 13 ✓: MET.
  - (a11y) PREREQ doc: MET — no `focus` in the `//!` doc, `:disabled` still named, only `//!` lines changed, the test passes (1 passed).
  - (tests) `ci-leg.sh fast` + `doc` exit 0; workspace 430 · 0 · 4 unchanged; ci-scripts measured 37: MET.
  - (tests) CI on the pushed sha `verdict: green`, run 37409303977: MET.
  - No capability claimed: MET (`matrix.py show --chunk` → claimed 0).
- Gates (implement P2, run dir 2026-10-06T03-23-46-implement; 20 green · 0 red · 3 operator):
  - `python3 -m unittest discover … test_agent_run.py -v` green · `agent-run.sh cleanup` green · `agent-run.sh status` green (exit 3) · `agent-run.sh boot` green (`"ready"`) · `agent-run.sh status` green · `agent-run.sh run stand` green (`"run.end"`, artifact fresh) · `logs | … stand ok count` green (13) · `logs | … scrub-key count` green (0) · `run all … a11y ok count` green (15) · `cleanup && cleanup && test ! -e` green · `health.py check … agent-driven` green (`check 13 · ✓`) · script census green (exit 1, 0) · regression-test diff probe green (exit 1, 0) · `//!` focus probe green (exit 1, 0) · `//!` `:disabled` probe green · `cargo test … --test dioxus_falsy_disabled` green (1 passed) · `git diff --quiet 913f5918 -- …` green · `ci-leg.sh fast` green · `ci-leg.sh doc` green · `ci-leg.sh a11y` green.
  - `leg = 'operator'`: `gate.py hygiene` → exit 0, `hygiene: clean` (evidence/operator-21-hygiene.txt) · `ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin build/escher-0.1.0` → exit 0, pushed `913f5918..73fd624c` (evidence/operator-22-push.txt) · `ci.py conclusion --sha HEAD --wait 1800` → exit 0, `verdict: green`, run 37409303977 on `73fd624c` (evidence/operator-23-ci.txt).
  - Smoke: skipped — no boot-path / UI-surface change; the contract ran live as gate entries.
- Watches: none folded.
- Outcome basis: the operator pass's final state — commit list `73fd624c` (one pre-CI commit, no fix commits) and CI run 37409303977 on that HEAD; implement's P4 report (this conversation) for the rest.
- Process hygiene: implement P4 census — `none left running` (host `ps` grep for cargo/rustc/agent-run read empty after the gate run; `target/agent-run/` absent after the cleanup entry). The operator pass's `ci-leg.sh fast` and `ci.py` ran in the foreground / a completed background task — both exited.
