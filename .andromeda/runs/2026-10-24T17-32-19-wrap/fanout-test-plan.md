proposals:
  - detector: D-tests-framework
    severity: warning
    section: "§9 CI Integration → Pipeline facts → Legs"
    change: "the `stand-repeat` leg runs, three rounds, the `tests/blitz-tests/tests/stand_*.rs` files, then ten named blitz-tests targets as literal `--test` words on the same `cargo test -p blitz-tests --locked` line (`inert_subtree` · `accessibility_states` · `scrolled_box_absolute_position` · `hit_clipped_at_scrolling_box` · `password_caret_click` · `transformed_client_rect` · `accessibility_active_descendant_unread` · `virtual_clock_engine` · `select_interaction` · `text_extent`, the files keeping their names, on the founder's «Список в скрипте (Рекомендую)», 2026-10-24), and seven_guis' eight process checks, the first red command ending the leg with its status — no retry, no `--ignored`, no env var; `test_ci_workflows.py` pins the ten words; `run stand` keeps its prefix rule, so the ten reach the agent harness through `run all` / `run {name}` alone"
    sidecar: "2026-10-24-stand-requirement-sweep: §9 Legs — stand-repeat also runs ten named non-`stand_` blitz-tests targets by a fixed `--test` list (founder, 2026-10-24); `run stand` unchanged"
    rationale: "Report Schema / config: `ci-leg.sh` `stand-repeat` (54-58) adds `--test` for the ten after the `stand_*.rs` glob, pinned by `test_stand_repeat_leg_runs_three_rounds` (359-362); Counts fact 1 lists `test-plan.md:374` as moved; Expected amendments name §9 Legs and the §3 note that `run stand` reaches the ten only through `run all` / `run {name}`; Harness / gate surface: agent-run selections unchanged; inputs#I1 the founder's word"
    basis: ".github/scripts/ci-leg.sh:54-58"
  - detector: D-tests-framework
    severity: warning
    section: "§10 Quality Gates & Coverage Targets → Flakiness budget"
    change: "the stand checks hold a zero-failure budget — the `stand-repeat` leg and its job `Stand checks repeated` run the `stand_*.rs` files, the ten named blitz-tests targets of its fixed `--test` list and seven_guis' eight process checks three times with no retry (§9 Legs)"
    sidecar: "2026-10-24-stand-requirement-sweep: §10 Flakiness budget — stand-repeat's selection gains the ten named targets"
    rationale: "Same claim as §9 Legs restated; report Counts fact 1 lists `test-plan.md:410` (\"run the `stand_*.rs` files and seven_guis' process checks\") as moved; Expected amendments name §10 Flakiness budget; CI `Stand checks repeated` 198 · 837 · 0 · 21 at CI#40114977936"
    basis: ".github/scripts/ci-leg.sh:54-58"
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: "§3 Test Harness Contract → Agent-run contract → Proof"
    change: "append the latest reading: re-counted at 2026-10-24-stand-requirement-sweep, `scripts/agent-run.sh` unchanged: `run stand`'s `run.end` reads passed 197 · failed 0 · ignored 7 over 48 files — +2 passed (`stand_requirement_sweep` +1 and `stand_lint` +1, in files already selected, with no script change); the ten named `stand-repeat` targets carry no `stand_` prefix and are not selected; `run all` not re-counted — as measured at escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md"
    sidecar: "2026-10-24-stand-requirement-sweep: §3 Proof — `run stand` 195 → 197 · 0 · 7 over 48 files"
    rationale: "Report Counts fact 12 (`run stand` 195 → 197 passed · 0 · 7 over 48 files; `test-plan.md:176` the latest reading appended); Symbols: `stand_requirement_sweep` 2 → 3, `stand_lint` 6 → 7; Outcome gate `bash scripts/agent-run.sh run stand` — `run.end` passed 197 · failed 0 · ignored 7; Harness / gate surface: `run stand` keeps the prefix rule"
    basis: "tests/blitz-tests/tests/stand_requirement_sweep.rs:994-1069"
  - detector: D-tests-framework
    severity: warning
    section: "§3 Test Harness Contract → Test-data bootstrap of the 5-command contract"
    change: "the closing reading reads `run stand` 197 · 0 · 7 over 48 files, as measured at escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md (where it read 195 · 0 · 7 at 2026-10-20-large-screen-in-parts)"
    sidecar: "2026-10-24-stand-requirement-sweep: §3 test-data bootstrap — `run stand` reading 195 → 197 · 0 · 7"
    rationale: "Duplicate of the §3 Proof reading; report Counts fact 12 lists `test-plan.md:189` (\"`run stand` 195 · 0 · 7 over 48 files\") as moved"
    basis: "escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md"
    dependent-of: D-tests-framework
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → blitz-dom covered behaviours"
    change: "…; `layout_as_text_input` has no unit test of its own; the text extent's three stated limits — RTL text read as an LTR line from the content box, vertical text read as a horizontal line, a transformed box's lines read at its client rect — are asserted by blitz-tests' `text_extent` in both layout modes (since 2026-10-24-stand-requirement-sweep), while the select's stated limits are asserted by no check, nor is the process-wide `Parallel` traversal lock"
    sidecar: "2026-10-24-stand-requirement-sweep: §1 blitz-dom covered behaviours — the text extent's stated limits now asserted by `text_extent` (3 → 6 tests); the select's still by none"
    rationale: "Report Counts fact 5 (`text_extent` 3 → 6, its three stated limits asserted; `test-plan.md:19` moved, the text extent's half only, the select's stays); Symbols: the three `text_extent.rs` tests (127-152, 154-174, 176-205), both layout modes, green under `-p blitz-tests` and `--workspace`"
    basis: "tests/blitz-tests/tests/text_extent.rs:127-205"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → Session lifecycle"
    change: "@c5793/@c6015: the driver sweep `stand_requirement_sweep` 3 (2 until 2026-10-24-stand-requirement-sweep) — `Session::run` alone on each lean task, both layout modes: one id per node line, an acting call's diff, an empty diff, the first Tab stop, two byte-equal shots, a `not-found` refusal, the `WITNESSES` record and, since 2026-10-24-stand-requirement-sweep, the `WITNESSES_020` record (rows v020-01…09, 46 witnesses, `Subject::Unit` naming a crate's unit test) with its leg check `every_0_2_0_requirement_names_its_witness_inside_stand_repeat` — every non-`Unit` witness inside `stand-repeat` by the `stand_` prefix, the ten named targets or seven_guis' eight, read from `ci-leg.sh`'s text"
    sidecar: "2026-10-24-stand-requirement-sweep: §3 → Session lifecycle — `stand_requirement_sweep` 2 → 3, the `WITNESSES_020` record and its stand-repeat leg check"
    rationale: "Report Counts fact 3 (`sweep-record`): `registries/contracts/test-plan/session-lifecycle.md:11` @c5793 (the 2 → 3) and @c6015 (the `WITNESSES` record) moved; Symbols: `WITNESSES_020` (538-818), `Subject::Unit` (227-229), the test (994-1069); `WITNESSES` byte-unchanged"
    basis: "tests/blitz-tests/tests/stand_requirement_sweep.rs:538-818"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → Session lifecycle"
    change: "@c2849: seven_guis `host_log` 3 (2 until 2026-10-24-stand-requirement-sweep added `a_recording_sessions_clients_log_nothing_of_it_at_trace`, unix only: the clients of a recording session — `start crud --record`, `type --id crud-name --text {SENTINEL}`, `snapshot`, `snapshot --part 1`, `lint`, `stop`, each run through `common::run` at `RUST_LOG=trace` in the state directories `hl-client` and `hl-client-rec`, guarded by `common::Started` — read 0 id, 0 name, 0 sentinel, 0 recording-path and 0 recorded needles over their own stderr; 1 until 2026-10-19-session-recording …)"
    sidecar: "2026-10-24-stand-requirement-sweep: §3 → Session lifecycle — seven_guis `host_log` 2 → 3, a recording session's clients' own stderr at `trace`, 0 needles"
    rationale: "Report Counts fact 8 (`host-log`): `registries/contracts/test-plan/session-lifecycle.md:11` @c2849 (\"seven_guis `host_log` 2\") moved; Symbols: `host_log.rs` `#[cfg(unix)] a_recording_sessions_clients_log_nothing_of_it_at_trace` (40-130); Outcome: `host_log` 3 under both builds, 0 of each needle class"
    basis: "examples/seven_guis/tests/host_log.rs:40-130"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Crate-local test helpers → blitz-tests (fixtures)"
    change: "the pages `password_labelled`, `password_named`, `covered` and, since 2026-10-24-stand-requirement-sweep, `many_items` with its `ITEMS` (400; tests/blitz-tests/tests/fixtures/mod.rs:64-86, moved unchanged from `stand_snapshot_parts`, read by `stand_snapshot_parts` and by `stand_act_spans`' paged part read)"
    sidecar: "2026-10-24-stand-requirement-sweep: §3 fixtures — the shared pages gain `many_items` / `ITEMS`, read by `stand_snapshot_parts` and `stand_act_spans`"
    rationale: "Report Counts fact 10 (`fixture-pages`): `test-plan.md:151` (\"the pages `password_labelled`, `password_named` and `covered`\") moved; Symbols: `fixtures/mod.rs` `pub const ITEMS: usize = 400` and `pub fn many_items()` (64-86) moved in; `stand_snapshot_parts.rs` reads `fixtures::many_items`; `stand_act_spans` holds `fixtures::hold(\"paged\", fixtures::many_items, …)` (685-693)"
    basis: "tests/blitz-tests/tests/fixtures/mod.rs:64-86"
  - detector: D-tests-coverage
    severity: warning
    section: "§7 Test Data & Fixtures → Fixture sources → tests/blitz-tests/tests/fixtures"
    change: "add to the shared pages: `many_items` (a screen that pages — `ITEMS` = 400 buttons under `#items`, ids `item-NNN`; tests/blitz-tests/tests/fixtures/mod.rs:64-86), since 2026-10-24-stand-requirement-sweep read by `stand_snapshot_parts` and `stand_act_spans`"
    sidecar: "2026-10-24-stand-requirement-sweep: §7 fixture sources — `many_items` joins the shared pages"
    rationale: "Duplicate of the §3 fixtures pages list; report Counts fact 10 lists `test-plan.md:320` (the same list) as moved; Symbols: `Needles::read` takes `#items > button` text, its 400 `item-NNN` ids join the id needles (243)"
    basis: "tests/blitz-tests/tests/fixtures/mod.rs:64-86"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§5 Integration Test Strategy → Boundaries covered → Session host ↔ what it writes"
    change: "…; a host that installs escher's sink and the `mcp` role write no id, name or typed text on stderr at `trace`, and since 2026-10-24-stand-requirement-sweep a recording session's clients (`start --record` through `stop`, a paged `snapshot --part` and `lint` among them) write none on their own stderr at `trace` — a standing check, `host_log`; a paged `part` read through the `escher-session` host's own sink stays unmeasured — the host boots only the four lean tasks and none pages; it is read in process by `stand_act_spans` — owned by its `.andromeda/residuals.md` line (target: next); the windowed binary …"
    sidecar: "2026-10-24-stand-requirement-sweep: §5 Session host ↔ what it writes — recording clients' own stderr a standing check at `trace`; the host-role paged part read a stated, owned limit"
    rationale: "Report Expected amendments: test-plan §5 — the `start --record` clients' own stderr read at `trace`, 0 needles; the paged part read in the sink capture; the host-role paged read a stated limit (Counts 6, 7, 8); fact 7 says the limit is new text with an owner, inputs#I2's Beyond-the-plan row 1, a new residuals line «a paged `part` read through the escher-session host's sink» (target: next) placed at this wrap; the body bullet states this boundary's limits (the windowed binary's) beside its detail in `test-plan-detail/integration-test-strategy.md`"
    basis: "examples/seven_guis/tests/host_log.rs:40-130"
note: "Not proposed, for the orchestrator: (1) the report does not say whether `stand_act_spans` now declares `mod fixtures;` (its added lines 6-8 are unnamed), so the reader counts in §3 fixtures (\"24 checks … 22 `stand_*` files\") and §3 test-data bootstrap (\"the 22 `stand_*` readers\") are left as they stand — re-check them against the code when applying the fixtures proposals; (2) the `host_log.rs` citations in §3 Stand log format (e.g. `host_log.rs:251-264`, `:28-345`) sit after the 97 lines added at 11-15 and 40-131 and are a cites re-point, not a claim change; (3) the residuals line for the host-role paged read must exist before the §5 proposal's owner clause lands (route-resolve, per inputs#I2)."
