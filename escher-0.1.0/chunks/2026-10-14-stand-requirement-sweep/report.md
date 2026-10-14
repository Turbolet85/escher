# Report — 2026-10-14-stand-requirement-sweep

**Chunk:** Stand requirement sweep — every 0.1.0 capability proven headless by an agent across the stand tasks, with
fixtures beside the stand where the tasks lack a case
**Date:** 2026-10-14
**Commits:** `55a256384 chore(2026-10-14-stand-requirement-sweep): operator pre-CI commit, for the run this chunk verdict
reads` (since `last_wrap` 2026-10-14T20:23:47Z; `git log --format='%h %s' 19756f2d..HEAD`)

## Changes (structured — detectors read this)
- **Files:** (basis `gate.py scope` at this wrap — changed 15 · listed 12 · recorded 3; `cites.py added`, base
  `19756f2d`)
  - new: `tests/blitz-tests/tests/stand_requirement_sweep.rs` (613 lines) · `tests/blitz-tests/tests/stand_non_lean_tasks.rs`
    (147) · `tests/blitz-tests/tests/password_caret_click.rs` (95);
  - stand markup, added `id:` lines only: `examples/seven_guis/src/tasks/temp_converter.rs` (13, 26) ·
    `circle_drawer.rs` (29, 40, 53, 106, 126) · `cells.rs` (375, 384);
  - tests extended: `tests/blitz-tests/tests/stand_actionable_keys.rs` · `stand_act_obstructed.rs` ·
    `hit_clipped_at_scrolling_box.rs` · `scrolled_box_client_rect.rs` · `range_input_interaction.rs` ·
    `examples/seven_guis/tests/cli_commands.rs`;
  - driver fixed text: `packages/escher-driver/src/refusal.rs` (22-24, 80-82, 268-270) · `json.rs` (675, 679-681) ·
    `lib.rs` (68-69, 71-73);
  - chunk evidence: `evidence/measurements.md` · `evidence/log-counts.md` with its instrument `evidence/log-counts.py`
    · `evidence/ci.md`; `scope-record.md` (3 lines).
