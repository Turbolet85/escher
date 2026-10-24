# P5 controls — 2026-10-24-upstream-sync-polish-and-ship

Every control fired AS ITS ENTRY (`gate.py run --plan escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/plan.md
--entry {n} --project-root {clone}`, no run dir) over a `git clone --shared` of the project at HEAD
`a356f908661f55cf866d3341337a54a79ca7ac25`, in each script's own `$(mktemp -d)`: `.andromeda/runs/2026-10-24T09-31-30-phase/p5-controls.sh`
(2026-10-24T11:30Z) and `.andromeda/runs/2026-10-24T09-31-30-phase/p5-controls-2.sh` (11:31Z). The clone role "take" is HEAD with
step 2's ten no-commit cherry-picks (the `28f848d3b` conflict in `construct.rs` resolved to upstream's line plus
`RangeInputData`) and step 3's `cargo update -p tokio` (`Updating tokio v1.53.2 -> v1.54.1`). A must-fail input is
planted in that clone, then reverted. Logs under `$TMPDIR/andromeda-gate/2026-10-24-upstream-sync-polish-and-ship/run-20261024T1130*Z/`.

## Must-pass — the take
- entry 1 (exact take): `green · exit 0 · 0 B` — no output.
- entry 2 (new files byte-identical): `green · exit 0`.
- entry 3 (the rename by upstream's lines): `green · exit 0 · 0 B`.
- entry 4 (construct.rs, six lines): under `['exit 0', 'last line 6']` it read `red · exit 0 ✗ (exit 1)`, its log `6`. Under `pipefail`, `diff` exits 1 whenever the sides differ, so the `expect` was re-spelt `['exit 1', 'last line 6']`. Re-fired by `p5-controls-2.sh`: `green · exit 1 · 2 B` (its log `6`).
- entry 5 (the resolved import line): `green · exit 0 · 2 B`.
- entry 6 (preservation guard): `green · exit 0`.
- entry 8 (lock numstat): `green · exit 0 · 8 B` (`+15 -14`).
- entry 9 (no lock name moved): `green · exit 1 · 2 B` (`0`).
- entry 10 (tokio fixed): `green · exit 0 · 12 B` (`tokio fixed`).
- entry 13 (the example's three env names): `green · exit 0 · 2 B` (`3`).
- entry 22 (145 files): `green · exit 0 · 4 B` (`145`).
- entry 25 (the audit): `green · exit 0 · 1538 B` — `advisories ok, licenses ok`.

## Must-fail — one planted input on the take
- entry 1 — our first changed-set insert in `mutator.rs` (`self.doc.changed_nodes.insert(parent_id);`) dropped: `red · exit 0 ✗ (exit 1) · 65 B`.
- entry 4 — `RangeInputData` dropped from the resolved line: `red · last line '6' ✗ (last: '4')`.
- entry 6 — a line appended to `.github/scripts/ci-leg.sh`: `red · exit 0 ✗ (exit 1)`.
- entry 7 — `planted = { git = "https://example.invalid/planted" }` appended to `Cargo.toml`: `red · exit 1 ✗ (exit 0)`.
- entry 9 — a `[[package]] name = "planted"` entry appended to `Cargo.lock`: `red · exit 1 ✗ (exit 0)`.
- entry 12 — `fn _planted() { let _ = std::time::Instant::now(); }` appended to `packages/blitz-dom/src/util.rs`: `red · exit 1 ✗ (exit 0)`.
- entry 15 — the first `--ignore-filename-regex 'keyboard_compat\.rs$'` dropped from `ci-leg.sh`: `red · last line '2' ✗ (last: '1')`.
- entry 30 — `tests/blitz-tests/tests/stand_planted.rs` minted: `red · last line '48' ✗ (last: '49')`.

## Typed controls (the entry's `run` with one sha operand changed; `/usr/bin/bash -o pipefail -c`, 11:31Z)
- entry 11 (the fixed-sha census of the ten) over `286874cc101cfaef2395b56e93c4f27887c38338` (the Dioxus sync, outside the take): `10`, exit 0 — the census counts log sites where they are (its `trace!` lines), against the take's `1`.
- entry 14 (`keyboard_compat.rs` at `286874cc1`) over its parent `286874cc101cfaef2395b56e93c4f27887c38338^`: `1`, exit 0 — the file stands there, against `0` at the commit.
