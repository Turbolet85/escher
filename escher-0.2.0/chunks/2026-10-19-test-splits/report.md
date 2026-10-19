# Report — 2026-10-19-test-splits

**Chunk:** escher-driver's cli.rs and recording.rs test modules out, host_log/mcp_log's shared fragment stated once
**Date:** 2026-10-19T21:24:54Z (wrap P1; run dir `.andromeda/runs/2026-10-19T21-24-54-wrap/`)
**Commits:** `62403b643 chore(2026-10-19-test-splits): operator pre-CI commit, for the run this chunk's verdict reads`
(the one commit since `last_wrap` 2026-10-19T20:19:12Z beside the last wrap's own `f98a4dce1`; `git log
--since=2026-10-19T20:19:12Z --format='%h %s'`)

## Changes (structured — detectors read this)
- **Files:** the diff from `f98a4dce1` (the parent of the oldest pre-CI commit `62403b643`) to the work tree, read by
  `gate.py scope` (`scope: clean — changed 8 · listed 8 · recorded 0`, `.andromeda/runs/2026-10-19T21-24-54-wrap/gate-2026-10-19-test-splits.json`):
  - new: `packages/escher-driver/src/cli_tests.rs` (559 lines), `packages/escher-driver/src/recording_tests.rs` (517),
    `packages/escher-driver/src/recording_off_unix_tests.rs` (15) — line counts from `## New text, by line` below;
  - modified: `packages/escher-driver/src/cli.rs` (1,032 → 478 lines), `packages/escher-driver/src/recording.rs` (981 →
    459) — `wc -l` read by the gate entry `for f in … cli.rs … recording.rs; do n=$(wc -l < "$f") …` at implement
    (`.andromeda/runs/2026-10-19T20-53-01-implement/gate-2026-10-19-test-splits.json`, entry 10's log: `cli.rs 478`,
    `recording.rs 459`); `examples/seven_guis/tests/common/mod.rs`, `examples/seven_guis/tests/host_log.rs`,
    `examples/seven_guis/tests/mcp_log.rs`;
  - chunk records: `escher-0.2.0/chunks/2026-10-19-test-splits/evidence/{driver-tests-before.txt, log-tests-before.txt,
    counts.md, ci.md}`.
- **Symbols / APIs:** no production symbol, public item, wire, carrier, verb, exit status, log field, span, env read or
  port changed. The `cli.rs` lines 1-475 and the `recording.rs` lines 1-452 are byte-identical to `f98a4dce1` (the gate
  entry `cmp <(git show f98a4dce1…:packages/escher-driver/src/cli.rs | head -n 475) …`, green). Test-only items:
  - `cli::tests` moved whole into `cli_tests.rs`, now declared `#[cfg(test)]` / `#[path = "cli_tests.rs"]` / `mod
    tests;` at `cli.rs:476-478` (the declaration's added lines `cli.rs:477-478` per `## New text, by line`); its five
    tests and their helpers at `cli_tests.rs:15-559`, a `//!` doc at `cli_tests.rs:1-4`;
  - `recording::off_unix_tests` moved whole into `recording_off_unix_tests.rs` (`#[cfg(all(test, not(unix)))]` /
    `#[path = "recording_off_unix_tests.rs"]` / `mod off_unix_tests;` at `recording.rs:453-455`), its one test at
    `recording_off_unix_tests.rs:6-15`;
  - `recording::tests` moved whole into `recording_tests.rs` (`#[cfg(all(test, unix))]` / `#[path =
    "recording_tests.rs"]` / `mod tests;` at `recording.rs:457-459`), its nine tests and helpers at
    `recording_tests.rs:14-517`;
  - every test keeps its path: `cargo test -p escher-driver --locked --lib -- --list` is byte-equal to
    `evidence/driver-tests-before.txt` (94 lines, the gate entry `… -- --list | cmp - …driver-tests-before.txt`, green);
    each moved body is token-equal to its base span (three `diff <(git show f98a4dce1… | sed -n '…') …` entries,
    green); no item's visibility widened;
  - seven_guis' shared test module `examples/seven_guis/tests/common/mod.rs` gains the log checks' fragment, stated
    once (`common/mod.rs:370-482`, with the `//!` doc's lines 5-8 naming it): `pub struct Needles { ids, names }`
    (371-376); `pub fn stand_needles(sentinel: &str) -> Needles` (378-422, `#[track_caller]`) — boots `LeanTask::Crud`
    through `stand::boot(…, stand::options(true))`, reads the id needles (author keys holding `-` and no `/`) and the
    name needles (CRUD rows, labels, the list's `aria-label`), asserting the list's name read, ≥ 15 ids, ≥ 3 rows and
    no needle empty or a part of the sentinel; `pub fn stamped_lines(stderr: &str) -> usize` (424-439,
    `#[track_caller]`) — ≥ 1 line and every line carrying `service.name=seven_guis`, returning the line count; `pub fn
    found(stderr, needles) -> usize` (441-447); `pub fn occurrences(stderr, needles) -> usize` (449-455, each
    occurrence counted — the per-image sum); `#[cfg(unix)] pub fn recorded_needles(kept: &Path) -> Vec<String>`
    (457-482, its `///` doc kept). Callers: `host_log.rs` (all six) and `mcp_log.rs` (all six) — no other target calls
    them; `common/mod.rs` keeps its `#![allow(dead_code)]` (line 10 now, 8 before);
  - `host_log.rs` and `mcp_log.rs` lose their copies (`recorded_needles`, the needle read and its asserts, the stamp
    check, the `found` closure, the image sum) and call `common`; each keeps its own `crud-surname` (host_log) /
    `crud-name` (mcp_log) key assertion, sentinel, flow, span counts, `cfg` gates and test names, and `mcp_log` its
    `NOBODY` needle and `tool_call` field and ending checks; `host_log.rs` 315 → 232 lines, `mcp_log.rs` 333 → 253
    (`wc -l` on the work tree beside `git show f98a4dce1:… | wc -l`); their `--list` is byte-equal to `evidence/log-tests-before.txt` (4 tests).
- **Crates / modules:** escher-driver gains three private `cfg(test)` child modules in files of their own
  (`cli::tests`, `recording::tests`, `recording::off_unix_tests`) — no module added to its public tree; no crate,
  target or `[[test]]` table added (seven_guis keeps its eight targets — the gate entry `git ls-files --others … | grep
  -c -v -x …`, last line 0).
- **Dependencies:** none (the gate entry `git diff --name-only f98a4dce1… -- … Cargo.toml Cargo.lock deny.toml | grep -c
  -v -x …`, last line 0).
- **Schema / config:** none.
- **Spec-master edits:** none.
- **Retired:** none.
- **Counts / qualifiers moved:**
  - F1 · `split-count`, `cli-moved`, `rec-moved` — escher-driver's unit tests by home: 92 on unix, unchanged, but
    the inline / `#[path]` split moves from 72 inline and 20 in two `#[path]` files (`mcp_tests.rs` 14 ·
    `schema_tests.rs` 6) to 58 inline and 34 in four (`mcp_tests.rs` 14 · `schema_tests.rs` 6 · `cli_tests.rs` 5 ·
    `recording_tests.rs` 9), and the one off-unix recording test now sits in a fifth `#[path]` file,
    `recording_off_unix_tests.rs`, which the windows job alone runs. Basis: `command grep -c '#\[test\]'` on each
    file (cli_tests 5, recording_tests 9, recording_off_unix_tests 1 — research.md §Sweeps' 5 / 9 + 1 at the base),
    `cargo test -p escher-driver --locked` `92 passed`. The per-file list names change too (`cli.rs 5` → `cli_tests.rs
    5`, `recording.rs 9` → `recording_tests.rs 9`), and two master citations point into moved spans:
    `test-plan.md:22`'s `packages/escher-driver/src/cli.rs:476-1032` (now `cli_tests.rs:1-559`) and
    `test-plan.md:145`'s `packages/escher-driver/src/recording.rs:469-981` (now `recording_tests.rs:1-517`; the
    off-unix refusal `recording_off_unix_tests.rs:1-15`). Sites (pre-pass sweep, `cascade-2026-10-19-test-splits-pre-pass.json`):
    `split-count` 10 standing — stale 9: `architecture.md:137` (@c884,953,1644,1663: the out-of-line set "sit in files
    of their own … `mcp_tests.rs` and `schema_tests.rs`" — F2), `architecture.md:309` (F2), `architecture.md:331` (the
    escher-driver row's "its tests in `schema_tests.rs`" / "`mcp_tests.rs`" — F2: the row names neither new file),
    `test-plan.md:22` (@c53,109 the 72 / 20 split — F1; @c2625,5400,6090,6733,9535,9859,10020 per-file counts,
    citations and history of `mcp_tests.rs` / `schema_tests.rs` — no-change), `test-plan.md:68` (@c117,148 — F2),
    `test-plan.md:139` (F2), `test-plan.md:145` (@c2424,4705,5068,5195,10372 `schema_tests.rs` / `mcp_tests.rs`
    descriptions and citations — no-change; the line's `cli.rs pins …` and `recording.rs` entries — F1, the files
    renamed), `registries/contracts/test-plan/session-lifecycle.md:10` (the off-unix list "`recording.rs`'s
    `off_unix_tests`" — the module now in `recording_off_unix_tests.rs`, path unchanged — F1; and F6 below),
    `session-lifecycle.md:11` (@c5676 the per-file list `cli.rs 5 … recording.rs 9` — F1; @c5740,6436 — no-change) ·
    no-change 1: `security-plan.md:318` (a citation of `mcp_tests.rs:605-606`, unmoved); `cli-moved` 1 standing
    (`test-plan.md:22` @c9705 — stale) · `rec-moved` 1 standing (`test-plan.md:145` @c5001 — stale).
  - F2 · `path-children`, `split-count` — out-of-line unit-test modules held in files of their own through
    `#[path]`: five → eight (dioxus-native-dom `dioxus_document::bridge_tests`, blitz-dom `document::changed_set_tests`
    and `document::font_reading_tests`, escher-driver `mcp::tests` and `schema::tests`, now with escher-driver
    `cli::tests`, `recording::tests` and `recording::off_unix_tests`), test paths unchanged. `path-children` 11
    standing — stale 4: `architecture.md:137` (@c1114,1368,1402,1435,1471 the module list), `architecture.md:309`,
    `test-plan.md:68` (@c177,297,331), `test-plan.md:139` · no-change 7 (a true claim about one of the five's own
    tests, counts or history): `architecture.md:328`, `test-plan.md:11`, `:12`, `:25`, `:57`, `:101`, `:336`; the
    leaf `.claude/rules/testing.md:14` ("five modules sit in files of their own") is the cascade's.
  - F3 · `cli-cite` — every master citation of `packages/escher-driver/src/cli.rs` at or before line 474 still
    resolves (the head is byte-identical): `cli-cite` 9 standing — no-change 8 (`architecture.md:183` ×12,
    `architecture.md:223` @c10015 `454-474`, `security-plan.md:84` ×8, `security-plan.md:409`, `design-system.md:396`
    ×3, `layout-templates.md:73` ×9, `obs-plan.md:69`, `obs-plan.md:185` ×4 — each range read ≤ 474) · stale 1
    (`test-plan.md:22`, the `cli-moved` row above).
  - F4 · `rec-cite` — every master citation of `packages/escher-driver/src/recording.rs` at or before line 452 still
    resolves: `rec-cite` 17 standing — no-change 16: 7 of `escher-driver/src/recording.rs` ≤ 452
    (`architecture.md:187` ×4, `:218`, `security-plan.md:80`, `:81`, `:192` ×2, `obs-plan.md:342` ×2,
    `session-lifecycle.md:5` ×2) and 9 of other files sharing the token — `stand_session_recording.rs` and seven_guis
    `tests/recording.rs` (`architecture.md:336`, `:337`, `security-plan.md:133`, `test-plan.md:10`, `:27`, `:52`,
    `:203` ×2, `:204`, `:230`) · stale 1 (`test-plan.md:145`, the `rec-moved` row above).
  - F5 · `log-cite`, `common-mod` — the line maps of the three seven_guis files moved: `common/mod.rs`'s doc grew 6 →
    8 lines, so every line after it moves +2 (a cited `common/mod.rs:200-208` now reads at 202-210; `:1-6` the doc is
    `:1-8`; the file 367 → 482 lines, `wc -l`); `host_log.rs` lost lines 36-61 (`recorded_needles`) and the needle read (78-123),
    so its `:212-225` image read, `:24-315` flow and `:30-61` gates read elsewhere (`:125`, `:125-144` moved up;
    `:1-11` and `:18` unchanged); `mcp_log.rs` lost lines 66-91 and the needle read (96-136), so `:51-333`, `:56-91`
    and `:200-213` read elsewhere. `log-cite` 24 standing — digits for P2's citation sweep 15 (each row read there):
    `architecture.md:214`, `:223`, `:337`, `security-plan.md:126`, `:323`, `:408`, `test-plan.md:10`, `:50`, `:51`,
    `:68` (@c7821), `:105`, `:186`, `obs-plan.md:70`, `:190`, `:341` · no-change 9 (blitz-tests' own `tests/common/mod.rs`
    or `tests/session_common/mod.rs`, other files sharing the token): `architecture.md:137`, `:336`, `test-plan.md:27`,
    `:102`, `:103`, `a11y-plan.md:217`, `:284`, `:330`, `session-lifecycle.md:8`.
  - F6 · `common-mod` — what seven_guis' shared module holds: beside its state dir, clear, `Host`, `drain`, `run`,
    `client`, the answer readers, `Speaker` and `Started`, now the log checks' needle readers, stamp check and counts
    (`Needles`, `stand_needles`, `stamped_lines`, `found`, `occurrences`, `recorded_needles`); `recorded_needles` is no
    longer `host_log`'s and `mcp_log`'s own. `common-mod` 21 standing — content-stale 4: `test-plan.md:10` (@c5582
    `mcp_log` "unix-gated with its `recorded_needles`"), `test-plan.md:68` (@c7433, @c7804 the shared module's
    description), `obs-plan.md:70` (@c12367 the shared module "installs no subscriber" — true; its contents now include
    the needle readers), `session-lifecycle.md:10` ("seven_guis `mcp_log`'s recorded test, `recorded_needles`") ·
    citation-only 8 (the F5 digits): `architecture.md:214`, `:223`, `:337`, `security-plan.md:323`, `test-plan.md:50`,
    `:51`, `:105`, `obs-plan.md:341` · no-change 9 (blitz-tests' module): `architecture.md:137`, `:336`,
    `test-plan.md:27`, `:102`, `:336`, `obs-plan.md:74`, `a11y-plan.md:217`, `:284`, `:330`.
  - F7 · `cov-reading`, `cov-tests-rs` — escher-driver's coverage reading moved, 5180/5268 = 98.33 % at CI#39319403188
    (`f98a4dce1`) → 4266/4350 = 98.07 % at CI#39324662409 (`62403b643`), floor 97 % held, 3 of 3 crates at or above their
    floors in both (`evidence/ci.md`, the coverage job's lines read from `gh run view … --log`); see **Spec claims
    disproved**. `cov-reading` 3 standing — no-change 3 (`architecture.md:66`, `security-plan.md:76`,
    `test-plan.md:348`: the floors, unchanged; no master states a measured reading — the pattern's `98\.33|5180/5268`
    alternatives drew 0 rows); `cov-tests-rs` 23 standing — no-change 23 (`coverage_floor.py`'s contract as written —
    `architecture.md:206` @c7573 "it sums `LF`/`LH` over every `SF` record …" is true of the script — and citations or
    counts of existing `*_tests.rs` files); no master states that a `*_tests.rs` file is counted.
  - F8 · `file-size` — `cli.rs` 1,032 → 478 and `recording.rs` 981 → 459 lines, both out of the over-800 set: no master
    site — the pattern's control refused, `control: file-size, jscpd, title never fired over the seven masters'
    pre-pass text (62403b64)`.
  - F9 · `jscpd` — 6 clones / 95 lines → 0 among `host_log.rs`, `mcp_log.rs`, `common/mod.rs` (jscpd 5.4.0, the gate
    entry `d=$(mktemp -d) && jscpd --format rust …`, last line 0): no master site — the same refusal line.
  - Unmoved, verified: escher-driver 92 unit tests, seven_guis' `host_log` and `mcp_log` 2 tests each, the workspace
    tally `lines 199 passed 1045 failed 0 ignored 12` (the fast leg's `target/ci-logs/test.log`, the gate entry `grep -E
    '^test result:' target/ci-logs/test.log | awk …`), every CI per-job tally (`evidence/ci.md`: Test [default features]
    199 · 1045 · 0 · 12, Test (windows) 180 · 1000 · 0 · 9, Stand checks repeated 156 · 624 · 0 · 21, macos and Coverage
    report 180 · 1042 · 0 · 12, a11y 6 · 43 · 0 · 0 — equal to the base run's), escher-driver's 2 `info_span!` sites,
    the env-read set across the seven_guis tests and escher-driver sources.
  - The `log-checks` pattern (reach for every claim about `host_log` / `mcp_log`): 28 standing — no-change 28: every
    needle class, level, build, span count and test name they state holds (the gates above), their embedded citations
    being F5's rows. The `owner-generic` pattern: 2 standing — no-change 2 (`architecture.md:90`, `security-plan.md:125`,
    other owners).
  - Sweep reach: 12 patterns (10 then 2, over two calls; 3 more refused unfired) · 9 moved facts, 9 with a pattern (the
    report's own count) · owner pattern control unfired (`control: file-size, jscpd, title never fired …`) · by line 38
    lines · 246 offsets (the first call's block; the second's `by line · 0 lines · 0 offsets`).
- **Dev-tool versions:** none — jscpd re-read at 5.4.0 (`jscpd --version`, dev host), the version research measured.
- **Harness / gate surface:** none — no agent-run script, CI step, leg or verdict shape changed; the `stand-repeat`
  leg and `ci-leg.sh:57` still name `host_log` and `mcp_log`, unchanged targets.
- **Cross-project / external claims:**
  - CI#39324662409 on `62403b643d44` — `verdict: green · checks 17/17` (`ci.py conclusion --sha HEAD --wait 1800`,
    `evidence/ci.md`), the coverage floor 3 of 3, the windows job running `off_unix_a_recording_is_refused_as_unsupported`
    from its new file; the verdict was taken on the pre-CI commit's tree, to which this wrap's own commit adds.
  - CI#39319403188 on `f98a4dce1` (the base) — `completed / success` (`gh run view 39319403188 --json
    conclusion,headSha,status`), read for the base's per-job tallies and coverage lines.
  - `inputs.py verify`: `inputs: 1 entries — … n/a 1 · uncited 1 · unparsed 0`; cited here:
    - inputs#I1 · `message: the operator, in this session, 2026-10-19 20:50Z` · copy · n/a (a message has no live
      source) — the operator's own word "yes" on the P5 review card
      `.andromeda/runs/2026-10-19T20-32-57-phase/p5-card-1.md`, approving the plan and its Beyond-the-plan row 1, its
      authority `word: "yes" — the operator, 2026-10-19`. The card's `## Beyond the plan` rows, verbatim:

      | # | proposal | kind |
      |---|---|---|
      | 1 | The audit's three other directions stay unowned audit directions for the founder's judgment. None is on the working entry, and this chunk does not take them: M1's reuse of `command.rs`/`wire.rs` readers in the recording module's field parsing, M1's `stand_replay` Counter variant vs `counter.rs`, and M2's per-verb helpers for `cli.rs::read`. | deferral (no owner proposed) |
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - "`coverage_floor.py` sums every `SF` record under the crate directory, so moved lines stay counted" and "The
    forecast is that the floor reading does not move" — stated in `research.md` §Files inspected
    (`.github/scripts/coverage_floor.py` row), `scope.md` §Boundaries ("moved lines stay counted") and `plan.md` §Test
    Commands' prose. Measured false: escher-driver's reading moved 5180/5268 = 98.33 % → 4266/4350 = 98.07 % (LF −918,
    LH −914) between CI#39319403188 and CI#39324662409. The coverage job's per-file report lists no `*_tests.rs` file in
    either run — neither the new `cli_tests.rs` / `recording_tests.rs` nor the older `mcp_tests.rs` / `schema_tests.rs`
    — while `cli.rs` reads 847 → 437 and `recording.rs` 1303 → 485 there and `client.rs` and `mcp.rs` are unchanged
    (`evidence/ci.md` § A premise the run falsified). So cargo-llvm-cov leaves a `*_tests.rs` sibling out of the lcov
    before `coverage_floor.py` reads it; the script's own contract (`architecture.md:206` @c7573) is true as written.
    Pattern `cov-tests-rs` / `cov-reading`: no master site states the falsified premise or a measured reading (23 + 3
    rows, all no-change, F7) — no master site; the claim lives in this chunk's research, scope and plan. The floor
    held, so no gate is red. Whether a master should state that the coverage leg omits `*_tests.rs` files (test-plan
    §10 Coverage floors, architecture's coverage-leg sentence) is P2's.
- **Expected amendments (from plan):**
  - test-plan §1 Test Scope Summary → escher-driver (72 inline / 20 in two → 58 inline / 34 in four, cli_tests 5,
    recording_tests 9, the off-unix module a fifth `#[path]` file; `cli.rs:476-1032` and `recording.rs:469-981`
    re-pointed) — carried: F1 · `split-count` test-plan 4 rows (`:22`, `:68`, `:139`, `:145`), `cli-moved` 1
    (`test-plan.md:22`), `rec-moved` 1 (`test-plan.md:145`).
  - test-plan §2 Directory pattern and §4 Conventions (five → eight out-of-line modules) — carried: F2 ·
    `path-children` test-plan 2 stale rows (`:68`, `:139`).
  - test-plan §3 → Session lifecycle key file (the per-file list names the new files; the shared module names its
    helpers) — carried: F1 · `split-count` `session-lifecycle.md:10`, `:11`; F6 · `common-mod`
    `session-lifecycle.md:10`.
  - architecture §Conventions → Tests (five → eight `#[path]` children; the escher-driver row names the new files) —
    carried: F2 · `split-count` architecture 3 rows (`:137`, `:309`, `:331`), `path-children` 2 (`:137`, `:309`).
  - security-plan §Logging & Monitoring and §Secret Management (re-point the citations into `host_log.rs`,
    `mcp_log.rs`, `common/mod.rs`) — carried: F5 · `log-cite` security-plan 3 rows (`:126`, `:323`, `:408`).
  - obs-plan §3 Logging stack (the shared module now holds the needle readers; the new files' silence census 0) —
    carried: F6 · `common-mod` `obs-plan.md:70`; F5 · `log-cite` obs-plan 3 rows (`:70`, `:190`, `:341`).
  - layout-templates and design-system (no amendment expected: every `cli.rs` citation ≤ 474) — carried, no change:
    F3 · `cli-cite` `design-system.md:396`, `layout-templates.md:73`, both read ≤ 474.
- **Coverage of new surfaces:** none — no external surface, hot-path operation or UI element is added; the three new
  files are `cfg(test)` modules and the shared fragment is test code.
  - `seven_guis tests/common/mod.rs` log-check fragment → validation n/a · instrumentation n/a (silence census 0: the
    gate entry `cat …cli_tests.rs …recording_tests.rs …recording_off_unix_tests.rs …common/mod.rs | grep -c -E
    'println!|…|set_global_default'`, last line 0) · PII n/a (assertion messages name a kind and a count, never a
    needle or a captured line) · tests integ (`host_log`, `mcp_log`) · a11y n/a · tokens n/a

## Deviations from intent
1. **A fifth shared helper, `common::occurrences`** (`common/mod.rs:449-455`). After step 4's four helpers, jscpd still
   read 1 clone (`host_log.rs` 207-213 ↔ `mcp_log.rs` 232-238, the per-image occurrence sum); the plan's
   Implementation notes direct a residual near-copy to be shared too, never a file excluded — shared, then 0.
2. **`host_log`'s two needle-sanity asserts are one** — step 4 states the shared assert as "no needle is empty or a part
   of the sentinel" (`mcp_log`'s combined form); `host_log` had two ("no needle is empty", "the sentinel is no part of a
   needle"). The condition is the same, the message merged; `host_log`'s own `crud-surname` assert now follows the
   shared asserts instead of preceding them.
3. **The needle reader's shape** — `stand_needles(sentinel) -> Needles { ids, names }`, a two-field struct; the plan
   named what it returns, not its type.
4. **The driver move was scripted** (`.andromeda/runs/2026-10-19T20-53-01-implement/split.py`: each base span's opening
   and closing lines asserted, a dedent of exactly four spaces, any other line refused), then `rustfmt` on the five
   files; the moved bodies hold only `\`-continued string literals, whose leading whitespace Rust strips, so the dedent
   changes no string's value. The byte-identity and three token-equality entries are green.
5. **`mcp_log` calls `common::recorded_needles` by path** inside its `#[cfg(unix)]` block, not through its `use` line,
   so the import needs no gate of its own off unix.

scope record: none — gate.py scope clean, 0 recorded (`scope: clean — changed 8 · listed 8 · recorded 0 (companion 0 ·
mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 54`).

## Decisions & corrections
- A coverage premise a test split rests on: cargo-llvm-cov leaves a `*_tests.rs` sibling (the `#[path]` test-child
  seam) out of its lcov, so moving inline tests into one moves the crate's line-coverage reading — escher-driver 98.33 %
  → 98.07 %, LF −918 — though `coverage_floor.py` sums every `SF` record it is given; research's "moved lines stay
  counted" read the script and not what the lcov holds. Evidence: `evidence/ci.md` § A premise the run falsified (the
  per-file rows of CI#39319403188 and CI#39324662409, no `_tests.rs` file in either). A curation candidate (a fact a
  future split's forecast needs).
- Sweep hazard: a pattern `recording\.rs:` matches `stand_session_recording.rs:` and seven_guis `tests/recording.rs:`
  citations (9 of `rec-cite`'s 17 rows); `tests/common/mod.rs:` matches blitz-tests' and seven_guis' modules alike (9 of
  `log-cite`'s 24) — a path-qualified pattern (`escher-driver/src/recording\.rs:`) and a range-bounded one (`cli-moved`,
  `rec-moved`) separate the moved sites. Evidence: the pre-pass sweep rows (`cascade-2026-10-19-test-splits-pre-pass.json`).
- Sweep hazard: the shell's `grep` refused a bounded repetition `.{200}` at this P1 and printed nothing (exit nonzero, no
  rows) — read by Python instead; a recurrence of the standing T1 learning (2026-10-07) and the global CLAUDE.md Shell
  rule, held at one re-issue.
- The operator's words this chunk, all given in this session (2026-10-19): "yes" on the P5 card (inputs#I1); "Run the
  operator pass for 2026-10-19-test-splits now, each entry in plan order: 23 hygiene; the commit with the subject exactly
  "chore(2026-10-19-test-splits): operator pre-CI commit, for the run this chunk's verdict reads"; 24 the fast leg with
  the clean-tree guard and the push; 25 ci.py conclusion; 26 the per-OS tallies. Record it in evidence/ci.md. Stop and
  tell me if any entry is red." — the pass fired on it, the commit staging the whole tree as the previous chunk's pre-CI
  commit `79603d1ef` did; and, at this wrap, "run P1 only and stop" (the two-window form).
- Entry 26 fired twice, the second with `--id 39319403188` (the base run), because its note names the base run as a
  comparison; both read `recorded`, `history: unmoved`.
- jscpd's residual clone after the first merge was anticipated by the plan's Implementation notes and resolved by
  sharing (Deviation 1), never by excluding a file.
- No guard or hook refusal this chunk; no failed command outstanding.

## Outcome
Acceptance criteria, re-asserted against the diff:
- (tests) `cargo test -p escher-driver --locked` reads `test result: ok. 92 passed; 0 failed`; its `--list` is
  byte-identical to `evidence/driver-tests-before.txt`; every `cli::tests::…` and `recording::tests::…` path unchanged —
  MET.
- (arch) `cli.rs` and `recording.rs` hold no inline test module (the `^mod (tests|off_unix_tests) \{` count reads 0) and
  carry three `#[path]` declarations (count 3); each moved body token-equal to its base span; `cli.rs` 1-475 and
  `recording.rs` 1-452 byte-identical; no item's visibility widened — MET.
- (tests) `cargo check --tests --locked --target x86_64-pc-windows-gnu -p escher-driver -p seven_guis` exits 0; the CI
  windows job ran the off-unix test from its new file and `mcp_log` — MET.
- (arch) each source body under 500 lines: `cli.rs` 478, `recording.rs` 459 — MET.
- (security) `cargo test -p seven_guis --locked` passes and the workspace leg runs `host_log` and `mcp_log` (2 `Running
  tests/(host_log|mcp_log).rs` lines); both still assert 0 typed-sentinel, 0 id, 0 name, 0 shot-prefix and, recording, 0
  recording needles at `RUST_LOG=trace` with their exact span-line counts; their `--list` equals
  `evidence/log-tests-before.txt` — MET.
- (tests) jscpd 5.4.0 reads 0 clones among the three files (base 6 / 95); the fragment is stated once in
  `common/mod.rs` — MET.
- (obs) the new files and the shared module carry 0 print, tracing or subscriber sites; escher-driver holds its 2
  `info_span!` sites; the env-read set reads as at the base — MET.
- (security) no tracked file outside the eight moves, no untracked file beyond the three new ones; `Cargo.toml`,
  `Cargo.lock`, `deny.toml` unchanged — MET.
- (tests) `ci-leg.sh fast` and `ci-leg.sh doc` green; the workspace tally `lines 199 passed 1045 failed 0 ignored 12` —
  MET.
- (tests) the CI run on the pushed sha reads `verdict: green` — CI#39324662409, its coverage job at the floors 3 of 3
  and its stand-repeat leg green — MET. (Its coverage reading moved; F7, Spec claims disproved — the criterion asks the
  floors, which hold.)
- (layouts · design) `escher-session`'s endings, statuses, stdout/stderr split and usage line untouched — no
  production line moved — MET.
- Capabilities: none claimed (`matrix.py show --chunk 2026-10-19-test-splits`: `claimed by 2026-10-19-test-splits: 0`).

Gates (implement's one firing, `.andromeda/runs/2026-10-19T20-53-01-implement/gate-2026-10-19-test-splits.json`:
`entries 26 · green 22 · red 0 · recorded 0 · timeout 0 · not-run 4`; 0 fix iterations; no deferral — rust delta 8):
- `cargo test -p escher-driver --locked` — green · exit 0 · `contains test result: ok. 92 passed; 0 failed`
- `cargo test -p escher-driver --locked --lib -- --list | cmp - …driver-tests-before.txt` — green · exit 0
- `cargo check --tests --locked --target x86_64-pc-windows-gnu -p escher-driver -p seven_guis` — green · exit 0
- `cat …cli.rs …recording.rs | grep -c -E '^mod (tests|off_unix_tests) \{'` — green · exit 1 · last line 0
- `cat …cli.rs …recording.rs | grep -c -E '^#\[path = "(cli|recording|recording_off_unix)_tests\.rs"\]$'` — green · exit 0 · last line 3
- `cmp <(git show f98a4dce1…:…cli.rs | head -n 475) … && cmp <(… recording.rs | head -n 452) …` — green · exit 0
- `diff <(git show f98a4dce1…:…cli.rs | sed -n '478,1031p' …) <(grep -v '^//!' …cli_tests.rs …)` — green · exit 0 · no output
- `diff <(… recording.rs | sed -n '471,980p' …) <(… recording_tests.rs …)` — green · exit 0 · no output
- `diff <(… recording.rs | sed -n '455,466p' …) <(… recording_off_unix_tests.rs …)` — green · exit 0 · no output
- `for f in …cli.rs …recording.rs; do n=$(wc -l < "$f"); … test "$n" -lt 500 || exit 1; done` — green · exit 0
- `cargo test -p seven_guis --locked 2>&1` — green · exit 0 · lacks FAILED
- `cargo test -p seven_guis --locked --test host_log --test mcp_log -- --list | cmp - …log-tests-before.txt` — green · exit 0
- `d=$(mktemp -d) && jscpd --format rust … host_log.rs mcp_log.rs common/mod.rs …` — green · exit 0 · last line 0
- `cat examples/seven_guis/tests/*.rs … packages/escher-driver/src/*.rs | grep -oE 'env!…' | sort | uniq -c | awk …` — green · exit 0 · the expected env-read line
- `cat …cli_tests.rs …recording_tests.rs …recording_off_unix_tests.rs …common/mod.rs | grep -c -E 'println!|…'` — green · exit 1 · last line 0
- `grep -r -h -F 'info_span!' packages/escher-driver/src | wc -l` — green · exit 0 · last line 2
- `git diff --name-only f98a4dce1… -- packages examples … | grep -c -v -x -E …` — green · exit 1 · last line 0
- `git ls-files --others --exclude-standard -- packages examples … | grep -c -v -x -E …` — green · exit 1 · last line 0
- `bash .github/scripts/ci-leg.sh fast` — green · exit 0 · artifact fresh (`target/ci-logs/test.log`)
- `grep -c -E 'Running tests/(host_log|mcp_log)\.rs' target/ci-logs/test.log` — green · exit 0 · last line 2
- `grep -E '^test result:' target/ci-logs/test.log | awk …` — green · exit 0 · `contains lines 199 passed 1045 failed 0 ignored 12`
- `bash .github/scripts/ci-leg.sh doc` — green · exit 0
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` — `leg = 'operator'`, fired as written by
  the operator pass: `hygiene: clean — read 46 (runs 40 · evidence 3 · inputs 3) …` (`evidence/ci.md`)
- `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin
  build/escher-0.2.0` — `leg = 'operator'`, fired through `gate.py run --operator 24`: green · exit 0 · `history moved:
  refs/remotes/origin/build/escher-0.2.0 f98a4dce→62403b64` (the intended push) (`evidence/ci.md`)
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800` — `leg =
  'operator'`, fired as written: `62403b643d44 verdict: green · checks 17/17 · wall 1026 s · runs CI#39324662409
  completed/success` (`evidence/ci.md`)
- `gh run view <id> -R Turbolet85/escher --log | sed … | awk …` — `leg = 'operator'`, `expect = []`: recorded · exit 0,
  fired with `--id 39324662409` and again with `--id 39319403188`; the outcome: every per-job tally equal to the base
  run's and to CI#39312031677's (Test [default features] 199 · 1045 · 0 · 12; Test (windows) 180 · 1000 · 0 · 9; Stand
  checks repeated 156 · 624 · 0 · 21; Test (macos) and Coverage report 180 · 1042 · 0 · 12; a11y 6 · 43 · 0 · 0) —
  disposition: unmoved, as predicted, nothing to explain by name (`evidence/ci.md`).
- Smoke: skipped — no boot-path or UI-surface change (test code only; the plan lists no smoke or self-verify entry).

Watches: none folded (scope.md: "Freight on the entry: none").

Outcome basis: the operator pass ran — Setup 4's commit list from `f98a4dce1`: `62403b643 chore(2026-10-19-test-splits):
operator pre-CI commit, for the run this chunk's verdict reads` — and the final HEAD's CI run CI#39324662409 is recorded
in `evidence/ci.md` (the record P7 re-verifies). Implement's report is this window's own (the implement run
`.andromeda/runs/2026-10-19T20-53-01-implement/`, its `outcome-2026-10-19-test-splits.md` written whole by it), the
basis for what only it holds (the gate firing, the deviations). The operator directive between implement and this
report: the operator pass, quoted under Decisions & corrections; it changed no file of the chunk (the commit stages, the
push publishes; `evidence/ci.md` is its record, written after the pass).

Process hygiene: implement P4's census — `census · ps · comm escher-session, seven_guis_nati, jscpd, cargo · args — ·
own tree excluded: 8` → `census: none running` (none started by hand; each gate entry's cargo, jscpd and `escher-session`
children ended with it); re-measured at this wrap with the same names: `census: none running`.
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: f98a4dce (the parent of the oldest pre-CI commit 62403b64) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### examples/seven_guis/tests/common/mod.rs — added 117 line(s) in 2 range(s)
added: 5-8 · 370-482
- 371-376 @373 «pub struct Needles {»
- 378-422 @384 «pub fn stand_needles(sentinel: &str) -> Needles {»
  - 388-394 «let ids: Vec<String> = harness»
  - 395-403 «let text_of = |selector: &str| -> Vec<String> {»
  - 407-412 «let names: Vec<String> = rows»
  - 415-420 «assert!(»
- 424-439 @427 «pub fn stamped_lines(stderr: &str) -> usize {»
  - 429-432 «let unstamped = stderr»
  - 434-437 «assert_eq!(»
- 441-447 @442 «pub fn found(stderr: &str, needles: &[String]) -> usize {»
  - 443-446 «needles»
- 449-455 @450 «pub fn occurrences(stderr: &str, needles: &[String]) -> usize {»
  - 451-454 «needles»
- 457-482 @460 «pub fn recorded_needles(kept: &Path) -> Vec<String> {»
  - 461-464 «let names: Vec<_> = fs::read_dir(kept)»
  - 469-473 «let dir_name = kept»
  - 474-481 «std::iter::once(dir_name)»
### examples/seven_guis/tests/host_log.rs — added 10 line(s) in 6 range(s)
added: 44-47 · 53-54 · 192 · 206 · 208 · 224
  - 44-47 «use common::{»
### examples/seven_guis/tests/mcp_log.rs — added 9 line(s) in 6 range(s)
added: 20 · 68-70 · 175 · 230 · 233 · 245-246
### packages/escher-driver/src/cli.rs — added 2 line(s) in 1 range(s)
added: 477-478
### packages/escher-driver/src/cli_tests.rs — new file · 559 line(s)
- 15-18 «fn line(words: &[&str]) -> Line {»
- 20-22 «fn text(value: &str) -> ArgValue {»
- 24-35 «fn instance(verb: &str, args: &[(&str, ArgValue)]) -> Line {»
  - 25-34 «Line::Instance {»
- 37-43 «fn session(command: SessionCommand) -> Line {»
  - 38-42 «Line::Session {»
- 45-53 «fn recorded_start(app: &str) -> Line {»
  - 46-52 «Line::Session {»
- 55-61 «fn served(record: Option<&str>) -> Line {»
  - 56-60 «Line::Serve {»
- 63-65 «fn malformed(fault: Fault) -> Line {»
- 67-419 @68 «fn a_line_is_read_by_its_rules_in_order() {»
  - 70-414 «let rows: Vec<(&[&str], Line)> = vec![»
  - 416-418 «for (row, (words, read)) in rows.into_iter().enumerate() {»
- 421-439 @423 «fn an_argument_that_is_not_text_is_a_usage_error() {»
  - 427-434 «assert_eq!(»
- 441-523 @442 «fn each_ending_writes_its_streams_and_returns_its_status() {»
  - 444-451 «let failed = |error: SessionError| {»
  - 452-489 «let rows = [»
  - 491-505 «for (row, (ending, stdout, stderr, status)) in rows.into_iter().enumerate() {»
  - 509-517 «assert_eq!(»
- 525-538 @526 «fn the_usage_line_is_built_from_the_table_and_the_apps() {»
  - 527-534 «assert_eq!(»
  - 535-537 «for verb in VERBS {»
- 540-559 @541 «fn a_line_that_is_no_command_or_is_refused_runs_nothing() {»
  - 542-544 «let never = |_: &str| -> Result<Session, SessionError> {»
  - 549-552 «assert_eq!(»
  - 555-558 «assert_eq!(»
### packages/escher-driver/src/recording.rs — added 4 line(s) in 2 range(s)
added: 454-455 · 458-459
### packages/escher-driver/src/recording_off_unix_tests.rs — new file · 15 line(s)
- 6-15 @7 «fn off_unix_a_recording_is_refused_as_unsupported() {»
  - 10-13 «assert!(matches!(»
### packages/escher-driver/src/recording_tests.rs — new file · 517 line(s)
- 14-23 @15 «fn scratch(name: &str) -> PathBuf {»
  - 16-19 «let dir = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/tmp")).join(format!(»
- 25-27 «fn text(value: &str) -> ArgValue {»
- 29-37 «fn call(verb: &str, args: &[(&str, ArgValue)]) -> Call {»
  - 30-36 «Call {»
- 39-49 «fn acted() -> Result<Outcome, Refusal> {»
  - 40-48 «Ok(Outcome::Acted {»
- 51-57 «fn mode(path: &Path) -> u32 {»
  - 52-56 «fs::metadata(path)»
- 59-86 @60 «fn the_digest_is_fnv_1a_64_of_the_written_answer() {»
  - 61-65 «let rows: [(&[u8], u64); 3] = [»
  - 67-69 «for (row, (bytes, digest)) in rows.into_iter().enumerate() {»
  - 73-76 «assert_eq!(»
  - 77-79 «let shown = Ok(Outcome::Screen {»
  - 80-82 «let written = Outcome::Screen {»
- 88-175 @89 «fn a_recording_reads_back_as_the_calls_it_kept() {»
  - 93-137 «let calls = [»
  - 139-141 «for (row, (call, masked, answer)) in calls.iter().enumerate() {»
  - 147-159 «for (row, (step, (call, _, answer))) in check.steps().iter().zip(&calls).enumerate() {»
  - 168-170 «assert!(text.contains(»
- 177-219 @178 «fn a_masked_type_keeps_the_stand_in_and_nothing_of_the_text() {»
  - 189-191 «assert!(written.contains(&format!(»
  - 195-198 «let longer = call(»
  - 201-204 «assert_eq!(»
  - 207-209 «let [masked, clicked] = check.steps() else {»
  - 211-217 «assert_eq!(»
- 221-242 @222 «fn a_call_no_record_can_spell_is_not_kept() {»
  - 225-232 «let rows = [»
  - 235-237 «for (row, unkept) in rows.iter().enumerate() {»
- 244-288 @245 «fn the_check_and_its_directory_are_owner_only_and_no_file_is_replaced() {»
  - 258-261 «assert_eq!(»
  - 262-265 «assert_eq!(»
  - 277-284 «for open_mode in [0o750, 0o705, 0o777] {»
  - 285-287 «for dir in [dir, created, open] {»
- 290-340 @291 «fn a_recording_that_would_pass_its_bound_or_cannot_write_ends_and_removes_its_check() {»
  - 295-297 «for row in 1..MAX_CHECK_STEPS {»
  - 300-303 «assert_eq!(»
  - 314-319 «assert_eq!(»
  - 324-330 «let widest = call(»
  - 332-334 «while recording.record(&widest, false, &acted()) {»
  - 337-339 «for dir in [dir, blocked, wide] {»
- 344-346 @345 «const UNANSWERED: &str =»
- 348-350 «fn check(records: &[&str]) -> Vec<u8> {»
- 352-388 @353 «fn a_check_in_the_grammar_is_read() {»
  - 354-378 «let rows = [»
  - 380-382 «for (row, bytes) in rows.iter().enumerate() {»
  - 384-386 «let [step] = read.steps() else {»
- 390-488 @391 «fn a_check_outside_the_grammar_is_refused_as_malformed() {»
  - 393-480 «let rows = [»
  - 482-487 «for (row, bytes) in rows.iter().enumerate() {»
- 490-517 @491 «fn a_check_at_its_bounds_is_read_and_one_past_them_is_refused() {»
  - 497-500 «assert_eq!(»
  - 502-505 «assert_eq!(»
  - 512-515 «assert_eq!(»
