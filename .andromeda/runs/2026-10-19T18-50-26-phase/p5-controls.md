# P5 controls — 2026-10-19-upstream-sync-visual-lint-and-real-app-readiness

Scratch clone: `$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/{session}/scratchpad/clone`, `git clone --shared` of the project at HEAD `67f17b9d632ceb66f6c01b347d9f9ba036dd29b6`, `refs/remotes/upstream/main` fetched into it. Every control fired AS THE ENTRY through `gate.py run --plan {plan} --entry {n} --project-root {clone}` with no `--run-dir`, unless marked typed.

## The take, as steps 2–3 write it (in the clone)
- `git cherry-pick --no-commit 17de049b1…`: exit 0.
- `git cherry-pick --no-commit 38ca4e777…`: exit 1, `CONFLICT (content): Merge conflict in packages/blitz-dom/src/mutator.rs`, auto-merged `document.rs`, `events/keyboard.rs`, `node/node.rs`; staged `stylo.rs`, `all.rs`, `focus_redraw.rs`, `focus_within.rs`, `WPT_COMMIT`.
- Resolved by keeping ours then upstream's (one block: ours 3 lines, theirs 6 lines), `git add packages/blitz-dom/src/mutator.rs`, `git cherry-pick --quit`: exit 0; `git status` lists changes to be committed and no cherry-pick in progress.

## Must-pass (the clone after the take)
- entry 1 · `green · exit 0 · 0.01s · 0 B` — the exact take reads no output.
- entry 2 · `green · exit 0 · 0.01s · 0 B` — nothing outside the take moved.
- entry 5 · `green · exit 1 · 2 B` (log `0`).
- entry 6 · `green · exit 1 · 2 B` (log `0`).
- entry 9 · `green · exit 1 · 2 B` (log `0`) — the stub is gone.
- entry 17 · `green · exit 0 · 4 B` (log `138`).
- entry 25 · `green · exit 0 · 3 B` (log `44`).

## Must-fail (the same clone, with control inputs left uncommitted)
Inputs: our `self.doc.changed_nodes.insert(parent_id);` line dropped from `mutator.rs`; `# control` appended to `.github/scripts/ci-leg.sh`; `Cargo.toml`'s `taffy` line replaced by `upstream/main:Cargo.toml:100` (git Taffy `ea07c4c3`); `fn _control() { let _ = std::time::Instant::now(); }` appended to `packages/blitz-dom/src/util.rs`.
- entry 1 · `red · exit 0 ✗ (exit 1) · 350 B` — its log names all three code-path inputs: the two `taffy` lines, `-            self.doc.changed_nodes.insert(parent_id);`, `+fn _control() { … Instant::now(); }`.
- entry 2 · `red · exit 0 ✗ (exit 1)` — `ci-leg.sh`, `Cargo.toml` and `util.rs` lie outside the take's exclusions.
- entry 3 · `red · exit 1 ✗ (exit 0)` (log `1`) — the planted git Taffy.
- entry 4 · `red · exit 0 ✗ (exit 1)` (log `0`).
- entry 6 · `red · exit 1 ✗ (exit 0)` — the planted `Instant::now()`.
- then one `--ignore-filename-regex 'keyboard_compat\.rs$'` dropped from `ci-leg.sh`: entry 8 · `red · last line '2' ✗ (last: '1')`.
- then `tests/blitz-tests/tests/stand_control.rs` minted: entry 25 · `red · last line '44' ✗ (last: '45')`.

## Typed controls (input operand only, `bash -o pipefail`, on the real tree)
- entry 5 with the sha `36877a23` (one of ours, which adds tracing sites): `16`, exit 0 — the red side.
- entry 7 with the path `apps/bump/Cargo.toml` at `17de049b1` (retired upstream): `0`, exit 0 — the red side.

## Entry 34 (operator, the per-OS tally read; check 4 (13) log arm)
Its `run` with `<id>` = `39302945352` (the base's completed green run), under `bash -o pipefail`, 2026-10-19T19:08Z: Test [default features] 197 · 1034 · 0 · 12; Test (macos) 178 · 1031 · 0 · 12; Coverage report 178 · 1031 · 0 · 12; Test (windows) 178 · 989 · 0 · 9; Accessibility (a11y) tests 6 · 43 · 0 · 0; Stand checks repeated 156 · 624 · 0 · 21; exit 0.
