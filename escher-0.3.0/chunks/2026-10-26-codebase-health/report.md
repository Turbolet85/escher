# Report — 2026-10-26-codebase-health

**Chunk:** Codebase health — the lint's findings walk and the two table-asserting tests under the cognitive-complexity
ceiling of 15; test functions counted, the engine function untouched (v030-01)
**Date:** 2026-10-27
**Commits:** since `last_wrap` 2026-10-25T20:26:11Z (`git log --format='%h %s'`): `5e15b78d6` the 0.2.0 cold-agent-bar
wrap commit · `7914a166e` chore(epoch5): the Epoch 5 diagnosis and code audit · `19f9ff91c` chore(route): escher-0.3.0
working route, requirements and verification matrix (this chunk's base) · `3c04042ed` chore(2026-10-26-codebase-health):
operator pre-CI commit, for the run this chunk's verdict reads — the chunk's one commit; no fix commit after it
(`git log --format='%h %s' 19f9ff91c..HEAD` lists it alone).

## Changes (structured — detectors read this)
- **Files:** source — `packages/escher-driver/src/lint.rs`, `packages/escher-driver/src/mcp_tests.rs`,
  `tests/blitz-tests/tests/stand_requirement_sweep.rs` (the plan's three files to modify, no other; `gate.py scope`:
  `scope: clean — changed 3 · listed 3 · recorded 0`). Records — the chunk folder (`scope.md`, `research.md`,
  `plan.md`, `inputs/` I1–I3, `evidence/{complexity,counts,row-controls,operator-pass,ci}.md` — `operator-pass.md` and
  `ci.md` uncommitted at this wrap's start and riding its commit, per inputs#I3), `escher-0.3.0/verification-matrix.json`
  (v030-01 `planned → implemented`, with its ref), `escher-0.3.0/working-route.md` (the entry's `[marker]` stamp),
  `.andromeda/master-route.md` (the `pending` record), `.andromeda/friction-log.ndjson`, the phase and implement run
  dirs. Basis: `git diff --name-only 19f9ff91ca -- . ':!.andromeda/runs'`.
- **Symbols / APIs:** no public symbol, signature, verb, field, carrier, socket, port or env var added or changed.
  Private items only, each new-text range copied from `## New text, by line`:
  - `lint.rs` — `findings` (`lint.rs:247`, signature unchanged) now walks `readings` per entry (its call at
    `lint.rs:273-274`); new private `readings` (`lint.rs:280-300`, head `:282`: the entry's `(Defect, Reason)` pairs
    in push order — zero-size alone for a box with no area, else placement, then truncated, then contrast) and
    `placement` (`lint.rs:302-311`, head `:303`: the off-screen / clipped match). `findings` keeps its one non-test
    caller `Session::lint` (`pub fn lint` at `lint.rs:221`, its call of `findings` inside `:221-230`, above the first
    hunk at `:273`, unmoved). `ordered` moved `:299 → :315`, `overlaps` `:334 → :350` (heads by
    `grep -n 'fn ordered\|fn overlaps'` at HEAD), both bodies unchanged; the inline `mod tests` moved
    `:527 → :543` (the file 604 → 620 lines, `wc -l` at `19f9ff91ca` and HEAD). `Session::lint`, `flatten`,
    `overlaps`, `holds_content`, `truncated`, `contrast`, `clip_region`, `clips`, `padding_box`, `holds_text`,
    `is_text_control` untouched.
  - `mcp_tests.rs` — `the_listing_is_the_twelve_verbs_of_the_table` (`mcp_tests.rs:399`, unmoved) keeps its outer
    assertions and its row walk, its row name assert inline, calling (`mcp_tests.rs:415-416`) three new private
    `#[track_caller]` helpers: `assert_described` (`:420-432`, head `:422`), `assert_schema` (`:434-466`, head `:437`),
    `assert_property` (`:468-496`, head `:470`). Every line after `:480` of the base moved +17 (hunk `@@ -480,0 +420,78`;
    the file 786 → 803 lines); `env!("CARGO_MANIFEST_DIR")` `:633 → :650`, `env!("CARGO_PKG_VERSION")` `:247` unmoved
    (`grep -n 'env!'` at HEAD). Still 16 `#[test]` (15 on unix).
  - `stand_requirement_sweep.rs` — `every_0_2_0_requirement_names_its_witness_inside_stand_repeat`
    (`stand_requirement_sweep.rs:1016`, unmoved) keeps its table-order and leg checks and calls, per row, new private
    `#[track_caller]` `assert_0_2_0_row` (`:1050-1073`, head `:1053`), which calls `assert_0_2_0_witness`
    (`:1075-1093`, head `:1078`); the target predicate is new private `inside_stand_repeat` (head `:1097`, by `grep -n`
    at HEAD, inside the added range `:1046-1107`, the listing naming no block row for it), now holding the two
    path-prefix constants `BLITZ_TESTS` · `SEVEN_GUIS` the test held (hunk `@@ -1017,2 +1016,0`). The file 1090 → 1109
    lines. Every message unchanged.
- **Crates / modules:** none added, removed or split — escher-driver's seventeen private modules, blitz-tests' 147 `.rs`
  files and 146 targets unchanged (the guard entry: `guard: allowed 3 … changed 3 · excluded 0` · `guard: clean`).
- **Dependencies:** none — no `Cargo.toml`, `Cargo.lock` or `deny.toml` change (the guard entry).
- **Schema / config:** none.
- **Spec-master edits:** none.
- **Retired:** none.
- **Counts / qualifiers moved:**
  - escher-driver's coverage reading 4918/5019 = 97.99 % → 4930/5031 = 97.99 % (+12 counted lines, +12 covered; the
    floor 97 % holding; dioxus-native-dom 2741/3202 = 85.60 % and blitz-test-harness 952/1008 = 94.44 % unchanged;
    `coverage floor: 3 of 3 crates at or above their floors`) — implement entry `ci-leg.sh coverage`
    (`evidence/counts.md` row 15). Pattern `coverage`: 1 hit · 1 changed
    (`.andromeda/test-plan-detail/quality-gates-coverage-targets.md:16`, its last reading `4918/5019 = 97.99 %`) · 0
    no-change; leaves 2 (`.claude/rules/testing.md:48`, `.claude/docs/tests-summary.md:42`) — P2's cascade.
  - The audit's complexity population over 15: 88 → 85; the three rows gone (`findings` 16 → 7,
    the listing test 18 → 3, the sweep test 18 → 3), `clear_interaction_state_for_removed_node` 17.0 standing, each new
    helper 1–4, `overlaps` 13 unmoved (`evidence/complexity.md`). Pattern `complexity`: 1 hit · 0 changed · 1 no-change
    (`.andromeda/design-system-detail/color-palette.md:107` — `hover 152` matched by `over 15`, a colour) — no master
    site: the population is the audit run's reading (`.andromeda/runs/2026-10-25T21-43-01-code-audit/`) and the
    matrix's, stated in no master. Pattern `factored` (the factored and new functions' names): 5 hits · 0 changed · 5
    no-change — `architecture-detail/standard-contracts.md:76` (`readings` there is blitz-dom `node/select.rs`'s
    reader, another function), `standard-contracts.md:150`, `security-plan-detail/api-security.md:30`,
    `security-plan-detail/input-validation.md:54` (the `findings` result field, a true claim sharing the token),
    `test-plan-detail/test-scope-summary.md:120` (the sweep test's assertions, unchanged); leaves 4, same subjects.
  - Master citations whose cited lines moved — the line spans are the P2 citation sweep's digits (`cites.py apply`);
    per pattern:
    - Pattern `lintcite` (`lint\.rs:\d+`): 11 hits · 2 changed (`test-plan-detail/test-scope-summary.md:61` and
      `test-plan-detail/unit-test-strategy.md:18` — `packages/escher-driver/src/lint.rs:526-604`, the inline tests,
      now `:542-620` by the hunk `@@ -296,0 +280,33` net +16) · 9 no-change: `architecture-detail/standard-contracts.md:188`
      (its `src/lint.rs` spans `:190-230` · `:176-187` · `:95-139` · `:141-174` · `:16-27` all above the first hunk at
      `:273`); `standard-contracts.md:196`, `layout-templates.md:54`, `test-scope-summary.md:123` · `:124` · `:125`,
      `a11y-plan-detail/visual-design-verification.md:14` · `:15` (each hit is a `tests/blitz-tests/tests/stand_lint.rs:`
      citation — the pattern's token hazard; `stand_lint.rs` unedited); `obs-plan.md:93` (`lint.rs:176-230`, above the
      first hunk).
    - Pattern `mcptestcite` (`mcp_tests\.rs:\d+`): 5 hits · 4 changed (`security-plan.md:295` `:633-634` → `:650-651`;
      `test-scope-summary.md:60` `:754-785` → `:771-802`; `test-scope-summary.md:62` `:1-786` → `:1-803`;
      `unit-test-strategy.md:18` `:615-717` → `:632-734`; each +17 by the hunk `@@ -480,0 +420,78`) · 1 no-change
      (`test-scope-summary.md:59` `:167-196`, above the first hunk at `:415`).
    - Pattern `sweepcite` (`stand_requirement_sweep\.rs:\d+`): 4 hits · 2 changed (`architecture-detail/existing-scopes.md:115`
      and `test-scope-summary.md:120` — `:1015-1090`, the test to the file's end, now the test plus its three helpers
      to `:1109`) · 2 no-change (`existing-scopes.md:114` `:92-218` · `:245-1013`; `test-scope-summary.md:119`
      `:92-218` · `:245-540` · `:978-1013` — each ending above the first hunk at `:1017`); the other spans on the two
      changed lines (`:231-233` · `:542-837` · `:839-863`) unmoved.
    - Pattern `envread` (`CARGO_MANIFEST_DIR`): 6 hits · 1 changed (`security-plan.md:295`, the same `mcp_tests.rs`
      site as above) · 5 no-change (`architecture-detail/occupied-resources.md:30` · `:32`,
      `standard-contracts.md:264`, `security-plan.md:299`, `obs-plan.md:89` — other readers of the variable).
  - The silence census over the touched files: the plan's entry read 0 over `lint.rs`, `mcp_tests.rs` and
    `stand_requirement_sweep.rs` (exit 1 · `0`), `mcp_tests.rs`'s first reading in a census (plan §Implementation notes).
    Pattern `silence`: 9 hits · 1 bearing (`obs-plan.md:93`, the lint's census line, «the silence census reading 0 over
    the four files» — `lint.rs`, `text_extent.rs`, `stand_lint.rs`, `text_extent` check; whether `mcp_tests.rs`'s first
    reading lands there or elsewhere is P2's) · 8 no-change (`obs-plan.md:78` · `:90` · `:91` · `:92` · `:96` · `:97`,
    `obs-plan-detail/log-coverage.md:123` · `:124` — other subjects' censuses).
  - Unchanged counts (re-read, no site moves): escher-driver 100 unit tests, `mcp_tests.rs` 15 on unix,
    `stand_requirement_sweep` 3, `stand_lint` 7, stand checks 205 · 0 · 7, a11y 14 · 85 · 0 · 0, stand-repeat 204 · 864 ·
    0 · 21, workspace 208 · 1117 · 0 · 12, CI scripts 108, escher-driver `info_span!` 2 (`evidence/counts.md`).
  - Sweep reach: 11 patterns · 8 moved facts, 8 with a pattern (the report's own count: the lint.rs test-module
    citation, the `mcp_tests.rs` citations, the sweep-test citation, the `env!` line, the coverage reading, the
    complexity population and per-function readings, the silence census's new file, the sweep test's reach) · owner
    pattern control unfired (`title · control UNFIRED over the seven masters (bodies and sets) at 3c04042e`) · by line
    1 lines · 1 offsets. Basis: `cascade.py sweep --pre-pass`, trail
    `.andromeda/runs/2026-10-27T02-06-43-wrap/cascade-2026-10-26-codebase-health-pre-pass.json`, `total (11 patterns) ·
    129 rows over 36 files`. The by-line row (`test-plan-detail/ci-integration.md:42` @c1094 `sweeptest`, a per-round
    test count) — no-change. Pattern `ownerphrase`: 1 hit · 0 changed (`security-plan-detail/input-validation.md:197`,
    another subject's owner). A listing reaches its patterns' wording only; the detectors' own sweeps stay the reach
    control.
- **Dev-tool versions:** none — `rust-code-analysis-cli` (the complexity reading's analyser) re-read at 0.0.25 on the
  dev host (`~/.cargo/bin`, installed 2026-10-03, before this chunk); no lockfile resolves it.
- **Harness / gate surface:** none changed — no agent-run script, CI step or leg edited (`ci-leg.sh`, `listing.rs`,
  `VERBS` and the committed checks byte-unchanged: the guard entry and `evidence/row-controls.md`'s sha256 table). The
  plan's complexity entry is a dev-host reading, no CI leg.
- **Cross-project / external claims:**
  - CI#40434241601 (push) on `3c04042ed096` — `verdict: green · checks 17/17 · wall 1078 s`; its six libtest jobs'
    tallies equal the base run CI#40389488295 on `19f9ff91ca` (Test [default features] 208 · 1117 · 0 · 12 · Test
    (macos) 189 · 1114 · 0 · 12 · Coverage report 189 · 1114 · 0 · 12 · Test (windows) 189 · 1068 · 0 · 9 · a11y 14 · 85 ·
    0 · 0 · Stand checks repeated 204 · 864 · 0 · 21) — `evidence/ci.md`. The sha measured is HEAD at this wrap's start.
  - `inputs.py verify` (this wrap, after the I3 snap): `inputs: 3 entries — unchanged 1 · drifted 0 · vanished 0 ·
    broken 0 · altered 0 · unreachable 0 · n/a 2 · uncited 1 · unparsed 0` — the uncited entry I3, snapped at this
    wrap and cited here:
    - I1 · `../additional/escher-overseer/relays/route-0.3.0-founder-scope-2026-10-26.md` · copy no-repo · unchanged —
      the founder's M1 «Код escher, тесты таблицами (Рекомендую)».
    - I2 · message: the operator, 2026-10-26 23:36Z · copy message · n/a — the take-up directive.
    - inputs#I3 · message: the operator, in the escher session, 2026-10-27 02:06Z · copy message · n/a — this wrap's
      directive: P1 only, then stop (resume at P2); `operator-pass.md` and `ci.md` ride the wrap commit; two implement
      notes need a wrap owner (the sweep test's reach; the moved line citations); forks to the operator as a dialog;
      transcripts and run dir kept.
  - No inputs entry names a P5 review card.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - The sweep test's reach. `architecture-detail/existing-scopes.md:115` states «every other witness found inside
    `stand-repeat` by the `stand_` prefix or by the named `--test` list of `ci-leg.sh`'s own text»;
    `a11y-plan-detail/ci-integration.md:9` states the stand-repeat list's names «their membership held by
    `stand_requirement_sweep`'s leg check»; `architecture-detail/standard-contracts.md:273` «read from the script's own
    text by `stand_requirement_sweep`'s leg check». Measured at implement's row-guard control (a)
    (`evidence/row-controls.md`, `outcome-…md` §Deviations): the test reads `ci-leg.sh` whole by `contains`, so
    `--test select_interaction` dropped from `stand-repeat` alone (`ci-leg.sh:57`) leaves it green while the `a11y`
    leg (`ci-leg.sh:39`) still names it; only both sites removed read red (`named target 8: the leg runs it`). The
    property predates this chunk (the leg check is unchanged by the factoring). Read, not run, at this wrap:
    `.github/scripts/test_ci_workflows.py:361-366` (`test_stand_repeat_leg_runs_three_rounds`) asserts
    `select_interaction` among the `stand-repeat` leg's own logged words, so stand-repeat membership is held by the two
    checks jointly, not by the sweep test alone. `test-plan-detail/test-scope-summary.md:120` («`ci-leg.sh` … holding
    the … glob and `--test {name}` for all 18») states the read exactly. Patterns `sweeptest` (23 hits · 3 bearing,
    the three lines above · 1 exact, `test-scope-summary.md:120` · 19 no-change — counts, names and history) and
    `selint` (25 hits · 0 bearing · 25 no-change — each names `select_interaction` as a target, a leg member or a
    count). Disposition: Validate check 6; the test's reach gap itself needs an owner (inputs#I3) — P5's (see Outcome).
- **Expected amendments (from plan):** (each searched by the site sweep above; every hit in a master or its category
  file — no `.andromeda/registries/` key-file hit for `lintcite` · `mcptestcite` · `sweepcite` · `envread` · `coverage`
  · `silence`)
  - test-plan §1 / `test-scope-summary.md` + `unit-test-strategy.md:18` line spans — carried, partly: the moved spans
    are `lint.rs:526-604` (2 sites), `mcp_tests.rs:754-785` · `:1-786` · `:615-717` (3 sites) and
    `stand_requirement_sweep.rs:1015-1090` (`test-scope-summary.md:120`); `mcp_tests.rs:167-196` and the sweep file's
    other spans unmoved. Not carried: the plan's other listed `lint.rs` spans (`:27-109` · `:111-142` · `:174-413` ·
    `:227-275` · `:415-442`) are `stand_lint.rs` citations (`test-scope-summary.md:123-125`), a file this chunk did not
    edit. Patterns `lintcite`, `mcptestcite`, `sweepcite` (Counts / qualifiers moved).
  - architecture `standard-contracts.md:188,196` + `existing-scopes.md:114-115` — carried for `existing-scopes.md:115`
    (`:1015-1090`); not carried for `standard-contracts.md:188` (its `lint.rs` spans above the first hunk),
    `standard-contracts.md:196` (`stand_lint.rs` only) and `existing-scopes.md:114` (spans above the first hunk).
    Patterns `lintcite`, `sweepcite`.
  - security-plan §Secret Management `security-plan.md:295` (`mcp_tests.rs:633-634`) — carried: the line moved,
    `:650-651`. Patterns `mcptestcite`, `envread`.
  - obs-plan §3 `obs-plan.md:93` (`lint.rs:176-230`) — not carried for the span (above the first hunk, unmoved);
    `mcp_tests.rs` entering the silence census — carried (Counts / qualifiers moved, the census bullet). Patterns
    `lintcite`, `silence`.
  - layout-templates `layout-templates.md:54` — not carried: its hits are `stand_lint.rs` citations, unedited. Pattern
    `lintcite`.
  - a11y-plan §6 `visual-design-verification.md:14-15` — not carried: `stand_lint.rs` citations only. Pattern
    `lintcite`.
  - test-plan §10 escher-driver's coverage reading — carried (Counts / qualifiers moved, 4930/5031 = 97.99 %). Pattern
    `coverage`.
- **Coverage of new surfaces:**
  - `readings` · `placement` (private, `lint.rs`) → validation n/a (no input crosses; reads stay through `Option` /
    `get_node`) · instrumentation n/a (silence census 0; `info_span!` 2, no event) · PII n/a · tests integ
    (`stand_lint` 7 both layout modes, both builds; control (e) red then green) · a11y n/a (the contrast arm's reader
    `contrast` unchanged) · tokens n/a
  - `assert_described` · `assert_schema` · `assert_property` · `assert_0_2_0_row` · `assert_0_2_0_witness` ·
    `inside_stand_repeat` (private test helpers) → validation n/a · instrumentation n/a · PII n/a (messages name row and
    index only) · tests unit/integ (controls (a)–(d) red then green) · a11y n/a · tokens n/a

## Deviations from intent
- Step 4 control (a): `ci-leg.sh` names `--test select_interaction` twice (`a11y` `:39`, `stand-repeat` `:57`);
  removing the stand-repeat site alone leaves the sweep test green, so the control removed both — the plan's «removed
  from `ci-leg.sh`» read literally. The guard's reach is the disproved claim above; no change made (predates the
  chunk, out of scope).
- Step 5: the complexity entry also prints `packages/blitz-dom/src/document.rs get_text_selection_ranges 32.0`
  (`document.rs` is a named file); over the ceiling at the base too, untouched, counted in both 88 and 85, read by no
  atom.
- The plan's expected-amendment list named six `lint.rs` spans in `test-scope-summary.md` of which five are
  `stand_lint.rs` spans (the `lint\.rs:` token hazard) — dispositioned in Expected amendments; no deviation of the
  work.
- scope record: none — `gate.py scope` clean, 0 recorded (`scope: clean — changed 3 · listed 3 · recorded 0 (companion
  0 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 565`).

## Decisions & corrections
- The operator's wrap directive (inputs#I3): the two-window form — P1 here, P2–P7 at a resume in
  `.andromeda/runs/2026-10-27T02-06-43-wrap/`; `evidence/operator-pass.md` and `evidence/ci.md` ride the wrap commit;
  the founder asleep, every fork or card to the operator as a dialog; every transcript and the run dir kept.
- The operator named two implement notes needing a wrap owner (inputs#I3): (1) the sweep test cannot see
  `select_interaction` dropped from stand-repeat alone — P2 disposes the three master sentences (Spec claims disproved),
  P5 places the reach gap's owner; (2) the line citations into the three files moved (e.g. `mcp_tests.rs:633 → :650`) —
  P2's citation sweep, the sites listed in Counts / qualifiers moved.
- Sweep hazards met at this report's site sweep: `lint\.rs:\d+` also matches `stand_lint.rs:` citations (9 of 11 rows
  were `stand_lint.rs`, and the plan's expected-amendment list carried five of them as `lint.rs` spans); `over 15`
  (ignore-case) matches `hover 152` in a colour row; `` `readings` `` names both the new `lint.rs` helper and blitz-dom
  `node/select.rs`'s reader. Anchor a file citation pattern on its path's directory (`src/lint\.rs:`) and a count on a
  word boundary.
- A TOML literal string cannot hold a `'`: the patterns file's `escher-driver('s)?` alternative refused the sweep
  (exit 2, `not TOML`), re-written as `'''…'''` as the letter says.
- The Bash guard's `cd` arm refused two of this wrap's reads (`cd` into a chunk dir and into `.andromeda/`); each
  re-issued once as a `( cd … )` subshell.
- From implement's outcome (`.andromeda/runs/2026-10-27T01-14-55-implement/outcome-2026-10-26-codebase-health.md`
  §Decisions & corrections): overseer1's directive that the phase's friction line `2026-10-27T00:38:38Z-d` carry
  «tools moved mid-run, inert (overseer1, F362)», done by a validated python rewrite of that one ledger line because
  `evolve.py` only appends (recorded as `problem`: process · overridden); overseer1's «if a gate is red with no
  assertion failing on a value, stop and say so before a re-run» — no gate red; the Bash guard's `cd` arm refused one
  Setup read, re-issued with absolute paths; standing learnings held (grep exit codes with `command grep`, the
  per-package `Intrinsic` warning not the change's, the hand-mutation restore rule with sha256 compare); the handoff at
  session start stale (Branch `build/escher-0.2.0`, Status clean, while the tree was on `build/escher-0.3.0`) — the
  wrap's P6 refreshes it.
- A rust-code-analysis fact the plan's form rests on (research.md §Patterns detected): `cognitive.sum` aggregates
  nested spaces, so a closure or nested `fn` keeps its branches counted in its parent — only module-level sibling items
  lower a reading (`findings` 16 over 3 spaces, each closure 0).

## Outcome
Acceptance criteria, re-asserted against the diff (three files, private items only):
- v030-01 — the complexity reading lists none of the three over 15, no new item listed (each 1–4), `over 15: 85`
  (88 at base), `clear_interaction_state_for_removed_node 17.0` standing — **met** (entry 1; `evidence/complexity.md`).
  The two tests keep names and targets and are counted; controls (a)–(e) red then green restored, sha256-equal —
  **met** (`evidence/row-controls.md`). Nothing under `packages/blitz-dom/` changes — **met** (the guard entry). Every
  check green — **met**: `fast`, `doc`, `a11y`, `stand-repeat`, `coverage` green at implement, `fast` again at the
  operator pass, CI#40434241601 green 17/17 on HEAD.
- `Session::lint`'s findings and order and its `fonts-unavailable` refusal — **met**: the diff leaves `Session::lint`
  and every reader untouched and keeps the overlap pushes first; `stand_lint` 7 green under both builds (entries 3–4),
  `[0, 1, 0, 0]` at boot, the planted seven, the twelfth-Create `clipped` row; `stand_cells_served`, `stand_load_page`,
  `stand_replay` green.
- The listing test's twelve tools in `VERBS` order with every assertion; `mcp_tests.rs` 15 and escher-driver 100 —
  **met** (entry 2; controls (c), (d)).
- The 0.2.0 sweep test's nine rows, witnesses, stand-repeat checks; 3 tests; blitz-tests 147 `.rs` / 146 targets —
  **met** (entries 3, 16; no file added — the guard). Its reach is as before the chunk (Spec claims disproved).
- obs — census 0, `info_span!` 2, no event or `#[instrument]`; `stand_act_spans`, `stand_mcp_tools`, `host_log`,
  `mcp_log` green — **met** (entries 3–9).
- a11y — the contrast arm through `Harness::text_contrast` and `required_ratio()`, the planted row 2.8490272 against 4.5
  (`stand_lint` green), the a11y leg 14 · 85 · 0 · 0 — **met** (entries 3, 13).
- security — no manifest, lock or `deny.toml` change; `env!` 2 in `mcp_tests.rs`; no `unwrap`/`expect`/`panic!` added in
  `lint.rs` (the diff adds none) — **met** (entries 7, 10).
- CI on the pushed sha `verdict: green`, per-job tallies equal CI#40389488295's — **met** (`evidence/ci.md`).

Gates (implement run `.andromeda/runs/2026-10-27T01-14-55-implement/`, `gate-2026-10-26-codebase-health.json`:
`entries 22 · green 18 · red 0 · recorded 0 · timeout 0 · not-run 4`; no deferral — Rust carries the delta):
- the complexity reading (`rust-code-analysis-cli … over 15`) — green (`over 15: 85`, `files read 429`, the engine row
  17.0, the three rows absent)
- `cargo test -p escher-driver --locked` — green (100 · 0 · 0)
- `cargo test -p blitz-tests --locked --test stand_lint … --test stand_replay` — green (27 · 0 · 2 ignored)
- `cargo test --workspace --locked --test stand_lint … --test stand_act_spans` — green (18 · 0 · 2 ignored)
- `cargo test -p seven_guis --locked --test host_log … --test recording` — green (30 · 0 · 0)
- the silence census `grep -c -E 'println!|…'` over the three files — green (exit 1 · `0`)
- `grep -c 'env!' …mcp_tests.rs` — green (`2`)
- `git grep -c -E 'info_span!' … | awk` — green (`2`)
- `git grep -c -E '#\[instrument|…' … | wc -l` — green (exit 1 · `0`)
- `gate.py guard … --allow '…'` — green (`guard: clean`)
- `bash scripts/agent-run.sh boot` — green (`"outcome": "ready"`)
- `bash scripts/agent-run.sh run stand` — green (205 · 0 · 7)
- `bash .github/scripts/ci-leg.sh a11y` — green (14 · 85 · 0 · 0)
- `bash .github/scripts/ci-leg.sh stand-repeat` — green (`stand-repeat: 3 of 3 rounds green`, 204 · 864 · 0 · 21)
- `bash .github/scripts/ci-leg.sh coverage` — green (`coverage floor: 3 of 3 crates at or above their floors`)
- `bash .github/scripts/ci-leg.sh fast` — green (208 · 1117 · 0 · 12; CI scripts 108 OK)
- the workspace tally over `target/ci-logs/test.log` — green (`lines 208 passed 1117 failed 0 ignored 12`)
- `bash .github/scripts/ci-leg.sh doc` — green
- `gate.py hygiene` (`leg = 'operator'`) — green, `hygiene: clean — read 552 …` (`evidence/operator-pass.md`)
- `ci-leg.sh fast && git diff --quiet && … && git push origin build/escher-0.3.0` (`leg = 'operator'`) — green, exit 0,
  the push `19f9ff91 → 3c04042e` (`evidence/operator-pass.md`)
- `ci.py conclusion --sha HEAD --wait 1800` (`leg = 'operator'`) — green, `3c04042ed096 verdict: green · checks 17/17`
  (`evidence/ci.md`)
- `gh run view <id> … | awk …` (`leg = 'operator'`, `expect = []`) — recorded, exit 0: CI#40434241601's six job tallies
  equal the base run's (`evidence/ci.md`) — outcome green, the operator pass's, no red to dispose.
- Smoke: the agent-run boot/run pair, green.

Owners this report names for later phases:
- P2 — the citation digits listed under Counts / qualifiers moved (the citation sweep); the coverage reading at
  `quality-gates-coverage-targets.md:16`; `mcp_tests.rs` in the silence census (`obs-plan.md:93`); the three
  sweep-reach sentences (Validate check 6).
- P5 — an owner for the sweep test's reach gap (predates this chunk; held jointly with
  `test_stand_repeat_leg_runs_three_rounds` today): a `.andromeda/residuals.md` line or a route CARRY, the operator's
  word asked where the placement is a fork (inputs#I3).
- P6 — the handoff refresh (the stale Branch line).

Watches: none.

Outcome basis: the operator pass ran — the pre-CI commit `3c04042ed`, no fix commit after it
(`git log --format='%h %s' 19f9ff91c..HEAD`); the final HEAD's CI run CI#40434241601, recorded in `evidence/ci.md`;
implement's persisted report `.andromeda/runs/2026-10-27T01-14-55-implement/outcome-2026-10-26-codebase-health.md`
(the only implement run dir for this marker), read whole for what only it holds (deviations, the per-function
readings, the census); the operator pass's records `evidence/operator-pass.md`. No plan revision between implement and
this report.

Process hygiene: implement's census — `census: none running` for cargo, rustc, `escher-session`,
`rust-code-analysis-cli` and the test binaries; re-measured at this wrap — `gate.py census --comm escher-session --comm
cargo --comm rustc --comm rust-code-analysis-cli --args stand_requirement_sweep --args stand_lint --args
target/debug/deps --args target/llvm-cov-target --args agent-run.sh`: `census: none running`.
## Addendum — resumed 2026-10-27
- inputs#I4 · message: the operator, in the escher session, 2026-10-27 02:18Z · copy message · n/a — the resume
  directive: P2 on in this run dir (the resume line's `2026-10-27T02-03-18-wrap` corrected by the operator to
  `2026-10-27T02-06-43-wrap`), every batch as one Workflow, every transcript and the run dir kept, forks and cards to
  the operator as a dialog, a hard cascade-check failure stops the wrap
  - bears on **Cross-project / external claims**: adds to it
- inputs.py verify, re-fired: `inputs: 4 entries — unchanged 1 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 3 · uncited 1 · unparsed 0`; moved since the report: uncited 1 → 0

## New text, by line
Generated by `cites.py added` (cites v1.7); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 19f9ff91 (the parent of the oldest pre-CI commit 3c04042e) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### packages/escher-driver/src/lint.rs — added 35 line(s) in 2 range(s)
added: 273-274 · 280-312
- 280-300 @282 «fn readings(»
  - 287-292 «if !entry.bounds.has_area() {»
  - 295-299 «placement(entry)»
- 302-311 @303 «fn placement(entry: &Entry) -> Option<(Defect, Reason)> {»
  - 304-310 «match (entry.region, entry.visible) {»
### packages/escher-driver/src/mcp_tests.rs — added 80 line(s) in 2 range(s)
added: 415-416 · 420-497
- 420-432 @422 «fn assert_described(row: usize, listed_tool: &Value, verb: &VerbSpec) {»
  - 423-425 «let Some(Value::String(described)) = listed_tool.get("description") else {»
  - 428-431 «for field in verb.fields {»
- 434-466 @437 «fn assert_schema(row: usize, schema: &Value, verb: &VerbSpec) {»
  - 441-443 «let Some(Value::Object(properties)) = schema.get("properties") else {»
  - 447-450 «assert!(»
  - 453-460 «let order: Vec<usize> = names»
  - 463-465 «for ((name, kind), arg) in names.iter().zip(&types).zip(verb.args) {»
- 468-496 @470 «fn assert_property(row: usize, property: &Value, kind: &str, arg: &ArgSpec) {»
  - 472-475 «assert!(»
  - 476-480 «let listed_enum = match arg.kind {»
  - 481-484 «match listed_enum {»
  - 485-489 «let maximum = match arg.kind {»
  - 491-495 «assert!(»
### tests/blitz-tests/tests/stand_requirement_sweep.rs — added 62 line(s) in 1 range(s)
added: 1046-1107
- 1050-1073 @1053 «fn assert_0_2_0_row(row: usize, requirement: &str, witnesses: &[Witness]) {»
  - 1055-1058 «let units = witnesses»
  - 1059-1069 «if ["v020-01", "v020-02"].contains(&requirement) {»
  - 1070-1072 «for (index, witness) in witnesses.iter().enumerate() {»
- 1075-1093 @1078 «fn assert_0_2_0_witness(row: usize, index: usize, &(subject, file, test): &Witness) {»
  - 1079-1081 «let Some(text) = source(file) else {»
  - 1082-1085 «assert!(»
  - 1086-1088 «if subject == Unit {»
  - 1089-1092 «assert!(»
