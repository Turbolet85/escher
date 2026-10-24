# Report — 2026-10-24-stand-requirement-sweep

**Chunk:** Stand requirement sweep — every 0.2.0 capability proven headless on stand or fixture, inside stand-repeat;
coverage floors held, new verbs' stderr id-free
**Date:** 2026-10-24
**Commits:** since `last_wrap` 2026-10-24T14:02:36Z (`git log --since`): `126f56e67` feat(2026-10-24-upstream-sync-polish-and-ship) (the
previous wrap's own commit) · `7ad040af6` chore(u55): the six door sidecar headings in marker form · `4b4804ba5`
chore(cites): the F334 backlog — 252 category-file citations re-pointed · `18b50a2a2` chore(2026-10-24-stand-requirement-sweep):
operator pre-CI commit, for the run this chunk's verdict reads. This chunk's base is `4b4804ba5`, the parent of its one pre-CI
commit `18b50a2a2` (`git log --format='%h %s' 4b4804ba5..HEAD`: that commit alone).

## Changes (structured — detectors read this)
- **Files:** 9 changed, every one on research's Files to modify (`gate.py scope`: `scope: clean — changed 9 · listed 9 ·
  recorded 0`), no new file:
  - `tests/blitz-tests/tests/stand_requirement_sweep.rs` — the 0.2.0 record and its leg check (2 → 3 tests);
  - `tests/blitz-tests/tests/stand_lint.rs` — C1's co-reading page and test (6 → 7 tests);
  - `tests/blitz-tests/tests/text_extent.rs` — C1's three stated limits (3 → 6 tests);
  - `tests/blitz-tests/tests/stand_act_spans.rs` — a paged part read in the sink capture (35 → 36 calls per mode);
  - `tests/blitz-tests/tests/fixtures/mod.rs` — `ITEMS` and `many_items` moved in;
  - `tests/blitz-tests/tests/stand_snapshot_parts.rs` — reads `fixtures::many_items`;
  - `examples/seven_guis/tests/host_log.rs` — the `start --record` clients' own stderr at `trace` (2 → 3 tests);
  - `.github/scripts/ci-leg.sh` — `stand-repeat` names ten more targets;
  - `.github/scripts/test_ci_workflows.py` — the leg's pin gains those ten words.
- **Symbols / APIs:** test code only, no library surface; every coordinate below is copied from `## New text, by line`.
  - `stand_requirement_sweep.rs`:
    - `Subject::Unit` (227-229): a crate's unit test, run by the workspace `test` leg, outside `stand-repeat` by
      construction;
    - `WITNESSES_020: [(&str, &[Witness]); 9]` (538-818): rows v020-01…09, 46 witnesses — v020-01 and v020-02 hold
      `Unit` witnesses alone (six unit tests in `packages/blitz-dom`, `packages/escher-driver`,
      `packages/dioxus-native-dom` and `examples/seven_guis/src/tasks/timer.rs`), v020-08 one `Unit` (`node/select.rs`);
    - `STAND_REPEAT_NAMED: [&str; 10]` (820-832), `STAND_REPEAT_SEVEN_GUIS: [&str; 8]` (834-844), `CI_LEG`, and 28 new
      `source()` arms, among them `.github/scripts/ci-leg.sh` through `include_str!`;
    - the test `every_0_2_0_requirement_names_its_witness_inside_stand_repeat` (994-1069). It asserts:
      - the nine rows in order, each naming a witness that is found as `fn {test}(` in its file;
      - v020-01/02 hold `Unit` witnesses alone, and every other row holds a non-`Unit` one;
      - every non-`Unit` blitz-tests witness is a `stand_*` file or one of `STAND_REPEAT_NAMED`, and every
        `examples/seven_guis/tests/` witness is one of the eight;
      - the leg's text holds the `tests/blitz-tests/tests/stand_*.rs` glob and `--test {name}` for all 18 names.
    `WITNESSES` and `every_requirement_names_its_witness` are byte-unchanged.
  - `stand_lint.rs`: in-file page `reading_page()` (110-141) — `fx-cut-text` (a nowrap line past its 100 × 40
    `overflow: hidden` box), `fx-padded-text` (a 60 px paragraph whose `padding-right: 100px` alone passes its box),
    `fx-off-text` (wholly right of the 800 px viewport) — and
    `a_clipped_or_off_screen_node_holding_text_reads_truncated_beside_it` (226-274), both layout modes. It reads
    `fx-cut-text` clipped · `fx-cut-text` truncated · `fx-padded-text` clipped · `fx-off-text` truncated ·
    `fx-off-text` off-screen. Row 1's line is seen in part and row 3's not at all. The `lint` verb answers the same.
  - `text_extent.rs`: page paragraphs `#ltr` · `#rtl` (`dir="rtl"`) · `#vertical` (`writing-mode: vertical-lr`) ·
    `#scaled` (`transform: scale(2)`) (25-28), the helper `extent` (120-125) and three tests, both layout modes, green
    under `-p blitz-tests` and `--workspace`:
    - `right_to_left_text_reads_as_a_left_to_right_line_from_the_content_box` (127-152);
    - `vertical_text_reads_as_a_horizontal_line` (154-174);
    - `a_transformed_box_reads_its_lines_at_its_client_rect` (176-205): the content box equals
      `get_client_bounding_rect`, which is 2 × `offset_rect` about the same centre, and the lines are unscaled.
  - `stand_act_spans.rs`: in `child_records_each_command_at_info`, `fixtures::hold("paged", fixtures::many_items, …)`
    reads `snapshot_part(2)` as `Outcome::Part { part: 2, parts ≥ 2 }` (685-693). `Needles::read` also takes
    `#items > button` text as name needles (243); its 400 `item-NNN` ids join the id needles. The call count is
    35 → 36 (727).
  - `fixtures/mod.rs`: `pub const ITEMS: usize = 400` and `pub fn many_items() -> Element` (64-86), moved unchanged
    from `stand_snapshot_parts.rs`.
  - `host_log.rs`: `#[cfg(unix)] a_recording_sessions_clients_log_nothing_of_it_at_trace` (40-130). It runs each
    client command through `common::run` with `RUST_LOG=trace`, in the state directories `hl-client` and
    `hl-client-rec` under `CARGO_TARGET_TMPDIR`, guarded by `common::Started { dir, pid }`. The commands:
    `start crud --record`, `type --id crud-name --text {SENTINEL}`, `snapshot`, `snapshot --part 1`, `lint`, `stop`.
    Over all the clients' stderr it reads 0 id, 0 name, 0 sentinel, 0 recording-path and 0 `recorded_needles`.
- **Crates / modules:** none added or removed; no `[[test]]` target added (`test_blitz_tests_targets.py`'s pin unmoved).
- **Dependencies:** none (`git diff --name-only 4b4804ba5 -- Cargo.toml Cargo.lock deny.toml packages …`: 0 — gate entry 10).
- **Schema / config:** `.github/scripts/ci-leg.sh` `stand-repeat` (54-58): `stand+=(--test inert_subtree --test
  accessibility_states --test scrolled_box_absolute_position --test hit_clipped_at_scrolling_box --test
  password_caret_click --test transformed_client_rect --test accessibility_active_descendant_unread --test
  virtual_clock_engine --test select_interaction --test text_extent)`, after the `stand_*.rs` glob, on the same
  `cargo test -p blitz-tests --locked` line every round. A comment names the founder's word (2026-10-24). The
  seven_guis half, `--locked`, no retry, no `--ignored`, no env var, and the `a11y` and `coverage` legs are unchanged.
  `.github/scripts/test_ci_workflows.py` `test_stand_repeat_leg_runs_three_rounds` (359-362) asserts the ten words.
- **Spec-master edits:** none.
- **Retired:** none.
- **Counts / qualifiers moved:** each fact names its pattern (`report-patterns.toml`). The sweep is the pre-pass over
  the masters, their category files and the registries at HEAD `18b50a2a`: `cascade.py sweep --pre-pass`, trail
  `cascade-2026-10-24-stand-requirement-sweep-pre-pass.json`. Dispositions were read per row through each row's
  clause.
  Sweep reach: 15 patterns · 12 moved facts, 12 with a pattern (the report's own count) · owner pattern control unfired
  (`control: owner-title never fired over the seven masters' pre-pass text, bodies and sets (18b50a2a)`) · by line
  1 lines · 4 offsets.
  1. **stand-repeat's selection: the `stand_*` files and seven_guis' eight, plus ten named blitz-tests targets**
     (`leg-select` standing 28, `ten-witnesses` standing 69, `stand-files` standing 4).
     - **Moved:**
       - `architecture-detail/standard-contracts.md:256` (both clauses: `--test` per `stand_*.rs`; "the script reads
         … the `stand_*.rs` set");
       - `test-plan.md:374` ("runs the stand checks and seven_guis' process checks");
       - `test-plan.md:410` ("run the `stand_*.rs` files and seven_guis' process checks");
       - `test-plan-detail/ci-integration.md:8` ("one `--test` per … `stand_*.rs` — 48 files");
       - `test-plan-detail/quality-gates-coverage-targets.md:22` ("run the 48 `stand_*.rs` files and seven_guis' eight");
       - the 8 rows placing a ten-witness target's runners, each gaining stand-repeat beside them (the runner clause
         read on each):
         - `architecture-detail/existing-scopes.md:102` (accessibility_states "run by the workspace test leg and per
           package");
         - `a11y-plan.md:53` (accessibility_states) and `a11y-plan.md:55` (inert_subtree,
           accessibility_active_descendant_unread);
         - in `test-plan-detail/test-scope-summary.md`: `:22` (text_extent "not by `run stand`"), `:42`
           (password_caret_click "not selected by `run stand`"), `:96` (accessibility_states), `:99` (the
           inert_subtree and active-descendant clause) and `:122` (select_interaction "not by `run stand`").
     - **No-change:**
       - the rows naming the leg, job, cache or budget alone: `architecture.md:220` · `246` · `247` · `315`,
         `inherited-defaults.md:14`, `occupied-resources.md:70`, `stack-and-technologies.md:77`,
         `security-plan.md:264`, `input-validation.md:28`, `test-plan.md:378`, `ci-integration.md:9` · `15` · `21` · `38`,
         `test-scope-summary.md:18`, `test-strategy.md:22`, `unit-test-strategy.md:27` · `28`;
       - the "by its prefix" rows of `stand_*` files, still true: `test-scope-summary.md:121` · `122` · `123`;
       - the three `stand_*` a11y files: `a11y-plan.md:235`, `a11y-plan-detail/ci-integration.md:8`;
       - the other 61 `ten-witnesses` rows (69 standing less the 8 above), which cite or describe a test and name no
         leg it runs in — among them `test-scope-summary.md:36` · `111` · `112` · `120` (virtual_clock_engine,
         hit_clipped, scrolled box, transformed rect), read with no runner clause;
       - the runner rows of tests outside the ten: `test-scope-summary.md:98` (upstream's focus checks) and `:110`
         (range_input_interaction);
       - the `stand-files` count of 48 is unmoved: the glob still matches 48 files.
  2. **The leg pin gains the ten words** (`leg-pin` standing 18). Moved: `test-plan-detail/ci-integration.md:8` · `:9`
     (they name what `test_stand_repeat_leg_runs_three_rounds` pins). No-change: the 16 other rows cite
     `test_ci_workflows.py` for other invariants.
  3. **`stand_requirement_sweep` 2 → 3, a second table (`WITNESSES_020`), `Subject::Unit`, the leg check**
     (`sweep-record` standing 15). Moved:
     - `architecture-detail/existing-scopes.md:113` ("`WITNESSES`, 16 rows");
     - `test-plan-detail/test-scope-summary.md:117` ("`stand_requirement_sweep` 2", "`WITNESSES` … 16 rows");
     - `obs-plan.md:77` ("its `WITNESSES` table");
     - `registries/contracts/test-plan/session-lifecycle.md:11` @c5793 (the 2 → 3) and @c6015 (the `WITNESSES` record).

     No-change: the dated readings `test-plan.md:174` and `ci-integration.md:58`, and 9 rows naming the file.
  4. **`stand_lint` 6 → 7, and "a clipped or off-screen node holding text also reads `truncated`" asserted**
     (`lint-coread` standing 22). Moved:
     - `architecture-detail/standard-contracts.md:181` ("asserted by no check" → asserted by `stand_lint`);
     - `test-plan-detail/test-scope-summary.md:120` ("`stand_lint` 6");
     - `test-scope-summary.md:121` ("is asserted by no check").

     No-change:
     - the dated readings `test-plan.md:175`, `ci-integration.md:39` · `60`;
     - the select's and `PARALLEL_TRAVERSAL_LOCK`'s "asserted by no check" clauses, `test-scope-summary.md:22` @c1162
       and `:23`;
     - 13 rows naming the file.

     `test-plan.md:19` and `test-scope-summary.md:22` (its text-extent clause) also stand on this pattern; they move
     under fact 5.
  5. **`text_extent` 3 → 6, its three stated limits asserted** (`extent-limits` standing 16). Moved:
     - `test-plan.md:19` ("the text extent's … stated limits are asserted by no check", the text extent's half only;
       the select's stays);
     - `test-plan-detail/test-scope-summary.md:22` (`text_extent` 3; "its stated limits … are asserted by no check";
       the runners, see fact 1).

     No-change:
     - `architecture-detail/standard-contracts.md:54`: the limits stand as limits; a pointer to their check may be added;
     - `standard-contracts.md:181`: fact 4;
     - `security-plan-detail/input-validation.md:234`: the select's limits;
     - the dated readings `test-plan.md:175`, `ci-integration.md:39` · `60`;
     - 8 rows naming the reader (`architecture.md:145`, `existing-scopes.md:19`, `security-plan.md:141`,
       `input-validation.md:250` · `251`, `test-strategy.md:48`, `obs-plan.md:92`, `span-trace-coverage.md:28`).
  6. **`stand_act_spans` 35 → 36 calls per mode, a paged part read (part 2 of a screen that pages) in the sink capture**
     (`spans-calls` standing 32). Moved: `test-plan-detail/test-scope-summary.md:108` ("35 calls per mode — one
     `snapshot` with `part` 1 among them"); `obs-plan-detail/pii-scrubbing-compliance.md:9` ("35 calls per mode … a part
     read among them"). No-change: 30 rows naming the file, its children or the span rule, among them
     `standard-contracts.md:165`, `logging-monitoring.md:13` and `log-coverage.md:41` (the part-read line's shape,
     unchanged), and `session-lifecycle.md:11` @c5172 ("`stand_act_spans` 2 (+2 ignored children)", unchanged) and
     @c10118 (provenance).
  7. **A paged part read through the `escher-session` host's own sink stays unmeasured** (the host boots the four lean
     tasks, none pages) — a stated limit with an owner, inputs#I2's card row 1 (`paged-limit` standing 11). All 11 rows
     state the paged answer's form (`{"text","part","parts"}`), not a reading of its log: no-change; the limit is new
     text, placed beside fact 8's sites.
  8. **`host_log` 2 → 3; the `start --record` clients' own stderr read at `trace`, 0 needles, a standing check**
     (`host-log` standing 45). Moved:
     - `test-plan-detail/test-scope-summary.md:15` ("`tests/host_log.rs` (2 tests");
     - `registries/contracts/test-plan/session-lifecycle.md:11` @c2849 ("seven_guis `host_log` 2");
     - the four rows stating the 2026-10-14 one-off reading of "a client's own stderr", now joined by a standing check
       at `trace` under `--record`: `security-plan-detail/logging-monitoring.md:14`, `test-plan-detail/ci-integration.md:29`,
       `test-plan-detail/integration-test-strategy.md:22` and `obs-plan-detail/pii-scrubbing-compliance.md:10`.

     No-change: `standard-contracts.md:156` (the `start --record` grammar); `session-lifecycle.md:11` @c8391 · @c9148
     (provenance); 38 rows naming `host_log` for its standing readings.
  9. **The session checks' state directories gain `hl-client` and `hl-client-rec`** (`state-dirs` standing 2). Moved:
     `architecture-detail/occupied-resources.md:25`, `obs-plan-detail/ci-integration.md:15` (the lists "`hb-*`,
     `hl-trace`, `cc-*`, `cf-*` and `ht-timer`"; `hl-rec` / `hl-rec-kept` are listed at neither, a pre-existing gap the
     detectors may raise).
  10. **The shared fixture pages gain `many_items` (and `ITEMS`), now read by `stand_snapshot_parts` and
      `stand_act_spans`** (`fixture-pages` standing 15). Moved: `test-plan.md:151` ("the pages `password_labelled`,
      `password_named` and `covered`"), `test-plan.md:320` (the same list). No-change: `conventions.md:61`,
      `test-scope-summary.md:90` (the generic "pages more than one check reads"); 11 rows citing a page or the module.
  11. **The tallies: workspace 206 · 1099 → 206 · 1105; CI linux 206 · 1099 → 206 · 1105, macos and coverage
      187 · 1096 → 187 · 1102, windows 187 · 1052 → 187 · 1057, the stand repeat 168 · 684 · 0 · 21 →
      198 · 837 · 0 · 21, a11y 6 · 43 unmoved** (`tallies` standing 9). Moved: `test-plan-detail/ci-integration.md:41`
      and `:61` (the latest readings, a new reading appended); `quality-gates-coverage-targets.md:22` ("168 · 684 · 0 ·
      21"). No-change: `ci-integration.md:39` · `40` (dated); 4 citation-digit rows (`layout-templates.md:10`,
      `test-plan.md:110` · `330`, `test-data-fixtures.md:24`; the tool's warning row).
  12. **`run stand` 195 → 197 passed · 0 · 7 over 48 files** (`run-stand` standing 26, `stand-files`). Moved:
      `test-plan.md:176` (the latest reading appended), `test-plan.md:189` ("`run stand` 195 · 0 · 7 over 48 files").
      No-change: the dated readings `test-plan.md:170`-`175`; the contract rows `test-plan.md:161` · `164` · `169` ·
      `182` · `215`; `obs-plan.md:116`; and the runner rows of fact 1.

  `owner-generic`: 1 standing row, `security-plan-detail/input-validation.md:197`, a clause about how a residual is
  owned and not about this chunk: no-change. The coverage floors are unmoved: escher-driver 4918/5019 = 97.99 %,
  dioxus-native-dom 2740/3202 = 85.57 %, blitz-test-harness 949/1008 = 94.15 %, equal to CI#40105170961 (gate entry
  16's log).
- **Dev-tool versions:** none.
- **Harness / gate surface:** the `stand-repeat` leg's selection (Schema / config). Its CI job `Stand checks repeated`
  runs it unchanged, read at CI#40114977936: 198 result lines · 837 passed · 0 failed · 21 ignored. The agent-run
  selections are unchanged: `run stand` keeps the prefix rule, and the ten reach the agent harness through
  `run all` / `run {name}` alone.
- **Cross-project / external claims:**
  - CI#40114977936 on `18b50a2a20f3`: `verdict: green · checks 17/17`, its per-job tallies in
    `evidence/ci.md` (gate entries 22-23, `evidence/operator-pass.md`).
  - `inputs.py verify`: `inputs: 2 entries — unchanged 1 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 1 · uncited 1 · unparsed 0`.
  - I1 · `../additional/escher-overseer/relays/stand-sweep-founder-2026-10-24.md` · copy · unchanged — the founder's
    word «Список в скрипте (Рекомендую)» (2026-10-24), the named `--test` list in `stand-repeat`; cited `plan.md:21` ·
    `:402` · `:437` · `:444` · `:490`, and here (Schema / config).
  - inputs#I2 · `message: the operator, in the phase session, 2026-10-24 about 16:43Z` · copy · n/a (a message has no
    live source) — the operator's own word «yes — apply.» on the P5 review card
    `.andromeda/runs/2026-10-24T15-06-04-phase/p5-card-2.md`, approving the plan and its one Beyond-the-plan row;
    `verify` printed it `UNCITED` before this report, which cites it. Card 2 governs (no other entry names a card).
    Its `## Beyond the plan` rows, verbatim, the word's authority the operator's own:

    | # | Proposal | Owner / place |
    |---|---|---|
    | 1 | A paged `part` read through the `escher-session` host's own sink stays unmeasured: the host boots only the four lean tasks, and none pages. It is stated as a limit in the masters (Expected amendments), and its measurement is owned. | a new `.andromeda/residuals.md` line, «a paged `part` read through the escher-session host's sink» (target: next), placed at this chunk's wrap |
- **Reverted / negative API facts:** `ci-leg.sh`'s ten were first drafted as a bash array looped into `--test "$f"`. That
  form was rewritten to literal `--test {name}` words before any gate ran, because the record's check reads
  `--test {name}` in the script's text.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:** none. Every prediction measured as stated:
  - C1's findings list (plan step 2);
  - `text_extent`'s three limits under both builds;
  - every count of the plan's gates (`evidence/counts.md`).
- **Expected amendments (from plan):**
  - test-plan §9 Legs and §10 Flakiness budget — stand-repeat runs the `stand_*` files, the ten named and the eight
    seven_guis targets, ratified by the founder (2026-10-24) «Список в скрипте (Рекомендую)» (inputs#I1); §3: `run
    stand` reaches the ten only through `run all` / `run {name}` — carried: Counts 1 and 12 (`leg-select` 16 test-plan
    rows of its 28 — `test-plan.md:374` · `378` · `410` and 13 in `test-plan-detail/`; `run-stand` 25 test-plan rows of
    its 26 — 13 in `test-plan.md`, 12 in `test-plan-detail/`; counted from the sweep's rows).
  - test-plan §1 and §2 — `stand_requirement_sweep` 3, `stand_lint` 7, `text_extent` 6 inside stand-repeat,
    `stand_act_spans` 36 calls, `host_log` 3, the fixture pages gaining `many_items`; `test-plan.md:19` "asserted by
    no check" → asserted — carried: Counts 3, 4, 5, 6, 8, 10 (`sweep-record`, `lint-coread`, `extent-limits`,
    `spans-calls`, `host-log`, `fixture-pages`; test-plan hits at the rows named there).
  - test-plan §5 and obs-plan §6/§3 — the `start --record` clients' own stderr read at `trace`, 0 needles; the paged
    part read in the sink capture; the host-role paged read a stated limit — carried: Counts 6, 7, 8 (`host-log`:
    14 obs-plan rows — `obs-plan.md` 8, `obs-plan-detail/` 6 — and 17 test-plan rows — `test-plan.md` 2,
    `test-plan-detail/` 14, its key file `session-lifecycle.md` 1 — counted from the sweep's rows; `spans-calls`;
    `paged-limit` 11, all no-change).
  - architecture §Standard Contracts → CI contracts (stand-repeat's selection, `standard-contracts.md:256`), → Driver
    session (the co-reading asserted, `standard-contracts.md:181`), → Scrolling, selection, tree (`text_extent`'s
    limits asserted, `standard-contracts.md:54`); §Occupied Resources → filesystem (`hl-client`, `hl-client-rec`,
    `occupied-resources.md:25`); §Existing Scopes (blitz-tests counts, `existing-scopes.md:102` · `113`) — carried:
    Counts 1, 3, 4, 5, 9 (`leg-select`, `lint-coread`, `extent-limits`, `state-dirs`, `sweep-record`; architecture
    hits at each named row).
  - security-plan §Logging & Monitoring — the client's own stderr re-read at `trace` under `--record` — carried:
    Count 8 (`host-log`; `security-plan-detail/logging-monitoring.md:14`).
  - a11y-plan §3/§9 — `accessibility_states`, `inert_subtree` and `accessibility_active_descendant_unread` also run in
    stand-repeat, the a11y leg's six files unchanged — carried: Count 1 (`ten-witnesses`; `a11y-plan.md:53` · `55`).
  - matrix#v020-03, #v020-05, #v020-06, #v020-08 notes — the dated line that the cap's witnesses now run inside
    stand-repeat (the ten), and for v020-05 the co-reading and the reader's limits asserted; status, ref and acceptance
    untouched — `ledger-note — owner P7.3`, no master site (a ledger write).
- **Coverage of new surfaces:** no new external surface, hot-path op or UI element — test files and one CI leg only.
  - `stand-repeat`'s named list → validation n/a · instrumentation n/a · PII n/a · tests `test_ci_workflows.py` pin +
    `stand_requirement_sweep`'s leg check · a11y n/a · tokens n/a.

## Deviations from intent
- **Step 3, the transformed box.** The plan asks for `#scaled`'s content to be "twice the twin's content width", but its
  only named twin, `#ltr`, is 300 px wide and `#scaled` 80 px. The test takes the box's own `offset_rect` (CSSOM,
  transform-blind) as its untransformed twin: client width = 2 × offset width, and the two centres are equal. The text
  width is compared with `#ltr`'s as planned, and no size is restated.
- **Step 3, the condition first.** The RTL test first asserts `#rtl` carries `dir="rtl"`, and the vertical test asserts
  its twin's line is wider than it is high (`.claude/rules/testing.md`: assert the condition first).
- **Step 2, two reason assertions beyond the list.** Row 1 (`fx-cut-text` truncated) reads a visible part narrower than
  its text, and row 3 (`fx-off-text` truncated) reads no visible part, so the two arms of the co-reading are told apart.
- **Step 8, two additions.** The leg check also asserts that the script holds the `tests/blitz-tests/tests/stand_*.rs`
  glob, so the prefix half of "inside stand-repeat" is read from the leg too. The eight seven_guis targets are a named
  const. Each Subject was read from the test body. The choices worth naming:
  - `stand_replay::…turns_red…` is `Fixture` (a changed Counter app);
  - the other `stand_replay` witness, `recording.rs` and both `cli_commands` witnesses are `Task(Counter)`;
  - `stand_lint::the_lint_verb_answers…`, `stand_mcp_tools`' lint test and `stand_session_recording` are `AllFour`;
  - `stand_virtual_clock` is `Task(Crud)`.
- **Process.** While the backgrounded gate block ran at implement P2, a `ScheduleWakeup` fallback (3600 s) was
  scheduled. The skill names the completion notification as the only wake. The notification came first, and the
  pending wakeup was left with no work, on the operator's instruction "do nothing with it".
- scope record: none — gate.py scope clean, 0 recorded (`scope: clean — changed 9 · listed 9 · recorded 0 (companion 0 ·
  mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 486`).

## Decisions & corrections
- **The founder's word (inputs#I1).** «Список в скрипте (Рекомендую)» (2026-10-24): the ten 0.2.0 witnesses outside the
  `stand_` prefix enter `stand-repeat` by a fixed `--test` list in `ci-leg.sh`, the files keeping their names. It
  ratifies the selection change. Passed over: «Rename them stand_*» · «Founder's call — park it».
- **The operator's word on P5 card 2 (inputs#I2).** «yes — apply.», approving the plan and its Beyond-the-plan row 1,
  a new `.andromeda/residuals.md` line «a paged `part` read through the escher-session host's sink» (target: next),
  placed at this chunk's wrap — **P5 route-resolve acts on it**.
- **The operator's directive for this wrap** (this session, 2026-10-24): "run P1 only and stop. (A U55 debug-loop chunk
  on the newly deployed F336 letters: at P2, after the clear, the operator's word will be «forced sample».)" — **P2
  acts on it**: the fan-out runs as a forced-sample run when the operator gives that word after the clear.
- **The operator pass's directive** (this session, 2026-10-24): run entries 20 → commit → 21 → 22 → 23 by hand in plan
  order, record them in `evidence/operator-pass.md`, stop on any red, and do not start the wrap ("I hold it for a
  deploy"); "If your pending wakeup fires, do nothing with it". All four were green or recorded, and the wrap waited.
- **A plan-wording interpretation** (step 3's "twin"). It was settled from the code (`offset_rect`) without a question
  to the operator. An `input.plan-step-ambiguous` evolve record was filed at implement's code step.
- **Guard refusal** (implement P1). A `cat` heredoc writing a scratchpad script was refused by `bash-guard.sh` ("a
  cat/tee heredoc with a file target"). It was re-issued once through the Write tool and ran. At this wrap a `cd` into
  the skill's references dir was refused the same way ("cd … moves the session cwd") and re-issued as an absolute-path
  grep.
- **Sweep hazards met at this wrap's site sweep.**
  - A TOML literal string cannot hold a `'`: `''` inside `'…'` is refused `not TOML — Expected newline or end of
    document` (exit 2); the pattern takes `'''…'''`.
  - An alternative `pages` (ignore-case) would match every "fixture pages"; it was narrowed to `screen that pages`
    before firing.
  - The bare-number pattern `\b(1099|1096|1052|684)\b` drew the tool's citation-digit warning; four of its nine rows
    are citation digits.
- **A standing learning met, measured otherwise.** CLAUDE.md's Tier-1 line of 2026-10-24 says `cascade.py sweep` "read[s]
  the bodies alone — a zero row … there proves no absence in the category files". This wrap's `cascade v1.16` sweep
  listed rows in `.andromeda/*-detail/` files (e.g. `architecture-detail/standard-contracts.md:256`,
  `test-plan-detail/ci-integration.md:8`), and `amendment-flow.md` §Tool now says the sweep reads "each body and, for an
  extracted master, every category file of its set". The line may be superseded for the sweep — a P3 candidate (the
  correction form), its `cites.py` and `leaves` halves unread here.
- **A rule met and kept.** `.claude/rules/testing.md` 2026-10-19: the new `cfg(unix)` item was checked for
  `x86_64-pc-windows-gnu` (gate entry `cargo check --tests --locked --target x86_64-pc-windows-gnu -p seven_guis`, green).
- **Red before green** (implement). Seven must-fail controls, one per new test or reading, each red by assertion, the
  file restored and compared by sha256 (`evidence/red-before-green.md`).

## Outcome
Acceptance criteria, each re-asserted against the diff (the nine files and the leg's selection):
- `every_0_2_0_requirement_names_its_witness_inside_stand_repeat` passes — **met** (gate entry 3; 9 rows in order, every
  named test found, every integration witness inside `stand-repeat` by prefix or by the named list read from
  `ci-leg.sh`, v020-01/02 `Unit` alone).
- `ci-leg.sh stand-repeat` ends `stand-repeat: 3 of 3 rounds green` with the ten in every round; `test_ci_workflows.py`
  pins the ten words — **met** (local 198 · 837 · 0 · 21; CI `Stand checks repeated` 198 · 837 · 0 · 21; `fast`'s
  ci-scripts leg green).
- `stand_lint::a_clipped_or_off_screen_node_holding_text_reads_truncated_beside_it` in both layout modes — **met**, the
  findings exactly as predicted (plan step 2); the verb answers the same.
- `text_extent`'s three tests in both layout modes under `-p blitz-tests` and `--workspace` — **met** (gate entries 1, 2;
  CI linux, macOS and windows test jobs).
- `host_log` 3 and `mcp_log` 2 under both builds, the new test reading 0 of each needle class; `stand_act_spans` 36
  calls per mode with a paged part 2 of ≥ 2, no id or name of the paged page on a line — **met** (gate entries 3, 5, 6).
- `coverage` ends `coverage floor: 3 of 3 crates at or above their floors`, the floors unchanged — **met** (gate entry 16).
- The manifest/source guard reads 0 and `gate.py guard` reads `guard: clean`; no engine, driver, telemetry, stand-source,
  manifest, lock or committed-check change, and no new target, env var, listener or crate — **met**. The diff adds two
  test state directories (`hl-client`, `hl-client-rec` under `target/tmp/`), an arch §Occupied Resources filesystem
  line owed at P2 (Count 9, Expected amendments), not a new env var or listener. `RUST_LOG` on the client `Command` is
  the registered `RUST_LOG` read.
- `ci-leg.sh a11y` over its six files, 43 tests, and no platform adapter in the stand graph — **met** (gate entries 14, 9).
- The silence census reads 0 — **met** (gate entry 8).
- `fast` and `doc` pass, and the CI run on the pushed sha reads `verdict: green` with `Stand checks repeated` and
  `Coverage report` among its checks — **met**: CI#40114977936 on `18b50a2a20f3`.

Gates — implement's run `.andromeda/runs/2026-10-24T16-43-19-implement/` (`entries 23 · green 19 · red 0 · recorded 0 ·
timeout 0 · not-run 4`), every entry green on its first firing; each named by its `run`:
- `cargo test -p blitz-tests --locked --test text_extent --test stand_lint` — green (exit 0, lacks FAILED): 7 · 6 passed.
- `cargo test --workspace --locked --test text_extent --test stand_lint` — green: 7 · 6.
- `cargo test -p blitz-tests --locked --test stand_requirement_sweep --test stand_act_spans --test stand_snapshot_parts` —
  green: 3 · 2 (+2 ignored) · 3.
- `cargo test -p blitz-tests --locked --test inert_subtree … --test text_extent` (the ten) — green: 48 passed.
- `cargo test -p seven_guis --locked --test host_log --test mcp_log --test recording` — green: 3 · 2 · 4.
- `cargo test --workspace --locked --test host_log --test mcp_log` — green: 3 · 2.
- `cargo check --tests --locked --target x86_64-pc-windows-gnu -p seven_guis` — green.
- the silence census (`cat … | grep -c -E 'println!|…|blitz_net'`) — green (exit 1, last line 0).
- `cargo tree --locked -p seven_guis -e normal --prefix none | grep -c -E '^accesskit_(xplat|winit) '` — green (exit 1, 0).
- `git diff --name-only 4b4804ba5… -- Cargo.toml Cargo.lock deny.toml packages … | wc -l` — green (0).
- `gate.py guard … --allow '…' -- packages tests apps wpt .github scripts examples` — green: `guard: allowed 9 —
  .github/scripts/ci-leg.sh, .github/scripts/test_ci_workflows.py, examples/seven_guis/tests/host_log.rs,
  tests/blitz-tests/tests/fixtures/mod.rs, tests/blitz-tests/tests/stand_act_spans.rs,
  tests/blitz-tests/tests/stand_lint.rs, tests/blitz-tests/tests/stand_requirement_sweep.rs,
  tests/blitz-tests/tests/stand_snapshot_parts.rs, tests/blitz-tests/tests/text_extent.rs · companions excluded 0 ·
  changed 9 · excluded 0` / `guard: clean`.
- `bash scripts/agent-run.sh boot` — green (`"outcome": "ready"`).
- `bash scripts/agent-run.sh run stand` — green: `run.end` passed 197 · failed 0 · ignored 7.
- `bash .github/scripts/ci-leg.sh a11y` — green: 6 · 6 · 4 · 8 · 5 · 14 = 43.
- `bash .github/scripts/ci-leg.sh stand-repeat` — green: `stand-repeat: 3 of 3 rounds green` (360 s).
- `bash .github/scripts/ci-leg.sh coverage` — green: 97.99 · 85.57 · 94.15 %, `3 of 3`.
- `bash .github/scripts/ci-leg.sh fast` — green (artifact `target/ci-logs/test.log` fresh).
- the workspace tally (`grep -E '^test result:' target/ci-logs/test.log | awk …`) — green: `lines 206 passed 1105 failed 0 ignored 12`.
- `bash .github/scripts/ci-leg.sh doc` — green.
- `leg = 'operator'` entries, fired by hand in the operator pass, recorded in `evidence/operator-pass.md`:
  - `gate.py hygiene` — `hygiene: clean`, before the pre-CI commit;
  - `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin
    build/escher-0.2.0` — green, `history moved: refs/remotes/origin/build/escher-0.2.0 4b4804ba→18b50a2a` (the
    intended push);
  - `ci.py conclusion --sha HEAD --wait 1800` — `verdict: green · checks 17/17` on CI#40114977936;
  - `gh run view <id> … | awk …` with `<id>` 40114977936 — `recorded`, its outcome: Test [default features] 206 · 1105 ·
    0 · 12; Test (macos) and Coverage report 187 · 1102 · 0 · 12; Test (windows) 187 · 1057 · 0 · 9; a11y 6 · 43 ·
    0 · 0; Stand checks repeated 198 · 837 · 0 · 21 — 0 failed in every job, each move explained by name in
    `evidence/ci.md`; the conclusion is this chunk's and green.
- Smoke: `skipped — no boot-path / UI-surface change`. The agent-run boot and `run stand` pair ran as gates.
- No deferral: every gate ran, under both builds where the plan lists both.

Watches: none folded.

Outcome basis: the operator pass ran (`18b50a2a2`, its one commit), so the verdicts above rest on its final HEAD:
CI#40114977936 on `18b50a2a20f3`, recorded in `evidence/operator-pass.md` and `evidence/ci.md`. This window holds
implement's run whole, and its persisted report is
`.andromeda/runs/2026-10-24T16-43-19-implement/outcome-2026-10-24-stand-requirement-sweep.md` (the one
`*-implement` dir carrying this marker's gate trail), which agrees. The other evidence is `evidence/counts.md` and
`evidence/red-before-green.md`. The operator directives between implement and this report are named in Decisions &
corrections and changed no file.

Process hygiene: implement P4's census, `census: none running` (escher-session, cargo, seven_guis_native, stand_act_spans,
agent-run.sh). Re-measured at this wrap with the same arms: `census: none running`.

| process | started by | final state |
|---|---|---|
| `escher-session` hosts (gate entries 5, 6, 15; the operator pass's `fast`) | a leg | terminated — none running at either census |
| `cargo` / test binaries (gates 1–19, seven controls, the operator pass) | this run / the operator pass | terminated |
| `agent-run.sh` (gates 12–13) | this run | terminated |
## New text, by line
Generated by `cites.py added` (cites v1.5); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 4b4804ba (the parent of the oldest pre-CI commit 18b50a2a) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .github/scripts/ci-leg.sh — added 5 line(s) in 1 range(s)
added: 54-58
### .github/scripts/test_ci_workflows.py — added 4 line(s) in 1 range(s)
added: 359-362
### examples/seven_guis/tests/host_log.rs — added 97 line(s) in 2 range(s)
added: 11-15 · 40-131
- 40-130 @42 «fn a_recording_sessions_clients_log_nothing_of_it_at_trace() {»
  - 45-48 «use common::{»
  - 56-61 @57 «let client = |args: &[&str]| -> Ran {»
  - 63-68 «let mut ended = |step: usize, ran: Ran| -> String {»
  - 70-80 «let started = ended(»
  - 82-85 «let _guard = Started {»
  - 86-92 «let calls: [&[&str]; 5] = [»
  - 93-101 «let answers: Vec<String> = calls»
  - 102-105 «assert!(»
  - 106-109 «assert!(»
  - 120-128 «assert!(»
### tests/blitz-tests/tests/fixtures/mod.rs — added 23 line(s) in 1 range(s)
added: 64-86
- 67-85 @68 «pub fn many_items() -> Element {»
  - 70-84 «rsx! {»
### tests/blitz-tests/tests/stand_act_spans.rs — added 14 line(s) in 4 range(s)
added: 6-8 · 243 · 685-693 · 727
### tests/blitz-tests/tests/stand_lint.rs — added 87 line(s) in 3 range(s)
added: 4-7 · 110-142 · 226-275
- 110-141 @113 «fn reading_page() -> Element {»
  - 114-140 «rsx! {»
- 226-274 @227 «fn a_clipped_or_off_screen_node_holding_text_reads_truncated_beside_it() {»
  - 228-234 «let rows = [»
  - 235-273 «for incremental in [false, true] {»
### tests/blitz-tests/tests/stand_requirement_sweep.rs — added 458 line(s) in 6 range(s)
added: 10-15 · 227-229 · 232 · 538-847 · 892-952 · 993-1069
- 538-818 @542 «const WITNESSES_020: [(&str, &[Witness]); 9] = [»
  - 543-562 «(»
  - 563-582 «(»
  - 583-647 «(»
  - 648-672 «(»
  - 673-722 «(»
  - 723-747 «(»
  - 748-762 «(»
  - 763-797 «(»
  - 798-817 «(»
- 820-832 @821 «const STAND_REPEAT_NAMED: [&str; 10] = [»
- 834-844 @835 «const STAND_REPEAT_SEVEN_GUIS: [&str; 8] = [»
- 994-1069 @995 «fn every_0_2_0_requirement_names_its_witness_inside_stand_repeat() {»
  - 1000-1006 «assert!(»
  - 1008-1010 «let Some(leg) = source(CI_LEG) else {»
  - 1011-1014 «assert!(»
  - 1015-1024 «for (index, name) in STAND_REPEAT_NAMED»
  - 1026-1068 «for (row, (requirement, witnesses)) in WITNESSES_020.iter().enumerate() {»
### tests/blitz-tests/tests/stand_snapshot_parts.rs — added 1 line(s) in 1 range(s)
added: 23
### tests/blitz-tests/tests/text_extent.rs — added 96 line(s) in 4 range(s)
added: 5-8 · 10 · 25-28 · 119-205
- 120-125 @121 «fn extent(harness: &Harness<blitz_html::HtmlDocument>, selector: &str, at: &str) -> TextExtent {»
- 127-152 @128 «fn right_to_left_text_reads_as_a_left_to_right_line_from_the_content_box() {»
  - 129-151 «for incremental in [false, true] {»
- 154-174 @155 «fn vertical_text_reads_as_a_horizontal_line() {»
  - 156-173 «for incremental in [false, true] {»
- 176-205 @177 «fn a_transformed_box_reads_its_lines_at_its_client_rect() {»
  - 178-204 «for incremental in [false, true] {»
