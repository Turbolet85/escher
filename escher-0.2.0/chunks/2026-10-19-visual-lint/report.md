# Report — 2026-10-19-visual-lint

**Chunk:** Visual lint — one command reports overlap, clipping, truncated text, off-screen, zero size and text contrast by stable id with a measured reason, fixture-proven
**Date:** 2026-10-20
**Commits:** since `last_wrap` 2026-10-19T23:29:30Z: `5a9b806de chore(2026-10-19-visual-lint): operator pre-CI commit, for the run this chunk's verdict reads` (the operator pass; base `e97a7f41b`, its parent)

## Changes (structured — detectors read this)
- **Files:** new `packages/blitz-dom/src/text_extent.rs` · `packages/escher-driver/src/lint.rs` ·
  `tests/blitz-tests/tests/text_extent.rs` · `tests/blitz-tests/tests/stand_lint.rs`; modified
  `packages/blitz-dom/src/lib.rs` (`mod text_extent;` and `pub use text_extent::TextExtent;`, lines 69-70 · 111) ·
  `packages/escher-driver/src/lib.rs` (a crate-doc paragraph on `Session::lint`, `mod lint;`, `pub use lint::{Bounds,
  Defect, Finding, Reason};`, lines 82-87 · 130 · 148); chunk evidence `evidence/{readings,counts,ci}.md` (basis: `cites.py
  added`, the listing pasted below; `gate.py scope`: `clean — changed 6 · listed 6 · recorded 0`).
- **Symbols / APIs:**
  - blitz-dom: `pub struct TextExtent { content: BoundingRect, text: BoundingRect }` (text_extent.rs:5-15) and
    `BaseDocument::text_extent(&self, NodeId) -> Option<TextExtent>` (text_extent.rs:18-54) — a new public read-only
    reader on every document. `content` is the client rect less border, padding and any scrollbar. `text` starts at the
    content box's top-left, moved down by the inline layout's `block_offset`, and is sized by Parley `Layout::width()` /
    `height()` over its scale. `None` for an id that does not resolve (`get_node`), a node that is not an inline root
    (`flags.is_inline_root()`), or one with no inline layout data. It mutates nothing and has no log site, print, env read or
    panic path. Its rustdoc states its limits: RTL and vertical text are read as LTR horizontal lines, and a transformed box is
    read at its client rect. Its one non-test caller is the lint.
  - escher-driver (lint.rs): `pub struct Bounds { x, y, width, height: f64 }` (16-27), a lint-owned rectangle in the
    snapshot bounds' space; `pub enum Defect` (95-110) with `Defect::ALL` (113-121) in the order `Overlap` ·
    `Clipped` · `Truncated` · `OffScreen` · `ZeroSize` · `Contrast`, and `Defect::name()` (123-133) giving `overlap` ·
    `clipped` · `truncated` · `off-screen` · `zero-size` · `contrast`; `pub enum Reason` (141-174) —
    `Overlap { other: String, area: Bounds }` · `Clipped { visible: Bounds }` · `Truncated { text: Bounds, visible:
    Option<Bounds> }` · `OffScreen` · `ZeroSize` · `Contrast { ratio: f32, required: f32 }`; `pub struct Finding { id,
    defect, bounds: Bounds, reason }` (176-187). All derive `Debug`, `Clone`, `PartialEq`. None holds a `NodeId`, page
    text, a colour or a value.
  - `Session::lint(&mut self) -> Result<Vec<Finding>, Refusal>` (lint.rs:190-230), in process only:
    - It refuses `Cause::FontsUnavailable` before anything is read when `can_paint_text` is false.
    - It reads `harness.doc.snapshot()` and records each node's id in the session's `SeenIds`, as `snapshot` does.
    - It runs no step, settle, pump or time move.
    - Each node's clip region is `visible_region`, cut by the padding box of every DOM ancestor below the root whose
      computed `contain` includes `paint`. That walk is the lint's own; `visible_region` is unchanged.
    - The readings: overlap over the visible parts of non-ancestor pairs, reported once at the highest level, on the
      earlier node; clipped; truncated, a non-text-control node whose `text_extent` text box is not inside the clip
      region cut by its own padding box where it clips; off-screen; zero-size, a no-area node holding a name, a value,
      an `enabled` reading, focusability or text descendants; contrast, a node with direct non-whitespace text or a
      text control, exempt at `enabled == Some(false)`, `None` not measured, reported at `ratio < required_ratio()`.
    - Findings come in document order, then `Defect::ALL` order.
  - The computed `contain`, `overflow-x`/`-y` and `display` are read through the public, read-only
    `BaseDocument::resolved_style_value` as closed keyword sets. The caller set of `visible_region`,
    `Harness::text_contrast`, `can_paint_text`, `element_ids`, `get_client_bounding_rect` and `is_focussable` grows by
    the lint; every existing caller stays.
  - `Harness::text_contrast` gains its first non-test caller.
  - No verb, `Command`, `Outcome` variant, written form, MCP tool, recorded step, socket, CLI arm, env var or port is
    added. `VERBS` stays eleven. Basis: entry 7, the guard over every tracked file outside the six, read 0.
