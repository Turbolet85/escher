# Report — 2026-10-20-load-wait

**Chunk:** Load wait — an acting command meeting a load in flight waits it out within a stated bound, fixture-proven (v020-07)
**Date:** 2026-10-20
**Commits:** `8dfd1c02b chore(2026-10-20-load-wait): operator pre-CI commit, for the run this chunk's verdict reads` (since `last_wrap` 2026-10-20T01:30:32Z; `git log --format='%h %s' 165cfe443..HEAD`)

## Changes (structured — detectors read this)
- **Files:** `packages/escher-driver/src/session.rs` · `packages/escher-driver/src/execute.rs` ·
  `packages/escher-driver/src/schema.rs` · `packages/escher-driver/src/schema_tests.rs` ·
  `packages/escher-driver/src/lib.rs` · `tests/blitz-tests/tests/stand_load_wait.rs` (new, 164 lines) — the six code
  files (`gate.py scope`: `clean — changed 6 · listed 6 · recorded 0`); beside them the ledger
  `escher-0.2.0/verification-matrix.json` (v020-07), the chunk folder (`plan.md`, `research.md`, `scope.md`,
  `evidence/counts.md`, `evidence/ci.md`), the run dirs `.andromeda/runs/2026-10-20T01-41-31-phase/` (its
  `p5-controls.sh` lines 4–6 respelled at the operator pass, below) and `.andromeda/runs/2026-10-20T02-02-07-implement/`.
- **Symbols / APIs:**
  - `escher_driver::LOAD_WAIT` — new `pub const LOAD_WAIT: Duration = Duration::from_secs(5)`
    (`packages/escher-driver/src/session.rs:19`, inside the added block 16-53), re-exported at the crate root
    (`packages/escher-driver/src/lib.rs:158`: `pub use session::{LOAD_WAIT, Session};`). Doc: how long an acting step
    waits for a load in flight before it returns with the load still outstanding, 5 s of wall time, no time moving on
    the held instance.
  - `LOAD_POLL` (private, 10 ms, `session.rs:22`); `settle_waiting` (crate-private, `session.rs:24-29`) — settles the
    held harness through `Harness::settle` and, while the result is `Err(NotSettled { busy: Busy::Loads })`, settles
    again every `LOAD_POLL` until the load is answered or `LOAD_WAIT` has passed; `settle_within` (private generic loop
    over a settle closure, a bound and a poll, `session.rs:31-52`): returns the first result that is not `Loads`; the
    last sleep is `min(poll, time left)`, then one more settle, so a never-answered wait returns at or past the bound
    and within one settle of it; a `Render` or `Layout` result returns at once. It reads `std::time::Instant` and calls
    `std::thread::sleep` and nothing else — no harness `tick`, no `advance_time`, no `TimeStep`, no
    `take_changed_nodes`, no print, no `tracing` call.
  - `Session::act` (`session.rs:236`) now settles through `settle_waiting`; its doc (`session.rs:233-234`) reads «A load
    in flight is waited for up to [`LOAD_WAIT`], settling again as it is answered; no time moves while it waits».
  - `execute::settled_step` (`execute.rs:462`) settles through `settle_waiting`, so all five acting verbs (`click` ·
    `type` · `press` via `acted`, and the `advance` and `scroll` arms) wait on every carrier — in process, the socket,
    the command line and MCP. `snapshot` and `screenshot` run no step and do not wait. `Session::run`'s doc
    (`execute.rs:126-127`) moved from «Settling moves no time and waits on no load» to «Settling moves no time; a load in
    flight is waited for up to [`LOAD_WAIT`](crate::LOAD_WAIT)».
  - `Harness::settle` is unchanged and keeps its other callers (blitz-test-harness's unit tests, `stand_settle`,
    `stand_act_ids`, `font_face_sniffed_source` — research.md §Graph impact); its two production callers are exactly
    the two driver sites above, both now through the wait.
  - On a waited step the `command` span's `passes` is the passes of the settle that went quiet (recorded on `Ok` only,
    as before).
  - The `settled` result field's help (`schema.rs:121`, the `SETTLED` `FieldSpec` every acting verb carries) now reads
    «whether the app went quiet after the step, a load in flight waited for up to 5 s» — the program's own fixed table
    text, so it reaches `help`'s self-description and each acting MCP tool's description of its result fields.
