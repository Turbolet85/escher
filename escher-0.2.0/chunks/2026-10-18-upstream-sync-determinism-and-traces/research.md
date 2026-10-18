# Codebase Research — 2026-10-18-upstream-sync-determinism-and-traces

## Scope
- **Depth:** moderate · **Reads:** 9 · **Globs/Greps:** 14
- **Harness rules consulted:** none — no live leg in this chunk; the windowed smoke entry's form is the one the 2026-10-17 sync's plan carried from `.claude/rules/verification-harness.md` Session Additions 2026-10-06
- **Platform issues consulted:** none — no runner-only bullet (Setup 5a read three greens) and no CI-reading entry outside the operator leg
- **External inputs:**
  - `inputs#I1` — the operator's directive at this take-up: the founder's forks go to one `founder-forks.md` in the run dir, answered by the operator as delegate, recorded PROVISIONAL; technical forks in the dialog; the pre-CI commit subject is the guide form exactly.
  - `inputs#I2` — upstream/main `fb8d6c15e`, 5 commits past `23b9b26a9`; the per-commit file lists; the git Taffy at `upstream/main:Cargo.toml:100`; `keyboard_compat.rs` still listed; the three in-memory trial merges, each exit 0 with no conflicted file; crates.io's newest `taffy` 0.15.0.
  - `inputs#I3` — Taffy `ea07c4c3` is v0.15.0 plus 14 unreleased commits (floats #1303/#1304/#1305, grid #1320/#1322, baselines #1323, flexbox #1327/#1328/#1333/#1332/#1335, benchmarks, a zip requirement); the stand holds 0 `float:` declarations; upstream's lock delta.
  - `inputs#I4` — Taffy `ea07c4c3` and `v0.15.0` both declare `version = "0.15.0"` and `rust-version = "1.71"`.

## Files inspected
- `packages/blitz-dom/src/document.rs` (2030-2045, 2255-2300, 3041-3075; symbol lines by grep) — `update_cursor` (:2039) hands `get_cursor()` to `shell_provider.set_cursor`; `get_cursor` (:2260) returns `None` through `?` when neither `hover_hit_node_id` (filtered by `contains_key`) nor `get_hover_node_id()` resolves — the `None` `fb8d6c15e` turns into `Some(CursorIcon::Default)`. F8's functions sit below it and are not in upstream's hunk: `get_client_bounding_rect` :2373, `transformed_bounds` :2410, `is_transformed_chain` :2455, `transformed_fragment_rects` :2465, `node_client_rects` :2564. `hover_state_tests` opens at :3041; its `make_doc` builds with the two-argument `qual_name!(…, html)` (:3052-3068). Our `document.rs` holds 0 `eprintln` (`grep -c eprintln`), upstream's at `23b9b26a9` and at `fb8d6c15e` 2 each; both trial-merge trees hold 0 and the new test once.
- `packages/blitz-shell/src/lib.rs` (101-110) — `BlitzShellProvider::set_cursor`: `Some(icon)` shows the window cursor and sets it; `None` sets `Default` and HIDES the cursor (`set_cursor_visible(false)`). `fb8d6c15e`'s effect, windowed only: the pointer over no box keeps a visible default cursor where it was hidden.
- `packages/blitz-traits/src/shell.rs` (13) — the default `ShellProvider::set_cursor` is a no-op; every headless document (harness, stand, driver session) uses the `Dummy*` default.
- `tests/blitz-tests/tests/pointer_leave.rs` (13, 57-61, 225-241) — upstream's check (in `23b9b26a9`): a recording `ShellProvider`, asserting `!cursors().contains(&None)` and the last cursor `Pointer`. It is the standing check `fb8d6c15e` keeps green.
- `packages/blitz-dom/src/layout/inline.rs` (740-765) — `block_ctx.place_floated_box(output.size + margin_sum, min_y, direction, clear, false)` at :752; Taffy main's API adds `crate::taffy_node_id(node_id).into()` as its first argument (Taffy #1303, inputs#I3). The call runs only for a floated box.
- `Cargo.toml` (105) — `taffy = { version = "0.15", default-features = false, features = [` with the eight features below it; upstream's line 100 is the same with `git = "https://github.com/DioxusLabs/taffy", rev = "ea07c4c3…"` in place of `version = "0.15"`, the feature list unchanged — one line changes, so no root-manifest citation below it moves.
- `Cargo.lock` (2132-2138, 8038-8040) — `cssparser-macros` 0.7.1 depends on `syn 3.0.6`; `taffy` 0.15.0 from the registry with a checksum. 24 `"syn 3.0.6"` dependency lines here, 23 in upstream's lock (`grep -c '"syn 3.0.6"'`): the one upstream moved is `cssparser-macros`'s.
- `deny.toml` (whole, by grep) — 0 `sources` lines (`grep -c sources deny.toml`): the audit leg cannot see a git source.
- `.github/scripts/ci-leg.sh` (41-45) — both `cargo llvm-cov` lines carry `--ignore-filename-regex 'keyboard_compat\.rs$'`, the comment naming it upstream's temporary bridge.

## Graph impact (from the code-graph query)
- **get_cursor** — 4 callers (trace `.andromeda/runs/2026-10-18T05-47-23-phase/tree-query-2026-10-18-upstream-sync-determinism-and-traces.json`, rows 4): `update_cursor @ packages/blitz-dom/src/document.rs:2041`, its own sub-document recursion `@ document.rs:2274`, and two `hover_state_tests` tests `@ document.rs:3113` and `:3126`. No caller outside blitz-dom: escher's snapshot, driver, harness and stand read no cursor (grep `get_cursor|set_cursor` over `packages examples apps tests`: 12 hits, every one in blitz-dom, blitz-shell, blitz-traits or upstream's `pointer_leave.rs`).
- **update_cursor** — 2 callers (same trace, rows 2): `refresh_hover @ document.rs:2021` and `EventDriver::handle_ui_event @ packages/blitz-dom/src/events/driver.rs:306`. The changed value flows only to the shell provider, a no-op headless.
- **place_floated_box** — Taffy's symbol, not the workspace's: no `calls` row (the same query, rows 0 for it); one call site by grep, `inline.rs:752`.

