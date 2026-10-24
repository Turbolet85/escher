# Scope — 2026-10-24-stand-requirement-sweep

**Working entry (the intent anchor, verbatim):** Stand requirement sweep — every 0.2.0 capability proven headless on
stand or fixture, inside stand-repeat; coverage floors held, new verbs' stderr id-free (intent §Principles)

**Version:** escher-0.2.0 · Epoch 5 — Polish & ship · taken up 2026-10-24

**Intent anchor:** `escher-0.2.0/intent.md` §Principles, "Kept from 0.1.0 … The stand is the proof, with a minimal
fixture beside it where the tasks lack a case"; `escher-0.2.0/requirements.md:5-6`, "Every capability is verified
headless, on the 7GUIs stand or on a minimal fixture beside it where the tasks lack the case".

## What this chunk builds

- **The sweep.** Every 0.2.0 capability whose requirement is a behaviour of the running UI is proven headless on the
  stand's tasks or on a minimal fixture beside them, and the sweep records, per requirement of
  `escher-0.2.0/requirements.md`, the task or fixture that holds its case and the check that proves it there. A gap the
  record finds — a requirement with no such check — is filled with one.
- **The requirements in reach are v020-01…v020-09**, each `verified` in `verification-matrix.json` at take-up (9/11).
  v020-10 (Capabilities stated for a consumer) and v020-11 (Cold-agent bar) are out: the markerless entries «Capability
  statement» (working-route.md:53) and «Cold-agent bar» (working-route.md:55) name them and are ordered after this one.
- **The sweep re-proves; it does not re-claim.** (Closed at P3: `matrix.py show --dir escher-0.2.0 --unclaimed` reads
  the pool `[v020-10 v020-11]` alone, both owned by later entries, so this chunk has no cap to claim; a requirement whose
  witness this chunk widens records it in the cap's notes at the wrap, never by a re-claim.)
- **Two of the nine are not behaviours of a screen.** (Closed at P3: v020-01's acceptance is read on the code audit's
  complexity instrument, tokei line counts and the `fast`/`doc`/`a11y` legs; v020-02's on cargo-mutants runs, a tier run
  on the dev host alone and never in CI — `.claude/rules/testing.md` «Not yet measured»; its three direct tests are unit
  tests — `examples/seven_guis/src/tasks/timer.rs`, `packages/escher-driver/src/mcp_tests.rs`,
  `packages/dioxus-native-dom/src/dioxus_document_tests.rs:148` — which the `test` leg runs and `stand-repeat` never
  selects.) The sweep's record names their witnesses as they stand; whether the sweep re-reads either instrument, and
  whether any mutation run belongs to this chunk, is P4's to settle from what the matrix's `acceptance` and the test
  plan say.
- **The record's form** follows 0.1.0's. (Closed at P3: `tests/blitz-tests/tests/stand_requirement_sweep.rs:236`
  holds `WITNESSES: [(&str, &[Witness]); 16]`, one row per v010 requirement bar v010-15, each naming
  `(Subject, file, test)`; `every_requirement_names_its_witness` (`:579-613`) pins the length 16, the row order v010-01…17
  bar 15, and each witness found as `fn {test}(` through `source()`'s `include_str!` map (`:532-576`). No file outside it
  pins the table — research.md §Sweeps.) Whether 0.2.0's rows join that table, take a table of their own beside it, or
  take another form is a P4 fork.
- **"Inside stand-repeat".** (Closed at P3: the `stand-repeat` leg (`.github/scripts/ci-leg.sh:48-60`) runs every
  `tests/blitz-tests/tests/stand_*.rs` target and eight seven_guis targets three rounds under a zero-failure budget, and
  `test_stand_repeat_leg_runs_three_rounds` (`.github/scripts/test_ci_workflows.py:351-362`) pins named words of it, not
  its whole list. Of the witnesses the matrix's refs name for v020-01…09, ten files lie outside it, all run by the `test`
  leg alone: the integration checks `inert_subtree`, `accessibility_states`, `scrolled_box_absolute_position`,
  `hit_clipped_at_scrolling_box`, `password_caret_click`, `transformed_client_rect`,
  `accessibility_active_descendant_unread` (v020-03), `virtual_clock_engine` (v020-06) and `select_interaction` (v020-08),
  and C1's reader `text_extent`; plus v020-02's unit tests above. Every other named witness is a `stand_*` file or one of
  the eight seven_guis targets — research.md §Sweeps, the inventory.) Every integration witness the record names runs
  inside `stand-repeat` at this chunk's end; how a witness outside it gets inside (the leg's selection widened, or the
  proof carried by a `stand_*` check) is a P4 fork. v020-02's unit tests and v020-01's instruments cannot be placed in
  `stand-repeat` (a unit test is no `--test` target; the mutation tier never runs in CI) — the record states that.
