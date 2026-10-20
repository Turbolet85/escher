proposals:
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → escher-driver"
    change: "escher-driver's unit count reads 99 unit tests in sixteen files, 64 inline and 35 in four `#[path]` test files, the history gaining «94 in sixteen (60 inline, 34 in four `#[path]` files) until 2026-10-20-load-wait» — that chunk adding session.rs's four load-wait tests and schema_tests.rs's `settled` help pin"
    sidecar: "test-plan §1 escher-driver: 94 → 99 unit tests in sixteen files, 64 inline · 35 in `#[path]` files (2026-10-20-load-wait: session.rs +4, schema_tests.rs +1)"
    rationale: "Report Counts / qualifiers moved, fact `duunit`: escher-driver unit tests 94 → 99, still sixteen files, inline 60 → 64, `#[path]` files 34 → 35 (gate `cargo test -p escher-driver --locked`: 99 passed); pattern hit test-plan.md:22@c21 changed"
    basis: ".andromeda/test-plan.md:22"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → escher-driver"
    change: "the per-file clause «session.rs 6 (…)» reads «session.rs 10 (6 until 2026-10-20-load-wait, which added the load wait's four over a scripted settle — a load answered inside the bound ends the wait settled, a load never answered ends the wait at the bound, work other than a load returns at once, a quiet settle returns after one settle; …)» (packages/escher-driver/src/session.rs:307-369)"
    sidecar: "test-plan §1 escher-driver per-file: session.rs 6 → 10, the load wait's four tests named"
    rationale: "Report fact `duunit`: `session.rs` 6 → 10; New text rows session.rs 335-341, 343-350, 352-360, 362-368 name the four tests; Coverage of new surfaces: tests unit (`session::tests` 4)"
    basis: ".andromeda/test-plan.md:22"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → escher-driver"
    change: "the per-file clause «schema_tests.rs 6 (…)» reads «schema_tests.rs 7 (6 until 2026-10-20-load-wait, which added that the `settled` help states `LOAD_WAIT` — whole seconds, so the help's «5 s» is the value's exact rendering — that the five acting verbs carry the `SETTLED` field and that the wait ends before an answer read; …)» (packages/escher-driver/src/schema_tests.rs:130-142)"
    sidecar: "test-plan §1 escher-driver per-file: schema_tests.rs 6 → 7, the `settled` help pin named"
    rationale: "Report fact `duunit`: `schema_tests.rs` 6 → 7; Deviations: the help-against-value pin also asserts the five acting verbs carry `SETTLED` and `LOAD_WAIT` is whole seconds; New text row schema_tests.rs 130-142 `the_settled_help_states_the_load_wait_and_the_wait_ends_before_an_answer_read`"
    basis: ".andromeda/test-plan.md:22"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → Session lifecycle"
    change: "session-proof's escher-driver clause reads «escher-driver's 99 unit tests (… schema_tests.rs 7 · session.rs 10 · …) — 94 until 2026-10-20-load-wait added session.rs's four load-wait tests and schema_tests.rs's `settled` help pin, as measured at escher-0.2.0/chunks/2026-10-20-load-wait/report.md; 92 until 2026-10-19-visual-lint …»"
    sidecar: "test-plan contract session-lifecycle (session-proof): escher-driver 94 → 99 unit tests, schema_tests.rs 7 · session.rs 10"
    rationale: "Report fact `duunit`: standing 2 — `registries/contracts/test-plan/session-lifecycle.md:11@c5536` (per-file list) changed; session.rs 6 → 10, schema_tests.rs 6 → 7"
    basis: ".andromeda/registries/contracts/test-plan/session-lifecycle.md:11"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§4 Unit Test Strategy → What unit tests cover → escher-driver"
    change: "add: since 2026-10-20-load-wait session.rs pins the load wait over a scripted settle (`settle_within` with a bound and a poll) — a load answered inside the bound ends the wait settled, a load never answered ends it at the bound still `Loads`, a `Render` or `Layout` result returns at once, a quiet settle returns after one settle and no sleep — and schema_tests.rs pins that the `settled` help states `LOAD_WAIT` (whole seconds), that the five acting verbs carry `SETTLED`, and that the wait ends before an answer read (packages/escher-driver/src/session.rs:307-369; packages/escher-driver/src/schema_tests.rs:130-142)"
    sidecar: "test-plan §4 escher-driver: the load wait's four session.rs unit tests and the `settled` help pin described"
    rationale: "Report Symbols / APIs (`settle_waiting`, `settle_within`, the `settled` help at schema.rs:121) and Coverage of new surfaces: tests unit (`session::tests` 4, `schema::tests` 1); Deviations: unit test (c) asserts one settle and no wall time; New text rows session.rs 307-369, schema_tests.rs 130-142"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → tests/blitz-tests"
    change: "add after the `stand_lint` entry: since 2026-10-20-load-wait `stand_load_wait` 2, declaring `mod fixtures;` and `mod session_common;`, through `Session::run` in both layout modes on a one-box page whose `fx-load` click starts a load — a load its in-file `LateNet` answers after 200 ms ends the `click` settled, no `busy`, the diff naming `fx-box` changed with `Bounds` in `differs` at height 70, the next snapshot reading 70, wall time at least 200 ms and under `LOAD_WAIT`; a load `HeldNet` never answers ends it `settled` false, `busy` `Loads`, wall time at least `LOAD_WAIT` and under `LOAD_WAIT` + 2 s, `held()` 1, the box still at no height; `harness().time()` unmoved in both; picked up by `run stand` and the stand-repeat leg by its prefix (tests/blitz-tests/tests/stand_load_wait.rs:34-164) — as measured at escher-0.2.0/chunks/2026-10-20-load-wait/report.md"
    sidecar: "test-plan §1 tests/blitz-tests: `stand_load_wait` 2 (the acting step's bounded load wait, both arms, both layout modes) entered"
    rationale: "Report Changes Files: `tests/blitz-tests/tests/stand_load_wait.rs` (new, 164 lines); Outcome (tests) v020-07 met — both arms' assertions; (arch) virtual time unmoved; Deviations: arm 1's wall ≥ 200 ms and arm 2's box at no height; New text rows stand_load_wait.rs 34-46, 52-60, 107-138, 140-164"
  - detector: D-tests-coverage
    severity: warning
    section: "§2 Test Strategy → Test directory + naming conventions → Directory pattern"
    change: "the target count reads «141 `.rs` files, 140 test targets and the unbuilt `all.rs` (140 and 139 until 2026-10-20-load-wait added `stand_load_wait.rs`, its own auto-discovered target with no manifest line; 138 and 137 until 2026-10-19-visual-lint …)»"
    sidecar: "test-plan §2 Directory pattern: blitz-tests 140 → 141 `.rs` files, 139 → 140 test targets (`stand_load_wait.rs`)"
    rationale: "Report fact `tfiles`: blitz-tests `.rs` files 140 → 141, test targets 139 → 140; pattern hit `test-plan.md:68@c3405,c3422` changed"
    basis: ".andromeda/test-plan.md:68"
  - detector: D-tests-coverage
    severity: warning
    section: "§2 Test Strategy → Test directory + naming conventions → Directory pattern"
    change: "`tests/session_common/mod.rs` is read through `mod session_common;` by 27 checks (26 before `stand_load_wait.rs` at 2026-10-20-load-wait, 25 before `stand_lint.rs` at 2026-10-19-visual-lint, …)"
    sidecar: "test-plan §2 Directory pattern: `session_common` readers 26 → 27 (`stand_load_wait`)"
    rationale: "Report fact `screaders`: `session_common` readers 26 → 27 (`stand_load_wait` declares `mod session_common;`); pattern hit `test-plan.md:68@c1349` changed"
    basis: ".andromeda/test-plan.md:68"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Crate-local test helpers → blitz-tests (session checks)"
    change: "the reader count reads «27 readers since 2026-10-20-load-wait added `stand_load_wait`, which reads `Acted`, `act`, `click` and `snapshot`; 26 before `stand_load_wait`, since 2026-10-19-visual-lint added `stand_lint` …»"
    sidecar: "test-plan §3 session checks helpers: `session_common` 26 → 27 readers (`stand_load_wait`: `Acted`, `act`, `click`, `snapshot`)"
    rationale: "Report fact `screaders`: standing 2 — `test-plan.md:103@c2926` changed; `stand_load_wait` reads `Acted`, `act`, `click`, `snapshot`"
    basis: ".andromeda/test-plan.md:103"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§2 Test Strategy → Test directory + naming conventions → Directory pattern"
    change: "`tests/fixtures/mod.rs` is read through `mod fixtures;` by 21 checks — 20 `stand_*.rs` (`stand_load_wait` since 2026-10-20-load-wait, `stand_lint` since 2026-10-19-visual-lint, …) and `font_face_sniffed_source.rs`"
    sidecar: "test-plan §2 Directory pattern: `fixtures` readers 20 → 21, `stand_*` readers 19 → 20 (`stand_load_wait`)"
    rationale: "Report fact `fxreaders`: `fixtures` readers 20 → 21, `stand_*` readers 19 → 20 (`stand_load_wait` reads `HeldNet`); pattern hit `test-plan.md:68@c2501` changed"
    basis: ".andromeda/test-plan.md:68"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Crate-local test helpers → blitz-tests (fixtures)"
    change: "«21 checks declare `mod fixtures;` — 20 `stand_*` files (`stand_load_wait` since 2026-10-20-load-wait, reading `HeldNet`; `stand_lint` since 2026-10-19-visual-lint) and `font_face_sniffed_source`»"
    sidecar: "test-plan §3 fixtures helpers: 20 → 21 readers, 19 → 20 `stand_*` (`stand_load_wait`)"
    rationale: "Report fact `fxreaders`: standing 3 — `test-plan.md:104` changed"
    basis: ".andromeda/test-plan.md:104"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Test-data bootstrap of the 5-command contract"
    change: "«`run stand` runs the 20 `stand_*` readers of the shared module» (19 until 2026-10-20-load-wait added `stand_load_wait`)"
    sidecar: "test-plan §3 test-data bootstrap: `run stand` runs 20 `stand_*` readers of the fixtures module (was 19)"
    rationale: "Report fact `fxreaders`: `stand_*` readers 19 → 20 — the same claim restated on this line («the 19 `stand_*` readers of the shared module»), not among the report's three listed sites, found by this detector's sweep"
    basis: ".andromeda/test-plan.md:132"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Agent-run contract → Proof"
    change: "append: re-counted at 2026-10-20-load-wait, `scripts/agent-run.sh` unchanged: `run stand`'s `run.end` reads passed 185 · failed 0 · ignored 7 over 46 files — +2 passed (`stand_load_wait` 2, a new file picked up by its `stand_` prefix with no script change); `run all` not re-counted — as measured at escher-0.2.0/chunks/2026-10-20-load-wait/report.md"
    sidecar: "test-plan §3 Proof: `run stand` re-counted 183 · 0 · 7 over 45 → 185 · 0 · 7 over 46 files (`stand_load_wait` 2)"
    rationale: "Report fact `runstand`: `run stand` 183 · 0 · 7 over 45 files → 185 · 0 · 7 over 46 (gate `bash scripts/agent-run.sh run stand`); Harness / gate surface: `agent-run.sh` unchanged; pattern hit `test-plan.md:119@c10078` changed — a new reading appended"
    basis: ".andromeda/test-plan.md:119"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Test-data bootstrap of the 5-command contract"
    change: "the closing reading reads «`run stand` 185 · 0 · 7, as measured at escher-0.2.0/chunks/2026-10-20-load-wait/report.md»"
    sidecar: "test-plan §3 test-data bootstrap: `run stand` 183 → 185 · 0 · 7"
    rationale: "Report fact `runstand`: standing 4 — `test-plan.md:132` changed"
    basis: ".andromeda/test-plan.md:132"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§9 CI Integration → Pipeline facts → Legs"
    change: "the stand-repeat leg's file count reads «46 files, … `stand_lint` since 2026-10-19-visual-lint and `stand_load_wait` since 2026-10-20-load-wait»"
    sidecar: "test-plan §9 Legs: stand-repeat runs 46 `stand_*.rs` files (`stand_load_wait` joins by its prefix)"
    rationale: "Report fact `runstand`: `ls tests/blitz-tests/tests/stand_*.rs | wc -l` 46; pattern hit `test-plan.md:323@c1299` («45 files») changed; Harness / gate surface: `stand-repeat` globs `stand_*.rs`, the new check joined it with no script change"
    basis: ".andromeda/test-plan.md:323"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§10 Quality Gates & Coverage Targets → Flakiness budget"
    change: "«run the 46 `stand_*.rs` files … (§9 Legs; 45 until 2026-10-20-load-wait added `stand_load_wait`, whose CI `Stand checks repeated` on CI#39360124515 read 162 · 645 · 0 · 21, +3 · +6; 44 until 2026-10-19-visual-lint …)»"
    sidecar: "test-plan §10 Flakiness budget: 45 → 46 `stand_*.rs` files; the stand repeat on CI#39360124515 162 · 645 · 0 · 21"
    rationale: "Report facts `runstand` (`test-plan.md:352` «the 45 `stand_*.rs` files» changed) and `ostally` (`test-plan.md:352` ×2, the stand repeat's reading, changed: Stand checks repeated 159 · 639 · 0 · 21 → 162 · 645 · 0 · 21 on CI#39360124515)"
    basis: ".andromeda/test-plan.md:352"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§9 CI Integration → Pipeline facts → Per-OS test tallies"
    change: "append: re-read at 2026-10-20-load-wait on CI#39360124515, the pre-CI commit `8dfd1c02b` (green 17/17): linux 202 · 1062 · 0 · 12; macos and coverage 183 · 1059 · 0 · 12; windows 183 · 1017 · 0 · 9; the a11y job 6 · 43 · 0 · 0, unmoved; the stand repeat 162 · 645 · 0 · 21 — +1 result line and +7 passed on the four test jobs over CI#39348671810 (`stand_load_wait` 2, escher-driver's 5 units), +3 · +6 on the stand repeat — as measured at escher-0.2.0/chunks/2026-10-20-load-wait/evidence/ci.md"
    sidecar: "test-plan §9 Per-OS tallies: re-read on CI#39360124515 — linux 202 · 1062, macos/coverage 183 · 1059, windows 183 · 1017, stand repeat 162 · 645"
    rationale: "Report fact `ostally`: CI#39348671810 → CI#39360124515, Test [default features] 201 · 1055 → 202 · 1062, macos and coverage 182 · 1052 → 183 · 1059, windows 182 · 1010 → 183 · 1017 (· 0 · 9), a11y 6 · 43 unmoved, stand repeat 159 · 639 · 0 · 21 → 162 · 645 · 0 · 21; pattern hits `test-plan.md:333@c3877,c3984,c4013,c4088` and fact `wstally` `test-plan.md:333@c3944` changed"
    basis: ".andromeda/test-plan.md:333"
  - detector: D-tests-coverage
    severity: warning
    section: "§9 CI Integration → Pipeline facts → Local baseline"
    change: "append to the `cargo test --workspace` re-counts: re-counted at 2026-10-20-load-wait: 202 result lines, 1062 passed · 0 failed · 12 ignored, from 201 · 1055 — +1 result line (`stand_load_wait`) and +7 passed (`stand_load_wait` 2, escher-driver 5), timings not re-measured — as measured at that chunk's `ci-leg.sh fast` run (`target/ci-logs/test.log`; escher-0.2.0/chunks/2026-10-20-load-wait/evidence/counts.md)"
    sidecar: "test-plan §9 Local baseline: workspace tally 201 · 1055 → 202 · 1062 · 0 · 12"
    rationale: "Report fact `wstally`: workspace tally 201 result lines · 1055 passed → 202 · 1062 · 0 failed · 12 ignored (gate on `target/ci-logs/test.log`: `lines 202 passed 1062 failed 0 ignored 12`; +1 line, +2 `stand_load_wait`, +5 escher-driver); pattern hit `test-plan.md:339@c18738` changed"
    basis: ".andromeda/test-plan.md:339"
  - detector: D-tests-coverage
    severity: warning
    section: "§10 Quality Gates & Coverage Targets → Coverage floors"
    change: "«… since 2026-10-19-visual-lint, which added lint.rs, it read 4634/4732 = 97.93 %, and since 2026-10-20-load-wait 4695/4794 = 97.93 %, the 97 % floor holding, 3 of 3 crates still at or above their floors (as measured at the `Coverage report` job of CI#39360124515, read in escher-0.2.0/chunks/2026-10-20-load-wait/evidence/ci.md)»"
    sidecar: "test-plan §10 Coverage floors: escher-driver 4634/4732 → 4695/4794 = 97.93 % on CI#39360124515, floor holding"
    rationale: "Report fact `cover`: escher-driver coverage 4634/4732 = 97.93 % → 4695/4794 = 97.93 %, the 97 % floor holding, 3 of 3 crates (CI#39360124515's coverage job); pattern hits `test-plan.md:351@c2240` changed and, fact `ostally`, `test-plan.md:351@c2375` (the coverage job's run) changed"
    basis: ".andromeda/test-plan.md:351"
  - detector: D-tests-coverage
    severity: warning
    section: "§5 Integration Test Strategy → Boundaries covered → Session ↔ held instance"
    change: "«with no sleep and no wall-clock read in the checks — bar two recorded exceptions whose subject is wall time: `stand_virtual_clock`'s one 800 ms sleep, …, and, since 2026-10-20-load-wait, `stand_load_wait`, which reads `Instant::now()` around its call to measure the load wait against `LOAD_WAIT` and whose in-file `LateNet` stand-in answers from its own thread after a 200 ms sleep — …»"
    sidecar: "test-plan §5 Session ↔ held instance: `stand_load_wait`'s wall read and its stand-in's 200 ms sleep recorded as the second exception to «no sleep and no wall-clock read»"
    rationale: "Report fact `sleep`: a second recorded exception to the session checks' «no sleep and no wall-clock read» — `stand_load_wait` reads `Instant::now()` around its call and its stand-in thread sleeps 200 ms; pattern hit `test-plan.md:185@c2589,c2665` changed; Decisions & corrections: a `stand_*` check may now sleep and read the wall clock where its subject is wall time (plan step 8, the test-plan §5 Expected amendment)"
    basis: ".andromeda/test-plan.md:185"
  - detector: D-tests-coverage
    severity: warning
    section: "§5 Integration Test Strategy → Boundaries covered → Session ↔ held instance"
    change: "after «a step run through `Session::act` returns settled … or `SessionError::NotSettled(Busy::Render)` at the bound …» add: since 2026-10-20-load-wait an acting step meeting a load in flight waits for it up to `LOAD_WAIT` (5 s of wall time), settling again every 10 ms and moving no time — `stand_load_wait` 2 through `Session::run` in both layout modes: a load answered at 200 ms ends the `click` settled with its content on screen, a load never answered ends it at the bound `settled` false with `busy` `Loads`, `harness().time()` unmoved in both (tests/blitz-tests/tests/stand_load_wait.rs:107-164)"
    sidecar: "test-plan §5 Session ↔ held instance: the acting step's bounded load wait and its proof `stand_load_wait` stated"
    rationale: "Report Symbols / APIs: `Session::act` and `execute::settled_step` settle through `settle_waiting` (`LOAD_WAIT` 5 s, `LOAD_POLL` 10 ms, no time moving); Outcome (tests) v020-07 and (arch) virtual time unmoved — met; New text rows stand_load_wait.rs 107-138, 140-164"
    basis: ".andromeda/test-plan.md:185"
  - detector: D-tests-coverage
    severity: warning
    section: "§8 Mocking & Stubbing Discipline → Hand-written fakes and stubs"
    change: "add a Network row: `LateNet`, `stand_load_wait`'s in-file self-answering stand-in — its `fetch` answers each request from its own thread after a 200 ms sleep, so a load is in flight when the step settles and answered inside `LOAD_WAIT`; the never-answered arm uses the shared `HeldNet`; its threads end with the test process (tests/blitz-tests/tests/stand_load_wait.rs:52-60)"
    sidecar: "test-plan §8: `LateNet` (stand_load_wait's self-answering network stand-in) recorded beside `HeldNet`"
    rationale: "Report Expected amendments: test-plan `stand_load_wait` and its in-file self-answering stand-in; fact `sleep` (its stand-in thread sleeps 200 ms); fact `fxreaders` (`stand_load_wait` reads `HeldNet`); Outcome basis / Process hygiene: the `LateNet` answer threads ended with their test process; New text row stand_load_wait.rs 52-60 «impl NetProvider for LateNet»"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → Session lifecycle"
    change: "session-start: `act(step)` settles the held harness waiting on a load in flight up to `LOAD_WAIT` (5 s of wall time, settling again every 10 ms, no time moving on the held instance), and an instance still loading at the bound is `NotSettled(Loads)`; each acting verb of `run` (`click` · `type` · `press` · `advance` · `scroll`) settles the same way on every carrier, while `snapshot` and `screenshot` run no step and wait on nothing (since 2026-10-20-load-wait)"
    sidecar: "test-plan contract session-lifecycle (session-start): `act` and every acting verb wait on a load in flight up to `LOAD_WAIT`, moving no time"
    rationale: "Report Symbols / APIs: `Session::act` (session.rs:236) now settles through `settle_waiting`, its doc (session.rs:233-234) «A load in flight is waited for up to [`LOAD_WAIT`], settling again as it is answered; no time moves while it waits»; `execute::settled_step` (execute.rs:462) through it, so all five acting verbs wait on every carrier, `snapshot` and `screenshot` not; the contract's `act` clause «the harness is settled, and an instance that did not go quiet is `NotSettled(Busy)`» is the behaviour the session checks drive"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → Session lifecycle"
    change: "session-proof: add «since 2026-10-20-load-wait `stand_load_wait` 2 — `Session::run` in both layout modes: a `click` whose load is answered inside `LOAD_WAIT` ends settled with its content on screen, one whose load is never answered ends at the bound `settled` false with `busy` `Loads`, no time moved»"
    sidecar: "test-plan contract session-lifecycle (session-proof): `stand_load_wait` 2 entered"
    rationale: "Report Outcome (tests) v020-07 met — `stand_load_wait` 2 passed, both arms in both layout modes through `Session::run`; (arch) the wait moves no virtual time — met"
    basis: ".andromeda/registries/contracts/test-plan/session-lifecycle.md:11"
note: "D-tests-framework and D-tests-obs-harness hold — the chunk runs on the standard harness (`cargo test`, `agent-run.sh` unchanged, `stand-repeat` picking the new file up by its prefix) and changes no harness, status or log format; test-plan.md:91 (settle «waits on no load») stays true of `Harness::settle`, unchanged; obs-plan.md:79's fixtures census (20 → 21) is the obs-plan detector's to move beside test-plan.md:104. All dependent-of entries point at the D-tests-coverage primary of the same fact (duunit, screaders, fxreaders, runstand)."
