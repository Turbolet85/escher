# P5 controls — 2026-11-16-taffy-past-1372

Each fired as the entry, `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py run --chunk 2026-11-16-taffy-past-1372 --docs-root . --entry {n} --project-root {clone}`, no `--run-dir`, over a `git clone --shared` of the project checked out at `930ac5d6fc9e39677f1bcc6199e92259161fe93b` in the session scratchpad. Script: `$TMPDIR/{scratchpad}/controls.sh`.

## clone-pass (`$TMPDIR/{scratchpad}/clone-pass`, HEAD 930ac5d6f)

Unmodified — reads as on the real tree:
- 1 probe red · exit 0 ✗ (exit 1) · exit 1 — the manifests unmoved
- 2 probe green · exit 0 — one git source, Taffy at e90751fd, on the untouched tree
- 3 probe red · exit 0 ✗ (exit 1) · exit 1 — no taffy source at a3997d55 yet
- 4 probe red · exit 0 ✗ (exit 1) · exit 1 — the lock unmoved
- 5 probe red · exit 0 ✗ (exit 1) · exit 1 — no code delta yet

Must-pass — input: `git apply --index .andromeda/runs/2026-11-16T09-58-46-phase/take.patch` (step 1 as the plan builds it):
- 1 probe green · exit 0 (last line 2)
- 2 probe green · exit 0 (last line 1)
- 3 probe green · exit 0 (last line 1)
- 4 probe green · exit 0 (last line 2)
- 5 probe green · exit 0 (last line 16)

## clone-fail (`$TMPDIR/{scratchpad}/clone-fail`, HEAD 930ac5d6f)

Must-fail — input: the patch applied, then a second manifest line (`dioxus-core-macro` 0.7.3 → 0.7.2 in `Cargo.toml`), a second git source (a `zz-probe` package appended to `Cargo.lock`), the taffy source set back to `e90751fd`, and a 17th file (one comment line in `packages/blitz-dom/src/lib.rs`):
- 1 probe red · last line '2' ✗ (last: '4')
- 2 probe red · last line '1' ✗ (last: '2')
- 3 probe red · exit 0 ✗ (exit 1) · exit 1
- 4 probe red · last line '2' ✗ (last: '4')
- 5 probe red · last line '16' ✗ (last: '17')

## Entry 6 (the step-3 check)

Its red before green is implement's: the test does not exist on the untouched tree (the baseline reads it), and step 6's one-shot `gate.py control` reads it red with step 2 reverted.
