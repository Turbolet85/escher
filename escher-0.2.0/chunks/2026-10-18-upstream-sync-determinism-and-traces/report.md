# Report — 2026-10-18-upstream-sync-determinism-and-traces

**Chunk:** Upstream sync ahead of determinism and traces — upstream/main fb8d6c15e measured, five past 23b9b26a9 (four Taffy-main bumps, one cursor fix), the pin a fork; CI and audit green, per-OS tallies re-read, sink allowlist re-checked
**Date:** 2026-10-18T06:50Z
**Commits:** since `last_wrap` 2026-10-18T05:16:33Z (`git log --format='%h %s' fb842671..HEAD`):
- `275950dc0` chore(2026-10-18-upstream-sync-determinism-and-traces): operator pre-CI commit, for the run this chunk's verdict reads

Its body carries `(cherry picked from commit fb8d6c15e9ce062b3b41e71cd48988f0093506cc)`.

## Changes (structured — detectors read this)
- **Files:** `packages/blitz-dom/src/document.rs` alone in source. The take was `git cherry-pick --no-commit fb8d6c15e9ce062b3b41e71cd48988f0093506cc`: exit 0, +17 −3, no conflict, no hand edit, no `CHERRY_PICK_HEAD`. The basis is `evidence/take.md` and `gate.py scope`: `scope: clean — changed 1 · listed 1 · recorded 0 … excluded 97`, base `fb842671`. The chunk's records: `evidence/{before,take,counts,readings,sink,lock,operator-pass,ci}.md`.
- **Symbols / APIs:** `BaseDocument::get_cursor` (pub, `packages/blitz-dom/src/document.rs:2260`) now returns `Some(CursorIcon::Default)` where it returned `None` when neither `hover_hit_node_id` (filtered by `contains_key`) nor `get_hover_node_id()` resolves — a pointer over no box. Upstream #1220, `fb8d6c15e`, taken as upstream wrote it.
  - It still returns `None` for `cursor: none` (`stylo_to_cursor_icon`) and for a node with no primary styles, so the shell's `None` path stays reachable.
  - New test: `document::hover_state_tests::hovering_below_content_reports_default_cursor`, `document.rs:3133-3141` (the New text listing below).
  - Remaining callers, all unchanged (research.md §Graph impact, the line map below):
    - `update_cursor @ document.rs:2041` → `refresh_hover @ document.rs:2021`, and `EventDriver::handle_ui_event @ packages/blitz-dom/src/events/driver.rs:306`.
    - The sub-document recursion, now `document.rs:2278`.
    - The two standing `hover_state_tests`, now `document.rs:3117` and `:3130`.
  - The value reaches only `ShellProvider::set_cursor`. That is a no-op headless (the `Dummy*` default, `packages/blitz-traits/src/shell.rs:13`); windowed, `BlitzShellProvider::set_cursor` hides the cursor on `None` (`packages/blitz-shell/src/lib.rs:101-110`), so a pointer below the content now keeps a visible default arrow.
  - No escher reader — snapshot, driver, harness or stand — reads a cursor (research.md §Sweeps: `get_cursor|set_cursor` over `packages examples apps tests`, 12 hits · 0 changed · 12 no-change).
  - **The take's line map of `document.rs`** (`git diff -U0 fb842671 -- packages/blitz-dom/src/document.rs`, hunks `@@ -2264,2 +2264,3 @@` · `@@ -2268 +2269,4 @@` · `@@ -3127,0 +3132,10 @@`; 3575 → 3589 lines; `evidence/readings.md`): old 1–2263 unchanged · old 2264–2268 the changed `get_cursor` lines (new 2264–2272) · old 2269–3127 **+4** · old 3128–3575 **+14**.
