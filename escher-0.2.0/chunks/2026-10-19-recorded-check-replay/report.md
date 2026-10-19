# Report — 2026-10-19-recorded-check-replay

**Chunk:** Recorded-check replay — recorded checks run as ordinary tests under agent-run, CI and stand-repeat, steps
re-validated, each answer kept whole but a shot's pixels; red on behaviour change, green otherwise (v020-04, the
replay half).
**Date:** 2026-10-19
**Commits** since `last_wrap` 2026-10-19T06:45:12Z (`git log --format='%h %s' 6fd1d3daf..HEAD`, with 6fd1d3daf the
previous chunk's own wrap commit):
- `d2f010a2a chore(2026-10-19-recorded-check-replay): operator pre-CI commit, for the run this chunk's verdict reads`
- `f15c28fcb fix(2026-10-19-recorded-check-replay): operator fix after CI run 39225375577, drop the screenshot step
  from the committed checks — a shot's PNG bytes differ by OS`

Base for every reading below: `6fd1d3da`, the parent of the oldest pre-CI commit `d2f010a2`.

## Changes (structured — detectors read this)

### Files
Basis: `gate.py scope` (`scope: clean — changed 13 · listed 13 · recorded 0`) and the `## New text, by line` listing.
- **New:**
  - `packages/escher-driver/src/replay.rs` (318 lines)
  - `tests/blitz-tests/tests/stand_replay.rs` (261 lines)
  - `tests/blitz-tests/checks/counter-cli.json` (8 lines, 2,929 B)
  - `tests/blitz-tests/checks/counter-mcp.json` (11 lines, 3,720 B)
  - `examples/seven_guis/tests/flows/record_checks.sh` (65 lines)
- **Modified:**
  - `packages/escher-driver/src/{recording,lib,session}.rs`
  - `tests/blitz-tests/tests/{stand_session_recording,session_common/mod}.rs`
  - `examples/seven_guis/tests/recording.rs`
  - `examples/seven_guis/src/app.rs`
  - `.github/scripts/test_ci_workflows.py`
- **Ledger:** `escher-0.2.0/verification-matrix.json` (v020-04 → `implemented`).
- **Evidence:**
  - `chunks/{marker}/evidence/windows-gnu-check-082208Z.txt`
  - `chunks/{marker}/evidence/operator-pass.md`

### Symbols / APIs
Every coordinate is copied from the new-text listing.
- **The record's new `answer` key.** `record_of` (`recording.rs:108-113` signature) now takes the answer.
  - It writes `accepted`, `digest` (`answer_digest`, unchanged: FNV-1a 64 over the whole written line, a shot's PNG
    included), then `answer`.
  - `answer` is the kept line inlined as its own JSON object (`record.json("answer", &kept_line(answer))`,
    `recording.rs:134-136`), written by the crate's own `json::Object`.
  - Private `written` (`recording.rs:47-52`) is the line `Outcome::to_json` or `Refusal::to_json` writes.
  - `pub(crate) kept_line` (`recording.rs:55-60`) is that line, but a shot's is `json::shot_without_image(width,
    height)`, holding no `png`.
  - `Recording::record` computes the line once. Its sole caller is still `Session::run` (`execute.rs:145`, unchanged).
- **The reader.** `step_of`'s closed key list (`recording.rs:412-423`) adds `answer`.
  - `answer` is required and must be a JSON object (`recording.rs:443`). Missing, or a string, number, array or
    `null`, is refused `Cause::Malformed`.
  - Key order is not enforced, the same as for every other key: JSON objects are read as maps.
  - `Step` gains a private field `answer: serde_json::Value` (`recording.rs:317`) and a `pub(crate) fn answer(&self)
    -> &Value` (`recording.rs:385-388`). No public accessor returns a `serde_json` type.
  - `Check::from_bytes`' doc (`recording.rs:328-329`) states the key.
  - The bounds (`MAX_CHECK_STEPS` 400, `MAX_CHECK_BYTES` 1,048,576) are unchanged.
- **The replay — new public API in `replay.rs`.**
  - `impl Check { pub fn replay(&self, session: &mut Session) -> Replay }` (`replay.rs:40-70`, fn `41-69` @46):
    - a session whose `label()` is not `self.app()` runs nothing;
    - otherwise each step's `call()` runs through `Session::run`, so it is validated again;
    - the run stops at the first step that differs.
  - Private `change_of` (`replay.rs:72-93`) parses the live answer's kept line with `serde_json::from_str`, which reads
    only. A step differs when its acceptance differs or that value is not equal to the kept answer.
  - Each differing key is taken from the step verb's result fields in table order, then `refused` (a private
    `const REFUSED`). A key no table holds is not named.
  - `pub struct Replay` (`replay.rs:21-28`): `holds()`, `another_app()`, `ran()`, `change() -> Option<&Change>`
    (`replay.rs:95-115`).
  - `pub struct Change` (`replay.rs:30-38`): `step()`, `accepted_then()`, `accepted_now()`, `keys() -> &[&'static
    str]` (`replay.rs:117-139`).
  - Neither type holds a reading or a `serde_json` type. Both derive `Debug, Clone, PartialEq, Eq`.
  - Re-exported from the crate root: `mod replay;` (`lib.rs:127`), `pub use replay::{Change, Replay};` (`lib.rs:144`).
  - Unit tests in `replay.rs:141-318`: holds · acceptance flip · changed answer naming its keys and stopping the
    replay · a shot holding at its size whatever its pixels · another app running nothing.
- **Removed:** `session_common::replay(session, check) -> Vec<(bool, u64)>`. Its two call sites in
  `stand_session_recording` now use `Check::replay`.
- **Added:** `session_common::shot_prefix(&mut Session) -> String` (`session_common/mod.rs:240-250`), the first 64
  base64 characters of a whole-screen shot as the driver writes it. Its readers are `stand_replay` and
  `stand_session_recording`. `kept` stays.
- **seven_guis:** `pub fn TaskShell` with one doc line (`app.rs:162`, `app.rs:164`). It was private, and is otherwise
  unchanged.
- **No new** port, socket, env var, listener, CLI verb, MCP tool, wire field or log field.

### Crates / modules
- escher-driver gains the private module `replay` and two public types, `Replay` and `Change`.
- No crate is added or removed.

### Dependencies
None added or bumped. No `Cargo.toml` or `Cargo.lock` is in the diff.

### Schema / config
- **The kept-check record grammar**, every step: `tool` · `arguments` · `masked` (a `type` only) · `accepted` ·
  `digest` · `answer`.
  - `answer` is the answer's line as the driver wrote it — masked as on every carrier, and a shot's without `png`, so
    `{"width":…,"height":…}`. Written after `digest`, required on read.
  - This is F3 B, the founder's ratified widening (`escher-0.2.0/chunks/2026-10-19-session-recording/inputs/I4-relay-1.md.txt`;
    inputs#I1). A kept check now holds the screens' ids, names, values, snapshot text and diffs at rest; it held none
    before.
- **Two committed checks** in a public repository: `tests/blitz-tests/checks/counter-{cli,mcp}.json`, the founder's
  F1 A (inputs#I2), git mode `100644`. Their content is the stand Counter's ids, names, values, snapshot text and
  diffs, plus the scripts' calls.
  - No typed text, host path, name or secret: the operator read both (inputs#I3), and the wrap read both at implement.
  - No `screenshot` step since `f15c28fc` (inputs#I4).
- **The recording script.** `record_checks.sh`, POSIX `sh`, takes the binary and an existing output directory.
  - It makes `rec/` there (`mkdir -m 700`): `s` the session state directory, `c` and `m` the recording directories.
  - It prints only `step N` on failure, and nothing a screen reads.
  - **CLI session:** `start counter --record` · `snapshot` · `click counter-increment` · `advance 600` · `click
    counter-increment` · `snapshot` · `stop`.
  - **MCP session:** `initialize` · `notifications/initialized` · `start counter` · `snapshot` · `press tab` · `press
    tab` · `press enter` · `press space` · `advance 100` · `scroll counter-increment` · `snapshot` · `stop`.

### Spec-master edits
None.

### Retired
None. The removed `session_common::replay` was a test helper; its masters' sites are under the `replay` and
`sc-readers` patterns below.

### Counts / qualifiers moved
Each delta is read from the tree, with `6fd1d3daf` as base. Each names its pattern, and each site is from the pre-pass
sweep (`cascade-2026-10-19-recorded-check-replay-pre-pass.json`).
- **escher-driver unit tests:** 87 → 92, in fourteen → fifteen files, 67 → 72 inline. `replay.rs` +5; `recording.rs`
  stays 9 unix plus 1 off-unix, its rows reworked.
  - Basis: `grep -c '#[test]'` per file; the gate log `test result: ok. 92 passed`.
  - Pattern `unit-count`, standing 4: `test-plan.md:22` @c21 (stale) · `registries/contracts/test-plan/session-lifecycle.md:11`
    @c4266 (stale; its per-file list gains `replay.rs 5`) · `test-plan.md:25` (dioxus-native-dom, no-change) ·
    `test-plan.md:153` (blitz-dom, no-change).
- **`session_common` readers:** 24 → 25 (+`stand_replay`).
  - Basis: `grep -l '^mod session_common;' tests/blitz-tests/tests/*.rs` reads 25, and 24 at base by `git grep`.
  - Pattern `sc-readers`, standing 16:
    - stale — `test-plan.md:67` @c1090 («by 24 checks»), `test-plan.md:102` @c2738 («24 readers»);
    - stale, and already one short at the base — `architecture.md:333` @c20107 («twenty-three readers»); see Spec
      claims disproved;
    - the module's reader list — `test-plan.md:102` (the helper list naming `replay(session, check)`, stale) and
      `obs-plan.md:76` («through its `replay`, `kept` and `mode`», stale);
    - the other 11 rows are path citations or `declares no mod session_common` statements, no-change.
- **`stand_*.rs` files:** 43 → 44 (+`stand_replay`).
  - Basis: `ls tests/blitz-tests/tests/stand_*.rs | wc -l` reads 44, and 43 at base.
  - Pattern `stand-files`, standing 8: `test-plan.md:318` @c1285 («43 files», stale) and `test-plan.md:347`
    («the 43 `stand_*.rs` files», stale). The other 6 rows state the glob and no count, no-change.
- **blitz-tests `.rs` files and targets:** 135 `.rs` / 134 targets → 136 / 135.
  - Basis: `ls tests/blitz-tests/tests/*.rs | wc -l` reads 136, and 135 at base.
  - Pattern `bt-files`, standing 1: `test-plan.md:67` @c2485 (stale; @c3447 is its history clause, no-change).
- **seven_guis `recording` tests:** 3 (+1 off-unix) → 4 (+1 off-unix), adding
  `the_committed_checks_are_what_the_binary_records`.
  - Basis: `grep -c '#[test]'` reads 5, and 4 at base.
  - Pattern `rec-tests`, standing 4: `registries/contracts/test-plan/session-lifecycle.md:11` @c640 (stale).
    `test-plan.md:22` @c227 and `session-lifecycle.md:11` @c254 are the marker `…session-recording` followed by
    another count — a sweep hazard, no-change. `test-plan.md:102` @c2775 is a reader history, no-change.
- **`stand_session_recording`:** still 3 tests. Its first test is renamed
  `a_recorded_session_replays_and_keeps_no_screen_reading` → `a_recorded_session_keeps_each_answer_but_a_shots_image_and_replays`,
  and its assertions are reversed (below).
  - Pattern `reading`, standing 11 (below).
- **The qualifier «no screen reading» / «answers' digests alone» is now false:** each kept step keeps its answer whole.
  - Pattern `reading`, standing 11, all stale:
    - `architecture.md:119` ×2, `:182` @c7736 («sent in place of the answer» is the recording-failed word, a true
      claim sharing the token: no-change), `:186` @c884, `:192` @c21891;
    - `security-plan.md:125` @c6510, `:192`;
    - `layout-templates.md:73` @c5719;
    - `test-plan.md:27` @c17777;
    - `obs-plan.md:341` ×2;
    - `registries/contracts/test-plan/session-lifecycle.md:7` («in place of the answer», recording-failed, no-change),
      `:11` @c390 (stale).
  - So 9 stale and 2 no-change sites.
  - Pattern `record-keys`, standing 5: `architecture.md:186` ×2 (the record key list, gains `answer`, stale) ·
    `security-plan.md:81` ×2 (the closed key list «`tool` · `arguments` · `masked`…», stale) · `architecture.md:204`
    @c10751 and `security-plan.md:77` (the cold-agent check's closed keys, no-change) · `obs-plan.md:341` (the record's
    content, stale).
  - Pattern `at-rest`, standing 5: `architecture.md:119`, `security-plan.md:125` @c7353, `test-plan.md:27` @c17795,
    `obs-plan.md:341`, all stale (what a kept check holds at rest), and `obs-plan.md:76` («a file at rest, never a log
    line», true, no-change).
  - Pattern `digest`, standing 8: every row describes the digest, which is unchanged, so no-change on the digest
    itself. `security-plan.md:192` and `obs-plan.md:76` state the digest as all the answer content kept, which is
    stale with the `reading` rows.
- **`TaskShell`:** private → `pub`.
  - Pattern `taskshell`, standing 6: `architecture.md:180` @c1632 («`TaskShell` staying private», stale). The other 5
    rows name the mount, no-change.
- **The new measured limit:** a shot's PNG bytes differ by OS, so a committed check holds no `screenshot` step.
  - Basis: CI#39225375577, `Test (macos)` aarch64 recorded `counter-cli.json` at 3,047 B, the same length as linux's,
    with different bytes. The only digest an answer that held as JSON does not fix is the shot's. Detail in
    `evidence/operator-pass.md`.
  - Pattern `shot-os`, standing 2: `architecture.md:333` @c15414 and `session-lifecycle.md:11` @c2632 («byte-identical
    in two child runs and in the parent»). Both are true on one host; no-change, qualifier owed: one host, not across
    OSes.
- **The replay API** (`Check::replay` · `Replay` · `Change`).
  - Pattern `replay`, standing 17:
    - stale, owed at this entry — `architecture.md:119`, `security-plan.md:81`, `security-plan.md:192` (each «…owed at
      / refused until «Recorded-check replay»»);
    - stale, the replay helper — `test-plan.md:102` @c2489 («the module's new `replay(session, check)`»), `obs-plan.md:76`;
    - recorded-session replay, now through the API, stale — `test-plan.md:27` @c17674, `session-lifecycle.md:11` @c287;
    - the cold-agent pipe's own replay, no-change — `architecture.md:204` ×2, `:220` ×4, `security-plan.md:28`, `:77`
      ×3, `:410`, `layout-templates.md:72`, `test-plan.md:124` ×5, `:125` ×3, `:149` ×2, `obs-plan.md:96`.
- **The committed checks and their script** (`tests/blitz-tests/checks/`, `counter-cli.json`, `counter-mcp.json`,
  `record_checks.sh`): pattern `checks-dir`, 0 master sites. Basis: `checks-dir … control UNFIRED over the seven
  masters at f15c28fc`.
- **The replay target:** pattern `stand-replay`, 0 master sites. Basis: `stand-replay · control UNFIRED over the seven
  masters at f15c28fc`.
- **Owner pattern** `owner-title` («Recorded-check replay»), standing 3: `architecture.md:119`,
  `security-plan.md:81`, `security-plan.md:192`. Each states the answer key as owed here; each is discharged by this
  chunk, stale. `owner-generic` standing 2 (`architecture.md:90`, `security-plan.md:125` @c8883) names other owners,
  no-change.
- **Sweep reach:** 14 patterns (`total (14 patterns) · 191 rows over 26 files`), plus 2 refused for an unfired
  control · 10 moved facts, 10 with a pattern (the report's own count) · owner pattern 3 standing · by line 13 lines ·
  38 offsets.

### Dev-tool versions
None.

### Harness / gate surface
- **New blitz-tests target `stand_replay`.**
  - It is picked up by `agent-run.sh run stand`, by `stand-repeat`'s `stand_*.rs` glob (`ci-leg.sh:50-52`, no change)
    and by the CI `test` leg on three OSes.
  - Not unix-gated: it reads bytes and runs calls in process.
- **New seven_guis test** `recording::the_committed_checks_are_what_the_binary_records`.
  - It runs `record_checks.sh` through `CARGO_BIN_EXE_escher-session` into `CARGO_TARGET_TMPDIR/rc-fresh` and
    compares both files byte for byte with `include_bytes!` of the committed copies.
  - It is unix-gated, by `run`'s 90 s bound, with both streams drained and a `Started` guard.
- **`.github/scripts/test_ci_workflows.py:359`:** `test_stand_repeat_leg_runs_three_rounds` now names
  `stand_replay`.
- No CI workflow, leg script or agent-run script changed.

### Cross-project / external claims
- **CI#39225375577** on `d2f010a2`: `completed failure`.
  - The one red was `Test (macos)` (job 117972311781): `the_committed_checks_are_what_the_binary_records` failed on
    `counter-cli.json`, the same length and different bytes.
  - On that job `stand_replay` passed all 3 tests. `Test (windows)` (`stand_replay` 3 ok) and `Stand checks
    repeated` were success, as was every other job.
- **CI#39227884248** on `f15c28fc`: `verdict: green · checks 17/17 · wall 1030 s`.
  - `stand_replay` ok on macOS (job 117980673002) 3, Windows (117980673074) 3, linux (117978535102) 3, `Stand checks
    repeated` (117980672807) 9.
  - Freshness ok on macOS 1, linux 1, `Stand checks repeated` 3. On Windows it is unix-gated.
  - Read from the job logs. The sha is the record: this wrap's commit adds to `f15c28fc`.
- **Inputs:** `inputs.py verify` reads `inputs: 4 entries — unchanged 0 · drifted 0 · vanished 0 · broken 0 · altered
  0 · unreachable 0 · n/a 4 · uncited 2 · unparsed 0` (fired before this report). I3 and I4 are cited by this report.
  - inputs#I1 · message · copy message · n/a — the operator's take-up directive (F3 → B ratified; the windows-gnu
    check before a push touching platform-gated code).
  - inputs#I2 · message · copy message · n/a — F1 A, the founder's own word, «Коммитить две проверки (Рекомендую)».
  - inputs#I3 · message · copy message · n/a — the operator-pass directive. Snapped at this wrap, carried as cited.
  - inputs#I4 · message · copy message · n/a — the fix directive after CI#39225375577. Snapped at this wrap, carried
    as cited.
- No inputs entry names a P5 review card. The phase run dir's `p5-card-1.md` reads `## Beyond the plan` → `none`.

### Reverted / negative API facts
- **The screenshot step in both committed checks** was recorded at implement and removed at the operator pass
  (`f15c28fc`). A kept shot's digest hashes its PNG, the PNG's bytes differ between linux x86_64 and macOS aarch64,
  and the freshness check compares bytes (inputs#I4).
- The comparison was not loosened: no per-OS skip and no digest exemption. The shot's kept `answer` (width and height)
  held on macOS.

### Insufficient fixes (written, kept, not the remedy)
None.

### Spec claims disproved by measurement
1. **`architecture.md:333` @c20107 «twenty-three readers»** of `session_common`.
   - Measured 24 at base (`git grep -l '^mod session_common;' 6fd1d3daf -- 'tests/blitz-tests/tests/*.rs'`) and 25
     now. `test-plan.md:67` and `:102` already read 24.
   - The miscount predates this chunk. Pattern `sc-readers`.
2. **That a committed check with a `screenshot` step reads the same bytes on every OS** — the plan's open question
   (plan §Implementation notes, «Cross-OS determinism is unmeasured»; P5 card).
   - Measured false: CI#39225375577, macOS aarch64.
   - The masters' «byte-identical in two child runs and in the parent» (`architecture.md:333` @c15414,
     `session-lifecycle.md:11` @c2632) and the leaf «byte-identical across processes» (`CLAUDE.md:110`) hold on one
     host only. Pattern `shot-os`.
3. **inputs#I4 says «the shot path stays covered by the same-host CLI-vs-MCP test».**
   - Measured false: `examples/seven_guis/tests/recording.rs:129-164` (`the_command_line_and_the_mcp_role_keep_the_same_check`)
     runs `click` and `snapshot` only.
   - The shot path's kept form is covered by `stand_session_recording`, whose script ends `snapshot` · `screenshot`
     on each lean task, in process and on each OS. It asserts each record ends with the kept line (a shot's
     `{"width","height"}`) and that the check holds no `png`, and it replays green.
   - Recorded in `evidence/operator-pass.md`. The test is left unchanged.

### Expected amendments (from plan)
Each line names the pre-pass sweep patterns and their standing hits.
- **security-plan §Data Protection (At rest, Files (kept checks)) and §Input Validation** (the Recorded check row; the
  «no log, event or file carries…» sentence) — each kept answer whole at rest, a shot's pixels excepted, masked as on
  every carrier; Boundary widening, the founder's own F3 B (I4 of the previous chunk; inputs#I1).
  - Carried: Changes → Schema / config, and Counts (`reading`).
  - Sites in security-plan: `reading` 2 (`:125`, `:192`) · `record-keys` 2 (`:81` ×2) · `at-rest` 1 (`:125`) ·
    `owner-title` 2 (`:81`, `:192`).
- **security-plan §Data Protection: the two committed checks under `tests/blitz-tests/checks/`** as the stated
  exception to owner-only, the repository's own scripted Counter sessions in a public repository; Boundary widening,
  the founder's F1 A (inputs#I2).
  - Carried: Changes → Schema / config.
  - Search: `checks-dir` 0 (control unfired, no master states it yet). The owner master's kept-check site is
    `at-rest` → `security-plan.md:125` @c7353.
- **architecture §Occupied Resources → Filesystem: `tests/blitz-tests/checks/`** (two committed checks, written by
  `record_checks.sh`); the same F1 A.
  - Carried: Changes → Files and Schema / config.
  - Search: `checks-dir` 0. The Filesystem row naming the session checks' scratch is `sc-readers` →
    `architecture.md:212` @c4350.
- **architecture §Standard Contracts → Driver session: the record's `answer` key, `Check::replay` · `Replay` ·
  `Change`, the re-exports**; inside F3 B.
  - Carried: Changes → Symbols / APIs.
  - Sites: `digest` `architecture.md:182` (the re-export list «`Check`, `Step`, `answer_digest` and
    `MASKED_STAND_IN`»), `:186` ×2, `:328` · `record-keys` `:186` ×2 · `reading` `:186`, `:192` · `owner-title` `:119`.
- **architecture §Infrastructure Patterns: `pub fn TaskShell` (seven_guis), the `stand_replay` target,
  `record_checks.sh` beside the flows.**
  - Carried: Changes → Symbols / APIs and Harness / gate surface.
  - Sites: `taskshell` `architecture.md:180` @c1632 · `sc-readers` `architecture.md:333` @c20107 · `stand-replay` 0
    (control unfired).
- **obs-plan §9 and §4.** In §9 the «Recording directory» row's «no screen reading» becomes the answers kept whole, and
  a committed-checks row is added (in git, uploaded by no leg). In §4 `Check::replay` goes on the observed-absent list.
  - Carried: Changes → Schema / config and Symbols / APIs.
  - Sites: `reading` `obs-plan.md:341` ×2 · `at-rest` `:341` · `replay` and `sc-readers` `:76` · `digest` `:76`.
  - `checks-dir` 0 for the new row.
- **test-plan §3 Session lifecycle** (`session-start`: the `answer` key and `Check::replay`; `session-proof`: the replay
  target and the freshness check) **and §10** (stand-repeat's count, the flakiness budget's file count).
  - Carried: Changes → Counts and Harness / gate surface.
  - Sites:
    - `reading` `test-plan.md:27`, `session-lifecycle.md:11` · `replay` `test-plan.md:27`, `:102`,
      `session-lifecycle.md:11`;
    - `stand-files` `test-plan.md:318`, `:347` · `sc-readers` `:67`, `:102` · `bt-files` `:67`;
    - `unit-count` `:22`, `session-lifecycle.md:11` · `rec-tests` `session-lifecycle.md:11` @c640.
- **layout-templates §Surface: cli — the kept-check paragraph's «the calls and no screen reading».**
  - Carried: Changes → Counts (`reading`). Site: `layout-templates.md:73` @c5719.

### Coverage of new surfaces
- `Check::replay` (`Replay`, `Change`) → validation ✓ (every step through `Session::run`, which runs `validate`
  first; a check is read only by the strict `Check::from_bytes`) · instrumentation ✓ (each step's own `command` span;
  the module opens none, the silence probe reads 0) · PII n/a (holds no reading: closed schema words, bools, counts) ·
  tests unit (5) + integ (`stand_replay` 3, `stand_session_recording`) · a11y n/a · tokens n/a.
- The kept-check `answer` key → validation ✓ (required, a JSON object, closed keys, bounds unchanged; 40 refused and 8
  read rows) · instrumentation ✓ (never logged: the silence probe and `host_log` / `mcp_log` 0 needles) · PII
  redacted ✓ (a password's or file input's value as the mask, a masked `type` as `MASKED_STAND_IN`, a shot without
  pixels; sentinel 0) · tests unit and integ · a11y n/a · tokens n/a.
- `tests/blitz-tests/checks/counter-{cli,mcp}.json` → validation ✓ (`Check::from_bytes`) · instrumentation n/a · PII
  ✓ (Counter screens only; no typed text, host path, name or secret; inputs#I3) · tests integ (`stand_replay`,
  seven_guis freshness) · a11y n/a · tokens n/a.
- `record_checks.sh` → validation n/a (a test-side script; two positional arguments) · instrumentation n/a (prints
  only `step N`) · PII ✓ · tests integ (the freshness check runs it) · a11y n/a · tokens n/a.
- `pub fn TaskShell` → a visibility change only. No UI element added; tokens and a11y unchanged (the a11y leg's six
  files, per-file counts unchanged).

## Deviations from intent
1. **Plan step 10 named the test-only variant `VariantCounter`. It is a component named `Counter` in a test-local
   `mod variant` (`stand_replay.rs:112-137`).**
   - Why: a component's name enters every unkeyed element's stable id (`TaskShell/Counter/div:0`;
     `packages/dioxus-native-dom/src/element_id.rs`, the component path).
   - Under another name both checks would read changed ids and turn red at their first `snapshot`, not at their first
     activation step. Under `Counter` they turn red at step 1 (CLI) and step 3 (MCP), as the acceptance states.
2. **`stand_replay` reads each check's first activation step from its steps** (`activates`, `stand_replay.rs:71-79`)
   and asserts it equals the table's 1 and 3, beside the literal.
3. **`session_common::shot_prefix` added** (in a listed file). It serves the security acceptance's «no shot's first
   64 base64 characters» in `stand_replay` and `stand_session_recording`.
4. **The escher-driver grammar rows now carry `answer`,** so each refused row still isolates its own fault.
   - The refused table grows 35 → 40: missing, string, number, array, `null`, and an unknown key beside it.
   - The read table grows 6 → 8: an object `answer`, a refusal's `answer`, and one with keys out of order.
5. **`stand_session_recording`'s renamed test** checks each kept answer byte-wise: every record ends with
   `,"answer":{line}}`, the line rebuilt in the test. Its `escaped` and `sent` helpers and the name, value and line
   collections were removed with the assertion they served.
6. **The operator-pass fix** (inputs#I4, `f15c28fc`): both scripts' `screenshot` step was dropped, both checks
   re-recorded, and the diff removes exactly the two shot records.
   - This is the plan's own narrowing (plan §Implementation notes: «keep the two scripts to steps that hold on every
     OS»), the comparison byte-strict.
   - The local reads after it were `recording` 4 ok, `cli_flow` 2 ok, `stand_replay` 3 ok, and the windows-gnu entry
     `7 build green · exit 0` (`7.2.log`).

Scope record: none — `gate.py scope` clean, 0 recorded (`scope: clean — changed 13 · listed 13 · recorded 0`).

## Decisions & corrections
- **A test-only variant of a stand task must be a component of the task's own name.** Dioxus component names enter
  every unkeyed element's stable id, so a renamed variant changes ids before it changes behaviour.
  - Measured by the CLI check's first `snapshot` holding `TaskShell/Counter/div:0` (`tests/blitz-tests/checks/counter-cli.json:3`).
  - Evidence: implement's evolve record `2026-10-19T08:31:04Z-b` (`input.plan-step-ambiguous`).
- **A shot's PNG bytes differ between linux x86_64 and macOS aarch64,** so anything compared byte for byte across OSes
  — a committed check's digest included — must hold no shot.
  - Measured: CI#39225375577, job 117972311781, `counter-cli.json` 3,047 B both, different bytes, while `stand_replay`
    (answers as JSON values) held there.
  - The operator's word: «it is the plan's own narrowing, not a loosening: the comparison stays byte-strict»
    (inputs#I4).
- **The operator's directive** (inputs#I3): if CI reds on a cross-OS replay difference, stop and give the measured
  diff, and do not loosen the comparison. Followed: the pass stopped at `verdict: red` and reported the measurement
  before any fix.
- **A correction to the operator's own record line** (inputs#I4, «the shot path stays covered by the same-host
  CLI-vs-MCP test»): measured false (`recording.rs:129-164` has no `screenshot`). Recorded as measured in
  `evidence/operator-pass.md`, the test unchanged, and the operator told in the pass's closing message.
- **Guard refusal, a recurrence of the standing host-linux rule** (§Transports, documents through the Write tool):
  the Bash guard refused a `cat` heredoc writing a scratch Python script («a cat/tee heredoc with a file target»). It
  was re-issued once through the Write tool and run by path (implement P1).
- **A tracked evidence file written after the pre-CI commit would trip the push entry's clean-tree guard**
  (`git diff --quiet`).
  - The pass wrote `evidence/operator-pass.md` untracked before the first push, then wrote the fix and the limit into
    it before the fix commit so it rode that commit. Only the final verdict lines were written after the last push.
  - The file is modified, uncommitted, and rides this wrap's commit.
- **Sweep hazard:** a count pattern of the form `recording\W{0,3}\d+` matches a chunk marker ending in `…recording`
  followed by a historical count («…2026-10-19-session-recording, 73 until…»: `test-plan.md:22` @c227,
  `session-lifecycle.md:11` @c254). Read the hits; never count them.
- **A standing miscount found:** `architecture.md:333` «twenty-three readers» read 24 at the base (Spec claims
  disproved 1).
- **Two-window wrap:** the operator's invocation `/andromeda-wrap-session run P1 only and stop`. P1 ends here; P2–P7
  resume in this run dir (`.andromeda/runs/2026-10-19T09-49-14-wrap`).

## Outcome
### Acceptance criteria, re-asserted against the diff
- **(tests) v020-04 — MET.**
  - Both checks are committed as the binary wrote them (the second recording by `f15c28fc`), and `record_checks.sh`
    reads byte-equal (seven_guis `recording`).
  - Each holds in both layout modes on a fresh stand Counter (`stand_replay`).
  - Against the add-2 variant each is red at step 1 / 3, every earlier step held, with `changed` among the keys.
  - Green in CI's `test` leg on linux, windows and macos and in `stand-repeat`: CI#39227884248 on `f15c28fc`.
  - The scripts hold no `screenshot` step, a narrowing the acceptance text does not name. Matrix: `implemented`, ref
    in the ledger.
- **(arch) MET.**
  - The replay runs each kept call through `Session::run` alone (`replay.rs:41-69`).
  - No `Cargo.toml` or `Cargo.lock` in the diff.
  - `serde_json` is called as `from_str` / `from_slice` and never writes.
  - `Replay` and `Change` hold `bool`, `usize` and `Vec<&'static str>` only.
- **(security, reader) MET.** `answer` is required and must be an object. Missing, non-object or unknown-sibling-key
  is refused `malformed`. The bounds rows stand (`a_check_at_its_bounds_is_read_and_one_past_them_is_refused`).
- **(security, at rest) MET.**
  - A shot step's kept answer is `{"width","height"}` (`recording.rs` round-trip test; `stand_session_recording`).
  - Neither committed check holds a `png` key or the prefix: since `f15c28fc` they hold no shot at all, and
    `stand_replay` asserts it.
  - The masked sentinel occurs 0 times, `MASKED_STAND_IN` kept.
- **(obs) MET.**
  - The silence probe reads `last line 0`.
  - `stand_replay`'s failure messages hold `{mode} {file}` and a step index alone (read at implement).
  - `host_log` and `mcp_log` passed in the seven_guis gate.
- **(layouts / a11y) MET.** The untouched-stand probe passed. The `app.rs` probe reads `last line 3`.
- **(layouts / design) MET.** `mcp_server`, `cli_commands` and `recording` passed. `help` and `tools/list` are
  unchanged.
- **(a11y) MET.** The a11y leg passed, per-file counts 6 · 6 · 4 · 8 · 5 · 14.
- **(tests) MET.** `fast`, `doc`, `coverage` (escher-driver 98.33 %) and `stand-repeat` (`stand-repeat: 3 of 3 rounds
  green`) passed; the windows-gnu check is recorded; CI reads `verdict: green` on `f15c28fc`.

### Gates
Implement run `.andromeda/runs/2026-10-19T08-22-08-implement`, trail `gate-2026-10-19-recorded-check-replay.json`, at
the tree before `f15c28fc`:
- `cargo test -p escher-driver --locked` green (92)
- `cargo test -p blitz-tests --locked --test stand_replay --test stand_session_recording` green
- `cargo test -p seven_guis --locked` green
- the silence probe (`awk …`) green · `last line 0`
- the untouched-stand `git diff --quiet …` green
- the `app.rs` `git diff -U0 … | grep -c` green · `last line 3`
- `cargo check --tests --locked --target x86_64-pc-windows-gnu …` green, re-fired after the fix green (`7.2.log`)
- `ci-leg.sh fast` green · `ci-leg.sh doc` green · `ci-leg.sh stand-repeat` green (`contains stand-repeat: 3 of 3
  rounds green`) · `ci-leg.sh a11y` green · `ci-leg.sh coverage` green (`contains coverage floor: 3 of 3 crates…`) ·
  `ci-leg.sh audit` green
- summary `entries 16 · green 13 · red 0 · recorded 0 · timeout 0 · not-run 3`

The operator entries, recorded in `evidence/operator-pass.md`:
- `gate.py hygiene`: `hygiene: clean`.
- `ci-leg.sh fast && git diff --quiet && … git push`: green twice (pushes `6fd1d3da→d2f010a2`, `d2f010a2→f15c28fc`).
- `ci.py conclusion --sha HEAD --wait 1800`: red on `d2f010a2` (CI#39225375577, `Test (macos)`, this chunk's,
  fixed by `f15c28fc`), then green on `f15c28fc` (CI#39227884248).

Smoke: `agent-run.sh boot` → `run stand_replay` passed 3/3 → `status` → `cleanup` (the boot-path condition:
`recording.rs` is on `escher-session`'s path).

### Watches
None folded.

### Outcome basis
The operator pass ran:
- the commits `d2f010a2`, `f15c28fc`;
- the final HEAD's CI run CI#39227884248, recorded in `evidence/operator-pass.md`;
- implement's P4 report, persisted at `.andromeda/runs/2026-10-19T08-22-08-implement/outcome-2026-10-19-recorded-check-replay.md`
  and held whole in this window. It is superseded on the committed checks' content (no `screenshot` step; sizes 2,929
  and 3,720 B) and on its CI risk note, which the measurement answered.

### Process hygiene
- Implement P4's census: `census: none running`. The rows were the `escher-session` hosts and `mcp` children, the
  cargo and test binaries, all terminated.
- Re-measured here with `gate.py census --comm escher-session --comm stand_replay --comm cargo --args escher-session`:
  `census: none running`.
## Addendum — resumed 2026-10-19
- inputs#I5 · message: the operator, /andromeda-wrap-session resume invocation, 2026-10-19 09:56Z · copy · n/a — the
  resume directive: (a) the masters' «byte-identical» shot claims restated as same-host only, citing CI#39225375577,
  with the measured limit that a committed check holds no `screenshot` step; (b) the fix directive's coverage claim
  corrected: `stand_session_recording` covers the shot path on each OS; (c) at curation, the Tier-3 learning «A gated
  wrap with later chunks committed on top of its flip» retired, ES84's shared base resolver (gate v1.21) making it
  redundant.
  - bears on **Counts / qualifiers moved**: supersedes «Both are true on one host; no-change, qualifier owed: one
    host, not across OSes» — pattern `shot-os`'s two rows (`architecture.md:333` @c15414,
    `session-lifecycle.md:11` @c2632) are stale, each restated as same-host only, CI#39225375577 cited, the limit beside
    it.
  - bears on **Spec claims disproved by measurement**: adds to item 2 — its disposition is the operator's (a); adds to
    item 3 — its disposition is the operator's (b): the masters record `stand_session_recording` as the shot path's
    coverage on each OS, and no master states the fix directive's «same-host CLI-vs-MCP test».
  - bears on **Decisions & corrections**: adds a curation candidate — retire the Tier-3 entry «A gated wrap with later
    chunks committed on top of its flip», the operator's word (c).
- inputs.py verify, re-fired: `inputs: 5 entries — unchanged 0 · drifted 0 · vanished 0 · broken 0 · altered 0 ·
  unreachable 0 · n/a 5 · uncited 1 · unparsed 0`; moved since the report: I5 snapped at this resume, uncited 1 → cited
  by this addendum.

## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 6fd1d3da (the parent of the oldest pre-CI commit d2f010a2) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .github/scripts/test_ci_workflows.py — added 1 line(s) in 1 range(s)
added: 359
### examples/seven_guis/src/app.rs — added 2 line(s) in 2 range(s)
added: 162 · 164
### examples/seven_guis/tests/flows/record_checks.sh — new file · 65 line(s)
- 21-26 «fail() {»
- 42-56 @43 «replies=$(printf '%s\n' \»
- 57-61 «case $replies in»
### examples/seven_guis/tests/recording.rs — added 86 line(s) in 3 range(s)
added: 7-11 · 35 · 184-263
- 187-203 @188 «const COMMITTED: [(&str, &[u8]); 2] = [»
  - 189-195 «(»
  - 196-202 «(»
- 205-262 @207 «fn the_committed_checks_are_what_the_binary_records() {»
  - 210-214 @211 «let _guard = Started {»
  - 216-220 «command»
  - 222-226 @223 «let failed_step: Option<u32> = String::from_utf8_lossy(&ran.stdout)»
  - 227-231 «assert!(»
  - 232-237 «assert!(»
  - 238-241 «let mut names: Vec<_> = fs::read_dir(&out)»
  - 243-247 «assert!(»
  - 248-260 «for (name, committed) in COMMITTED {»
### packages/escher-driver/src/lib.rs — added 9 line(s) in 3 range(s)
added: 84-90 · 127 · 144
### packages/escher-driver/src/recording.rs — added 137 line(s) in 37 range(s)
added: 2 · 6-10 · 19 · 40-41 · 43-48 · 51-59 · 106-113 · 134-136 · 180 · 317 · 328-329 · 384-388 · 415-422 · 443 · 449
       586-594 · 620-621 · 623-628 · 635 · 806-808 · 822 · 826 · 830 · 833-840 · 842 · 870 · 872-879 · 884 · 887 · 890
       893 · 896 · 899 · 902 · 905 · 908-939 · 944
  - 56-59 «match answer {»
  - 385-388 @386 «pub(crate) fn answer(&self) -> &Value {»
### packages/escher-driver/src/replay.rs — new file · 318 line(s)
- 21-28 @24 «pub struct Replay {»
- 30-38 @33 «pub struct Change {»
- 40-70 «impl Check {»
  - 41-69 @46 «pub fn replay(&self, session: &mut Session) -> Replay {»
- 72-93 @73 «fn change_of(index: usize, step: &Step, answer: &Result<Outcome, Refusal>) -> Option<Change> {»
  - 77-79 «if step.accepted() == accepted_now && now.as_ref() == Some(then) {»
  - 81-86 «let keys = fields»
  - 87-92 «Some(Change {»
- 95-115 «impl Replay {»
  - 96-99 @97 «pub fn holds(&self) -> bool {»
  - 101-104 @102 «pub fn another_app(&self) -> bool {»
  - 106-109 @107 «pub fn ran(&self) -> usize {»
  - 111-114 @112 «pub fn change(&self) -> Option<&Change> {»
- 117-139 «impl Change {»
  - 118-121 @119 «pub fn step(&self) -> usize {»
  - 123-126 @124 «pub fn accepted_then(&self) -> bool {»
  - 128-131 @129 «pub fn accepted_now(&self) -> bool {»
  - 133-138 @136 «pub fn keys(&self) -> &[&'static str] {»
- 141-318 @142 «mod tests {»
  - 148-152 @149 «fn empty(app: &str) -> Session {»
  - 154-162 «fn call(verb: &str, args: &[(&str, crate::ArgValue)]) -> Call {»
  - 164-172 @165 «fn record(tool: &str, arguments: &str, answer: &Result<Outcome, Refusal>) -> String {»
  - 174-180 «fn check(app: &str, records: &[String]) -> Check {»
  - 182-187 @183 «const SCRIPT: [(&str, &str); 3] = [»
  - 189-210 @190 «fn recorded() -> Check {»
  - 212-221 @213 «fn a_check_replayed_on_a_fresh_session_holds() {»
  - 223-252 @224 «fn an_acceptance_that_flips_is_a_change_naming_the_keys_that_differ() {»
  - 254-284 @255 «fn a_changed_answer_names_its_keys_and_stops_the_replay() {»
  - 286-306 @287 «fn a_shot_holds_at_its_size_whatever_its_pixels() {»
  - 308-317 @309 «fn a_check_of_another_app_runs_nothing() {»
### packages/escher-driver/src/session.rs — added 7 line(s) in 2 range(s)
added: 92-93 · 135-139
### tests/blitz-tests/checks/counter-cli.json — new file · 8 line(s)
- 1-8 «[»
### tests/blitz-tests/checks/counter-mcp.json — new file · 11 line(s)
- 1-11 «[»
### tests/blitz-tests/tests/session_common/mod.rs — added 12 line(s) in 2 range(s)
added: 20 · 240-250
  - 243-246 «let line = session»
  - 247-249 «let (_, png) = line»
### tests/blitz-tests/tests/stand_replay.rs — new file · 261 line(s)
- 24-36 @25 «const CHECKS: [(&str, &[u8], usize); 2] = [»
  - 26-30 «(»
  - 31-35 «(»
- 41-43 «fn mode_name(incremental: bool) -> &'static str {»
- 45-59 @46 «fn checks() -> Vec<(&'static str, &'static str, Check, usize)> {»
  - 47-58 «CHECKS»
- 61-69 «fn text_arg<'a>(step: &'a Step, name: &str) -> Option<&'a str> {»
  - 62-68 «step.call()»
- 71-79 @73 «fn activates(step: &Step) -> bool {»
  - 74-78 «match step.call().verb.as_str() {»
- 81-110 @82 «fn each_committed_check_holds_on_the_unchanged_stand() {»
  - 83-109 «for incremental in [false, true] {»
- 112-137 @114 «mod variant {»
  - 117-135 @118 «pub fn Counter() -> Element {»
- 138-146 «.counter-root {»
- 148-158 «.counter-card {»
- 160-167 «.counter-display {»
- 169-178 «.counter-btn {»
- 180-182 «.counter-btn:hover {»
- 184-186 «.counter-btn:active {»
- 190-196 «fn variant_root() -> Element {»
  - 191-195 «rsx! {»
- 198-204 @199 «fn hold_variant(incremental: bool) -> Session {»
  - 200-202 «Session::start(slug(LeanTask::Counter), || {»
- 206-237 @207 «fn each_committed_check_turns_red_at_its_first_activation_on_a_changed_counter() {»
  - 208-236 «for incremental in [false, true] {»
- 239-261 @240 «fn a_check_on_a_session_of_another_app_runs_nothing() {»
  - 241-260 «for incremental in [false, true] {»
### tests/blitz-tests/tests/stand_session_recording.rs — added 52 line(s) in 12 range(s)
added: 1-15 · 25-27 · 32-33 · 98-102 · 104-105 · 110 · 124 · 161-164 · 166-167 · 170-183 · 273 · 275-276
- 25-27 «use escher_driver::{»