- **Crates / modules:** new private modules `blitz_dom::text_extent` and `escher_driver::lint`; no crate added.
- **Dependencies:** none (no manifest or `Cargo.lock` change — the plan's git guard entry 7 read 0).
- **Schema / config:** none.
- **Spec-master edits:** none.
- **Retired:** none.
- **Counts / qualifiers moved:**
  - `run stand` passed 178 · failed 0 · ignored 7 over 44 files → 183 · 0 · 7 over 45 (`stand_lint` +5).
    Basis: gate entry 10's `run.end`. Pattern `stand-count`: 11 standing · 2 changed (test-plan.md:119, the count
    history, @c9471, a new reading appended; test-plan.md:132, under `fixtures-readers`) · 9 no-change (test-plan.md:19,
    :27 ×5, :52, :110, :113, :118, :125, :150 and obs-plan.md:99 name the `run stand` selection, not its count).
  - `stand_*.rs` files 44 → 45, read by `run stand` and the stand-repeat leg. Basis: `ls tests/blitz-tests/tests/stand_*.rs`
    reads 45. Pattern `stand-files`: 3 standing · 3 changed (test-plan.md:119 @c9534, the history, appended;
    test-plan.md:323 @c1299; test-plan.md:352 @c294).
  - blitz-tests `.rs` files 138 → 140 and test targets 137 → 139 (the unbuilt `all.rs` unchanged). Basis: `ls
    tests/blitz-tests/tests/*.rs` reads 140. Pattern `bt-files`: 1 standing · 1 changed (test-plan.md:68 @c3309,3326).
  - Workspace tally 199 result lines · 1045 passed · 0 failed · 12 ignored → 201 · 1055 · 0 · 12. Basis: gate entry 14
    over the fresh `target/ci-logs/test.log`. Pattern `ws-tally`: 2 standing · 2 changed (test-plan.md:333 @c2896 and
    :339 @c18243).
  - Per-OS CI tallies, previous run CI#39336988025 → this run CI#39348671810. Pattern `ci-tally`: 1 standing ×5 ·
    changed (test-plan.md:333). Basis: entry 19, `evidence/ci.md`.

    | job | before | after |
    |---|---|---|
    | Test [default features] | 199 · 1045 · 0 · 12 | 201 · 1055 · 0 · 12 |
    | macos | 180 · 1042 · 0 · 12 | 182 · 1052 · 0 · 12 |
    | coverage | 180 · 1042 · 0 · 12 | 182 · 1052 · 0 · 12 |
    | windows | 180 · 1000 · 0 · 9 | 182 · 1010 · 0 · 9 |
    | a11y | 6 · 43 · 0 · 0 | 6 · 43 · 0 · 0 (unmoved) |
    | stand repeat | 156 · 624 · 0 · 21 | 159 · 639 · 0 · 21 |

  - `tests/fixtures/mod.rs` readers 19 checks (18 `stand_*` + `font_face_sniffed_source`) → 20 (19 `stand_*` + it),
    `stand_lint` declaring `mod fixtures;`. Likewise the `stand_*` readers of the shared module that `run stand` runs,
    18 → 19. Basis: `stand_lint.rs` `mod fixtures;`. Pattern `fixtures-readers`: 10 standing · 4 changed
    (architecture.md:360 @c286 "by 19 (18 `stand_*` files …)"; test-plan.md:68 @c2451; test-plan.md:104 @c818,854;
    test-plan.md:132 @c421) · 6 no-change (architecture.md:142, no count and its one-reader rule holding for `lint_page`;
    test-plan.md:27, :232, :281, a11y-plan.md:285 path citations; obs-plan.md:79, the module's silence, true).
  - `tests/session_common/mod.rs` readers 25 → 26, `stand_lint` declaring `mod session_common;` (it reads `hold`,
    `act`, `click`, `type_into`, `refused`, `focused`, `slug`). Pattern `session-readers`: 14 standing · 2 changed
    (architecture.md:360 @c157 "by 25"; test-plan.md:103 @c2926 "25 readers") · 12 no-change (path citations:
    architecture.md:137, :222, :231, :351; security-plan.md:323; test-plan.md:27, :50, :68, :103's other offsets;
    obs-plan.md:70, :75, :343; `registries/contracts/test-plan/session-lifecycle.md:8`).
  - escher-driver line coverage 4266/4350 = 98.07 % → 4634/4732 = 97.93 %; floor 97 % holds, 3 of 3 crates at or above
    their floors. Basis: the `Coverage report` job of CI#39348671810, its `coverage_floor.py` lines read with `gh run
    view --log`. Pattern `cov-driver`: 1 standing · 1 changed (test-plan.md:351 @c2168).
  - `Harness::text_contrast`'s callers were all tests; the lint is the first non-test caller. Basis: research.md §Graph
    impact. Pattern `text-contrast`: 12 standing · 2 changed (a11y-plan.md:80 and :233, §6 Color contrast, adds the
    lint's reading under the same exempt rule) · 10 no-change (architecture.md:181, :185, :344, :351; test-plan.md:27,
    :48, :90; obs-plan.md:70, :118; the a11y bootstrap contract file: true statements of the reader, unchanged).
  - Unmoved, measured: escher-driver's `info_span!` sites stay 2 (entry 5); the a11y leg stays 43 over 6 (entry 12);
    `VERBS` stays 11.
  - Sweep reach: 15 patterns (the tool's `total` lines, 14 + 1) · 11 moved facts, 11 with a pattern (the report's own
    count) · owner pattern control unfired (`title · control UNFIRED over the seven masters at 5a9b806d`) · by line 23
    lines · 82 offsets (22 · 81 + 1 · 1). Pattern sets: `.andromeda/runs/2026-10-20T00-54-29-wrap/report-patterns.toml`;
    trail `cascade-2026-10-19-visual-lint-pre-pass.json`.
  - The other standing rows, dispositioned per class:
    - `visible-region` 6: no-change; the reader is unchanged and reused.
    - `contain-paint` 7: no-change; the hit-test claims hold. The lint's own walk is a new fact for arch §Driver
      session.
    - `lint` 11: no-change; the lint leg and the upstream-sync marker paths, none stating the visual lint.
    - `driver-session` 14: no-change; architecture.md:191 is the home of the new `Session::lint` fact.
    - `spans` 32: no-change; two spans, unmoved.
    - `owner-generic` 2: no-change (architecture.md:90, security-plan.md:125); they name other owners.
    - `text-extent` and `at-boot`: control unfired, no master site (`text-extent · control UNFIRED …`,
      `at-boot · control UNFIRED …`).
- **Dev-tool versions:** none.
- **Harness / gate surface:** none changed. `scripts/agent-run.sh`, `.github/scripts/ci-leg.sh` and the workflows are
  byte-unchanged (the git guard, entry 7, reads 0). `run stand` and the stand-repeat leg pick up `stand_lint` by its
  prefix, and `run text_extent` reads it by name.
- **Cross-project / external claims:**
  - The CI run this chunk's verdict reads: CI#39348671810, push, sha `5a9b806de224`, `verdict: green · checks 17/17 ·
    wall 963 s` (entry 18, `evidence/ci.md`).
  - inputs (`inputs.py verify`): `inputs: 4 entries — unchanged 0 · drifted 0 · vanished 0 · broken 0 · altered 0 ·
    unreachable 0 · n/a 4 · uncited 2 · unparsed 0`. The two uncited before this report are I3 and I4, both cited here.
    - `I1 · message: the operator, with /andromeda-phase, 2026-10-19T23:47Z · copy message · n/a` — the founder-forks
      directive (cited by scope.md, research.md, plan.md).
    - `I2 · message: the operator, by question dialog at phase P4, 2026-10-20T00:00Z · copy message · n/a` — F1 «In
      process only (Most conservative)» and F2 «No new field (Recommended)», answered as delegate, PROVISIONAL; T1–T3
      (Recommended each).
    - inputs#I3 · `message: the operator, at phase P5, 2026-10-20T00:12Z · copy message · n/a` — the word «yes» on
      `.andromeda/runs/2026-10-19T23-47-25-phase/p5-card-1.md`, the operator's own word (a delegate's for F1 and F2,
      which stay PROVISIONAL). The card's `## Beyond the plan` rows, verbatim:

      | # | Proposal | Owner |
      |---|---|---|
      | 1 | F1 and F2, answered by the operator as delegate, stay PROVISIONAL for the founder's own word | the wrap that closes Epoch 4 lists them, with the carried `:focus-within` take, for the founder's boundary batch |

    - inputs#I4 · `message: the operator, in this session, 2026-10-20T00:54Z · copy message · n/a` — the operator pass
      directive (entries 16–19 in plan order, the pre-CI commit's exact subject, `evidence/ci.md`, stop on a red) and this
      wrap's «run P1 only and stop».
- **Reverted / negative API facts:**
  - A third `lint.rs` unit test, `a_box_is_cut_only_on_the_axes_asked`, was written and removed before any gate fired.
    It would have moved the planned counts (lint units 2, workspace +10 passed).
  - Two temporary diagnostic assertion messages in `stand_lint.rs`, which printed findings during development, were
    restored from a scratch copy; the second restore was checked by sha256.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - The plan's step 3 and its acceptance assume escher-driver can hold blitz-dom's `BoundingRect` as `Finding.bounds`
    (plan.md:62) and read a computed `contain` itself.
  - escher-driver depends on neither blitz-dom nor stylo (`packages/escher-driver/Cargo.toml` `[dependencies]`:
    blitz-test-harness, dioxus-native-dom, png, serde_json, tracing), and the plan's guard entry 7 forbids the manifest
    edit.
  - DISPOSED in the plan's favour of the guard: Deviations 1 and 2.
  - No master states the claim (`text-extent`'s control unfired; `driver-session`'s rows describe no lint type).
- **Expected amendments (from plan):**
  - arch §Standard Contracts → Driver session: `Session::lint`, `Finding`, `Defect`, the six readings, in process — carried
    (Symbols / APIs). Search: `driver-session` 7 rows in architecture.md (:90, :116, :165, :189, :191, :202, :211), the home
    :191; `lint` 0 sites of the fact. The pattern is «Boundary widening» (escalate); F1/F2 are PROVISIONAL (inputs#I2,
    inputs#I3 row 1).
  - arch §Standard Contracts → Scrolling, selection, tree: `BaseDocument::text_extent`, read-only, its limit — carried
    (Symbols / APIs). Search: `text-extent` control unfired; `visible-region` architecture.md :161, :165, :191, the home
    :161.
  - security-plan §Input Validation: a read-only reader on every document, and findings holding ids, closed words and
    numbers only, crossing nothing — carried (Symbols / APIs, Coverage). Search: `driver-session` security-plan.md :78,
    :79, :82, :84, :129; `text-extent` control unfired.
  - test-plan §1 / §2 / §3 / §9: `stand_lint` and `text_extent`, the target and file counts, the stand selection's
    count — carried (Counts / qualifiers moved; patterns `stand-count`, `stand-files`, `bt-files`, `ws-tally`,
    `ci-tally`, `fixtures-readers`, `session-readers`, `cov-driver`, their test-plan.md rows listed there).
  - obs-plan §3, the four new files' silence (gate entry 6: 0), and §4, no span for the in-process lint (gate entry 5:
    2) — carried (Coverage). Search: `spans` obs-plan.md :30, :70, :78, :99, :118, :119, :171, :172, :193, :312 (no-change
    rows; :78 already states 2 `info_span!` sites). The silence fact has no master site yet (`text-extent` control
    unfired).
  - a11y-plan §6: `Harness::text_contrast` gains its first non-test caller under the same exempt rule — carried
    (Counts / qualifiers moved). Search: `text-contrast` a11y-plan.md :80, :233.
  - layout-templates §Surface: desktop-native → IA notes: the stand at boot reads the measured lint readings, 0 findings
    on each lean task — carried (Outcome; `evidence/readings.md`). Search: `at-boot` control unfired; `visible-region`
    layout-templates.md :45.
- **Coverage of new surfaces:**
  - `Session::lint` (in-process API, escher-driver) → validation n/a (takes no argument, no external input) ·
    instrumentation n/a (no span or field, by F2; there is no verb in process to record) · PII redacted✓ (findings hold
    ids, closed words and numbers; a text control's value is never measured, a masked control never read) · tests
    unit (2) + integ (`stand_lint` 5, both modes) · a11y n/a (no UI; its contrast reading is SC 1.4.3's measure) ·
    tokens n/a.
  - `BaseDocument::text_extent` (read-only blitz-dom reader) → validation n/a (a `NodeId` through `get_node`) ·
    instrumentation n/a (no log site) · PII n/a (geometry only) · tests integ (`text_extent` 3, both modes, both
    builds through the fast leg) · a11y n/a · tokens n/a.
  - `lint_page` (a test fixture in `stand_lint.rs`, one reader) → tokens n/a: its hard-coded colours (`#999999`,
    `#ffffff`, `#dddddd`, `#bbbbbb`) are planted test data, not shipped UI.

## Deviations from intent
1. **`Finding.bounds` and every `Reason` rectangle are a lint-owned `Bounds`**, not `BoundingRect` (the same four
   numbers). escher-driver cannot name `BoundingRect` without a blitz-dom dependency. That dependency moves `Cargo.toml` and
   `Cargo.lock`, which guard entry 7 forbids.
2. **Computed `contain` / `overflow-x` / `overflow-y` / `display` are read through `BaseDocument::resolved_style_value`**
   against closed keyword sets: `paint` · `strict` · `content`; `visible`; `inline` · `contents` · `none` and the
   internal table displays other than `table-cell` and `table-caption` excluded. stylo's `Contain` / `Overflow` are
   unnameable for the same reason as item 1. `visible_region` is unchanged (the constraint held).
3. **Overlap is read over each box's visible part** (the box cut to its clip region), not the whole box. This is a
   definition choice no ruling covers, listed for the operator's review. Unless it is cut, a CRUD row scrolled out of
   `crud-list` overlaps whatever stands below the list on screen. On every page tested it reads the same as the
   plan's letter: no clipped planted or stand box lies over another.
4. `lib.rs` re-exports `Bounds` and `Reason` beside `Defect` and `Finding` (they are public field types) and carries a
   crate-doc paragraph on `Session::lint`.
5. `Reason::Truncated.visible` is `Option<Bounds>`, `None` when no part of the text box shows.
6. The planted root `fx-root` is 784 × 584 inside the body's default 8 px margin, not 800 × 600. An 800 × 600 root inside
   the margin passes the viewport and reads `clipped`; an out-of-flow root leaves `body` with no height, which reads
   `zero-size`.
7. `TextExtent.content` also subtracts a scrollbar's size (the CSS content box). There is none on the test pages.
8. `text_extent.rs`'s pages are HTML booted by `Harness::from_html_with(_, stand::options(incremental))`, and its stale
   id is made by `remove_and_drop_node`.
9. The planted-page check asserts one extra precondition: `text_contrast` measures row 6. The CRUD check asserts
   exactly one `clipped`/`off-screen` finding on the last row.
10. Step 12 (the gated criterion) was not run: the founder's own word on F1 is not recorded. This is plan-sanctioned,
    not a deviation, and is listed for the gated arm.

scope record: none — gate.py scope clean, 0 recorded (`scope: clean — changed 6 · listed 6 · recorded 0 (companion 0 ·
mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 61`, base e97a7f41).

## Decisions & corrections
- **Plan premise vs crate dependencies.** The plan named a type (`BoundingRect`) and a style read (`Contain`) that
  escher-driver cannot reach without a manifest edit its own guard forbids. Found at implement P1 by reading
  `packages/escher-driver/Cargo.toml` and the `pub use` lists of dioxus-native-dom and blitz-test-harness. It was resolved
  in-file (Deviations 1–2) and recorded as an evolve friction (`input.plan-step-ambiguous`, id
  2026-10-20T00:24:41Z-b). Learning candidate: a plan that types a field with an engine type should check that the
  crate holding the field names that type's crate.
- **Guard refusal.** The Bash guard refused `cd` into the chunk's `inputs/` directory ("moves the session cwd for
  every later call"). It was re-issued once with absolute paths and ran.
- **Fixture order slip.** The first `stand_lint` run read all seven findings but out of document order: `fx-offscreen`
  was placed after `fx-low-contrast` in the page. Fixed by reordering the page to the plan's order, then green. The
  first diagnostic restore was a plain copy; the second was checked by sha256.
- **Operator directive (inputs#I4).** The operator ran the operator pass in plan order with the pre-CI commit's exact
  subject, and recorded it in `evidence/ci.md`. Every entry read green or recorded (no red), and the push moved
  `origin/build/escher-0.2.0` `e97a7f41 → 5a9b806d` as intended. This wrap stops after P1 on the operator's word
  «run P1 only and stop».
- **Measured stand state.** The stand at boot reads 0 lint findings on all four lean tasks in both modes. That includes
  the two readings research left unmeasured: no truncated text, and no contrast failure outside `stand_contrast`'s rows
  (`evidence/readings.md`).
- **PROVISIONAL forks carried.** F1 and F2 stay PROVISIONAL for the founder's own word (inputs#I3, the card's row 1).
  The owner is the wrap that closes Epoch 4, which lists them with the carried `:focus-within` take for his boundary
  batch.
- **Untested reading.** A node that is clipped or off-screen and holds text also reads `truncated` under the plan's
  letter. No check asserts it either way, and it is not measured.

## Outcome
- Acceptance criteria, re-asserted against the diff:
  - (tests, v020-05 in process) MET — `stand_lint` 5/5 in both modes. The planted page reads exactly the seven
    findings in document order, with their reasons: overlap `other` = `fx-overlap-b`; both clips seen inside their
    boxes and narrower; text 200.6953125 wide against 80; contrast 2.8490272 = `text_contrast`'s, required 4.5. The
    negatives read nothing. Each lean task reads 0 findings, both modes. The lint changes nothing. After twelve Creates
    the last row `crud-person-14` reads `clipped`, visible 367 × 11.203125 inside `crud-list`, and its `click` is
    refused `off-screen`. With no font it refuses `fonts-unavailable`.
  - (tests, v020-05 gated) NOT MET, gated by design. The founder's own word on F1 is not recorded, so v020-05 is held
    below verified for P7's gated arm (matrix: `planned`, ref null).
  - (arch) MET — `text_extent` reads its three cases in both modes (entries 1 and 11). Both builds are covered by the fast
    leg's workspace run (entry 13), and no engine behaviour changes, since the reader writes nothing.
  - (obs) MET — two `info_span!` sites (entry 5) and the silence census 0 (entry 6).
  - (security) MET — no tracked file outside the six moved (entry 7: 0) and no code file beyond the four (entry 8: 0).
    Findings hold ids, closed words and numbers only (Symbols / APIs).
  - (a11y) MET — contrast through `text_contrast` / `required_ratio`, exempt at `enabled == Some(false)`; the a11y leg
    reads 43 over 6, unmoved (entry 12).
  - (tests) MET — `fast` and `doc` green; `run stand` 183 · 0 · 7; `run text_extent` 3 · 0 · 0; workspace 201 · 1055 ·
    0 · 12; CI green on the pushed sha, stand-repeat included.
- Gates (implement's block firing, `.andromeda/runs/2026-10-20T00-14-31-implement/gate-2026-10-19-visual-lint.json`,
  summary `entries 22 · green 15 · red 0 · recorded 0 · timeout 0 · not-run 7`):
  - `cargo test -p blitz-tests --locked --test text_extent 2>&1` — green, exit 0, `test result: ok. 3 passed; 0 failed`.
  - `cargo test -p escher-driver --locked lint 2>&1` — green, both unit test lines `ok`.
  - `cargo test -p blitz-tests --locked --test stand_lint 2>&1` — green, the five test lines `ok`.
  - `cargo check --tests --locked --target x86_64-pc-windows-gnu -p blitz-tests` — green, exit 0.
  - the `info_span!` census — green, last line 2.
  - the silence census — green, exit 1, last line 0.
  - the `git diff --name-only e97a7f41b…` guard — green, exit 1, last line 0.
  - the `git ls-files --others` guard — green, exit 1, last line 0.
  - `bash scripts/agent-run.sh boot` — green, outcome ready.
  - `bash scripts/agent-run.sh run stand` — green, `"selection": "stand", "passed": 183, "failed": 0, "ignored": 7`.
  - `bash scripts/agent-run.sh run text_extent` — green, 3 · 0 · 0.
  - `bash .github/scripts/ci-leg.sh a11y` — green, exit 0, 43 tests over 6 targets.
  - `bash .github/scripts/ci-leg.sh fast` — green, artifact `target/ci-logs/test.log` fresh.
  - the workspace tally — green, `lines 201 passed 1055 failed 0 ignored 12`.
  - `bash .github/scripts/ci-leg.sh doc` — green.
  - Operator entries, recorded in `evidence/ci.md`:
    - `gate.py hygiene` — `hygiene: clean`.
    - `ci-leg.sh fast && git diff --quiet && … git push origin build/escher-0.2.0` — green, history moved
      `e97a7f41→5a9b806d` (the intended push).
    - `ci.py conclusion --sha HEAD --wait 1800` — `verdict: green · checks 17/17`, CI#39348671810.
    - the per-OS tally read — recorded, exit 0; its conclusion is the table above, every job moved as predicted, 0
      failed, and it is this chunk's.
  - Three `defer` entries opening `gated:` — not run, `defer (key)`: the founder-word row, the gated `stand_lint` test
    and `cli_commands`' lint test. They carry no PREREQ; the `gated` record carries them.
  - Smoke: skipped — no boot-path or UI-surface change, and no smoke or self-verify entry.
- Watches: none (no `watch:` folded).
- Outcome basis:
  - The operator pass ran: commit list `5a9b806de` (Setup 4, `git log e97a7f41b..HEAD`), and the final HEAD's CI run
    CI#39348671810 green 17/17 (`evidence/ci.md`).
  - implement's P4 report, held whole in this window and persisted as
    `.andromeda/runs/2026-10-20T00-14-31-implement/outcome-2026-10-19-visual-lint.md` (the only `*-implement` dir of the
    marker).
  - `evidence/readings.md` and `evidence/counts.md` (implement); `evidence/ci.md` (the operator pass).
- Process hygiene: implement's census reads `none running`. Re-measured here by `gate.py census` (comm cargo,
  escher-session, seven_guis_native, rustc; args agent-run.sh): `census: none running`.

| process | started by | final state |
|---|---|---|
| cargo · rustc · agent-run.sh (gate entries, direct runs) | implement | terminated |
| cargo (the operator pass's fast leg) | the operator pass | terminated |
## Addendum — resumed 2026-10-20
- inputs#I5 · message: the operator, with /andromeda-wrap-session resuming, 2026-10-20T01:00Z · copy message · n/a — the operator rules Deviation 3: overlap is read over the visible parts of boxes, since a part scrolled out of view overlaps nothing on screen; and, the founder away, an escalation at P2 comes to the operator by dialog, his answer the delegate's, PROVISIONAL, listed for the founder's Epoch 4 batch
  - bears on **Deviations from intent**: supersedes «This is a definition choice no ruling covers, listed for the operator's review» (item 3) — the operator's ruling covers it
  - bears on **PROVISIONAL forks carried.**: adds to it — a P2 escalation answered by the operator joins F1 and F2 as PROVISIONAL for the founder's Epoch 4 batch
- inputs.py verify, re-fired: `inputs: 5 entries — unchanged 0 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 5 · uncited 1 · unparsed 0`; moved since the report: I5 entered, uncited 1 → 0 by this addendum

## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: e97a7f41 (the parent of the oldest pre-CI commit 5a9b806d) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### packages/blitz-dom/src/lib.rs — added 3 line(s) in 2 range(s)
added: 69-70 · 111
### packages/blitz-dom/src/text_extent.rs — new file · 55 line(s)
- 5-15 @8 «pub struct TextExtent {»
- 17-55 «impl BaseDocument {»
  - 18-54 @25 «pub fn text_extent(&self, node_id: NodeId) -> Option<TextExtent> {»
### packages/escher-driver/src/lib.rs — added 8 line(s) in 3 range(s)
added: 82-87 · 130 · 148
### packages/escher-driver/src/lint.rs — new file · 604 line(s)
- 16-27 @18 «pub struct Bounds {»
- 29-40 @30 «macro_rules! bounds {»
  - 31-39 «($rect:expr) => {{»
- 42-93 «impl Bounds {»
  - 43-45 «fn right(self) -> f64 {»
  - 47-49 «fn bottom(self) -> f64 {»
  - 51-53 «fn has_area(self) -> bool {»
  - 55-66 @56 «fn intersection(self, other: Bounds) -> Option<Bounds> {»
  - 68-74 @69 «fn within(self, other: Bounds) -> bool {»
  - 76-92 @77 «fn cut(self, other: Bounds, (on_x, on_y): (bool, bool)) -> Bounds {»
- 95-110 @97 «pub enum Defect {»
- 112-139 «impl Defect {»
  - 113-121 @114 «pub const ALL: [Defect; 6] = [»
  - 123-133 @124 «pub fn name(self) -> &'static str {»
  - 135-138 @136 «fn rank(self) -> usize {»
- 141-174 @143 «pub enum Reason {»
  - 144-150 @145 «Overlap {»
  - 151-155 @152 «Clipped {»
  - 156-162 @157 «Truncated {»
  - 167-173 @168 «Contrast {»
- 176-187 @178 «pub struct Finding {»
- 189-231 «impl Session {»
  - 190-230 @221 «pub fn lint(&mut self) -> Result<Vec<Finding>, Refusal> {»
- 233-245 @234 «struct Entry<'a> {»
- 247-295 «fn findings(harness: &Harness<DioxusDocument>, screen: &Snapshot) -> Vec<Finding> {»
  - 249-251 «for (node, id) in harness.doc.element_ids() {»
  - 253-255 «for root in &screen.roots {»
  - 258-263 «let finding = |entry: &Entry, defect: Defect, reason: Reason| Finding {»
  - 264-268 «for (earlier, later, area) in overlaps(&entries) {»
  - 269-293 «for (index, entry) in entries.iter().enumerate() {»
- 297-302 @299 «fn ordered(mut found: Vec<(usize, Finding)>) -> Vec<Finding> {»
- 304-330 @305 «fn flatten<'a>(»
  - 315-317 «let visible = region»
  - 319-326 «out.push(Entry {»
  - 327-329 «for child in &node.children {»
- 332-372 @334 «fn overlaps(entries: &[Entry]) -> Vec<(usize, usize, Bounds)> {»
  - 335-339 «let ancestors: Vec<Vec<usize>> = (0..entries.len())»
  - 341-370 «for (earlier, first) in entries.iter().enumerate() {»
- 374-388 @376 «fn clip_region(harness: &Harness<DioxusDocument>, element: NodeId) -> Option<Bounds> {»
  - 381-386 «while let Some(id) = current.filter(|&id| Some(id) != root) {»
- 390-403 @392 «fn contains_paint(harness: &Harness<DioxusDocument>, element: NodeId) -> bool {»
  - 395-397 «let paint = contain»
  - 399-401 «let applies = !matches!(display.as_str(), "inline" | "contents" | "none" | "")»
- 405-417 @407 «fn clips(harness: &Harness<DioxusDocument>, element: NodeId) -> (bool, bool) {»
  - 408-411 «let clips_axis = |property| {»
  - 413-416 «(»
- 419-432 @421 «fn padding_box(harness: &Harness<DioxusDocument>, element: NodeId) -> Option<Bounds> {»
  - 426-431 «Some(Bounds {»
- 434-457 @436 «fn holds_text(harness: &Harness<DioxusDocument>, element: NodeId, deep: bool) -> bool {»
  - 438-441 «let mut stack: Vec<NodeId> = doc»
  - 442-455 «while let Some(id) = stack.pop() {»
- 459-466 @460 «fn is_text_control(harness: &Harness<DioxusDocument>, element: NodeId) -> bool {»
  - 461-465 «harness»
- 468-479 @469 «fn holds_content(harness: &Harness<DioxusDocument>, node: &SnapshotNode, element: NodeId) -> bool {»
  - 470-473 «let focusable = harness»
  - 474-478 «!node.name.is_empty()»
- 481-504 @483 «fn truncated(harness: &Harness<DioxusDocument>, entry: &Entry, element: NodeId) -> Option<Reason> {»
  - 484-486 «if is_text_control(harness, element) {»
  - 488-490 «if !text.has_area() {»
  - 492-498 «let region = match axes {»
  - 499-501 «if region.is_some_and(|region| text.within(region)) {»
- 506-524 @507 «fn contrast(»
  - 512-514 «if node.state.enabled == Some(false) {»
  - 515-517 «if !is_text_control(harness, element) && !holds_text(harness, element, false) {»
  - 520-523 «(reading.ratio < required).then_some(Reason::Contrast {»
- 526-604 @527 «mod tests {»
  - 530-546 @531 «fn the_six_defects_are_named_in_order() {»
  - 548-573 «fn finding(id: &str, defect: Defect) -> Finding {»
  - 575-603 @576 «fn findings_follow_document_order_then_the_defect_order() {»
### tests/blitz-tests/tests/stand_lint.rs — new file · 304 line(s)
- 21-103 @23 «fn lint_page() -> Element {»
  - 24-102 «rsx! {»
- 105-111 @106 «fn named(findings: &[Finding]) -> Vec<(&str, &str)> {»
  - 107-110 «findings»
- 113-126 @115 «fn bounds(session: &Session, id: &str, at: &str) -> Bounds {»
  - 120-125 «Bounds {»
- 128-133 «fn within(inner: Bounds, outer: Bounds) -> bool {»
  - 129-132 «inner.x >= outer.x»
- 135-186 @136 «fn the_planted_defects_are_named_each_by_its_id_and_reason() {»
  - 137-145 «let rows = [»
  - 147-185 «for incremental in [false, true] {»
- 188-205 @189 «fn the_stand_at_boot_reads_no_finding() {»
  - 190-204 «for incremental in [false, true] {»
- 207-243 @208 «fn the_lint_reads_and_changes_nothing() {»
  - 209-242 «for incremental in [false, true] {»
- 245-280 @246 «fn the_rows_cut_by_the_list_agree_with_off_screen() {»
  - 247-279 «for incremental in [false, true] {»
- 282-304 @283 «fn an_app_with_no_font_is_refused_fonts_unavailable() {»
  - 284-303 «for incremental in [false, true] {»
### tests/blitz-tests/tests/text_extent.rs — new file · 110 line(s)
- 13-16 «<style>»
- 23-25 «fn boot(incremental: bool) -> Harness<blitz_html::HtmlDocument> {»
- 27-32 @28 «fn node(harness: &Harness<blitz_html::HtmlDocument>, selector: &str, at: &str) -> NodeId {»
- 34-56 @35 «fn a_line_that_cannot_wrap_reads_wider_than_its_box() {»
  - 36-55 «for incremental in [false, true] {»
- 58-83 @59 «fn a_wrapped_paragraph_reads_inside_its_box() {»
  - 60-82 «for incremental in [false, true] {»
- 85-110 @86 «fn an_element_with_no_inline_content_reads_none() {»
  - 87-109 «for incremental in [false, true] {»