- **Crates / modules:** none — `git diff --quiet fb842671 -- … ':(exclude)packages/blitz-dom/src/document.rs'` exit 0 over the manifests, every crate, the stand, the checks, the CI files and scripts (entry 3, the preservation guard).
- **Dependencies:** none. `Cargo.toml`, `Cargo.lock` and `deny.toml` are byte-identical to `fb842671` (`git diff --quiet` exit 0; `Cargo.lock` sha256 `117baa86…` on both sides; `evidence/lock.md`). Taffy stays the registry `0.15`, resolved 0.15.0 with checksum `3c848088…` (`Cargo.lock:8038-8041`). The root manifest holds no git dependency: entry 4 reads `0`, entry 5 reads `1`.
  - The four Taffy-main commits (`1a11761bc` · `4a5ab9cc9` · `510b592ab` · `1e5479ae6`) are NOT taken. Upstream's manifest at `fb8d6c15e` still names `taffy = { git = "https://github.com/DioxusLabs/taffy", rev = "ea07c4c304216c9b48bdd88c46153c1cbe3cae10", …` (`git show fb8d6c15e:Cargo.toml`, line 100). Taffy main is v0.15.0 + 14 unreleased commits, and crates.io's newest is 0.15.0 (inputs#I2, inputs#I3).
  - The sync record: **F1 answered B** — cherry-pick `fb8d6c15e` alone, Taffy stays registry `0.15` — by the operator in the founder's stead: `delegate the operator, 2026-10-18`, PROVISIONAL (inputs#I5; `.andromeda/runs/2026-10-18T05-47-23-phase/founder-forks.md`). The founder's rule of 2026-10-13 (no git dependency in the root manifest) is untouched.
- **Schema / config:** none.
- **Spec-master edits:** none (no master was changed before this wrap; `git diff --name-only fb842671 -- .andromeda/*.md` lists `.andromeda/master-route.md` alone, the phase's promotion).
- **Counts / qualifiers moved:**
  - **The upstream-sync record** (pattern `sync-line`, standing 8):
    - `architecture.md:233`, the `Upstream sync:` line — moves. Upstream `main` was read at `fb8d6c15e` (`git ls-remote upstream refs/heads/main`, unmoved at /implement, `evidence/before.md`), 5 commits past `23b9b26a9`. `fb8d6c15e` was taken alone by cherry-pick in the pre-CI commit `275950dc0`. The merge base stays `23b9b26a9` (`git merge-base HEAD fb8d6c15e`), and the four Taffy-main commits wait.
    - No-change, true historical statements of the 2026-10-17 merge at `23b9b26a9`, which stays the base: `architecture.md:75`, `:90`, `:135` @c2427, `security-plan.md:245`, `test-plan.md:10` @c86, `:26` @c12807, `a11y-plan.md:47` @c126.
    - Leaf: `.claude/docs/workflow.md:15` restates the sync line (P2 cascade).
  - **The Taffy pin's sync record** (pattern `taffy-pin`, standing 5):
    - `security-plan.md:245` ×2, the 2026-10-17 sync record — a record for this sync is owed beside it (expected amendment 2), with F1 named as the delegate's PROVISIONAL answer.
    - No-change, true and unchanged: `architecture.md:48` (the Layout row), `:72` ×2 ([Dependency pinning]) and `security-plan.md:242` (no git dependency).
    - `architecture.md:233` is the sync line above.
    - Leaves: `CLAUDE.md:51`, `.claude/rules/security.md:28`, `.claude/docs/gotchas.md:7`, `.claude/docs/stack.md:17` (true, unchanged), and `.claude/docs/workflow.md:15` (the sync line).
  - **The cursor over no box** (patterns `cursor`, standing 2, and `docrs-cite`):
    - `design-system.md:316` — moves. "Default otherwise (packages/blitz-dom/src/document.rs:2260-2316)" left out the over-no-box case, which answered `None` (hidden, windowed). It now answers `Default`, and the range's end moves 2316 → 2320. This row is from `docrs-cite`; the `cursor` pattern's wording does not reach it.
    - `design-system.md:317` ×2 — no-change: "`ShellProvider::set_cursor` takes an optional `CursorIcon` … None hides the cursor" stays true of the shell. `None` still flows for `cursor: none`; only the over-no-box `None` is gone.
    - `test-plan.md:12` "hover cursor" — no-change in wording; its range `document.rs:3004-3567` moves by the line map.
  - **blitz-dom's `document.rs` test counts** (pattern `dom-tests`, standing 1 — `test-plan.md:11` ×3) — move: "35 `#[test]` functions: 18 `document::` tests … 9 in document.rs" → **36 · 19 · 10**. Basis: `git show fb842671:packages/blitz-dom/src/document.rs | grep -c '#\[test\]'` = 9 → the work tree's = 10.
  - **`hover_state_tests`** (pattern `hover-tests`, standing 1 — `test-plan.md:293` ×2) — moves: "the two `hover_state_tests` … `2 passed`" → three. Entry 9: `running 3 tests`, `3 passed; 0 failed; 0 ignored; 0 measured; 95 filtered out`; the two standing tests and upstream's new one, whose precondition-free assertion uses the same two-argument `qual_name!` fixture with no skip.
  - **The workspace tally, test-plan §9 Local baseline** (pattern `tally-local`, standing 2):
    - Rows: `test-plan.md:328` (the label row) and `:331` ×2 @c16694,16712. The latest recorded re-count is "2026-10-17-upstream-sync-engine-debts: 187 result lines, 971 passed · 0 failed · 12 ignored".
    - This chunk measured **192 result lines · 991 passed · 0 failed · 12 ignored**: entry 12 over the fresh `target/ci-logs/test.log`, and the same in the operator pass's fast leg, entry 28's log.
    - Against the chunk start's 192 · 990 · 0 · 12 (the engine-debts wrap's `test.log`, `evidence/before.md`), that is **+1 passed**.
    - Attribution: a per-target pairing of every result line of the engine-debts wrap's fast-leg gate log against this run's. 192 lines each; exactly one moved, blitz-dom's lib (`Running unittests src/lib.rs (…blitz_dom-…)`) 97 → 98 = `hovering_below_content_reports_default_cursor` (`evidence/counts.md`).
    - The intermediate 192 · 990 was never written to the masters: `\b990\b` reads 0 over `.andromeda/*.md`, `.andromeda/registries` and the engine-debts chunk folder. The engine-debts report carried counts without re-reading the tallies (its `report.md:283-285`). So the master moves from 2026-10-17's 187 · 971 straight to this chunk's 192 · 991, and engine-debts' +5 lines · +19 passed is to be named in the amendment as that chunk's.
    - Leaf: `.claude/docs/tests-summary.md:30` ×2 (P2 cascade).
  - **Per-OS test tallies, test-plan §9** (pattern `tally-os`, standing 2):
    - `test-plan.md:325` ×4 — moves. The recorded reading is CI#39056637407, re-read at 2026-10-17: linux 187 · 971 · 0 · 12; macos and coverage 168 · 968 · 0 · 12; windows 168 · 944 · 0 · 9; a11y 6 · 43 · 0 · 0; stand repeat 141 · 576 · 0 · 21.
    - This chunk re-read **CI#39104364530** on `275950dc0`: linux 192 · 991 · 0 · 12; macos 173 · 988 · 0 · 12; coverage 173 · 988 · 0 · 12; windows 173 · 964 · 0 · 9; a11y 6 · 43 · 0 · 0; stand repeat 144 · 582 · 0 · 21; ios and android built only. That is entry 30's log, `evidence/ci.md`.
    - Against the chunk start's CI#39100630123 on `fb842671` (192 · 990 / 173 · 987 / 173 · 987 / 173 · 963 / 6 · 43 / 144 · 582, the plan's control fired at P4), four jobs moved **+1 passed** each. The move is upstream's `hovering_below_content_reports_default_cursor ... ok`, read once per job log: Test [default features] 06:34:42Z, Coverage report 06:42:35Z, Test (macos) 06:40:40Z, Test (windows) 06:43:41Z. The a11y and stand-repeat jobs did not move: they run named blitz-tests targets only.
    - CI#39100630123's own per-OS readings (engine-debts' final HEAD) were never written to the masters, so this record moves from CI#39056637407 directly.
    - No-change: `security-plan.md:239` @c3471 "(CI#39056637407 on `ae8c8daf`)", a dated audit reading, true.
  - **`document.rs` citations** (pattern `docrs-cite`, standing 70 rows over the seven masters, 0 in the registries):
    - Of the 113 citations those rows carry, **35 on 21 lines move** by the take's line map (architecture 8 lines · design-system 1 · test-plan 12), and 78 stay (cited lines ≤ 2263). None falls inside the changed lines 2264–2268. The classification is per citation, by a scratch reader of the line map over the same seven masters and the registries (`$TMPDIR/…/scratchpad/docrs_cites.py`, not committed).
    - The 21 lines that move, with digits re-pointed by P2's `cites.py apply`:
      - `architecture.md`: `:84`, `:86`, `:128` ×3, `:135`, `:141` ×4, `:163` ×2, `:263` ×2, `:266` ×2
      - `design-system.md:316`
      - `test-plan.md`: `:11`, `:12`, `:45` ×3, `:66` ×2, `:137` ×2, `:175` ×2, `:238` ×2, `:239`, `:240`, `:270`, `:293`, `:324`
    - Every other `docrs-cite` row cites lines ≤ 2263 and is no-change.
  - **Upstream's new test name** (pattern `new-test`): 0 master sites. The sweep refused it — `control: new-test, provisional never fired over the seven masters' pre-pass text (275950dc)` — which is the basis of the zero.
  - **F1's delegate answer, PROVISIONAL** (pattern `provisional`): 0 master sites, by the same refusal line. The playbook's «Provisional discharge» rule is a base, not a master. The answer lands with the sync line and the Pinning record above (P5 card row 4, below).
  - **CARRY 3's owner** (pattern `title`, standing 1 — `architecture.md:90`): "their merge surface, by file, is owned by a CARRY on «Upstream sync ahead of determinism and traces»" moves when P5 carries CARRY 3 on to «Upstream sync ahead of visual lint and real-app readiness» (P5 card row 3). Pattern `owner`: `architecture.md:90` (the same site) and `security-plan.md:123` @c8547 ("it becomes one is owned by a route entry" — another subject, no-change); base rows `playbook.md:58`, `:66` (bases, not dispositioned here).
  - **Lines over 2 000 chars, (line, fact) pairs** (the `by line` block and the row offsets):
    - moves: `architecture.md:86` @c1705 (docrs); `:128` @c378,1495,1541 (docrs); `:135` @c1720 (docrs); `:163` @c14185,14256 (docrs); `test-plan.md:66` @c352,398 (docrs); `:324` @c3693 (docrs); `:331` @c16694,16712 (tally).
    - no-change: `architecture.md:134` @c429,552,871,1530,1671,3451,3495,3539,3896,4442 (docrs, every cited line ≤ 2263); `:135` @c2427 (sync, true); `:177` @c236 (docrs ≤ 2263); `:209` @c131,175,497,879,1355 (docrs ≤ 2263); `:225` @c136,270,3864 (docrs ≤ 2263); `security-plan.md:123` @c8547 (owner, other subject); `:124` @c5490 (docrs ≤ 2263); `:239` @c3471 (tally-os, dated, true); `test-plan.md:10` @c86 (sync, true); `:26` @c12807 (sync, true); `a11y-plan.md:36` @c740 (docrs ≤ 2263); `:47` @c126 (sync, true); `:216` @c495 (docrs ≤ 2263).
  - Sweep reach: 10 patterns · 11 moved facts, 11 with a pattern (the report's own count; two patterns refused unfired, `new-test` and `provisional`) · owner pattern 1 standing · by line 7 lines · 20 offsets. The basis is `cascade.py sweep --pre-pass`, trail `.andromeda/runs/2026-10-18T06-47-20-wrap/cascade-2026-10-18-upstream-sync-determinism-and-traces-pre-pass.json`: `total (10 patterns) · 103 rows over 13 files`.
- **Dev-tool versions:** none.
- **Harness / gate surface:** none. No agent-run script, CI step, leg or verdict shape changed (entry 3 green over `.github` and `scripts`).
- **Cross-project / external claims:**
  - **DioxusLabs/blitz:** `upstream/main` read at `fb8d6c15e9ce062b3b41e71cd48988f0093506cc`, by `git ls-remote upstream refs/heads/main` at /implement, 2026-10-18T06:16Z, unmoved since P3. `fb8d6c15e` "Show the default cursor when the pointer is over no box (#1220)", by Nico Burns, changes one file (`git show --stat`).
  - **DioxusLabs/taffy:** `ea07c4c3` = v0.15.0 + 14 unreleased commits (inputs#I3, not re-read live at /implement or here).
  - **Fork CI:**
    - Chunk start: CI#39100630123 on `fb842671`, green 17/17 (scope.md §CI verdict; the plan's per-OS control fired at P4).
    - The pushed take: **CI#39104364530 on `275950dc0985f258c4fd26fc98faa80fb29df31d`**, `verdict: green · checks 17/17 · wall 842 s`, completed/success. Entry 29 is recorded in `evidence/operator-pass.md`, its per-OS read in `evidence/ci.md`. The sha is the record: this wrap's commit adds to that tree.
  - **Inputs** — `inputs.py verify`: `inputs: 6 entries — unchanged 0 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 6 · uncited 1 · unparsed 0`.
    - `I1 · message · copy · n/a` — the operator's directive at take-up: the founder away, his forks to `founder-forks.md` answered by the operator as delegate, PROVISIONAL; the guide-form pre-CI subject. Cited by scope.md, research.md and plan.md.
    - `I2 · message · copy · n/a` — upstream at `fb8d6c15e`, the five commits, trial merges clean, `keyboard_compat.rs` not retired, no Taffy release past 0.15.0.
    - `I3 · message · copy · n/a` — Taffy `ea07c4c3` = 0.15.0 + 14, the stand's 0 floats, upstream's lock delta.
    - `I4 · message · copy · n/a` — Taffy `ea07c4c3` and v0.15.0 both declare `rust-version = "1.71"`.
    - `I5 · message · copy · n/a` — F1 answered B by the operator as delegate, PROVISIONAL. The operator agrees both not-his leans (the cursor change is upstream behaviour; no widening). The cited founder "ruling 9" is not found in this repository.
    - `I6 · message · copy · n/a` — `verify` read it `UNCITED`; it is cited here as **inputs#I6**: the operator's P5 word "yes" on `.andromeda/runs/2026-10-18T05-47-23-phase/p5-card-1.md` (the operator, 2026-10-18, in session). It approves the plan and the card's four rows, accepts planlint check 4 (13) on gate 30, and states ruling 9 is the founder's 2026-10-06 sync rule — a real merge, with cherry-pick as the fallback — kept in the overseer's records. The card's `## Beyond the plan` rows, verbatim, approved by the operator's own word (rows 1–3 are the wrap's route-resolve; row 4 concerns F1, whose answer stays the delegate's, PROVISIONAL):

      | # | proposal | owner / place |
      |---|---|---|
      | 1 | CARRY 1 — upstream's four Taffy-main commits past `23b9b26a9` (git Taffy `ea07c4c3`, unreleased) move on, left out again on F1 B | «Upstream sync ahead of visual lint and real-app readiness» (escher-0.2.0/working-route.md:32), a CARRY written by the wrap's route-resolve |
      | 2 | CARRY 2 — whether upstream retired `keyboard_compat.rs` (still listed at `fb8d6c15e`); the coverage exclusion drops with it | «Upstream sync ahead of visual lint and real-app readiness», a CARRY written by the wrap's route-resolve |
      | 3 | CARRY 3 — the engine-debts seven fixes' merge surface, no upstream PR, re-read per row at the next sync | «Upstream sync ahead of visual lint and real-app readiness», a CARRY written by the wrap's route-resolve |
      | 4 | F1's delegate answer, PROVISIONAL, joins the founder's epoch-boundary discharge batch beside the engine-debts batch | the playbook's «Provisional discharge» rule; the wrap lists it with its sites |