- **Symbols / APIs:**
  - **The stand's author ids**, nine new:
    - Temperature Converter: `temp-celsius`, `temp-fahrenheit`;
    - Circle Drawer: `circle-undo`, `circle-redo`, `circle-canvas`, `circle-diameter` (the dialog's range input)
      and `circle-close` (the last two exist only while the dialog is open);
    - Cells: `cell-{A..Z}{0..25}` on the 676 data cells through the file's own `col_label(c)` (cells.rs:375), and
      `cell-input` on the one edit input (cells.rs:384).
    - No CSS selects by them. No element, class, style, wrapper or order moved (the probe `git diff -U0 19756f2d --
      examples/seven_guis/src/tasks` counts 0 non-id changed lines).
  - **`Cause::Covered`'s fixed meaning** (`refusal.rs:80-82`) now carries a second measured limit after the
    `contain: paint` one: "a box clipped on one axis stops a hit at its whole padding box, so an element lying outside
    it along the axis it does not clip can read `covered` though it shows".
    - Where it is carried: the `Cause::Covered` doc comment (`refusal.rs:22-24`), the json test's pinned sentence
      (`json.rs:679-681`, its comment now "The limits measured"), and the crate doc (`lib.rs:68-73`: "Two readings
      are known to be off").
    - What reads it: `describe()`'s causes on both surfaces (the CLI `help` and MCP `help`), and every `covered`
      refusal's JSON line.
    - The remedy, the cause set (nine) and the cause names are unchanged.
  - **New test helpers**, private to their files:
    - `stand_requirement_sweep`: `read_line`, `closing_quote`, `screen`, `acting`, `enum Subject { AllFour,
      Task(LeanTask), Fixture }`, `type Witness`, `WITNESSES` (16 rows), `source(file)`;
    - `stand_non_lean_tasks`: `open(card, title, incremental)`, `reacting_without_role`,
      `circle_drawer_with_dialog`;
    - `cli_commands`: `a_state_directory_at_the_bound_holds_a_session_and_one_byte_more_is_refused(bound)`, the old
      94/95 body moved into it.
  - No public API, no wire field, no verb, no node key, no log field and no span changed.
- **Crates / modules:** none added or removed. The three test targets are new and auto-discovered: no `[[test]]`
  table (`tests/blitz-tests/Cargo.toml` untouched).
- **Dependencies:** none.
- **Schema / config:** none — the driver's schema tables (`VERBS`, `NODE_FIELDS`, `CAUSES`) unchanged; only
  `Cause::Covered.meaning()`'s text moved.
- **Spec-master edits:** none (no `.andromeda/` master or sidecar changed; `git status --short -- .andromeda/` at Setup
  read only this run dir untracked).
- **Counts / qualifiers moved:** (master sites from `cascade.py sweep --pre-pass`, this run dir's
  `report-patterns.toml`, baseline `55a25638`)
  - **The non-lean pin.** `unkeyed_actionable()` on Temperature Converter · Circle Drawer · Cells goes from 2 · 3 ·
    676 to **0 · 0 · 0**, both modes; it reads 0 with Circle Drawer's dialog open too. `nonlean-pin`: 3 standing, all
    change — architecture.md:136, test-plan.md:26, a11y-plan.md:282 ("reads Temperature Converter 2 · Circle Drawer 3
    · Cells 676 on the three tasks not yet keyed").
  - **`covered`'s limits** go from one to **two**. `covered-limit`: 7 standing.
    - Change, 4: architecture.md:133 ×3 (@18340 the quoted meaning; @19772 the crate docs' "one reading as off";
      @20965 "the one reading still off") · security-plan.md:351 ("`covered`'s meaning now ends with the sentence…") ·
      test-plan.md:181 ("one reading stays as measured").
    - No-change, 2: architecture.md:87 (the hit contract — still true) · architecture.md:263 (the
      `hit_clipped_at_scrolling_box` case list).
    - test-plan.md:26 (the same file's case list) gains the new cases, under per-file counts below.
  - **`run stand`** goes from 162 · 0 · 7 over 38 files to **168 · 0 · 7 over 40 files**, +6 (`stand_requirement_sweep`
    2 · `stand_non_lean_tasks` 3 · `stand_act_obstructed` +1). `stand-run-count`: 1 standing, change —
    test-plan.md:116 (a re-count to append).
  - **Workspace totals** go from 182 result lines · 937 passed · 0 · 12 to **185 · 951 · 0 · 12**: +3 result lines,
    +14 tests (the sweep 2 · non-lean 3 · obstructed 1 · hit 4 · client rect 1 · range 2 · password caret 1).
    `ws-totals`: 1 standing, change — test-plan.md:330 (a re-count to append).
  - **Per-file test counts:**
    - `hit_clipped_at_scrolling_box` 9 → **13**;
    - `scrolled_box_client_rect` 6 → **7**;
    - `range_input_interaction` 6 → **8**;
    - `stand_act_obstructed` 5 → **6**;
    - `stand_actionable_keys` 3 and `cli_commands` 11 are unchanged (the 90/91 test is
      `cfg(not(target_os = "linux"))`, so Linux still counts 11, and macOS counts 11 too).
    - `file-counts`: 8 standing. Change, 2: architecture.md:263 (range 6, hit 9, client rect 6) · test-plan.md:26
      (obstructed 5, range 6, client rect 6, hit 9). No-change, 6 (dated history): test-plan.md:116, :143, :200,
      :324, :330 and `registries/contracts/test-plan/session-lifecycle.md:11`, as re-counts or dated readings.
  - **Shared-module readers.** `tests/common/mod.rs` readers go from 19 to **21** (both new stand files declare
    `mod common;`; `password_caret_click` declares none). `tests/session_common/mod.rs` readers go from 20 to **21**
    (`stand_requirement_sweep`). `common-readers`: 12 standing.
    - Change, 4: obs-plan.md:70 ("19 of which read the shared module") · architecture.md:263 ("by 19 stand
      checks … the nineteenth") · test-plan.md:66 ("by 19 `stand_*.rs` checks … by 20 checks") ·
      architecture.md:115 (the `session_common` reader list).
    - No-change, 8: the citations into `session_common/mod.rs` — architecture.md:149, :150 · security-plan.md:313 ·
      test-plan.md:49, :101 · obs-plan.md:331 · `session-lifecycle.md:8` — and test-plan.md:26, a bare path mention.
  - **The host's and the `mcp` role's log counts**, workspace build: the `blitz_dom::document` `focussed node` INFO
    line now prints **twice** where it printed once. The host goes from 5 lines / 860 B to **6 / 978 B**, the `mcp`
    role from 10 lines to **11**. Built alone it is unchanged: 4 / 742 B and 9. Needles are 0 everywhere
    (`evidence/log-counts.md`).
    - `log-counts`: 1 standing, change — obs-plan.md:296 (742 B / 860 B / one `blitz_dom::document` line).
    - `mcp-lines`: 3 standing, change — obs-plan.md:181, test-plan.md:103, security-plan.md:397 ("9 lines built alone
      and 10 built with the workspace — the tenth `INFO blitz_dom::document`…").
    - `focussed-node`: 1 standing, no-change — obs-plan.md:188, the event's site citation.
  - **The C4 / C5 / C6 qualifiers.** architecture.md:124 says "whether it stops at an image's, a sub-document's or a
    text input's own box is not measured", "a target lying outside the box along its visible axis reads in view and
    is not reached by a hit" with no driver reading, and "no check reads a box that is both transformed and
    scrolled". All three are now measured (below, Spec claims disproved). `depth-hits` row architecture.md:124 ×2 and
    `one-axis` row architecture.md:124: change. The other `one-axis` rows — architecture.md:87, :263, test-plan.md:26 —
    are true claims, no-change.
  - **The password caret qualifier**, "not measured by any check, owned by a `CARRY:` on the route entry 'Stand
    requirement sweep'", is now measured (caret index 5 vs 2). Its owner moves to a residuals line (P5, card row 6).
    `pw-caret`: 9 standing.
    - Change, 3: architecture.md:129 · security-plan.md:128 · a11y-plan.md:281.
    - No-change, 6: architecture.md:196, :263 · test-plan.md:18, :26, :153 · a11y-plan.md:242 — caret painting,
      true.
  - **The state-directory bound off Linux**, "90 on every other target (read, not measured)", is now **measured** on
    the macOS matrix leg: CI#38642493451, job 116123587286 — 90 holds a session, 91 ends `state-dir-too-long`,
    status 3, with no path in the message. `state-dir-bound`: 6 standing.
    - Change, 4: security-plan.md:79 (the "read, not measured" qualifier) · test-plan.md:10 and :200 (the
      `cli_commands` description gains the off-Linux 90/91 test) · security-plan.md:351 ×2 (the same bound restated).
    - No-change, 2: architecture.md:133 ×2 (the bound stated, true) · test-plan.md:143 (`error.rs`' unit test, true).
- **Dev-tool versions:** none.
- **Harness / gate surface:**
  - `run stand` picks up the two new `stand_` files by prefix, with no script change (40 files).
  - The `a11y` CI leg is unchanged: six files, 6 · 6 · 4 · 8 · 5 · 14.
  - `BlitzTestsTargetsTest` passes with three new auto-discovered targets (`ci-leg.sh fast` green).
  - No CI workflow edit: the macOS matrix leg already runs every `seven_guis` target (`ci.yml`, read at P3).
- **Cross-project / external claims:**
  - **The CI run the gates read:** CI#38642493451 on sha `55a256384c801a50ea86cbd34c16c908bd05cd79` (the pre-CI
    commit), `verdict: green`, 16/16 checks, wall 886 s. `Test (macos)`, job 116123587286, ran
    `a_state_directory_of_90_bytes_holds_a_session_and_one_of_91_is_refused_with_its_bound ... ok` in `cli_commands`
    (11 passed) on `aarch64-apple-darwin` (`evidence/ci.md`). The verdict was taken on that sha; this wrap's commit
    adds bookkeeping to it.
  - `inputs.py verify` (inputs v1.2), this wrap — summary `4 entries — unchanged 1 · drifted 1 · vanished 0 · broken 0
    · altered 0 · unreachable 0 · n/a 2 · uncited 1 · unparsed 0`:
    - `I1 · message: the operator, /andromeda-phase invocation arguments, 2026-10-14 20:46Z · copy · n/a`: the
      founder's word stands, no mutation runs. Cited in scope.md, research.md and plan.md.
    - `I2 · ../additional/escher-overseer/relays/requirement-sweep-founder-answers.md · copy · drifted` — **DRIFT: the
      relay file grew after I2's snap.** I3 snaps the same file later with all eight answers, and plan.md:468 names I2
      superseded by I3. No fact here rests on I2.
    - `I3 · ../additional/escher-overseer/relays/requirement-sweep-founder-answers.md · copy · unchanged`: the
      founder's answers to the eight forks, relayed verbatim (C4 «Отложить (Рекомендую)», C5 «Ничего, замерить и
      записать (Рекомендую)», C14 «Отложить на следующую версию (Рекомендую)», C10 «Отложить на следующую версию
      (Рекомендую)», C7 «Отложить (Рекомендую)», C15 «Оставить по наличию, отложить (Рекомендую)», C11 «Замерить
      сейчас, исправить потом (Рекомендую)», C3 «Замерить сейчас, поле позже (Рекомендую)»), and his words «но надо все
      отложеное где то записывать».
    - `I4 · message: the operator, at the P5 review, 2026-10-14 21:20Z · copy · n/a` — cited here (`verify` printed it
      UNCITED before this report existed): the operator's "yes — apply" on the P5 card.
  - The card, `.andromeda/runs/2026-10-14T20-46-15-phase/p5-card-1.md`. Its `## Beyond the plan` rows, verbatim, on
    the operator's own word (I4: "yes — apply. … Write each with the founder's label verbatim, in its own script."):

    > Each row is a `.andromeda/residuals.md` line, `(target: next)`, written at this chunk's wrap and naming the label it
    > rests on (the founder's words, inputs#I3):
    > 1. C4: widening the hit's stop to a `contain: paint` box. The founder's «Отложить (Рекомендую)». The pin in
    >    `hit_clipped_at_scrolling_box` and `covered`'s text stand.
    > 2. C14: fixing `Node::absolute_position` for a self-scrolled box, with its seven readers. The founder's «Отложить на
    >    следующую версию (Рекомендую)». `scrolled_box_absolute_position` stands as its pin.
    > 3. C10: moving the overlay scrollbar fade onto the harness clock. The founder's «Отложить на следующую версию
    >    (Рекомендую)». The `screenshot` help's limit stands.
    > 4. C7: reading `aria-checked` (as `checked` or `pressed`). The founder's «Отложить (Рекомендую)».
    >    `accessibility_states`' fourth test stands.
    > 5. C15: narrowing `disabled` focusability to `button` · `input` · `select` · `textarea`. The founder's «Оставить по
    >    наличию, отложить (Рекомендую)». `stand_keyboard`'s pin stands.
    > 6. C11 (fix): placing a password's click caret by the bullet layout. The founder's «Замерить сейчас, исправить потом
    >    (Рекомендую)». This chunk's measurement is the owner's starting reading.
    > 7. C3 (field): a snapshot reading for "reacts to a click". The founder's «Замерить сейчас, поле позже (Рекомендую)».
    >    This chunk's per-task count is the owner's starting reading.
    > 8. C8: Cells stating its selected cell (`aria-selected` under a grid role). The operator's "Key all three; C8
    >    deferred".
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - **C4.** architecture.md:124: "whether it stops at an image's, a sub-document's or a text input's own box is not
    measured". Now measured (`hit_clipped_at_scrolling_box`, both modes, both builds):
    - a hit at an `img` holding injected raster data answers the `img`;
    - a hit inside a block of an `iframe`'s `srcdoc` sub-document answers the **`iframe`**, since the parent's walk
      does not enter the sub-document (pinned as measured);
    - a hit inside a filled text `input` answers the `input`.
  - **C5.** architecture.md:124 reads the one-axis clip as "reads in view and is not reached by a hit". The CARRY's
    hypothesis was that a driver `click` there is refused `covered`, not `off-screen`. Measured on the driver
    (`stand_act_obstructed::a_target_beside_a_box_clipped_on_one_axis_is_refused_covered`, both modes): refused
    **`covered`**, the instance unchanged. The hypothesis holds, and the limit is stated in `covered`'s text.
  - **C6(1).** architecture.md:124: "no check reads a box that is both transformed and scrolled". Now one does
    (`a_box_both_transformed_and_scrolled_is_hit_where_its_rows_show`):
    - the box is `translate(50px, 30px)`, scrolled 280;
    - the hit where the ninth row shows answers the row;
    - a point inside the box where the first row lies unscrolled, beside the narrower row showing there, answers the
      box.
  - **C6(2).** The CARRY says "no fixture of `scrolled_box_client_rect` sets a vertical `writing-mode`". Now one does:
    a `writing-mode: vertical-lr` box scrolled (70, 120) reads its unscrolled rect (40, 50, 300, 200). It is green under
    `-p blitz-tests` and under `--workspace`, where `writing-mode` compiles `layout/writing_mode.rs`' bounds body.
  - **C11.** architecture.md:129 · security-plan.md:128 · a11y-plan.md:281: "not measured by any check". Now measured
    (`password_caret_click`, both modes, the bundled font, 20 px): a click 30 px into the content box places the caret
    at index **5** after eight narrow characters and **2** after eight wide ones. The CARRY's hypothesis holds: a
    click-placed caret depends on the real characters' widths.
  - **C9.** security-plan.md:79: "90 on every other target (read, not measured)". Now measured on the macOS leg
    (above).
  - **A reading measured beside C6(1), stated by no master:** `BaseDocument::get_client_bounding_rect` reads a box
    under `transform: translate(50px, 30px)` at its layout position `(0, 40, 300, 140)`, not where the transform moves
    it. The C6(1) test pins this as its fixture's condition and computes its points from it. `transform-rect`: 36
    standing.
    - No-change, 35: CSS transform support, `text-transform`, example names and paint culling — none states a
      client-rect reading.
    - architecture.md:124 is the C6(1) claim above.
    - The CSSOM View `getBoundingClientRect` includes transforms, so this is an engine divergence with no owner. P5
      gives it one.
- **Expected amendments (from plan):** (each search is a `report-patterns.toml` pattern, its standing count per master)
  - architecture §Standard Contracts → Dioxus DOM bridge (`unkeyed_actionable()` reads 0 on all seven tasks; the pin
    0 · 0 · 0) — carried: Counts → the non-lean pin. `nonlean-pin`: architecture 1 (:136), test-plan 1, a11y-plan 1.
  - architecture §Standard Contracts → Driver session (`covered` states the one-axis limit; the sweep named among the
    session checks) — carried: Symbols → `Cause::Covered`; Counts → `covered`'s limits.
    - `covered-limit`: architecture 4 rows (:87, :124, :133, :263), the change rows :133 ×3.
    - `session-checks`: architecture 1 row (:263 ×3), where `stand_requirement_sweep` joins the session checks.
  - security-plan §Input Validation, Driver command schema row (`covered`'s second limit, fixed table text in a
    ratified carrier, playbook :44) — carried. `covered-limit`: security-plan 1 (:351).
  - security-plan §Input Validation, Driver session state directory row (the off-Linux 90 measured on the macOS leg)
    — carried: Counts → the state-directory bound. `state-dir-bound`: security-plan 2 (:79, :351).
  - security-plan §Input Validation, password row (the click-placed caret measured; its fix owned by a residuals line)
    — carried: Spec claims disproved → C11. `pw-caret`: security-plan 1 (:128).
  - test-plan §1, §3, §9 (three new targets, the extended files' counts, `run stand`, the workspace totals, the
    sweep's `WITNESSES` as the requirement-to-check record) — carried: Counts → `run stand`, workspace totals,
    per-file counts, shared-module readers.
    - `stand-run-count` test-plan 1 (:116) · `ws-totals` test-plan 1 (:330) · `file-counts` test-plan 7 rows (change
      :26) · `common-readers` test-plan 4 rows (change :66).
    - `WITNESSES` is new — no master site names it.
  - obs-plan §2, §6, §8 (the dated host and `mcp` counts confirmed or amended) — carried: Counts → log counts,
    **amended** (workspace build 6 / 978 B and 11; alone unchanged).
    - `log-counts` obs-plan 1 (:296) · `mcp-lines` obs-plan 1 (:181).
    - The same dated figures also stand in test-plan 1 (:103) and security-plan 1 (:397).
  - obs-plan §3 (the census over the three new files) — carried. The two new stand files and `password_caret_click`
    install no subscriber, read no env and print nothing (gate entry 7 reads 0). `common-readers`: obs-plan 2 rows
    (change :70; :331 no-change).
    - The pattern "silence census" never fired over the masters (the sweep's refusal, `control: … silence-census …
      never fired over the seven masters' pre-pass text (55a25638)`). obs-plan states the census as "installs no
      subscriber … and no `println!`" at :70.
  - a11y-plan §7 (the actionable-key counts 0 on the non-lean tasks; C3's measured count, its field deferred) —
    carried: Counts → the non-lean pin; C3 under Outcome.
    - `nonlean-pin` a11y-plan 1 (:282).
    - `generic-role`: a11y-plan 2 (:104, :108), both the role mapping, no-change.
    - The pattern "reacts to a click" never fired over the masters (the same refusal), so C3's count is a new
      statement for a11y-plan §7.
  - layout-templates §Surface: desktop-native → Primary screens (the non-lean tasks' new ids) — carried: Symbols →
    the stand's author ids. `nonlean-names`: layout-templates **0** (the 3 hits are architecture :136, test-plan :26
    and a11y-plan :282). layout-templates.md:10 names Home's card ids (`task-card-circle-drawer`, `task-card-cells`),
    not the tasks' own controls. No master is named owner here; the layout-templates detector decides whether its
    Primary-screens section takes them.
  - design-system §Depth Strategy (the three measured hits; the citations into the three task files re-pointed) —
    carried: Spec claims disproved → C4.
    - `depth-hits`: design-system 3 rows (:13, :213, :290). :213 is the "Engine depth behavior" heading, whose hit
      bullet (:216) states the overflow stop; :13 and :290 are no-change.
    - The task-file citations move with the inserted `id:` lines — P2's citation sweep.
  - matrix#v010-01 notes (every driver screen of the four lean tasks reads one distinct id per node, through
    `Session::run`, `stand_requirement_sweep`) — ledger-note — owner P7.3.
  - matrix#v010-05 notes (focus read back through the driver's screen text on all four lean tasks,
    `stand_requirement_sweep`) — ledger-note — owner P7.3.
  - matrix#v010-16 notes (the actionable-key rule reads 0 on the three non-lean tasks too) — ledger-note — owner P7.3.
- **Coverage of new surfaces:**
  - `Cause::Covered` meaning text (a fixed `'static` string in a ratified carrier — CLI stdout, MCP stdio, the socket's
    answer) → validation n/a (no input) · instrumentation n/a (no log field; the `command` span keeps eight fields,
    `tool_call` two) · PII n/a (fixed text, no id, name or value; `json.rs` asserts no `/` and no line break) · tests
    unit (`refusal.rs` table row, `json.rs` pinned sentence) + integ (`help` bytes = `describe()` on both surfaces,
    `cli_commands` · `mcp_server`) · a11y n/a · tokens n/a
  - nine stand author ids → validation n/a (author keys: non-empty, `/`-free, first in pre-order) · instrumentation n/a
    · PII n/a (ids are stand fixtures) · tests integ (`stand_actionable_keys`, `stand_non_lean_tasks`) · a11y ✓ (every
    actionable element of the three tasks keyed, a11y-plan §7; no focus or colour change) · tokens n/a (no style)

## Deviations from intent
- **The sweep's step order.**
  - Implemented: snapshot → `click task-title` (empty diff) → `press tab` (the first Tab stop focused) → the acting
    call → two screenshots → the `not-found` click.
  - The plan lists the acting call before the Tab press. Since 2026-10-13 a click focuses a button, so a Tab after
    `click("counter-increment")` focuses the stop after it, not `tab_order(task)[0]`, and a heading click after a
    focus risks a blur in its diff.
  - Every listed assertion is made.
- **`WITNESSES`' subjects.**
  - Two v010-14 witnesses (`help`) reach no session, and the plan's subject set (a lean task · `fixture` · `all four`)
    has no task-less value. They carry `AllFour`, and `Subject::AllFour`'s doc reads "Every lean task; for a verb that
    reaches no session, whichever task a host boots".
  - Matrix refs that are unit tests in `packages/escher-driver/src` (v010-13, v010-14) are not carried: the plan makes
    a witness's file a test file.
  - A row names one or more witnesses per task-specific half, trimmed from the matrix refs (`matrix.py`/the ledger
    read at implement).
- **C6(1)'s fixture.** "A hit inside the box where scrolled-out content would lie" has no point inside the box as
    written, because scrolled-out content lies outside it. The fixture gives it one:
    - the first row is as wide as the box and the rest are 200 wide;
    - the point lies where the first row lies unscrolled, beside the narrower row showing there;
    - it answers the box. A walk that dropped the scroll would answer row 0, and one that dropped the transform would
      answer the stage.
- **C6(1)'s client-rect pin.** The test asserts the box's client rect reads its layout position `(0, 40, 300, 140)` as
    its fixture condition. That pins an engine reading the plan did not name; it is a measured fact, not a behaviour
    change. Its owner is a P5 route question.
- **`cli_commands`' helper.** It adds `assert!(!message.contains(&given))`, which the plan asks for the 90 case. Since
    the helper is shared, the Linux 94 test now asserts it too.
- **C11's read.** The caret index is read once, in characters, from the editor's selection. That is the bullet it
    paints at, by the paint path's own map (`packages/blitz-paint/src/render.rs:997-1000`).
- **The step 11 capture** is `cargo build --workspace --locked` for the workspace build (the dated host reading used
    `cargo test --workspace --no-run`, the same feature set for this binary). It left `target/debug/escher-session`
    workspace-built.
- **The scope record** — `gate.py scope` at this wrap: `scope: clean — changed 15 · listed 12 · recorded 3 (companion 0 ·
  mechanical 0 · in-intent 3 · widening 0)`:
  - in-intent:
    - `packages/escher-driver/src/refusal.rs` · serves step 5 · self
    - `packages/escher-driver/src/json.rs` · serves step 5 · self
    - `packages/escher-driver/src/lib.rs` · serves step 5 · self
  - Research listed `schema.rs` for C5 ("where a measured limit is stated in a verb's help"). Plan step 5 puts the
    sentence in `covered`'s meaning, its pinned test sentence and the crate doc instead. `schema.rs` is untouched.

## Decisions & corrections
- **The operator's word for the operator pass** (2026-10-14, this session): "Run the operator pass and stop before the
  wrap."
  - Fire entry 15 (hygiene) AS WRITTEN, by its own command line, never through `gate.py run --operator`.
  - Commit as `chore(2026-10-14-stand-requirement-sweep): operator pre-CI commit, for the run this chunk verdict
    reads`.
  - Fire 16 and 17, confirm the macOS job ran the C9 test, write `evidence/ci.md`.
  - Done in that order. `evidence/ci.md` was written after the commit and rides this wrap's commit.
- **The operator's word for this wrap** (2026-10-14): "run P1 only and stop". The two-window form; P2 resumes from this
  run dir (Setup 2a).
- **The operator's P5 word** (inputs#I4): "yes — apply. … Write each with the founder's label verbatim, in its own
  script." The eight residuals lines are P5's to write (the card's rows above), each label in Cyrillic as given.
- **The C2 / C8 and sweep-form answers** (the operator, by question dialog at P4): "Key all three; C8 deferred" and
  "One driver sweep check + table". The founder's eight forks: inputs#I3, labels above.
- **A guard refusal at implement.** `bash-guard.sh` refused a `cat` heredoc writing a scratch script to a file ("a
  cat/tee heredoc with a file target"). It was re-issued once through the Write tool and ran.
- **A recurrence of a standing learning at this wrap** (session-learnings 2026-10-07 and the global CLAUDE.md Shell
  rule: the Bash tool's `grep` is ugrep and refuses a bounded repetition). A bare `grep -n -o '.\{0,60\}937…'` …
  `grep … '.\{0,40\}census.\{0,40\}'` call printed one hit and then nothing. The absence was re-read with `command grep`
  before being trusted.
- **A recurrence of a standing learning at this wrap** (session-learnings 2026-10-13: obs-plan never says "silent
  census"). The plan's own word "silence census" was fired as a sweep pattern and its control never fired over the
  masters. obs-plan states the census as "installs no subscriber … no `println!`" (obs-plan.md:70).
- **A sweep hazard.** The `log-counts` pattern `742 B|860 B|742 / 860|9 / 10|4 / 5 lines` found the host's dated
  counts (obs-plan.md:296) and missed the `mcp` role's, which the masters word "9 lines built alone and 10 built with
  the workspace" (obs-plan.md:181, test-plan.md:103, security-plan.md:397). A second pattern, `mcp-lines`, found
  them.
- **Measured at implement and not predicted:** the client rect ignores `transform` (above). That surfaced at a fixture
  assertion, red on its first run, read through a temporary message and restored before the gate pass.
- **The implement outcome** (`.andromeda/runs/2026-10-14T21-20-05-implement/outcome-2026-10-14-stand-requirement-sweep.md`):
  green + smoke, one gate pass, no soft-exit, no operator correction during implement.

## Outcome
- **Acceptance criteria**, against the diff:
  - **Met — the driver sweep.** Through `Session::run` alone, both modes, each lean task: the screen fits
    `SNAPSHOT_TEXT_BUDGET`; every node line carries one id, pairwise distinct, each an accessibility node's
    `author_id`; one id-addressed acting call (`click counter-increment` · `click flight-return` · `advance 300` ·
    `click crud-person-0`) returns settled with a non-empty diff equal to the check's own; `click task-title` returns
    an empty diff; `press tab` reads `tab_order(task)[0]` as the one focused node; two screenshots are byte-equal 800 ×
    600 PNGs; an unread id is refused `not-found`, the instance unchanged, its remedy in `Refusal::to_json`
    (`stand_requirement_sweep::every_requirement_holds_through_the_driver_on_every_lean_task`, green per-package and
    workspace).
  - **Met — `WITNESSES`.** 16 rows in requirement order, each with at least one witness, every named test found as
    `fn {test}(` through `include_str!`, and the every-screen requirements naming the sweep for `all four`
    (`every_requirement_names_its_witness`). Its existence read went red on the one witness not yet written, and green
    once it was.
  - **Met — the non-lean keys.** `unkeyed_actionable()` reads 0 · 0 · 0 on the three non-lean tasks, both modes; the
    dialog's controls read their keys with it open; the four lean tasks and Home read 0.
  - **Met — the task-file probe.** The task files differ from `19756f2d` by added `id:` lines alone (0 other changed
    lines). No lean-task file, `app.rs` or `stand.rs` moved (gate 8).
  - **Met — C3.** Measured and pinned 0 · 0 · 0 · 0 · 0 · Circle Drawer 1 · Cells 676 (`evidence/measurements.md`). No
    snapshot field was added (`NODE_FIELDS` thirteen).
  - **Met — C12.** The dialog range input's width is 240, so the rule applies.
  - **Met — C11.** Caret index 5 vs 2, both pinned, the difference recorded. No message names the typed text.
  - **Met — C5.** The one-axis target is refused `covered`, the instance unchanged. `Cause::Covered`'s meaning states
    the limit; `describe()` carries it on both surfaces (the help bytes equal `describe()` in `cli_commands` and
    `mcp_server`, gate 5).
  - **Met — C4 / C6(1).** Three hits pinned (`img` / `iframe` / `input`). The transformed-and-scrolled box is hit at
    its row and not through scrolled-out content.
  - **Met — C6(2).** The vertical-lr scrolled box reads its rect, green under both builds (gates 2 and 3).
  - **Met — C13.** Removing `value` re-seeds, removing `step` / `min` / `max` re-sanitizes, the type change clears and
    re-seeds, both modes.
  - **Met — C9.** The macOS leg ran the 90/91 test green on CI#38642493451, `verdict: green`.
  - **Met — C16.** `evidence/log-counts.md` holds both builds × `info` and `trace` × host and `mcp`, beside the dated
    values. `host_log` and `mcp_log` are green under both builds (gates 5 and 6). The href reading is not captured: no
    stand source renders an `a` with `href` (`command grep -rnE '\ba \{|href' examples/seven_guis/src`: 0).
  - **Met — the silence census.** It reads 0 (gate 7). The `command` and `tool_call` spans keep eight and two fields
    (no span file changed).
  - **Met — the preservation guard.** No `packages` change but the three driver text files; no manifest, workflow,
    script or lean-task change; no listener, port, env var or crate (gate 8).
  - **Met — the gate counts.** `run stand` ends `passed`, 168 · 0 · 7, re-counted from 162 by exactly the 6 stand
    tests added. `ci-leg.sh a11y`, `fast` and `doc` exit 0.
- **Gates** (the implement pass, trail `.andromeda/runs/2026-10-14T21-20-05-implement/`, summary `entries 17 · green 14 ·
  red 0 · recorded 0 · timeout 0 · not-run 3`):
  - `cargo test -p blitz-tests --locked --test stand_requirement_sweep --test stand_non_lean_tasks --test stand_actionable_keys` — green (exit 0 · lacks running 0 tests · lacks FAILED)
  - `cargo test -p blitz-tests --locked --test stand_act_obstructed --test hit_clipped_at_scrolling_box --test scrolled_box_client_rect --test range_input_interaction --test password_caret_click` — green
  - `cargo test --workspace --locked --test scrolled_box_client_rect --test hit_clipped_at_scrolling_box --test stand_requirement_sweep` — green
  - `cargo test -p escher-driver --locked` — green
  - `cargo test -p seven_guis --locked --test cli_commands --test mcp_server --test host_log --test mcp_log` — green
  - `cargo test --workspace --locked --test host_log --test mcp_log` — green
  - the silence census `cat … | grep -c -E 'println!|…'` — green (exit 1 · last line 0)
  - the preservation guard `git diff --quiet 19756f2d… -- packages … ':(exclude)…json.rs' …` — green (exit 0)
  - the task-file probe `git diff -U0 19756f2d… -- examples/seven_guis/src/tasks | … | wc -l` — green (exit 1 · last
    line 0)
  - `bash scripts/agent-run.sh boot` — green
  - `bash scripts/agent-run.sh run stand` — green (`run.end` 168 · 0 · 7, outcome passed, 40 files)
  - `bash .github/scripts/ci-leg.sh a11y` — green (artifact fresh: `target/ci-logs/a11y.log` 21:34:48Z)
  - `bash .github/scripts/ci-leg.sh fast` — green (`target/ci-logs/test.log` 185 · 951 · 0 · 12)
  - `bash .github/scripts/ci-leg.sh doc` — green
  - `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` — `leg = 'operator'`, fired as written in
    the operator pass: `hygiene: clean — read 69 (runs 60 · evidence 3 · inputs 6) …` (`evidence/ci.md`)
  - `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin build/escher-0.1.0` —
    `leg = 'operator'`, fired through `--operator 16`: green · exit 0 · 135.65 s · `history moved:
    refs/remotes/origin/build/escher-0.1.0 19756f2d→55a25638` (the push, intended); its log reads 185 result lines,
    951 · 0 · 12, CI scripts `Ran 79 tests` OK (`evidence/ci.md`)
  - `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800` — `leg =
    'operator'`, fired as written: `55a256384c80 verdict: green · checks 16/16 · wall 886 s · runs CI#38642493451
    completed/success` (`evidence/ci.md`)
  - Smoke: the plan lists none. The `escher-session` binary booted in its host and `mcp` roles under both builds in
    step 11's capture, 8 runs, exit 0.
- **Watches:** none folded.
- **Outcome basis:**
  - The operator pass ran. Commit list `55a256384 chore(2026-10-14-stand-requirement-sweep): operator pre-CI commit,
    for the run this chunk verdict reads`, and the final HEAD's CI run CI#38642493451, green, recorded in
    `evidence/ci.md`.
  - The implement P4 report as given in this window, persisted as
    `.andromeda/runs/2026-10-14T21-20-05-implement/outcome-2026-10-14-stand-requirement-sweep.md`.
  - Post-implement artifacts: `evidence/log-counts.md`, `evidence/measurements.md`, `evidence/ci.md`.
  - **Two owners P5 needs to place:**
    - the eight residuals lines (the card's rows), each naming the founder's or the operator's label verbatim;
    - an owner for the client-rect-ignores-`transform` reading (Spec claims disproved, last bullet).
  - **A route note for P5:** this entry's sixteen CARRYs are each discharged here, or moved to a residuals line by the
    card.
- **Process hygiene:**
  - From implement's census, measured by `ps -eo pid,comm,args` at 21:39:15Z: no `cargo`, `escher-sessio` or
    `seven_guis` process. The gate entries, the session hosts the session tests start, and the 8 step 11 runs were all
    terminated.
  - The operator pass started entry 16's `fast` leg and push, which ended exit 0, and entry 17's `ci.py` poll, which
    ended exit 0. Re-measured here: the census reads no `cargo`, `escher-sessio` or `seven_guis` process.
## Addendum — resumed 2026-10-14
- inputs#I5 · message: the operator, /andromeda-wrap-session invocation arguments, 2026-10-14 22:09Z · copy · n/a — the overseer directive: (1) the transformed-box client-rect fact gets a residuals line, target next; (2) security-plan states the measured C11 password-caret leak (index 5 vs 2) as a known 0.1.0 limit, with its fix owned by the C11 residuals line
  - bears on **A reading measured beside C6(1), stated by no master:**: supersedes «an engine divergence with no owner. P5 gives it one» — its owner is a `.andromeda/residuals.md` line, `(target: next)`
  - bears on **C6(1)'s client-rect pin.**: supersedes «Its owner is a P5 route question» — the same residuals line
  - bears on **Two owners P5 needs to place:**: supersedes «an owner for the client-rect-ignores-`transform` reading» — placed by the directive, a residuals line
  - bears on **The password caret qualifier**: adds to it — security-plan.md's change states the measured click-caret reading (index 5 after eight narrow characters, 2 after eight wide) as a known 0.1.0 limit, its fix owned by the card's row 6 residuals line (C11)
  - bears on **C11.**: adds to it — the same security-plan statement
- inputs.py verify, re-fired: inputs: 5 entries — unchanged 1 · drifted 1 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 3 · uncited 1 · unparsed 0; moved since the report: I4 uncited → cited (report.md:315); I5 new, uncited before this addendum
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 19756f2d (the parent of the oldest pre-CI commit 55a25638) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### examples/seven_guis/src/tasks/cells.rs — added 2 line(s) in 2 range(s)
added: 375 · 384
### examples/seven_guis/src/tasks/circle_drawer.rs — added 5 line(s) in 5 range(s)
added: 29 · 40 · 53 · 106 · 126
### examples/seven_guis/src/tasks/temp_converter.rs — added 2 line(s) in 2 range(s)
added: 13 · 26
### examples/seven_guis/tests/cli_commands.rs — added 20 line(s) in 7 range(s)
added: 544-555 · 577-578 · 581 · 590-591 · 598 · 607 · 610
- 547-551 @549 «fn a_state_directory_of_90_bytes_holds_a_session_and_one_of_91_is_refused_with_its_bound() {»
### packages/escher-driver/src/json.rs — added 4 line(s) in 2 range(s)
added: 675 · 679-681
### packages/escher-driver/src/lib.rs — added 5 line(s) in 2 range(s)
added: 68-69 · 71-73
### packages/escher-driver/src/refusal.rs — added 9 line(s) in 3 range(s)
added: 22-24 · 80-82 · 268-270
### tests/blitz-tests/tests/hit_clipped_at_scrolling_box.rs — added 201 line(s) in 3 range(s)
added: 16-21 · 23 · 402-595
- 403-410 @405 «const OWN_BOX_PAGE: &str = r#"<html><body style="margin:0">»
  - 407-408 «<iframe id="frame" style="display:block; width:200px; height:100px; border:0"»
- 412-425 @413 «fn own_box_page(incremental: bool) -> HtmlDocument {»
  - 417-422 «doc.get_node_mut(picture)»
- 427-430 «fn centre_of(doc: &HtmlDocument, selector: &str) -> (f64, f64) {»
- 432-454 @433 «fn a_hit_at_an_image_answers_the_image() {»
  - 434-453 «for incremental in [false, true] {»
- 456-482 @459 «fn a_hit_inside_a_sub_document_answers_the_iframe() {»
  - 460-481 «for incremental in [false, true] {»
- 484-507 @485 «fn a_hit_inside_a_filled_text_input_answers_the_input() {»
  - 486-506 «for incremental in [false, true] {»
- 512-530 @514 «const TRANSFORMED_PAGE: &str = r#"<html><body style="margin:0">»
  - 515-529 «<div id="stage" style="width:400px; height:500px">»
- 532-535 @533 «fn moved(point: (f64, f64)) -> (f64, f64) {»
- 537-595 @544 «fn a_box_both_transformed_and_scrolled_is_hit_where_its_rows_show() {»
  - 545-594 «for incremental in [false, true] {»
### tests/blitz-tests/tests/password_caret_click.rs — new file · 95 line(s)
- 15-18 «const PAGE: &str = r#"<html><body style="margin:0">»
  - 16-17 «<input id="field" type="password"»
- 27-45 @29 «fn caret(harness: &Harness<HtmlDocument>) -> (usize, f32) {»
  - 32-36 «let data = doc»
  - 40-43 «let width = data»
- 47-95 @48 «fn a_click_at_one_offset_places_the_caret_by_the_real_characters() {»
  - 49-94 «for incremental in [false, true] {»
### tests/blitz-tests/tests/range_input_interaction.rs — added 111 line(s) in 2 range(s)
added: 6-9 · 301-407
- 302-361 @303 «fn attribute_removals_reseed_and_resanitize_the_value() {»
  - 304-360 «for incremental in [false, true] {»
- 363-407 @364 «fn a_type_change_clears_the_range_data_and_a_change_back_reseeds_it() {»
  - 365-406 «for incremental in [false, true] {»
### tests/blitz-tests/tests/scrolled_box_client_rect.rs — added 34 line(s) in 3 range(s)
added: 7 · 30-38 · 163-186
- 30-37 @32 «const ONE_VERTICAL_BOX: &str = r#"<html><body style="margin:0">»
  - 34-36 «<div id="box" style="writing-mode:vertical-lr; margin-left:40px; width:300px; height:200px; overflow:auto">»
- 163-185 @164 «fn a_scrolled_box_under_a_vertical_writing_mode_reads_the_client_rect_it_read_unscrolled() {»
  - 165-184 «for incremental in [false, true] {»
### tests/blitz-tests/tests/stand_act_obstructed.rs — added 65 line(s) in 3 range(s)
added: 15-17 · 140-154 · 566-612
- 140-153 @142 «fn one_axis_clip_fixture() -> Element {»
  - 144-152 «rsx! {»
- 567-612 @571 «fn a_target_beside_a_box_clipped_on_one_axis_is_refused_covered() {»
  - 572-611 «for incremental in [false, true] {»
### tests/blitz-tests/tests/stand_actionable_keys.rs — added 7 line(s) in 3 range(s)
added: 4 · 142-144 · 153-155
### tests/blitz-tests/tests/stand_non_lean_tasks.rs — new file · 147 line(s)
- 19-22 «fn app_root() -> Element {»
- 24-35 @25 «fn open(card: &str, title: &str, incremental: bool) -> Harness<DioxusDocument> {»
- 37-63 @39 «fn reacting_without_role(harness: &Harness<DioxusDocument>) -> usize {»
  - 40-46 «let roles: HashMap<u64, Role> = harness»
  - 48-62 «harness»
- 65-69 «const NON_LEAN: [(&str, &str); 3] = [»
- 71-90 @73 «fn circle_drawer_with_dialog(incremental: bool) -> Harness<DioxusDocument> {»
  - 77-81 «assert_eq!(»
  - 85-88 «assert!(»
- 92-120 @93 «fn elements_that_react_to_a_click_and_state_no_role_are_counted() {»
  - 94-102 «const MEASURED: [(&str, usize); 7] = [»
  - 103-119 «for incremental in [false, true] {»
- 122-132 @123 «fn circle_drawers_dialog_rule_applies() {»
  - 124-131 «for incremental in [false, true] {»
- 134-147 @135 «fn circle_drawers_open_dialog_reads_every_control_keyed() {»
  - 136-146 «for incremental in [false, true] {»
### tests/blitz-tests/tests/stand_requirement_sweep.rs — new file · 613 line(s)
- 23-25 «use session_common::{»
- 27-49 @30 «fn read_line(text_line: &str) -> Option<(&str, Vec<&str>)> {»
  - 32-34 «if text_line.matches(FIELD).count() != 1 {»
  - 39-47 «while !rest.is_empty() && !rest.starts_with('@') {»
- 51-63 @53 «fn closing_quote(quoted: &str) -> Option<usize> {»
  - 55-61 «for (offset, character) in quoted.char_indices() {»
- 65-72 @67 «fn screen(session: &mut Session, at: &str) -> String {»
  - 68-71 «match session.run(&snapshot()) {»
- 74-83 @76 «fn acting(task: LeanTask) -> Call {»
  - 77-82 «match task {»
- 85-211 @86 «fn every_requirement_holds_through_the_driver_on_every_lean_task() {»
  - 87-210 «for incremental in [false, true] {»
- 213-222 @215 «enum Subject {»
- 233-528 @236 «const WITNESSES: [(&str, &[Witness]); 16] = [»
  - 237-247 «(»
  - 248-267 «(»
  - 268-278 «(»
  - 279-289 «(»
  - 290-325 «(»
  - 326-341 «(»
  - 342-357 «(»
  - 358-368 «(»
  - 369-384 «(»
  - 385-395 «(»
  - 396-431 «(»
  - 432-456 «(»
  - 457-471 «(»
  - 472-492 «(»
  - 493-512 «(»
  - 513-527 «(»
- 530-576 @532 «fn source(file: &str) -> Option<&'static str> {»
  - 533-575 «Some(match file {»
- 578-613 @579 «fn every_requirement_names_its_witness() {»
  - 580-583 «const EVERY_SCREEN: [&str; 11] = [»
  - 584-587 «let requirements: Vec<String> = (1..=17)»
  - 589-595 «assert!(»
  - 596-612 «for (row, (requirement, witnesses)) in WITNESSES.iter().enumerate() {»
