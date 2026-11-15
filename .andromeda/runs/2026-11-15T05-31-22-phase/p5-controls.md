# P5 controls — check 4 (9), entries 1–4

These were fired as the entries themselves through `gate.py run --chunk 2026-11-15-upstream-sync-errors-and-explanation --docs-root . --entry {n} --project-root {clone}`, with no `--run-dir`. The clone was `git clone --shared` of the project at HEAD `ba44c3559`, at `$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/869dbb04-7653-4f3f-9b54-59d982138d7d/scratchpad/ctl`. The script is `$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/869dbb04-7653-4f3f-9b54-59d982138d7d/scratchpad/controls.sh`.

## C0 — the unmodified clone (must read as the tree did)
```
  1 probe       green · exit 0 · git diff --quiet ba44c355… -- manifests
  2 probe       green · exit 1 · grep -c 'source = "git+' Cargo.lock
  3 probe       red · exit 0 ✗ (exit 1) · exit 1 · taffy 0.15.1 count
  4 probe       green · exit 1 · taffy delta census
```
These are the same four words the tree read at baseline.

## C1 — the take applied: must-pass
Input: `cherry-pick --no-commit f5ea1caf1 c30096724 d2ee20b98` (clean), plus `Cargo.lock` from the registry trial `target/tmp/trial-eae` (taffy 0.15.1). The diff against HEAD is 5 files changed, 17 insertions(+), 3 deletions(-).
```
  1 probe       green · exit 0
  2 probe       green · exit 1
  3 probe       green · exit 0
  4 probe       green · exit 1
```

## C2 — inputs each entry must fail on
```
  1 probe       red · exit 0 ✗ (exit 1) · exit 1   — a comment line appended to Cargo.toml
  2 probe       red · exit 1 ✗ (exit 0) · exit 0   — a [[package]] with source = "git+https://example.invalid/x#0" appended to Cargo.lock
  4 probe       red · exit 1 ✗ (exit 0) · exit 0   — `tracing::info!("control");` appended to packages/blitz-dom/src/net.rs
  3 probe       red · exit 0 ✗ (exit 1) · exit 1   — Cargo.lock restored to HEAD's (taffy 0.15.0)
```
Each input was reverted after its firing. Every entry reads red on its fail input and green on the take.