- **Reverted / negative API facts:** none written. Option A at P4 — merge `upstream/main` whole, a git Taffy back in the root manifest — was not taken (F1 B), so nothing of it was written or reverted.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:** none. `design-system.md:316`'s "Default otherwise" is restated as this chunk's change (above), and the tallies are dated re-counts, true as dated.
- **Expected amendments (from plan):**
  - architecture §Project Intent, the `Upstream sync:` line (read at `fb8d6c15e`; `fb8d6c15e` taken by cherry-pick in `275950dc0`; base stays `23b9b26a9`; the four Taffy-main commits wait; F1 delegate, PROVISIONAL, by the playbook's «Provisional discharge»): **carried** — Counts «The upstream-sync record». Pattern `sync-line`: architecture 4 rows (`:233` the line; `:75`, `:90`, `:135` no-change).
  - security-plan §Dependency Security → Pinning, the sync record (Taffy registry `0.15`, no git dependency, lock unchanged, the delegate answer PROVISIONAL): **carried** — Dependencies, Counts «The Taffy pin's sync record». Pattern `taffy-pin`: security-plan 2 rows (`:242` no-change, `:245` ×2, the record owed beside it).
  - design-system §Iconography (Cursors and favicons (engine)) — over no box the cursor reads `Default` (shown), no longer `None` (hidden), upstream #1220 `fb8d6c15e`; its `document.rs` citations re-pointed: **carried** — Symbols / APIs, Counts «The cursor over no box». Patterns `cursor` (design-system 1 row, `:317`, no-change) and `docrs-cite` (design-system `:316`, moves).
  - test-plan §9 Local baseline and §9 Per-OS test tallies; §1's blitz-dom `document.rs` test count +1: **carried** — Counts «The workspace tally», «Per-OS test tallies», «blitz-dom's `document.rs` test counts», «`hover_state_tests`». Patterns `tally-local` (test-plan 2 rows), `tally-os` (test-plan 1 row `:325`; security-plan 1 row no-change), `dom-tests` (test-plan 1 row), `hover-tests` (test-plan 1 row).
  - every `packages/blitz-dom/src/document.rs` citation below line 2260 re-pointed by the take's line map, with no claim text changed: **carried** — Symbols / APIs (the line map), Counts «`document.rs` citations». Pattern `docrs-cite`: 70 rows over architecture, security-plan, design-system, test-plan, obs-plan and a11y-plan, 35 citations moving, all in architecture (16), design-system (1) and test-plan (18). The cited lines that move are old ≥ 2269, not "below line 2260" as the plan words it — the plan's "below" means further down the file. layout-templates 0 rows.
  - the three CARRYs re-read here and moved on with the next upstream-sync entry (the wrap's route-resolve, named in the review card's `## Beyond the plan`): **carried** to P5 — inputs#I6's rows 1–3, the readings in `evidence/readings.md` (CARRY 1: upstream still pins git Taffy `ea07c4c3`; CARRY 2: `keyboard_compat.rs` still listed at `fb8d6c15e` and both exclusions stand; CARRY 3: F1–F7 untouched, F8's `document.rs` moved in other functions only, no PR). Pattern `title`: architecture 1 row (`:90`, the owner title that moves with CARRY 3).
- **Coverage of new surfaces:**
  - `BaseDocument::get_cursor` over no box (a changed engine read, no new surface) → validation n/a (no input) · instrumentation n/a (0 log, print, env or listener sites added — entry 6 reads `0` over `fb8d6c15e`'s added `.rs` lines) · PII n/a · tests unit (`hovering_below_content_reports_default_cursor`) + integ (`pointer_leave`, no `None` cursor sent, the last `Pointer`) · a11y n/a (the cursor icon is no accessibility-tree, focus or contrast surface; the a11y leg 6 · 43 · 0 unmoved) · tokens n/a. The windowed cursor's visibility is witnessed by upstream's unit test alone: the windowed smoke cannot see a cursor.

## Deviations from intent
- Plan step 1's "a clean tree apart from this chunk's own folder and run dir" did not hold literally. The tree also carried the pipeline's own post-wrap bookkeeping:
  - three modified `.andromeda/` ledgers (`friction-log.ndjson`, `master-route.md`, the engine-debts wrap's `evolve-…json`);
  - `.claude/session-handoff.md` and `escher-0.2.0/working-route.md`;
  - the untracked `.andromeda/runs/` dirs of the evolve-diagnose, code-audit and phase runs, plus the engine-debts wrap's `health-…json`.

  Justification: every one is excluded from the scope read and none is source, manifest, check or CI. All rode the pre-CI commit `275950dc0` (`evidence/before.md`).
- Two readings beyond the plan's letter, both additive:
  - The +1 attributed per target, against the engine-debts wrap's fast-leg gate log, ruling out an offsetting move.
  - The take's measured line map, for this wrap's citation re-pointing.
- `evidence/operator-pass.md` and `evidence/ci.md` were written after the pre-CI commit, so they are untracked and ride this wrap's commit. Entry 27's hygiene read and the commit then saw the same tree.
- scope record: none — `gate.py scope` clean, 0 recorded (`scope: clean — changed 1 · listed 1 · recorded 0 (companion 0 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 97`).

## Decisions & corrections
- **The operator ran the pass by direction.** "Run the operator pass for 2026-10-18-upstream-sync-determinism-and-traces now, each entry by hand in plan order: entry 27 hygiene; the pre-CI commit with EXACTLY the subject you named and the cherry-picked-from line in the body; entry 28 via gate.py run --operator 28 in the background; entry 29 as written; entry 30 with the run id from 29. Record it in evidence/operator-pass.md and evidence/ci.md. Stop and tell me if any entry is red; start no wrap." (the operator, 2026-10-18, in session). It was done in that order, and no entry was red.
- **This wrap is two-window.** The operator's word at invocation: "run P1 only and stop" (the operator, 2026-10-18). P2–P7 run at a resume of this run dir (`.andromeda/runs/2026-10-18T06-47-20-wrap/`).
- **The founder's forks were answered by delegate.** The founder is away; his forks went to one `founder-forks.md`, answered by the operator in his stead and recorded as the delegate's, PROVISIONAL (inputs#I1). F1 was answered B (inputs#I5). It joins the founder's epoch-boundary discharge batch (playbook «Provisional discharge»; inputs#I6 row 4).
- **"Ruling 9"**, cited by the operator for cherry-pick as the fallback, is the founder's 2026-10-06 sync rule: a real merge, with cherry-pick as the fallback. It is kept in the overseer's records, not this repository (`grep -i cherry` over `.andromeda`, `escher-0.1.0`, `escher-0.2.0`: absent; inputs#I5, inputs#I6). It is recorded in the operator's words only.
- **A first for this fork.** A sync recorded as a partial take by `git cherry-pick --no-commit` of upstream's text, landing in the operator's pre-CI commit with the `(cherry picked from commit …)` trailer, the merge base unmoved. The exact-take entry (a diff of the chunk's `document.rs` delta against upstream's commit) proves it line for line.
- **Sweep hazard (a count the plan names that the masters never recorded).** The plan's "192 · 990 · 0 · 12" baseline and CI#39100630123's per-OS figures stand in no master: `\b990\b` reads 0 over `.andromeda/*.md`, `.andromeda/registries` and the engine-debts chunk. The masters' latest local and per-OS readings are 2026-10-17's (187 · 971; CI#39056637407). Engine-debts' wrap carried counts without re-reading the tallies (its `report.md:283-285`). A sweep for the plan's own numbers therefore finds nothing, and the amendment moves from the older figure.
- **Gate logs persist outside the tree.** A re-count can be attributed per target against a prior run: the engine-debts wrap's `$TMPDIR/andromeda-gate/2026-10-18-engine-debts/wrap-2026-10-18T04-30-47/27.log` held its fast leg's 192 result lines, paired against this run's entry 11 log (1 of 192 moved).
- **Guard refusal (implement).** bash-guard refused a bare `cd` into the skill's references dir ("Blocked: cd (…/andromeda-implement/references) moves the session cwd for every later call"). It was re-issued once with absolute paths and ran. This is a recurrence of the standing host-linux «Paths & the working directory» learning (`outcome-…md`, Decisions & corrections).
- **Sweep refusal (this wrap).** The pre-pass refused `new-test` and `provisional` with "control: new-test, provisional never fired over the seven masters' pre-pass text (275950dc)". The re-fire named the ten fired patterns. That is the designed refusal, and its line is the zero's basis.

## Outcome
- **(arch) Met.** `fb8d6c15e` was taken by a no-commit cherry-pick of upstream's text and nothing else of upstream's landed. The patch-present, exact-take and preservation entries are green. The merge base stays `23b9b26a9`, and the four Taffy-main commits are recorded as waiting (`evidence/take.md`, `evidence/readings.md`). The diff adds no surface the criterion excludes.
- **(security) Met.** The root manifest holds no git dependency and Taffy stays the registry `0.15`, with `Cargo.lock` byte-identical to `fb842671`. Audit: `advisories ok, licenses ok`, `deny.toml` unchanged, one `license-not-encountered` warning for NCSA as before. F1 is recorded as the delegate's PROVISIONAL answer (`evidence/lock.md`).
- **(obs) Met.** The taken commit adds 0 `tracing`/`log`/print/env/listener sites (the fixed-sha entry green, its log `0`). `host_log` and `mcp_log` pass under the per-package build (`cargo test -p seven_guis`) and the workspace build (inside `fast`). `packages/escher-telemetry/` is unchanged (`evidence/sink.md`).
- **(tests) Met.** `hovering_below_content_reports_default_cursor` passes under `cargo test -p blitz-dom`, and `pointer_leave` passes. `fast` and `doc` pass. The workspace tally is 192 · 991 · 0 · 12 against 192 · 990 · 0 · 12, the +1 attributed by name. Every blitz-tests file but `all.rs` ran by name (132 files, 131 targets, 0 absent). `run stand` (170 · 0 · 7) and `run all` (562 · 0 · 12) pass, equal to baseline.
- **(tests) Met.** Fork CI on the pushed pre-CI commit reads `verdict: green · checks 17/17`, CI#39104364530 on `275950dc0`, recorded in `evidence/ci.md`. Its per-OS tallies were re-read, and every moved count against CI#39100630123 (four jobs, +1 passed each) is explained by name.
- **(tests) Met.** CARRY 2: upstream still lists `keyboard_compat.rs` at `fb8d6c15e` (entry 7 `1`), and both coverage-leg exclusions stand (entry 8 `2`).
- **(a11y) Met.** The a11y leg reads 6 lines · 43 passed · 0 failed. No platform accessibility adapter is in the stand graph (entry 24 `0`).
- **(design · layouts) Met.** `run stand` passes in both layout modes at 170 · 0 · 7, equal to baseline; no geometry, Tab-order, contrast or screenshot figure moved.
- **(CARRY 3) Met.** `evidence/readings.md` holds one row per F1–F8: F1–F7 untouched; F8 the same file, other functions (`get_cursor`, `hover_state_tests`), its five functions shifted +4 lines. No upstream PR.
- **Gates**, the `[[gate]]` entries by `run`, in order — /implement's firing, run `.andromeda/runs/2026-10-18T06-16-11-implement/` (trail `gate-2026-10-18-upstream-sync-determinism-and-traces.json`), 30 entries · green 25 · red 0 · recorded 1 · not-run 4, the first firing:
  - `git show fb8d6c15e… -- packages/blitz-dom/src/document.rs | git apply --check -R` — green, exit 0
  - `diff <(git diff fb842671… -- …document.rs | grep …) <(git show fb8d6c15e… -- …document.rs | grep …)` — green, exit 0, no output
  - `git diff --quiet fb842671… -- Cargo.toml Cargo.lock deny.toml packages examples tests apps wpt .github scripts ':(exclude)…document.rs'` — green, exit 0
  - `grep -c -E 'DioxusLabs/taffy|git = ' Cargo.toml` — green, exit 1, last line 0
  - `grep -c -E '^taffy = \{ version = "0\.15",' Cargo.toml` — green, exit 0, last line 1
  - `git show fb8d6c15e… -- '*.rs' | grep -c -E '^\+[^+].*(tracing::|…)'` — green, exit 1, last line 0
  - `git ls-tree -r --name-only fb8d6c15e… -- …keyboard_compat.rs | wc -l` — green, last line 1
  - `grep -c -F -e "--ignore-filename-regex 'keyboard_compat\.rs\$'" .github/scripts/ci-leg.sh` — green, last line 2
  - `cargo test -p blitz-dom --locked --lib hover_state_tests 2>&1` — green, exit 0, no FAILED, the new test `... ok`
  - `cargo test -p blitz-tests --locked --test pointer_leave 2>&1` — green, exit 0, `Running tests/pointer_leave.rs`
  - `bash .github/scripts/ci-leg.sh fast` — green, exit 0, artifact `target/ci-logs/test.log` fresh
  - `grep -E '^test result:' target/ci-logs/test.log | awk …` — green, `lines 192 passed 991 failed 0 ignored 12`
  - `for f in tests/blitz-tests/tests/*.rs; … | grep -c absent` — green, exit 1, last line 0
  - `grep -E '^Ran [0-9]+ tests|^OK$|^FAILED' target/ci-logs/ci-scripts.log` — recorded: `Ran 100 tests in 8.581s` · `OK`
  - `bash .github/scripts/ci-leg.sh doc` — green, exit 0
  - `bash .github/scripts/ci-leg.sh audit` — green, exit 0, `advisories ok`, `licenses ok`
  - `bash .github/scripts/ci-leg.sh a11y` — green, exit 0, no FAILED, artifact fresh
  - `grep -E '^test result:' target/ci-logs/a11y.log | awk …` — green, `lines 6 passed 43 failed 0`
  - `bash scripts/agent-run.sh boot` — green, exit 0
  - `bash scripts/agent-run.sh run stand` — green, `"outcome": "passed"`, run.end 170 · 0 · 7
  - `ls tests/blitz-tests/tests/stand_*.rs | wc -l` — green, last line 41
  - `bash scripts/agent-run.sh run all` — green, `"outcome": "passed"`, run.end 562 · 0 · 12
  - `cargo test -p seven_guis --locked 2>&1` — green, exit 0, no FAILED
  - `cargo tree --locked -p seven_guis -e normal --prefix none | grep -c -E '^accesskit_(xplat|winit) '` — green, exit 1, last line 0
  - `cargo build -p seven_guis --bin seven_guis_native --locked` — green, exit 0
  - `RUST_LOG=info timeout 10 target/debug/seven_guis_native; test $? -eq 124` — green, exit 0, `service.name=seven_guis` (`WAYLAND_DISPLAY` set; the smoke)
  - `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` — leg operator; fired as written by the operator pass, `hygiene: clean` (`evidence/operator-pass.md`)
  - `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin build/escher-0.2.0` — leg operator; `gate.py run … --operator 28`: green, exit 0, 131.1 s, `history moved: refs/remotes/origin/build/escher-0.2.0 fb842671→275950dc` (the intended push, `fb8426711..275950dc0`); its fast leg 192 · 991 · 0 · 12 (`evidence/operator-pass.md`)
  - `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800` — leg operator; fired as written: `275950dc0985 verdict: green · checks 17/17 · wall 842 s · runs CI#39104364530 completed/success` (`evidence/operator-pass.md`)
  - `gh run view <id> -R Turbolet85/escher --log | sed … | awk …` — leg operator; `--operator 30 --id 39104364530`: recorded, exit 0, `history: unmoved`. The outcome it read: CI#39104364530, green, this chunk's run; tallies in `evidence/ci.md`, every move explained by name.
  - No `defer` entry, no deferral, no `--void`; the walk-class line read `0 marked`. The coverage leg is not in the block: its reading is CI's "Coverage report" job, green in CI#39104364530.
  - Smoke: the windowed boot ran green as the plan's `role = 'smoke'` entry. `agent-run.sh status` after `run all` read `all · passed 562 · failed 0 · ignored 12`, and `cleanup` read `done`.
- **Watches:** none — no `watch:` folded (scope.md §Folded freight).
- **Outcome basis:**
  - The operator pass ran. Its commit list is `275950dc0` alone (`git log --format='%h %s' fb842671..HEAD`). The final HEAD's CI run, CI#39104364530 on `275950dc0`, green 17/17, is recorded in `evidence/operator-pass.md` and `evidence/ci.md` — the record P7 re-verifies.
  - Implement's P4 report is held whole in this window and persisted as `.andromeda/runs/2026-10-18T06-16-11-implement/outcome-2026-10-18-upstream-sync-determinism-and-traces.md` (green + smoke, 1 iteration, no deviation from the plan's steps).
  - The operator directive between implement and this report ran the operator pass in plan order and changed no code; its quote is under Decisions & corrections.
  - Post-implement artifacts: `evidence/operator-pass.md` and `evidence/ci.md`.
- **Process hygiene:**
  - At implement P4: `census: none running` (`seven_guis_native`, `escher-session`, `cargo`, `rustc`, `agent-run.sh`). The windowed `seven_guis_native` of the smoke was terminated by its `timeout 10`, and agent-run cleaned up.
  - Re-measured at this wrap: `gate.py census --comm seven_guis_native --comm escher-session --comm cargo --comm rustc --args agent-run.sh --run-dir .andromeda/runs/2026-10-18T06-47-20-wrap …` → `census: none running`.
  - The operator pass's entries 28 (the fast leg) and 30 (the `gh` read) ended with their calls.
## Addendum — resumed 2026-10-18
- inputs#I7 · message: the operator, 2026-10-18 06:53Z, at the wrap resume · copy · n/a — the resume directive: (a) the founder is away; F1 = B stands as the delegate's PROVISIONAL answer and joins the epoch batch listed for his word, with no card waited on; (b) test-plan §9 moves from the masters' recorded tallies straight to CI#39104364530, the skipped engine-debts reading named in the amendment; (c) P5 moves the three CARRYs to «Upstream sync ahead of visual lint and real-app readiness», with the owner title on `architecture.md:90`; then through to the commit, no next skill.
  - bears on **Dependencies**: adds to it — F1's delegate answer B stays PROVISIONAL and is listed in the founder's epoch batch, no escalation card raised for it at this wrap.
  - bears on **Counts / qualifiers moved**: adds to it — «Per-OS test tallies» and «The workspace tally» move from CI#39056637407 · 187 · 971 straight to CI#39104364530 · 192 · 991, the amendment naming the engine-debts reading (CI#39100630123, 192 · 990) as skipped, never written.
  - bears on **Expected amendments (from plan)**: adds to it — the three CARRYs' entry is carried to P5 on the operator's direction, its owner «Upstream sync ahead of visual lint and real-app readiness», and the CARRY 3 owner title at `architecture.md:90` moves to it.
- inputs.py verify, re-fired: `inputs: 7 entries — unchanged 0 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 7 · uncited 1 · unparsed 0`; moved since the report: a new entry I7, uncited before this addendum (I6 cited since the report: uncited 1 → I7 alone).

## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: fb842671 (the parent of the oldest pre-CI commit 275950dc) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### packages/blitz-dom/src/document.rs — added 17 line(s) in 3 range(s)
added: 2264-2266 · 2269-2272 · 3132-3141
  - 3133-3141 @3135 «fn hovering_below_content_reports_default_cursor() {»