## Patterns detected
- **A sync takes upstream's text and edits ours only where a conflict or a gate requires** (`escher-0.2.0/chunks/2026-10-17-upstream-sync-engine-debts/plan.md` steps; arch-history 2026-10-10-upstream-sync-agent-surfaces): every edit to upstream-owned code is named and minimal.
- **A partial take recorded by its pin** (2026-10-17's `Upstream sync:` line, `23b9b26a9` with the four Taffy commits "waiting"): the line names the taken sha, the commit and the commits left.
- **A measured no-op sync** (master-route.md records 2026-10-06-upstream-sync-observation-model and 2026-10-07-upstream-sync-driver-core: "0 ahead: measured no-op, no merge"): a sync that takes nothing still runs its readings.
- **Evidence in the sync's own files** (2026-10-17's `evidence/{before,merge,lock,counts,readings,sink,ci,operator-pass}.md`): the start readings, the merge record, the lock delta, the re-count, the sink census, the per-OS tallies.

## Conventions to follow
- **Merge, never rebase** — the branch is pushed and shared; each sync landed one merge commit with two parents (2026-10-17 plan, its merge-witness entry).
- **Coupled pins as one set** (CLAUDE.md Critical Warnings): `taffy` moves with upstream's lock, never `cargo update`.
- **Fixed-sha reads in gates** — a gate reading what the chunk changed names its base sha (`fb842671137fc4f5e48059f269e796b57cf25017`, HEAD at this take-up), never `HEAD`.

## New files to create
- none — every new file this chunk writes is evidence under its own chunk folder

## Files to modify
- `packages/blitz-dom/src/document.rs` — upstream's `fb8d6c15e` hunks exactly as written (the `get_cursor` fallback to `Some(CursorIcon::Default)`, one `hover_state_tests` test), by a no-commit cherry-pick (P4 F1 answered B, inputs#I5)

## The fork's rejected branches (P4 F1, answered B)
- `Cargo.toml`, `Cargo.lock` and `packages/blitz-dom/src/layout/inline.rs` would have moved only under option A (the four Taffy-main commits). B leaves them byte-identical to HEAD `fb842671`. The plan's preservation guard reads that.

