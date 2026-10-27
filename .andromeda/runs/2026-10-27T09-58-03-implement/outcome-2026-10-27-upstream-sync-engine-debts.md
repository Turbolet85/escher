# /andromeda-implement — 2026-10-27-upstream-sync-engine-debts

**Outcome: green + smoke.** The founder's «Одиннадцать коммитов (Рекомендую)» take is in the working tree (uncommitted). Next → the OPERATOR PASS, entries 32–35, then `/andromeda-wrap-session`.

## What was done
- **The take** (`evidence/take.md`): eleven `git -c rerere.enabled=false cherry-pick --no-commit`, in upstream order. Eight picked clean. Three conflicted in one hunk each, as the plan predicted, and were resolved as step 2 says:
  - `e6f4615ec` `runtime.rs`: upstream's side whole, with our select-by-id comment moved to `runtime/events.rs:171-173`.
  - `7fc9ee3af` `dioxus_document.rs`: our side plus `input_focused_on_mount_enables_ime` alone. `task_polled_during_render_can_access_document` (`286874cc1`) was dropped. The result removes 0 lines against the base.
  - `53e9d81c6` `mutator.rs`: the range arm and the `open` arm as two independent `if`s, ours first.
  - `.git/rr-cache` held 16 entries before and after the take.
- **The lock** (`evidence/lock.md`): `Cargo.toml`, `Cargo.lock` and `deny.toml` are byte-identical to `de85792f5`, with no git dependency. Every standing ignore is still encountered, and the audit shows its two standing warnings. The sixteen commits left out are listed with their reasons.
- **Gates**: one run, 35 entries. 30 green, 1 recorded (CI scripts: Ran 108 · OK), 0 red. The 4 operator legs were not run.
  - The exact-take entry shows no output. The file-identity entry shows 0 differing files. `events.rs` reads `+2 -2`.
  - Workspace tally: **209 · 1125 · 0 · 12**, as predicted (`evidence/counts.md` names each moved count). `run all` reads 633 · 0 · 12 (+3). Stand: 205 · 0 · 7 over 50 files. a11y: 14 · 85 · 0.
  - `fast`, `doc` and `audit` are green.
- **Readings** (`evidence/readings.md`): the engine-debts files and the clock files are untouched. The select-model files are reached where predicted. Each row carries its owner, `.andromeda/residuals.md:36`, quoted. Nothing was sent upstream.
- **Sink** (`evidence/sink.md`): the fixed-sha census has 2 hits, both named. The clock invariant reads 0. The silence census of the new test reads 0 · 0 · 0. `host_log` and `mcp_log` pass under both builds. escher-telemetry and escher-driver are unchanged.
- **Smoke**: entry 31 (`role = 'smoke'`, the windowed `seven_guis_native`) ran as a P2 gate and passed. `WAYLAND_DISPLAY` was set and the log printed `service.name=seven_guis`. Not re-run at P3.
- **Matrix**: `v030-03` moved planned → implemented. Its ref names the local entries and names the fork-CI clause as operator entry 34.

## Files changed (41, all on research's lists; scope `clean`)
- **New (23):**
  - `packages/blitz-vibey-script/src/dom/element/{attributes,form_controls,geometry,markup,scrolling,selectors}.rs`
  - `packages/blitz-vibey-script/src/runtime/{bootstrap,css,events,fetch,modules,timers,window}.rs` and the nine `runtime/bootstrap/*.js`
  - `tests/blitz-tests/tests/pseudo_element_flow_classification.rs`
- **Modified (18):**
  - `.github/workflows/publish-browser.yml`
  - `apps/browser/src/{main,tab}.rs`
  - `wpt/WPT_COMMIT`
  - blitz-dom `layout/construct.rs`, `mutator.rs`, `node/element.rs`, `stylo.rs`
  - blitz-shell `lib.rs`, `window.rs`
  - blitz-vibey-script `document.rs`, `fetch.rs`, `lib.rs`, `dom/element.rs`, `runtime.rs`, `tests/dom.rs`
  - `packages/dioxus-native-dom/src/dioxus_document.rs`
  - `tests/blitz-tests/tests/all.rs`
- **Evidence:** `escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/{before,take,lock,readings,sink,counts}.md`. `ci.md` is the operator pass's.
- **Ledger:** `escher-0.3.0/verification-matrix.json` (`v030-03`).

## Deviations from plan
- none. No scope-record line was needed: every file was on research's lists. The three resolutions are the plan's own, recorded as non-additive edits in `evidence/take.md`. There was no further gate-driven edit.
- A factual note for the wrap: the trial worktree `target/tmp/trial-wt` is detached at `02b24fccf` (shape A), not `fb1f87191` (shape B) as research.md and the plan's notes say. Left as it is; its cleanup is the founder's.

## Guard entries
- none. The plan has no `gate.py guard` entry. Its preservation guard is entry 5, a `git diff --quiet`, which is green.

## Surfaced / stuck
- none.

## Next — the operator pass
- **32** `gate.py hygiene` (fired as written, before the commit)
- the pre-CI commit, subject exactly `chore(2026-10-27-upstream-sync-engine-debts): operator pre-CI commit, for the run this chunk's verdict reads`. Its body names the take on the founder's «Одиннадцать коммитов (Рекомендую)» and carries the eleven `(cherry picked from commit {full sha})` trailers in step 2's order.
- **33** fast, the clean-tree guard, then the push
- **34** `ci.py conclusion --sha HEAD --wait 2400`
- **35** the per-OS tallies with the run id → `evidence/ci.md` against CI#40434241601

Then `/andromeda-wrap-session`.

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo / rustc / blitz-tests binaries (entries 10–28) | a leg (this run's gate entries) | terminated — census: none running |
| `seven_guis_native` (entry 31) | a leg (this run's smoke entry) | terminated (timeout's 124) — census: none running |
| `escher-session` hosts (seven_guis checks) | a leg (entry 28 and `fast`) | terminated — census: none running |

Census: `gate.py census --comm seven_guis_native --comm escher-session --comm cargo --comm rustc --args blitz-tests --args agent-run` → `census: none running` (own tree excluded: 8).

## Decisions & corrections
- The operator's directives, followed:
  - The trial worktree was left untouched.
  - No fork arose, so no dialog was needed.
  - No gate was red, so the stop-before-re-run rule never applied.
- The stamp-ahead hook caught a time I had estimated in `evidence/take.md` (10:05Z against a clock of 10:00:41Z). I corrected it from `date -u` in one edit.
- bash-guard refused a `cd` into the skill's references dir at Setup. I re-issued the command with an absolute path.
