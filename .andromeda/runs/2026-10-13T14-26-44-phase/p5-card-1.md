# P5 review — 2026-10-13-upstream-sync-polish-and-ship

**Chunk:** Upstream sync ahead of polish and ship (working-route.md:86, promoted `pending`, master-route.md:46)
**Plan:** `escher-0.1.0/chunks/2026-10-13-upstream-sync-polish-and-ship/plan.md` (12 steps · 50 gate entries, 4 of them operator · 15 acceptance criteria) · research.md · scope.md

## Goal
Merge DioxusLabs/blitz `upstream/main` at the pin `434e06615` into `build/escher-0.1.0` as one merge commit with both parents. The merge is 16 commits and 45 files past the base `7832c177`. Then prove that our logic survived it. No mutation tier runs (the founder's word, inputs#I2).

## What the merge meets (P3, read against the code)
- **Two conflicts, both in dioxus-native-dom.**
  - `lib.rs`: both sides' re-exports are kept.
  - `mutation_writer.rs`: upstream's #1188 `is_bool_attr` replaces our `BOOLEAN_ATTRIBUTES`. This is a de-fork. Upstream's list holds all 27 of our names plus `inert`, so a falsy `inert` is cleared too.
- **#1185 drops a node Dioxus removes.** Our unit test `a_removed_element_reads_no_id` (element_id.rs:396-431) asserts the old detached lifetime and turns red. It is brought to the new lifetime: the removed node no longer resolves and reads no id. Architecture [DOM semantics] ("node_id_mapping kept for reuse") becomes stale, and the plan records that as an amendment for the wrap.
- **The textually merged lock refuses `--locked`.** It lists `escher-driver` and `escher-telemetry` at beta.2 while the workspace moves to beta.3. `cargo update --workspace` refreshes those two entries only. A gate entry diffs the lock against upstream's lock at the pin and asserts that no third-party package moved.
- **#1186 lands on CSS, not on our by-name reads.** `attr()` matches the local name alone, so element ids and the aria state reads are unmoved. Unprefixed attribute selectors (`input[type=…]`, `[dir]`, `details[open]`, …) now match Dioxus elements for the first time. No lean-task screen is predicted to move. Circle Drawer's own `.cd-dialog input[type="range"]` rule starts applying, which the report records for "Stand requirement sweep".
- **Predicted, to be measured.** Taffy 0.15 (git → registry) and #1175 (button inputs as flow roots) may move stand bounds, snapshot sizes or pixels. A moved number goes to `evidence/moved-readings.md` and waits on your word; it is never loosened.
- **Our merge surfaces don't meet upstream's hunks** (hunk ranges compared, all apart). These are the mask in `render.rs`, the readers and the bounds reader in `document.rs`, `hit_inner` in `node.rs`, the four `dioxus_document.rs` hunks, `scrolling.rs`, `accessibility.rs` and `writing_mode.rs`. Each is guarded by a count entry.
- **Front page:** `README.md:48` changes to eleven driver verbs and the built screenshot (the folded CARRY).

## Touchpoints
`packages/dioxus-native-dom/src/{lib,mutation_writer,element_id}.rs` · `Cargo.lock` (merge plus two entries) · `README.md` · `tests/blitz-tests/tests/common/mod.rs` only on your word under step 10. Everything else arrives by the merge alone: 40 files, upstream's five new test files among them, each its own target.

## Test commands (headline)
- **Merge witness and guards:** no conflict marker; pins taken as one set; lock set equal to upstream's; fork-owned sources untouched (`git diff --quiet a7031368 -- …`); our hunks counted present; conflict resolutions in place; fixed-sha census of upstream's delta.
- **Workspace build:** `ci-leg.sh fast`. Its tally is predicted at 179 lines and 893 passed (+44 upstream tests). The walk reads 0 test files absent by name. Upstream's five files each ran once. No `all` target ran or was built.
- **Per-package build:** `agent-run.sh run stand` / `run all`, `cargo test -p seven_guis` (sink needles), and `cargo test -p dioxus-native-dom` (step 6's test plus upstream's new bridge tests).
- **Legs:** `ci-leg.sh doc`, `audit` (`advisories ok`) and `a11y` (3 lines, 15 passed). Also adapter absence, the two feature reads, and the windowed boot smoke.
- **Operator:** hygiene, push behind `fast` and a clean-tree guard, `ci.py conclusion --sha HEAD --wait 3600` (`verdict: green`), and a job-log read.
- **Baselines** (2026-10-13T14:49–14:56Z):
  - 43 entries were fired.
  - 12 read red. Each red is the subject this chunk builds: the pin, the new pins, the lock, the conflict lines, the README, and upstream's test names.
  - Every green guard has a known-positive control.
  - Every red that reads a literal has a must-pass reading over the trial-merge tree `2481e052`.

## Leans (decisive, stated in the plan's Provenance — no fork asked)
- Take upstream's falsy-attribute predicate. Our own trap says the copy goes stale, and upstream's list is a superset.
- No escalate pattern holds:
  - Upstream's delta adds no listener, env read or carrier.
  - #1189 shares only the program's and the embedder's UA CSS between documents.
  - #1186 and #1188 change in-process styling and attribute writes.
  - No PROVISIONAL mark is touched.
- Bring our removed-element test to the new lifetime rather than revert #1185.

## Concretization preview
None. A sync makes no 0.1.0 capability verifiable, so this chunk claims no matrix entry. The one unclaimed capability, `v010-15`, belongs to "Cold-agent test".

## Beyond the plan
none