- **Crates / modules:** none added or removed; escher-driver's `session` and `execute` modules changed.
- **Dependencies:** none (gate: `git diff --name-only 165cfe44… -- … Cargo.toml Cargo.lock … | wc -l` reads 0).
- **Schema / config:** no verb, argument, result field, `Outcome` variant, refusal cause, session error, exit status,
  span field, env read or listener added; the describe count row `[11, 13, 12, 3, 8, 4, 15]` unmoved (escher-driver
  unit gate). One help text changed (`settled`, above).
- **Spec-master edits:** none.
- **Retired:** none. (Two recorded statements stop being true — «settling … waits on no load» for the driver and «no
  driver command waits on a load»; their sites are under Counts / qualifiers moved, fact `wait`.)
- **Counts / qualifiers moved:** (sites from `cascade.py sweep --pre-pass --patterns-file
  .andromeda/runs/2026-10-20T03-00-02-wrap/report-patterns.toml`, trail `cascade-2026-10-20-load-wait-pre-pass.json`,
  baseline 8dfd1c02; each fact names its pattern id; a master count is its `standing` figure)
  - **wait** — an acting driver command now waits on a load in flight up to `LOAD_WAIT` (5 s), moving no time;
    `Harness::settle` itself still never waits. Pattern `wait`: standing 4 · leaf 5.
    `architecture.md:182@c8318` (Test harness settle «never waits on one») no-change — settle itself is unchanged ·
    `architecture.md:192@c38779` («settling moves no time and waits on no load», Driver session) changed ·
    `security-plan.md:222@c2` label (the row's name «Settle and loads in flight») · `security-plan.md:222@c187` («A
    harness settle (`Harness::settle`, and the driver session's `act` over it) … never waits on one … so the absent
    request timeout above cannot hang it») changed — the driver's `act` now waits, bounded by `LOAD_WAIT`, which is
    what still keeps the absent request timeout from hanging a command · `security-plan.md:222@c2234` («no driver
    command waits on a load (the operator, 2026-10-07)») changed · `test-plan.md:91` (settle «waits on no load»)
    no-change — settle.
  - **reexp** — escher-driver's crate root re-exports 55 → 56 names (`LOAD_WAIT`; `lib.rs:158`). Pattern `reexp`:
    standing 1 — `architecture.md:192@c48` changed.
  - **tfiles** — blitz-tests `.rs` files 140 → 141, test targets 139 → 140 (`ls tests/blitz-tests/tests/*.rs | wc -l`
    reads 141). Pattern `tfiles`: standing 1 — `test-plan.md:68@c3405,c3422` changed.
  - **screaders** — `session_common` readers 26 → 27 (`stand_load_wait` declares `mod session_common;` and reads
    `Acted`, `act`, `click`, `snapshot`). Pattern `screaders`: standing 2 — `test-plan.md:68@c1349` changed ·
    `test-plan.md:103@c2926` changed.
  - **fxreaders** — `fixtures` readers 20 → 21, `stand_*` readers 19 → 20 (`stand_load_wait` reads `HeldNet`). Pattern
    `fxreaders`: standing 3 — `test-plan.md:68@c2501` changed · `test-plan.md:104` changed · `obs-plan.md:79` («its 20
    readers», the shared module's silence census) changed.
  - **runstand** — `run stand` 183 · 0 · 7 over 45 files → 185 · 0 · 7 over 46 (`ls tests/blitz-tests/tests/stand_*.rs |
    wc -l` 46; gate `bash scripts/agent-run.sh run stand`). Pattern `runstand`: standing 4 — `test-plan.md:119@c10078`
    (history of readings) changed — a new reading appended · `test-plan.md:132` changed · `test-plan.md:323@c1299` («45
    files») changed · `test-plan.md:352` («the 45 `stand_*.rs` files») changed.
  - **duunit** — escher-driver unit tests 94 → 99, still sixteen files: `session.rs` 6 → 10, `schema_tests.rs` 6 → 7;
    inline 60 → 64, `#[path]` files 34 → 35 (gate `cargo test -p escher-driver --locked`: `99 passed`). Pattern
    `duunit`: standing 2 — `test-plan.md:22@c21` changed · `registries/contracts/test-plan/session-lifecycle.md:11@c5536`
    (per-file list) changed.
  - **wstally** — workspace tally 201 result lines · 1055 passed → 202 · 1062 · 0 failed · 12 ignored (gate on
    `target/ci-logs/test.log`: `lines 202 passed 1062 failed 0 ignored 12`; +1 line, +2 `stand_load_wait`, +5
    escher-driver). Pattern `wstally`: standing 2 — `test-plan.md:333@c3944` changed · `test-plan.md:339@c18738`
    changed.
  - **ostally** — per-OS CI tallies, CI#39348671810 → CI#39360124515 (`evidence/ci.md`): Test [default features] 201 ·
    1055 → 202 · 1062; macos and coverage 182 · 1052 → 183 · 1059; windows 182 · 1010 → 183 · 1017 (· 0 · 9); a11y 6 ·
    43 unmoved; Stand checks repeated 159 · 639 · 0 · 21 → 162 · 645 · 0 · 21. Pattern `ostally`: standing 3 —
    `test-plan.md:333@c3877,c3984,c4013,c4088` changed · `test-plan.md:351@c2375` (the coverage job's run) changed ·
    `test-plan.md:352` ×2 (the stand repeat's reading) changed.
  - **cover** — escher-driver coverage 4634/4732 = 97.93 % → 4695/4794 = 97.93 %, the 97 % floor holding, 3 of 3 crates
    (CI#39360124515's coverage job, `evidence/ci.md`). Pattern `cover`: standing 1 — `test-plan.md:351@c2240` changed.
  - **settled** — the `settled` field's help states the 5 s bound (`schema.rs:121`). Pattern `settled`: standing 1 —
    `architecture.md:192@c11697` («present only when `settled` is false», the `busy` field) no-change — true; the help
    text itself is quoted by no master (0 hits for «whether the app went quiet»), its home the Driver session contract
    and layout-templates §Surface: cli (Expected amendments).
  - **clockrd** — escher-driver's `session.rs` now reads a wall clock (`Instant::now`) and sleeps, for the load wait and
    nothing else; `execute.rs`, `schema.rs` and every other module stay clock-free (probe `grep -rln -E
    'Instant::now|thread::sleep' packages/escher-driver/src`: `client.rs host.rs mcp.rs session.rs`). Pattern
    `clockrd`: standing 6 — `architecture.md:182@c8485` (settle reads no clock but the harness's) no-change ·
    `architecture.md:192@c24965` («its executor, session, schema, validation and JSON writer … read no clock or
    environment») changed — the session now reads one · `security-plan.md:83@c2692` (validation reads no clock)
    no-change · `security-plan.md:83@c4819` (the executor «reads no clock and keeps no `NodeId` between calls») changed —
    the executor's settle now reads the wall clock through `settle_waiting` · `security-plan.md:83@c8549` («executor,
    session … read no clock … the clocks are the host's … and … the MCP server's») changed — the session's wait joins
    them · `security-plan.md:222@c1861` (validation reads no clock) no-change · `test-plan.md:91` (settle) no-change ·
    `obs-plan.md:70@c2807` («its `host` reads a clock for the idle expiry, and its MCP server one when a request
    arrives») changed.
  - **sleep** — a second recorded exception to the session checks' «no sleep and no wall-clock read»: `stand_load_wait`
    reads `Instant::now()` around its call and its stand-in thread sleeps 200 ms. Pattern `sleep`: standing 1 —
    `test-plan.md:185@c2589,c2665` changed.
  - **motion** — the wait moves no clock, so a timer or an animation does not run during it. Pattern `motion`:
    standing 1 — `design-system.md:290` ×2 («a harness settle … none holds it open and none is waited on or
    advanced») no-change — true of settle; the wait's own sentence is the Expected amendment.
  - **matrix** — v020-07 planned → implemented (ref the two `stand_load_wait` tests): no master site — a ledger entry.
  - Sweep reach: 14 patterns · 14 moved facts, 14 with a pattern (the report's own count) · owner pattern control
    unfired (`control: title never fired over the seven masters' pre-pass text (8dfd1c02)`) · by line 5 lines · 9
    offsets.
- **Dev-tool versions:** none.
- **Harness / gate surface:** none changed — `stand-repeat` globs `stand_*.rs` (`.github/scripts/ci-leg.sh:48-59`), so
  the new check joined it with no script change; `agent-run.sh` unchanged.
- **Cross-project / external claims:** CI#39360124515 (push, sha `8dfd1c02b090`, the operator pre-CI commit):
  `verdict: green · checks 17/17 · wall 1127 s` (`ci.py conclusion`, `evidence/ci.md`). `inputs.py verify`: `inputs:
  absent — no external input snapshotted` — none.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:** none.
- **Expected amendments (from plan):**
  - architecture §Standard Contracts → Driver session (an acting verb's settle waits on a load up to `LOAD_WAIT`, moving
    no time; «settling … waits on no load» and «no driver command waits on a load» retire; re-exports 56) — carried:
    Symbols / APIs, facts `wait` · `reexp` · `clockrd` (`architecture.md:192`; patterns `wait` 1 changed of 4,
    `reexp` 1, `clockrd` 1 of 6 in architecture).
  - security-plan §API Security, the «Settle and loads in flight» row (the absent request timeout still cannot hang a
    command, because `LOAD_WAIT` ends every wait; settle itself never waits) — carried: fact `wait`
    (`security-plan.md:222@c187`; pattern `wait`).
  - security-plan §Input Validation, Driver command schema row (session.rs reads a wall clock for the wait; «no driver
    command waits on a load» → «an acting command waits on a load up to 5 s»; still no argument or result field of a
    wait) — carried: facts `wait` (`security-plan.md:222@c2234`) · `clockrd` (`security-plan.md:83@c4819,c8549`).
  - obs-plan §3 (session.rs joins the clock readers, for the wait and nothing else) — carried: fact `clockrd`
    (`obs-plan.md:70@c2807`).
  - obs-plan §4 (the «settle wait» absent line; `passes` on a waited step the passes of the settle that went quiet) —
    carried: Symbols / APIs (`passes`) and Schema / config (no span field); the site sweep holds no pattern for a «settle
    wait» absent line — the obs-plan detector's own sweep is the control for its wording.
  - test-plan §1, §3, §5, §9, §10 (`stand_load_wait` and its in-file self-answering stand-in; its wall reads the second
    recorded exception; the counts move) — carried: facts `tfiles` · `screaders` · `fxreaders` · `runstand` · `duunit`
    · `wstally` · `ostally` · `cover` · `sleep` (test-plan hits: 1 · 2 · 2 · 4 · 1 · 2 · 3 · 1 · 1; registry
    `session-lifecycle.md:11` 1 for `duunit`).
  - design-system §Motion §Animation runtime (the wait moves no clock, so a timer or an animation does not run during
    it) — carried: fact `motion` (`design-system.md:290`, standing 1, a true claim the amendment adds beside).
  - layout-templates §Surface: cli → Primary screens (the `settled` help, and with it each acting tool's description,
    states the 5 s bound) — carried: fact `settled`, Symbols / APIs; pattern `settled` 0 hits in layout-templates
    (`settled: standing 1`, its one row in architecture) — the layout-templates detector's own sweep is the control.
- **Coverage of new surfaces:**
  - `LOAD_WAIT` / the load wait (`settle_waiting`, every acting verb on every carrier) → validation n/a (no input: a
    constant, not an argument) · instrumentation `command` span✓ (unchanged eight fields; `passes` of the quiet settle;
    no new field, by obs-plan §4) · PII n/a (no field, no log) · tests unit (`session::tests` 4, `schema::tests` 1) +
    integ (`stand_load_wait` 2, both layout modes) · a11y n/a · tokens n/a
  - `settled` help text → validation n/a · instrumentation n/a · PII n/a (fixed `'static` text) · tests unit (the pin)
    · a11y n/a · tokens n/a

## Deviations from intent
- Plan step 1's stopping rule: the plan says the loop returns the last `Loads` «once the next poll would pass the
  bound» and also «never sleeps past `LOAD_WAIT`, so the wait ends within one settle of the bound». Read literally the
  first returns before the bound, which the arm-2 criterion «no earlier than `LOAD_WAIT`» contradicts. Built: the last
  sleep is `min(poll, time left)`, then one more settle, so a never-answered wait returns at or past the bound and
  within one settle of it (measured: both arms green in both modes, `stand_load_wait` 10.10 s).
- Unit test (c) asserts one settle and no wall time: a sleep only ever precedes another settle, so `settles == 1`
  proves it slept none; a wall-time assertion would be a CI flake source.
- The help-against-value pin (step 5) also asserts that the five acting verbs carry the `SETTLED` field and that
  `LOAD_WAIT` is whole seconds, so the help's «5 s» is the value's exact rendering.
- The check (step 8) adds two assertions beyond the plan's list: arm 1's wall time is also at least the stand-in's
  200 ms (the step did wait), and arm 2's next snapshot still reads the box at no height. `box_height` reads the
  driver's `snapshot` text line for `fx-box`.
- `execute.rs`'s doc links `LOAD_WAIT` by path (`crate::LOAD_WAIT`) rather than importing it: rustc read an import used
  only by a doc link as unused (`unused import: LOAD_WAIT`, the edit hook's diagnostic).
- scope record: none — `gate.py scope` clean, 0 recorded (`clean — changed 6 · listed 6 · recorded 0 (companion 0 ·
  mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 50`).

## Decisions & corrections
- The operator pass's hygiene entry (`gate.py hygiene`) read red on its first firing: `hygiene: refused 1 files — P1
  1`, the file `.andromeda/runs/2026-10-20T01-41-31-phase/p5-controls.sh` — phase P5's control script held the repo
  root, a session scratchpad path under `/tmp/claude-1000/…` and a home path on lines 4–6 (rows `in-root,tmp,home`, so
  `gate.py respell` could not take it). The operator's word (this session, 2026-10-20): «Yes: rewrite those three lines
  of p5-controls.sh in the run-dir spelling as you propose (ROOT=. · C=$TMPDIR/{scratchpad}/clone-load-wait ·
  GATE=~/.claude/skills/andromeda-tools/scripts/gate.py), name the respell in the pre-CI commit body, re-fire 17, and
  carry on through the commit and 18–20 if it reads clean». Re-fired: `hygiene: clean`; the commit body names it. A
  script a phase run writes into its run dir takes the run-dir spelling at write time — `respell` cannot take a tmp or
  home row.
- The operator's standing instruction for the pass: «Stop and tell me if any entry is red» — followed: the pass stopped
  at entry 17's red and resumed only on the operator's word.
- Plan step 1's two stopping clauses disagreed; resolved toward the criterion (Deviations), recorded as an evolve
  `input.plan-step-ambiguous` friction (`.andromeda/friction-log.ndjson`, id `2026-10-20T02:07:01Z-b`).
- Sweep hazard (host): the Bash tool's shell has `$TMPDIR` unset, so `> "$TMPDIR/claude-cov.log"` became `> /claude-cov.log`
  and failed `Permission denied` — the CI log was saved to the session scratchpad instead. A redirect target is spelled
  absolutely, never through `$TMPDIR` (gate-contract §Tool already names `$TMPDIR` as the OS temp dir's NAME, never the
  shell variable).
- Sweep hazard (search): a whole-tree `grep -rn` from the repo root ran past the Bash tool's 120 s and was moved to the
  background (it walks `target/`); `git grep` answered the same question at once. Search tracked files with `git grep`.
- rustc reads an import used only by an intra-doc link as unused (`unused import: LOAD_WAIT` on `execute.rs:16`); a
  doc-only reference takes the path form `[`X`](crate::X)`.
- A `stand_*` check may now sleep and read the wall clock where its subject is wall time — the plan's narrowly stated
  second exception beside `stand_virtual_clock`'s 800 ms sleep (plan.md Implementation Steps 8; the test-plan §5
  Expected amendment).

## Outcome
- (tests) v020-07 — met: `stand_load_wait` reads 2 passed, both arms in both layout modes through `Session::run`
  (`a_load_answered_inside_the_bound_ends_the_step_settled_with_its_content_on_screen`: `settled` true, no `busy`, the
  diff naming `fx-box` changed with `Bounds` in `differs` at height 70, the next snapshot reading `70`, wall time ≥
  200 ms and under `LOAD_WAIT`; `a_load_never_answered_ends_the_step_at_the_bound_still_loading`: `settled` false,
  `busy == Some(Busy::Loads)`, wall time ≥ `LOAD_WAIT` and under `LOAD_WAIT` + 2 s, `HeldNet::held()` 1).
- (arch) the wait moves no virtual time — met: `session.harness().time()` reads the same before and after each arm's
  call (both tests).
- (arch) `Harness::settle` unchanged, blitz-dom gains no wall read — met: the `git diff` probe reads 0, the blitz-dom
  `Instant::now|SystemTime::now` probe reads 0.
- (security) `LOAD_WAIT` is 5 s and under `ANSWER_BOUND` (30 s), stated in the `settled` help — met: pinned by
  `schema::tests::the_settled_help_states_the_load_wait_and_the_wait_ends_before_an_answer_read`.
- (security) nothing crosses that did not before — met against the diff: no verb, argument, result field, refusal
  cause, session error, exit status or `Outcome` variant (describe row `[11, 13, 12, 3, 8, 4, 15]` unmoved); no
  manifest or lock change; the wall read in `session.rs` beside `client.rs`, `host.rs`, `mcp.rs`.
- (obs) the `command` span keeps its eight fields and escher-driver its two span sites, no event — met (`info_span!`
  probe 2, event probe 0); `passes` on a waited step is the quiet settle's.
- (obs) the new check and its stand-in are silent — met (census probe over `stand_load_wait.rs` 0).
- (tests) `run stand` 185 · 0 · 7 over 46 files — met; CI `verdict: green` with `stand-repeat` holding the new check
  three rounds (162 · 645 · 0 · 21) and escher-driver coverage 97.93 % ≥ 97 % — met (CI#39360124515).
- (a11y) the a11y leg green, its per-file counts unmoved 6 · 6 · 4 · 8 · 5 · 14 = 43 — met.
- (tests) `ci-leg.sh fast` and `ci-leg.sh doc` pass — met.
- Gates (implement's run `.andromeda/runs/2026-10-20T02-02-07-implement/`, `entries 20 · green 16 · red 0 · recorded 0 ·
  timeout 0 · not-run 4`, one iteration):
  - `cargo test -p escher-driver --locked 2>&1` — green · exit 0 · lacks FAILED (99 passed)
  - `cargo test -p blitz-tests --locked --test stand_load_wait 2>&1` — green · `test result: ok. 2 passed; 0 failed`
  - `grep -rln -E 'Instant::now|thread::sleep' packages/escher-driver/src | sort | paste -sd' '` — green · last line
    `… client.rs … host.rs … mcp.rs … session.rs`
  - `grep -rn -E 'Instant::now|SystemTime::now' packages/blitz-dom/src | grep -v -c 'clock.rs'` — green · exit 1 · last
    line 0
  - `git diff --name-only 165cfe44… -- packages/blitz-test-harness packages/blitz-dom Cargo.toml … | wc -l` — green ·
    last line 0
  - `grep -rc 'info_span!' packages/escher-driver/src | awk …` — green · last line 2
  - `grep -rn -E 'tracing::(event|info|warn|debug|trace|error)!|…' packages/escher-driver/src | wc -l` — green · exit
    1 · last line 0
  - `grep -c -E 'println!|…|blitz_net' tests/blitz-tests/tests/stand_load_wait.rs` — green · exit 1 · last line 0
  - `git diff --name-only 165cfe44… -- packages examples tests apps wpt .github scripts | grep -c -v -x -E …` — green ·
    exit 1 · last line 0
  - `git ls-files --others --exclude-standard -- … | grep -c -v -x 'tests/blitz-tests/tests/stand_load_wait.rs'` —
    green · exit 1 · last line 0
  - `bash scripts/agent-run.sh boot` — green · contains `"outcome": "ready"`
  - `bash scripts/agent-run.sh run stand` — green · contains `"selection": "stand", "passed": 185, "failed": 0,
    "ignored": 7`
  - `bash .github/scripts/ci-leg.sh a11y` — green · exit 0
  - `bash .github/scripts/ci-leg.sh fast` — green · exit 0 · artifact `target/ci-logs/test.log` fresh
  - `grep -E '^test result:' target/ci-logs/test.log | awk …` — green · contains `failed 0` (`lines 202 passed 1062
    failed 0 ignored 12`)
  - `bash .github/scripts/ci-leg.sh doc` — green · exit 0
  - `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` — leg operator: first firing red
    (`hygiene: refused 1 files — P1 1`), re-fired green `hygiene: clean` after the operator-directed respell
    (`evidence/ci.md`)
  - `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin
    build/escher-0.2.0` — leg operator: green · exit 0 · 99.65 s · `history moved:
    refs/remotes/origin/build/escher-0.2.0 165cfe44→8dfd1c02` (the intended push; `evidence/ci.md`)
  - `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800` — leg operator:
    `8dfd1c02b090 verdict: green · checks 17/17 · wall 1127 s · runs CI#39360124515 completed/success`
    (`evidence/ci.md`)
  - `gh run view <id> -R Turbolet85/escher --log | sed … | awk …` — leg operator, `recorded` (expect = []), fired with
    `<id>` 39360124515: Coverage report 183 · 1059 · 0 · 12; Accessibility (a11y) tests 6 · 43 · 0 · 0; Test (windows)
    183 · 1017 · 0 · 9; Test (macos) 183 · 1059 · 0 · 12; Test [default features] 202 · 1062 · 0 · 12; Stand checks
    repeated 162 · 645 · 0 · 21 — every moved count the prediction (+1 line, +7 passed per blitz-tests job; +3 lines,
    +6 on the three-round stand repeat), disposition: the CI run is green and this chunk's (`evidence/ci.md`)
  - smoke: skipped — no boot-path or UI-surface change (library-only; the plan lists no smoke or self-verify entry).
- Watches: none folded.
- Outcome basis: the operator pass ran — its commit list `8dfd1c02b chore(2026-10-20-load-wait): operator pre-CI
  commit, for the run this chunk's verdict reads` (Setup 4) and the final HEAD's CI run CI#39360124515 recorded in
  `chunks/2026-10-20-load-wait/evidence/ci.md`; implement's P4 report as this window holds it whole, persisted at
  `.andromeda/runs/2026-10-20T02-02-07-implement/outcome-2026-10-20-load-wait.md`; the operator directive between
  implement and this report — the `p5-controls.sh` respell (Decisions & corrections), which changed no code; post-implement
  artifacts `evidence/counts.md` and `evidence/ci.md`.
- Process hygiene: implement P4's census `census: none running` (cargo, rustc, `stand_load_wait`, `escher-session`,
  `seven_guis_nati`); the `LateNet` answer threads ended with their test process. Re-measured at this wrap: `gate.py
  census --comm cargo --comm rustc --comm stand_load_wait --comm escher-session --comm seven_guis_nati --run-dir
  .andromeda/runs/2026-10-20T03-00-02-wrap --marker 2026-10-20-load-wait` → `census: none running`.
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 165cfe44 (the parent of the oldest pre-CI commit 8dfd1c02) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### packages/escher-driver/src/execute.rs — added 4 line(s) in 3 range(s)
added: 16 · 126-127 · 462
### packages/escher-driver/src/lib.rs — added 1 line(s) in 1 range(s)
added: 158
### packages/escher-driver/src/schema.rs — added 1 line(s) in 1 range(s)
added: 121
### packages/escher-driver/src/schema_tests.rs — added 14 line(s) in 1 range(s)
added: 130-143
- 130-142 @131 «fn the_settled_help_states_the_load_wait_and_the_wait_ends_before_an_answer_read() {»
  - 132-135 «let stated = format!(»
  - 138-140 «for acting in [&CLICK, &TYPE, &PRESS, &ADVANCE, &SCROLL] {»
### packages/escher-driver/src/session.rs — added 106 line(s) in 6 range(s)
added: 5 · 7 · 16-53 · 233-234 · 241 · 307-369
- 24-29 @27 «pub(crate) fn settle_waiting(harness: &mut Harness<DioxusDocument>) -> Result<Settled, NotSettled> {»
- 31-52 @33 «fn settle_within(»
  - 39-51 «loop {»
  - 307-310 «const QUIET: Result<Settled, NotSettled> = Ok(Settled {»
  - 312-314 «fn busy(busy: Busy) -> Result<Settled, NotSettled> {»
  - 316-333 @318 «fn settle_scripted(»
  - 335-341 @336 «fn a_load_answered_inside_the_bound_ends_the_wait_settled() {»
  - 343-350 @344 «fn a_load_never_answered_ends_the_wait_at_the_bound() {»
  - 352-360 @353 «fn work_other_than_a_load_returns_at_once() {»
  - 362-368 @363 «fn a_quiet_settle_returns_after_one_settle() {»
### tests/blitz-tests/tests/stand_load_wait.rs — new file · 164 line(s)
- 34-46 @35 «fn load_page() -> Element {»
  - 37-45 «rsx! {»
- 52-60 «impl NetProvider for LateNet {»
  - 53-59 «fn fetch(&self, _doc_id: usize, request: Request, handler: Box<dyn NetHandler>) {»
- 62-74 @63 «fn hold(net: Arc<dyn NetProvider>, incremental: bool) -> Session {»
  - 64-72 «Session::start("load-wait", || {»
- 76-93 @78 «fn box_height(session: &mut Session, mode: &str) -> String {»
  - 79-81 «let Ok(Outcome::Screen { text }) = session.run(&snapshot()) else {»
  - 82-85 «let line = text»
  - 86-88 «let (_, size) = line»
  - 89-91 «let (_, height) = size»
- 95-105 @98 «fn click_load(session: &mut Session, mode: &str) -> (Acted, Duration, bool) {»
  - 101-102 «let acted = act(session, &click("fx-load"))»
- 107-138 @108 «fn a_load_answered_inside_the_bound_ends_the_step_settled_with_its_content_on_screen() {»
  - 109-137 «for incremental in [false, true] {»
- 140-164 @141 «fn a_load_never_answered_ends_the_step_at_the_bound_still_loading() {»
  - 142-163 «for incremental in [false, true] {»