- **"Coverage floors held".** (Closed at P3: the `coverage` leg's floors (`ci-leg.sh:46`: `escher-driver` 97 ·
  `dioxus-native-dom` 83 · `blitz-test-harness` 90, through `.github/scripts/coverage_floor.py`) read at HEAD on
  CI#40105170961's coverage job 97.99 % (4918/5019) · 85.57 % (2740/3202) · 94.15 % (949/1008), `3 of 3 crates at or
  above their floors`; blitz-dom, with no floor, reads 78.98 % whole-crate (15652/19817, upstream's body included) while
  its 0.2.0 files read `clock.rs` 90.00 % · `node/range.rs` 99.25 % · `node/select.rs` 100 % · `text_extent.rs` 100 % —
  research.md §Patterns.) The floors hold at this chunk's end. Whether a floor is re-read only, raised to what 0.2.0
  holds, or a crate added is a P4 question; a floor change is a CI gate change.
- **"New verbs' stderr id-free".** [premise-corrected: `examples/seven_guis/tests/host_log.rs:4-10,39-176` and
  `mcp_log.rs:4-11,68-163` already drive one accepted `lint`, one `snapshot` part read and, in their recording tests,
  `serve --record` and `mcp --record`, at `RUST_LOG=trace`, asserting 0 id, 0 name, 0 typed-sentinel and 0 recording
  needles — and they run under both builds: the `test` leg (`cargo test --workspace`) and `stand-repeat`
  (`cargo test -p seven_guis`)] The sweep's record names these as the witnesses of the 0.2.0 verbs' id-free stderr, and
  the sweep closes the two readings that stay unmeasured: (a) a part read on a screen that pages — the host role boots
  the four lean tasks alone and each fits the budget, so `host_log`'s and `mcp_log`'s part reads answer part 1 of 1
  (`host_log.rs:174`, `mcp_log.rs:162`) and no sink-installing binary has read a paged screen's part; (b) the
  `escher-session` client's own stderr at `trace` under `start --record` — `record_checks.sh` asserts its clients write
  nothing to stdout or stderr at the default level only (`examples/seven_guis/tests/recording.rs:233`). Whether each is
  measured by a new reading or stated as a limit is a P4 question.

## Folded freight (the one CARRY at working-route.md:49)

Folded as a hypothesis: each named artifact and line is re-verified at P3 before it shapes the plan.

- **C1 · the visual lint's readings no check asserts** (from 2026-10-19-visual-lint): "a clipped or off-screen node
  holding text also reads `truncated`, and `BaseDocument::text_extent`'s stated limits (RTL and vertical text read as
  LTR lines, a transformed box at its client rect); measured at
  escher-0.2.0/chunks/2026-10-19-visual-lint/report.md (Decisions «Untested reading»; Symbols / APIs)". Each reading
  gets a check that asserts it, on a fixture, in both layout modes — or, where the reading is judged a defect rather
  than a limit, a fork (a change to `Session::lint`'s findings changes a driver answer on every carrier; a change to
  `text_extent` is an engine change for every document — playbook's engine-change rule).

## Mechanism claims carried in the freight (marker text kept verbatim; closed at P3 against HEAD)

- C1, "measured at escher-0.2.0/chunks/2026-10-19-visual-lint/report.md (Decisions «Untested reading»; Symbols /
  APIs)". [premise-corrected: the report's «Untested reading» (`report.md:239-240`) says the reading "is not measured";
  re-derived at HEAD from `packages/escher-driver/src/lint.rs:269-293` and `:483-503`, `truncated()` runs on every
  element with an area, independent of the `clipped`/`off-screen` match, and fires when the text box
  (`text_extent(..).text`, from the content box's top-left) is not inside the clip region cut by the element's own
  padding box where it clips — so an off-screen element holding text with area always reads `truncated` too, while a
  clipped one reads it only when its text box reaches past the region (a box clipped in its padding alone, its lines
  inside, reads `clipped` without `truncated`)] No check asserts either: `stand_lint`'s CRUD-row test filters its
  findings to `Clipped | OffScreen` (`tests/blitz-tests/tests/stand_lint.rs:263-267`). The `text_extent` limits stand
  verbatim in its doc (`packages/blitz-dom/src/text_extent.rs:22-24`), and its three tests read neither an RTL, a
  vertical nor a transformed box (`tests/blitz-tests/tests/text_extent.rs:35,59,86`).

## CI (Setup 5a — every commit from the last wrap's flip `126f56e67` through HEAD)

- CI#40105170961 on `4b4804ba5d42` (HEAD, `chore(cites)`): in progress at Setup's read, **green** at P3's
  (`gh run view 40105170961`: completed/success).
- CI#40101552119 on `7ad040af66c2` (`chore(u55)`, sidecar headings): **green**, checks 17/17, wall 1225 s.
- CI#40101465212 on `126f56e67002` (the last wrap's flip): **not green** — completed/cancelled, checks 14/14, wall
  91 s; not success, verbatim: `Test (${{ matrix.platform.name }})=cancelled · Documentation=cancelled ·
  Accessibility (a11y) tests=cancelled · Dependency audit=cancelled · Stand checks repeated=cancelled · Build [default
  features]=cancelled · Build counter example=cancelled · MSRV Build [Rust 1.91]=cancelled · Coverage report=cancelled
  · Build wasm examples=cancelled · Test [default features]=cancelled · Clippy=cancelled · run CI=cancelled`. (Closed
  at P3 against those runs: CI#40101465212 was created 14:12:28Z and ended cancelled at 14:14:03Z, after the next push's
  CI#40101552119 started at 14:13:40Z; `git diff --stat 126f56e67 7ad040af6` changes six `.andromeda/*-amendments.md`
  lines and no code, so CI#40101552119's green 17/17 — `Stand checks repeated` and `Coverage report` included — reads the
  same code tree.) Nothing red is carried.

## Boundaries

- v020-10 and v020-11 are «Capability statement»'s and «Cold-agent bar»'s; their CARRYs (working-route.md:53) are that
  entry's. «A11y re-read» (working-route.md:51) owns the stand's Tab order, Enter/Space activation and contrast
  re-asserted after 0.2.0's engine changes and the new a11y checks in the `a11y` leg; this chunk does not take them.
- An engine change for every document, a boundary widening (a new carrier, field or exit; a driver answer that moves
  on every carrier) or a new gate direction is the founder's fork, raised as a question dialog for the operator to take
  to him — never decided as a lean (`.andromeda/playbook.md`).
- `.andromeda/residuals.md`'s open lines are not this chunk's freight: the route folds a residual only through an
  entry's annotation. The F334 line's four-citation remainder (`.andromeda/residuals.md:37`) is documentation hygiene,
  not a stand requirement.
- A reading the sweep measures and leaves unbuilt keeps an owner — a `.andromeda/residuals.md` line or a CARRY on a
  later entry, placed at this chunk's wrap and listed on the P5 card's `## Beyond the plan`; never dropped.

## Surfaces and contracts touched (grounded at P3 — research.md §New files / §Files to modify)

- The stand checks `tests/blitz-tests/tests/stand_*.rs`, their shared `common/`, `session_common/` and `fixtures/`,
  and the 0.1.0 sweep's record `stand_requirement_sweep.rs`; new fixture checks one file and one test target per
  behaviour, both layout modes (`.github/scripts/test_blitz_tests_targets.py` pins the manifest's one `[[test]]` table,
  not the target list, so a new file moves nothing there).
- `examples/seven_guis/tests/{host_log,mcp_log}.rs` (the stderr measurement) and its shared `tests/common/mod.rs`.
- CI: `.github/scripts/ci-leg.sh` (`stand-repeat`'s selection, `coverage`'s floors) and its invariants
  `.github/scripts/test_ci_workflows.py` — only where a P4 fork answers "change".
- The engine (`packages/blitz-dom/src/text_extent.rs`) and the lint (`packages/escher-driver/src/lint.rs`) only where
  a founder's fork answers "change"; `text_extent`'s one non-test caller is `lint::truncated` (`lint.rs:487`, code
  graph).
