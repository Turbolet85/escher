# Report — 2026-10-16-fast-failing-mutants

**Chunk:** the six timeout-only mutants each caught by a fast-failing test, a scoped mutation run at 0 survivors (v020-02)
**Date:** 2026-10-17
**Commits (since last_wrap 2026-10-16T17:52:00Z, from the base `494abd25`):**
- `79cfb5d1` chore(2026-10-16-fast-failing-mutants): operator pre-CI commit, for the run this chunk verdict reads
- `f072cd61` fix(deps): ignore the zip extraction advisories by ID

## Changes (structured — detectors read this)

- **Files** (`git diff --name-status 494abd25` plus the untracked files, run dirs aside):
  - Code: `examples/seven_guis/src/tasks/timer.rs` (M), `packages/escher-driver/src/mcp_tests.rs` (M),
    `packages/dioxus-native-dom/src/dioxus_document_tests.rs` (M).
  - Config: `.config/nextest.toml` (A, new), `deny.toml` (M), `.gitignore` (M).
  - Ledgers and route: `.andromeda/residuals.md` (M, one `open` line appended at this wrap, inputs#I4),
    `escher-0.2.0/verification-matrix.json` (M), `escher-0.2.0/working-route.md` (M, the take-up marker),
    `.andromeda/master-route.md` (M, the `pending` record), `.andromeda/friction-log.ndjson` (M),
    `.claude/session-handoff.md` (M).
  - Chunk folder: `plan.md`, `research.md`, `scope.md`, `scope-record.md`, `inputs/` (I1–I4), and `evidence/`
    (`controls.md`, `mutants-{timer,framing,writer}.json`, `operator-mutation-runs.txt`, `operator-pass.md`).
  - Run dirs: `2026-10-16T18-14-30-phase`, `2026-10-16T20-04-49-implement`, `2026-10-16T20-13-52-phase`,
    `2026-10-16T20-26-29-implement`, this wrap's `2026-10-17T07-59-25-wrap`, and the previous wrap's
    `2026-10-16T17-27-46-wrap` (its evolve trail).
- **Symbols / APIs:** no public symbol, signature, verb, field, cause, span field, listener, port or env var is added or
  changed. Three `#[test]` functions are added in `cfg(test)` code. Coordinates are from §New text, by line.
  - `tasks::timer::tests::a_delivered_tick_is_taken_once_and_an_empty_wait_parks`: a new `#[cfg(test)] mod tests` at
    `examples/seven_guis/src/tasks/timer.rs:197-226` (the module 198-226, the test 206-225), after the `CSS` constant;
    lines 1–196 are byte-equal to the base. It polls the private `TimerTicks::next()` under `Waker::noop()` over a
    3-row table, asserting the row count. `TimerTicks::next` keeps its one caller, `Timer`.
  - `mcp::tests::read_request_takes_each_line_with_its_terminator`: `packages/escher-driver/src/mcp_tests.rs:167-196`.
    It calls `super::framing::read_request` (`pub(super)`, unchanged) over two readers, the slice and a
    `BufReader::with_capacity(1, …)`; each reader makes at most count + 1 reads, and the count is asserted.
    `read_request` keeps its one caller, `serve_mcp`.
  - `dioxus_document::bridge_tests::a_created_text_node_is_mapped_and_left_on_the_stack`:
    `packages/dioxus-native-dom/src/dioxus_document_tests.rs:147-166`. It calls
    `WriteMutations::create_text_node` on a `MutationWriter` over the document's private fields and reads the stack
    growth, the id mapping and the node's text. The edit also adds one `//!` clause (lines 4–6) and the
    `WriteMutations` import (line 12). `mutation_writer.rs` is byte-identical to upstream's at `434e0661`.
- **Crates / modules:** no crate is added or removed. `seven_guis` gains `tasks::timer::tests`, its library's second
  unit test. `dioxus_document::bridge_tests` goes from 6 tests to 7.
- **Dependencies:** none added or bumped. `Cargo.toml`, `Cargo.lock` and `.github/` are unchanged since the base
  (`git diff --quiet 494abd25 -- Cargo.lock Cargo.toml .github`: exit 0).
- **Schema / config:**
  - `.config/nextest.toml` (new, 13 lines) holds one profile, `mutants`, and leaves the default profile untouched:
    - `fail-fast = { max-fail = 1, terminate = "immediate" }`, `test-threads = 4`;
    - one override at `priority = 100` naming the three direct tests.
    - It is read by cargo-nextest under `NEXTEST_PROFILE=mutants`, a host tool's variable that workspace source never
      reads.
  - `deny.toml` `[advisories] ignore` grows from 2 entries to 5. RUSTSEC-2026-0367, RUSTSEC-2026-0368 and
    RUSTSEC-2026-0369 are added at lines 23–25, per ID, each with the reason: zip 2.4.2 reaches the graph only through
    anyrender_serialize 0.8.0 (apps/browser capture feature only), which reads entries by_name into memory and never
    extracts to disk; no fixed release is reachable (anyrender_serialize 0.8.0 is the latest, requiring zip ^2.1; the
    fix is in zip 9.0); re-checked by the audit leg on every push. This is on the founder's word, inputs#I3.
  - `.gitignore:37-38`: `escher-0.3.0-incubator/` under the Future-version incubator block, with a one-line comment
    in the 0.2.0 form, on the operator's word (inputs#I4).
- **Spec-master edits:** none.
- **Counts / qualifiers moved** (each count measured by this chunk's own gate entries, whose exact-count `expect`
  atoms held at implement's block run and again before the push; every site from the pre-pass sweep, trail
  `cascade-2026-10-16-fast-failing-mutants-pre-pass.json`):
  - The seven_guis library's unit tests go from 1 to 2. `cargo test -p seven_guis --locked --lib` reads
    `2 passed; 0 failed`. Pattern `sgui-lib`, 3 standing, 2 changed and 1 no-change:
    - `test-plan.md:10` ×2 (@c6031 "the library's first unit test", @c6073 its name) — changed: a second library
      test joins, and "first" stays true;
    - `registries/contracts/test-plan/session-lifecycle.md:11` ("seven_guis' library unit test 1") — changed, 1 → 2;
    - `test-plan.md:331` — no-change, a dated §9 tally of an earlier chunk.
  - escher-driver's unit tests go from 73 to 74. `cargo test -p escher-driver --locked` reads `74 passed`. Pattern
    `drv-unit`, 2 standing, 2 changed: `test-plan.md:21` (§1, "73 unit tests in thirteen files, 56 inline") and
    `session-lifecycle.md:11` ("escher-driver's 73 unit tests"). Its 2 leaf rows go to P2: `.claude/docs/services/escher-driver.md:60`
    and `.claude/docs/tests-summary.md:24`.
  - `mcp_tests.rs` goes from 11 tests to 12, the file from 624 lines to 655 (`wc -l`). Pattern `mcp-tests`, 2
    standing over 3 sites:
    - `test-plan.md:21` @c4802 ("mcp_tests.rs 11") — changed;
    - `test-plan.md:21` @c7423 (the citation `packages/escher-driver/src/mcp_tests.rs:1-624`) — the file now ends at
      655, so this is P2's citation sweep;
    - `session-lifecycle.md:11` ("mcp_tests.rs 11") — changed.
    - Its 2 leaf rows go to P2: `services/escher-driver.md:60` and `tests-summary.md:24`.
  - dioxus-native-dom's unit tests go from 68 to 69. `cargo test -p dioxus-native-dom --locked --lib` reads
    `69 passed`. Pattern `dnd-unit`, 1 standing, changed: `test-plan.md:24` ("68 unit tests in nine files").
  - `bridge_tests` goes from 6 tests to 7. Pattern `bridge-six`, 1 standing, changed: `architecture.md:254` ("six unit
    tests of the bridge"; this is arch §Existing Scopes, dioxus-native-dom's row).
  - The cargo-deny ignores go from 2 to 5. Pattern `zip-ign`, 1 standing, changed: `security-plan.md:237` ×2
    (§Dependency Security, @c1023/@c1042, one clause, "Two ignores stand. RUSTSEC-2026-0192 …"). Its 2 leaf rows go to
    P2: `.claude/rules/security.md:26` ×2 and `.claude/docs/security-summary.md:42`.
  - A mutation-tier recipe and cargo-nextest as a host instrument are new. Pattern `mut-tier`, 1 standing,
    no-change: `architecture.md:210` ("0 surviving mutants of 1,558", 0.1.0's audit as recorded). test-plan §10 holds
    no mutation-tier line, so its text is new (Expected amendments).
  - `.config/nextest.toml` in the directory tree is new. Pattern `tree-cfg`, 5 standing, all no-change: the root
    `deny.toml` mentions at `architecture.md:67` @c1155, `architecture.md:159`, `security-plan.md:237` @c471,
    `security-plan.md:268` and `security-plan.md:277`. 0 rows name `.config/`, so the tree line is new.
  - The `.gitignore` incubator line is new. Pattern `incubator`: its control is unfired over the seven masters
    (`control: incubator, title never fired over the seven masters' pre-pass text (f072cd61)`), so no master states
    the incubator lines.
  - The residuals ledger gains one `open` line, `.andromeda/residuals.md:29`. No master site — a ledger entry
    (route-resolve's file).
  - Owner pattern `owner-gen` (`owned by a (route entry|CARRY)`): 1 standing, `security-plan.md:124`, an id's owner and
    not this chunk's fact; 2 base rows in `playbook.md`.
  - Sweep reach: 9 patterns · 11 moved facts, 10 with a pattern (the report's own count) · owner pattern control
    unfired · by line 3 lines · 3 offsets.
- **Dev-tool versions:** none changed. cargo-nextest was re-read at 0.9.146 and cargo-mutants at 27.1.0 on the dev
  host (`cargo nextest --version` and `cargo mutants --version` at implement's start, 2026-10-16T20:27Z). Research
  measured cargo-nextest as already available. Neither is lockfile-resolved, and neither runs in CI. Their role nouns:
  cargo-mutants is the mutation tier's tool, and cargo-nextest its test tool (the "instrument").
- **Harness / gate surface:**
  - The mutation tier's recipe for the three files, as built and run: cargo-mutants 27.1.0 with
    `--test-tool nextest` under `.config/nextest.toml`'s `mutants` profile, and `--baseline=skip`. Each unit's mutant
    test command is first read green directly on the unmutated tree (`NEXTEST_PROFILE=mutants cargo nextest run
    --locked`, a `-p` per package on the unit's list line, then `--lib` and its `--test=` targets).
  - Why the baseline is skipped: cargo-mutants 27.1.0's baseline tests only the mutated package but passes every
    `--cargo-test-arg` (`lab.rs:179-190`, `cargo.rs:160-162`; research.md §Patterns detected).
  - CI workflows, the CI legs and `agent-run.sh` are unchanged (`.github/` unchanged).
- **Cross-project / external claims:**
  - inputs I1 · message, the operator, 2026-10-16 18:13Z · copy message · n/a — the mutation-run limits; cited in
    scope, research and plan.
  - inputs I2 · message, the operator, 2026-10-16 20:13Z · copy message · n/a — implement stopped, the plan is
    revised; cited in scope, research and plan.
  - inputs#I3 · message, the operator, 2026-10-17 07:40Z · copy message · n/a — the founder's «Игнор по ID
    (Рекомендую)» (07:39Z, dialog) and the fix-push directive.
  - inputs#I4 · message, the operator, 2026-10-17 07:59Z · copy message · n/a — this wrap's directive: the two edits
    before the new-text listing, then P1 alone.
  - `inputs.py verify`: `inputs: 4 entries — unchanged 0 · drifted 0 · vanished 0 · broken 0 · altered 0 ·
    unreachable 0 · n/a 4 · uncited 2 · unparsed 0`. It was fired before this report existed; I3 and I4 are cited
    here. No drift.
  - P5 review card: no inputs entry names one. The revision's card, `.andromeda/runs/2026-10-16T20-13-52-phase/p5-card-1.md`,
    reads `## Beyond the plan: none`.
  - CI runs on `Turbolet85/escher`:
    - CI#39013904578 on `79cfb5d1`: `verdict: red`. The Dependency audit failed with `advisories FAILED, licenses ok`:
      RUSTSEC-2026-0367/0368/0369 on zip 2.4.2 (`Cargo.lock:912`), path `zip ← anyrender_serialize 0.8.0 ←
      browser`.
    - CI#39014589486 on `f072cd61`: `verdict: green`, checks 17/17, wall 888 s.
    - The base CI#38943566949 on `494abd25`: `success`.
    - All three are recorded in `evidence/operator-pass.md`.
  - The RustSec advisory database: the three advisories were new between the base's green run (2026-10-16T17:56Z) and
    the push (2026-10-17T07:29Z), over a byte-identical `Cargo.lock` and `deny.toml`.
  - cargo-mutants 27.1.0's source (`lab.rs`, `cargo.rs`, `outcome.rs`) was read from the local registry copy by research
    at the revision.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** the three direct tests catch the six mutants fast only under
  nextest's `mutants` profile.
  - Under plain `cargo test`, the six still read `timeout`: cargo runs the hung blitz-tests integration binaries
    (`stand_accessibility_ids`, `stand_act_diff`, `stand_mcp_masked`, `harness_smoke`) before the library tests
    (plan §Constraints; research.md §Patterns detected).
  - Owner: test-plan §10's mutation-tier recipe, so that the next audit runs with the profile (Expected amendments).
- **Spec claims disproved by measurement:** none in the masters. One plan claim was measured false and is already
  disposed:
  - The original plan step 7 ran cargo-mutants' default baseline with the audit's per-unit test set.
  - The stopped run measured it impossible: the seven_guis baseline was refused `no test target named
    font_face_sniffed_source`, cargo-mutants exited 4, and no mutant ran (`.andromeda/runs/2026-10-16T20-04-49-implement/outcome-2026-10-16-fast-failing-mutants.md`).
  - The plan revision absorbed it: `--baseline=skip` plus three direct baseline entries
    (`.andromeda/runs/2026-10-16T20-13-52-phase/`). v020-02's acceptance was re-concretized there, on the operator's
    word.
- **Expected amendments (from plan):**
  - test-plan §1, §3 session-proof, §4 — the counts (seven_guis library 1 → 2, escher-driver 73 → 74 with
    `mcp_tests.rs` 11 → 12, dioxus-native-dom 68 → 69 with `bridge_tests` 6 → 7) — carried: Counts / qualifiers moved.
    - Hits by pattern: `sgui-lib` (test-plan 2, its session-lifecycle key file 1), `drv-unit` (test-plan 1, key file
      1), `mcp-tests` (test-plan 1 line at 2 offsets, key file 1), `dnd-unit` (test-plan 1).
    - §4 (lines 133–166) holds no row of these patterns: no count is stated there.
  - test-plan §10 Quality Gates — the mutation tier's recipe: `--test-tool nextest` under the `mutants` profile,
    `--baseline=skip`, and the unit's mutant test command read green first; under plain `cargo test` the six read
    `timeout` — carried: Harness / gate surface and Insufficient fixes. Pattern `mut-tier` has 0 hits in test-plan
    (its 1 standing row is `architecture.md:210`), so the §10 text is new.
  - architecture — carried: Dev-tool versions, Schema / config and Counts.
    - §Stack and Technologies: cargo-nextest as a host instrument of the mutation tier, not a CI tool. Pattern
      `mut-tier`, arch 1 row, a different fact, no-change.
    - §Infrastructure Patterns: `.config/nextest.toml` in the directory tree. Pattern `tree-cfg`, arch 2 rows, both
      root `deny.toml`, no-change; 0 name `.config/`.
    - §Existing Scopes: dioxus-native-dom's `bridge_tests` count. Pattern `bridge-six`, arch 1 row, changed.
  - Beyond the list, arising at the operator pass: security-plan §Dependency Security — the ignores 2 → 5, with the
    founder's word. Pattern `zip-ign`, security-plan 1 row, changed. Carried: Schema / config.
- **Coverage of new surfaces:** none — no new external surface, hot-path operation or UI element. The change is three
  `cfg(test)` tests, a host tool's profile and an audit config.

## Deviations from intent

- **The first implement run stopped (surfaced).** The plan as first written could not run: cargo-mutants 27.1.0's
  baseline refused the audit's test set (Spec claims disproved).
  - On the operator's word («Re-plan the entries (Recommended)», at that run's halt), the plan was revised.
  - It is absorbed by `.andromeda/runs/2026-10-16T20-13-52-phase/`: `--baseline=skip`, plus three unmutated
    baseline entries before the runs.
  - Steps 1–6 stood, and the second run re-entered at step 7.
- **Step 3's loop.** It makes at most count + 1 reads and stops early at `End`, where the step said "bound each loop by
  the expected count plus one, never by End". Absorbed by the same revision, which accepted the form as built.
- **The nextest command-line note** that step 7 asks for is a prose section after the `controls.md` table, so the
  table's row count (entry `grep -c … controls.md`) stays 6.
- **The operator pass added a fix push outside the plan.** `deny.toml` gained three advisory ignores on the founder's
  word, after the pushed sha's CI read red on advisories new against an unchanged lockfile.
- **The scope record** (`gate.py scope`: `scope: clean — changed 6 · listed 4 · recorded 2 (companion 0 · mechanical
  0 · in-intent 0 · widening 2) · absorbed 0 · excluded 72`):
  - widening · `deny.toml` · serves step 8 (the CI entry, red at the Dependency audit on 79cfb5d1) · word: "Игнор по
    ID (Рекомендую)" — the founder, 2026-10-17 07:39Z, by question dialog, relayed verbatim by the operator
    (inputs#I3);
  - widening · `.gitignore` · serves the wrap's P1 line (the escher-0.3.0-incubator/ line pending since
    2026-10-16-file-splits' wrap) · word: "(1) .gitignore, under the Future-version incubator block, add
    escher-0.3.0-incubator/ with a one-line comment in the 0.2.0 form" — the operator, 2026-10-17, at this wrap's
    invocation (inputs#I4).

## Decisions & corrections

- **The founder's word on the zip advisories:** «Игнор по ID (Рекомендую)» (2026-10-17 07:39Z, by question dialog,
  relayed by the operator; inputs#I3). The three advisories are ignored by ID in `deny.toml` with the stated reason,
  and their drop is owned by the `.andromeda/residuals.md` line (`2026-10-16-fast-failing-mutants · open · …`,
  target: next). The operator directed that line and the incubator `.gitignore` line to ride this wrap's P1, made
  before the new-text listing (inputs#I4).
- **The operator's ruling at the first implement run's halt:** «Re-plan the entries (Recommended)», by question
  dialog. The mutation-run entries went back to `/andromeda-phase` for a revision, not fired by hand with
  `--baseline=skip` (`.andromeda/runs/2026-10-16T20-04-49-implement/outcome-2026-10-16-fast-failing-mutants.md`;
  inputs I2).
- **Measured tool behaviour.** cargo-mutants 27.1.0's baseline tests only the mutated package
  (`PackageSelection::Explicit`), ignores `--test-package`, and still passes every `--cargo-test-arg`. A per-unit test
  set naming another package's targets therefore cannot be its baseline. Run `--baseline=skip` and read the unit's
  mutant test command green directly first, because a test phase that cannot pass reads every mutant `caught`
  (`lab.rs:179-190`; the stopped run's entry: `red · exit 0 ✗ (exit 4) · 59.59s`).
- **Measured scheduling.** nextest's `priority = 100` override did not put the direct timer test on the first START
  line. It ran in the first batch of four (counter 1/201) and failed in 0.028 s (the `timer.rs:21:9` mutant log, line
  522; `evidence/controls.md` §The mutant test command).
- **External decay of the audit gate.** The Dependency audit went red between the base's green CI (2026-10-16T17:56Z)
  and the push (2026-10-17T07:29Z), over a byte-identical `Cargo.lock` and `deny.toml`. Three new RustSec advisories
  hit zip 2.4.2, so a chunk with no dependency change can read red at its CI entry (CI#39013904578).
- **Operator pass directives:** «Operator pass … (G3 deployed, go)», then the fix-push directive (inputs#I3), both
  recorded verbatim in `evidence/operator-pass.md`. `gate.py`'s stamp moved from `457c1da1` (implement) to `bebab50b`
  (the operator pass) with the tools' redeploy.
- **Guard refusals.** The Bash guard refused a `cd` into the skill's references dir once in each implement run, because
  it would move the session cwd. Each was re-issued once with absolute paths.
- **A recurring standing learning** (host-linux «Processes»). Both implement runs' census grep over `ps` args matched
  its own probing shell and grep, and was read on the `comm` column. At this wrap the census by command name matched 5
  cargo-mutants and cargo-nextest processes, all another project's (`viola`), not this chunk's.

## Outcome

**Acceptance criteria, re-asserted against the diff:**
- **(tests) The six fail fast, and the score holds at 0 survivors (v020-02) — MET.**
  - Baseline entries: 201 · 276 · 494 tests run, all passed.
  - Three mutation runs, each `exit 0`: timer.rs 19 (18 caught, 1 unviable) · mcp/framing.rs 42 (39, 3) ·
    mutation_writer.rs 76 (69, 7). Over the 137: 0 MissedMutant, 0 Timeout, 0 Failure.
  - The six read CaughtMutant at 2.47 · 2.87 · 2.62 · 2.62 · 2.72 · 2.97 s.
  - The six hand controls read red and were restored sha256-equal.
  - Sources: `evidence/mutants-*.json`, `operator-mutation-runs.txt`, `controls.md`. The matrix reads v020-02
    `implemented`, with its ref.
- **(tests) The three per-crate counts (2 · 74 · 69) — MET** (the three unit entries).
- **(tests) The `mutants` profile parses with no ignored key and lists the three tests — MET** (the `nextest list`
  entry, last line 3).
- **(arch) `mutation_writer.rs` byte-identical to `434e0661`, and `timer.rs` 1–196 byte-equal to the base — MET.**
- **(security) "No tracked file outside the three test sites moves, `Cargo.toml` and `Cargo.lock` included" — UNMET as
  written.**
  - The diff moves `deny.toml` and `.gitignore`, two tracked files outside the three sites. Each is a recorded
    `widening` with its word (Deviations).
  - The entry `git diff --name-only 494abd25… -- packages examples tests apps wpt scripts .github Cargo.toml
    Cargo.lock | grep -c -v …` stays green, because its pathspec reaches neither file. `Cargo.toml`, `Cargo.lock` and
    every path in that pathspec are unchanged.
  - The criterion is unlinked to the matrix, so it routes to a P2 escalation.
- **(security) The only new code-tree file is `.config/nextest.toml`, and the extended files carry no env read, bind or
  print — MET.**
- **(obs) The silence probe reads 0, and no span or `tracing` site is added — MET.**
- **(a11y) `ci-leg.sh a11y` passes with its counts unchanged — MET.**
- **(tests) fast and doc pass, and the pushed sha's CI reads `verdict: green` — MET** on the final HEAD `f072cd61`
  (CI#39014589486, 17/17). The first pushed sha, `79cfb5d1`, read red at the Dependency audit (above). The fix push
  answered it, and the base's own graph and audit config proved the red was not this chunk's.

**Gates** (implement's block run, trail `.andromeda/runs/2026-10-16T20-26-29-implement/gate-2026-10-16-fast-failing-mutants.json`;
`entries 25 · green 19 · red 0 · recorded 0 · timeout 0 · not-run 6`):
- `cargo test -p seven_guis --locked --lib` — green (exit 0 · `contains test result: ok. 2 passed; 0 failed`)
- `cargo test -p escher-driver --locked` — green (`74 passed`)
- `cargo test -p dioxus-native-dom --locked --lib` — green (`69 passed`)
- `out=$(NEXTEST_PROFILE=mutants cargo nextest list …)` — green (`last line 3`)
- The three unmutated baselines `L=…/targets-{seven_guis,escher-driver,dioxus-native-dom}.txt && NEXTEST_PROFILE=mutants
  cargo nextest run …` — green ×3 (`201` · `276` · `494 tests run: … passed`)
- The three mutation runs `o=target/tmp/ffm-{timer,framing,writer}-…` — `leg = 'operator'`. Each was fired once through
  `--operator`, green · exit 0 · history: unmoved (743.92 · 1893.94 · 2823.86 s), recorded in
  `evidence/operator-mutation-runs.txt`.
- The three `jq` probes over `evidence/mutants-*.json` — green (`last line 6` · `last line 0` · `last line 137`)
- `grep -c … controls.md` — green (6 rows)
- `git diff --quiet 434e0661… -- …mutation_writer.rs` — green
- `cmp <(git show 494abd25…:…timer.rs) <(head -n 196 …)` — green
- `git diff --name-only 494abd25… | grep -c -v -x -E …` — green (exit 1 · `last line 0`)
- `git ls-files --others … | grep -c -v -x -F '.config/nextest.toml'` — green (exit 1 · `last line 0`)
- The silence probe `grep -c -E 'println!|…'` — green (exit 1 · `last line 0`)
- `bash .github/scripts/ci-leg.sh a11y` — green (11.99 s)
- `bash .github/scripts/ci-leg.sh fast` — green (131.33 s)
- `bash .github/scripts/ci-leg.sh doc` — green (28.12 s)
- `gate.py hygiene` — operator, fired as written: `hygiene: clean — read 60 …`
- `ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push …` — operator, through `--operator`:
  - green on 79cfb5d1 (84.35 s, `history moved: … 494abd25→79cfb5d1`);
  - re-fired green on f072cd61 (79.28 s, `history moved: … 79cfb5d1→f072cd61`).
- `ci.py conclusion --sha HEAD --wait 1800` — operator, fired as written:
  - red on 79cfb5d1 (CI#39013904578, Dependency audit);
  - re-fired green on f072cd61 (CI#39014589486, 17/17).
  - The final HEAD's run is the record (`evidence/operator-pass.md`).
- Smoke: skipped. No boot path or UI surface changed; the timer.rs edit is a `cfg(test)` module.
- Local audit leg after the `deny.toml` edit, beside the gate tool: `ci-leg.sh audit` exit 0, `advisories ok, licenses
  ok`, with no `advisory-not-detected` warning for the three ignores.

**Watches:** none folded.

**Outcome basis:**
- The operator pass ran. Its commits from the base: `79cfb5d1` (pre-CI) and `f072cd61` (the `deny.toml` fix). The
  final HEAD's CI run, CI#39014589486, is green and recorded in `evidence/operator-pass.md`.
- Implement runs, in dir-name order:
  - `2026-10-16T20-04-49-implement` (outcome: surfaced) — superseded by the revision `2026-10-16T20-13-52-phase` and
    by the next run. Its census rows are kept.
  - `2026-10-16T20-26-29-implement` (outcome: green) — this window holds it whole.
- The `deny.toml` commit landed after implement's block run. Entry 24's fast leg re-ran green on it, and so did the
  local audit leg.

**Process hygiene:**
- Implement's census:
  - The first run's cargo test, cargo-mutants and nextest processes: terminated (exit 4), `target/mutants-tmp/` empty.
  - The second run's cargo test and nextest (entries 1–7 and the block), the 3 cargo-mutants runs with their nextest
    children, and the `ci-leg.sh` a11y, fast and doc legs: all terminated. `target/mutants-tmp/` empty.
  - The operator's overseer watch loop (pid 2983036): not this chunk's.
- Re-measured at this wrap (`ps -eo pid,comm,args`, 2026-10-17T08:03Z): 0 escher cargo-mutants or nextest processes.
  The 5 matching rows are another project's (`viola`, its tree and its temp workspaces), not this chunk's.
- Left on disk, gitignored, for the operator: `target/tmp/ffm-*` outputs (about 9 MB) and the earlier runs' logs.
## Addendum — resumed 2026-10-17
- inputs#I5 · message: the operator, 2026-10-17 08:06Z · copy · n/a — the resume's P2 directive.
  - bears on **(security) "No tracked file outside the three test sites moves, `Cargo.toml` and `Cargo.lock` included" — UNMET as written.**: supersedes «The criterion is unlinked to the matrix, so it routes to a P2 escalation». The criterion is recorded UNMET as written: its letter is unchanged and nothing is reverted. The two files outside the three sites are the scope record's two widenings (`scope-record.md`): `deny.toml` on the founder's word (inputs#I3), `.gitignore` on the operator's word (inputs#I4). No file under `packages`, `examples`, `tests` or `apps` moves beyond the three sites (`git diff --name-only 494abd25 -- packages examples tests apps`: the three test files; `git ls-files --others --exclude-standard -- packages examples tests apps`: none). The criterion is not matrix-tied, so v020-02 stands; the escalation is resolved by this word.
  - bears on **Counts / qualifiers moved**: adds to it — the `zip-ign` site, `security-plan.md:237` §Dependency Security («Two ignores stand»), is amended to five in the fan-out, naming RUSTSEC-2026-0367, RUSTSEC-2026-0368 and RUSTSEC-2026-0369 and their owner line `.andromeda/residuals.md:29`.
- inputs.py verify, re-fired: `inputs: 5 entries — unchanged 0 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 5 · uncited 1 · unparsed 0`; moved since the report: entries 4 → 5, uncited 2 → 1 (I3 and I4 now cited; I5 snapped at this resume, cited here).
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 494abd25 (the parent of the oldest pre-CI commit 79cfb5d1) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .config/nextest.toml — new file · 13 line(s)
### .gitignore — added 2 line(s) in 1 range(s)
added: 37-38
### deny.toml — added 3 line(s) in 1 range(s)
added: 23-25
### examples/seven_guis/src/tasks/timer.rs — added 30 line(s) in 1 range(s)
added: 197-226
- 198-226 @199 «mod tests {»
  - 206-225 @207 «fn a_delivered_tick_is_taken_once_and_an_empty_wait_parks() {»
### packages/dioxus-native-dom/src/dioxus_document_tests.rs — added 25 line(s) in 3 range(s)
added: 4-6 · 12 · 147-167
- 147-166 @148 «fn a_created_text_node_is_mapped_and_left_on_the_stack() {»
  - 155-159 «assert_eq!(»
  - 160-163 «let node = doc»
### packages/escher-driver/src/mcp_tests.rs — added 31 line(s) in 1 range(s)
added: 167-197
- 167-196 @168 «fn read_request_takes_each_line_with_its_terminator() {»
  - 170-175 «let expected = [»
  - 176-179 «let readers: Vec<Box<dyn BufRead>> = vec![»
  - 181-195 «for (row, mut reader) in readers.into_iter().enumerate() {»
