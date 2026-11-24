# P5 controls — 2026-11-24-upstream-sync-ahead-of-inside-the-canvas

Scratch clone: `git clone --shared` of the project at `3617bc017` into `$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/7a194ffc-c6ac-40af-a3ff-45312ca78ca4/scratchpad/ctl-clone`. The merge was staged there with `git -c rerere.enabled=false merge --no-commit --no-ff 29a439e0563cbed6252c8f1d8ab4ed68afe03968`, which exits 1 with 1 unmerged path, `packages/dioxus-native-dom/src/events.rs`, as research read it. Each control was fired as its entry: `gate.py run --chunk … --docs-root . --project-root {clone} --entry {n}`, with no run dir.

| Entry | On the tree at `3617bc017` (baseline) | In the clone, merge staged (role: must-pass / known positive) | Input |
|---|---|---|---|
| 1 MERGE_HEAD | red — exit 1, no MERGE_HEAD | green — exit 0 (must-pass) | the staged merge |
| 2 conflict markers | green — exit 1, no output | red — exit 0, 300 B of markers (known positive) | the unresolved `events.rs` |
| 3 left-out set absent | green — exit 0 | green with the merge (must-pass); red — exit 1 with an `AGENTS.md` minted (known positive) | the staged merge; then `touch AGENTS.md` |
| 4 no git source in the lock | red — exit 0, last line 1 (taffy's git entry) | green — exit 1, last line 0 (must-pass) | the staged merge's `Cargo.lock` |
| 5 Dioxus `"0.7.3"` ×13 | green — last line 13 | green with the merge (must-pass); red — last line 12 with one line edited to `"0.8.0-alpha.3"` (known positive) | the staged merge; then the one edit |
| 6 lock shortstat | red — no output | green — contains `1 file changed, 4 insertions(+), 2 deletions(-)` (must-pass; the literal from the trial's numstat `4 2`) | the staged merge's `Cargo.lock` |
| 16 re-check present | green — exit 0 | red — exit 2, the file absent from the clone (known positive; `{work_dir}` is gitignored) | the clone's absent evidence |