## Sweeps
- `get_cursor|set_cursor` over `packages examples apps tests` (`command grep -rn --include=*.rs`): 12 hits · 0 changed by us · 12 no-change (blitz-dom's own definition, callers and tests; blitz-shell's and blitz-traits' providers; upstream's `pointer_leave.rs`) — no escher reader.
- `float:` over `examples/seven_guis/src` (`command grep -rn`): 0 hits — the stand floats nothing, so Taffy's three float commits and `place_floated_box`'s argument reach no stand box.
- `display: *flex|display: *grid` over `examples/seven_guis/src`: 27 hits · 0 changed — the stand lays out with flexbox; Taffy's five flexbox and one baseline commit can reach it (a prediction, measured only by a take that carries Taffy main).
- Upstream's Rust delta `git diff 23b9b26a9 upstream/main -- '*.rs'`, added and removed lines matched against `println!|eprintln!|print!|dbg!|std::env|env::var|escher_telemetry|tracing_subscriber|UnixListener|TcpListener|bind\(|tracing::|debug!|trace!|info!|warn!|error!|log::`: 0 of 16 added lines (`grep -cE '^\+[^+]'` = 16) — no log, print, env read, subscriber or listener.
- `eprintln` in `packages/blitz-dom/src/document.rs`: HEAD 0 · `23b9b26a9` 2 · `fb8d6c15e` 2 · trial trees `32c795dd` and `6d7ae612` 0 — the merge keeps our skip-free test module.
- `"syn 3.0.6"` in `Cargo.lock`: HEAD 24 · `upstream/main` 23 — the one line upstream's `4a5ab9cc9` moved, `cssparser-macros`'s.

## Scope premise closure
Every `[inferred]` bullet of `scope.md` re-read against the reads above:
- The sync, the five commits and their files — VERIFIED (inputs#I2; `git log`, `git show --stat`).
- The pin is a fork, three shapes — VERIFIED (`fb8d6c15e^` is `1e5479ae6`, so a merge of the head takes the Taffy commits; a cherry-pick applies clean, tree `6d7ae612`; no Taffy release past 0.15.0, inputs#I2, inputs#I3).
- No textual conflict — VERIFIED (inputs#I2), and the merged `document.rs` keeps our 0 `eprintln` (Sweeps).
- Audit: no `[sources]` table — VERIFIED (`grep -c sources deny.toml`: 0).
- Sink re-check: upstream's Rust delta adds 0 log, print or env lines — VERIFIED (Sweeps, 0 of 16).
- The CARRY 1 re-read — VERIFIED (upstream still names a git Taffy; Taffy main is 14 unreleased commits past 0.15.0, inputs#I3).
- The CARRY 2 re-read — VERIFIED (`keyboard_compat.rs` still listed upstream, inputs#I2).
- The CARRY 3 re-read — VERIFIED: upstream's hunk is `get_cursor` and `hover_state_tests` in `document.rs`, none of F8's five functions (Files inspected), and `layout/inline.rs` is in no row.
- `get_cursor` read by no escher reader — VERIFIED (Graph impact; Sweeps).
- MSRV — VERIFIED and extended: Taffy main declares `rust-version = "1.71"`, as 0.15.0 does (inputs#I4).
- "None in upstream's delta (no `members` change; the lock delta is Taffy's alone)" — PREMISE-CORRECTED: the lock delta is Taffy's source line AND one `cssparser-macros` dependency line (`syn 3.0.6` → `syn 2.0.119`, upstream's `4a5ab9cc9`); no package enters or leaves (inputs#I3). scope.md amended.

## Open questions
- none — the one plan-decision question (which take) was answered at P4: F1 B, cherry-pick `fb8d6c15e` alone, delegate the operator, 2026-10-18, PROVISIONAL (inputs#I5). A scratch-clone trial at P4: `git cherry-pick --no-commit fb8d6c15e` exits 0, stages `packages/blitz-dom/src/document.rs` alone and leaves no `CHERRY_PICK_HEAD`; `git show fb8d6c15e -- packages/blitz-dom/src/document.rs | git apply --check -R` then exits 0.
