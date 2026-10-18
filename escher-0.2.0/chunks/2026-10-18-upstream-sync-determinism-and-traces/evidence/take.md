# The take — 2026-10-18-upstream-sync-determinism-and-traces

Made by /andromeda-implement, run `.andromeda/runs/2026-10-18T06-16-11-implement/`, 2026-10-18T06:17Z, from the project
root on HEAD `fb842671137fc4f5e48059f269e796b57cf25017`.

## The command
```
git cherry-pick --no-commit fb8d6c15e9ce062b3b41e71cd48988f0093506cc
```
- Output: `Auto-merging packages/blitz-dom/src/document.rs`, **exit 0**.
- No conflict, so no hand edit was made. `.git/CHERRY_PICK_HEAD` is absent after the call, and no commit was written.
- Taken: `fb8d6c15e` "Show the default cursor when the pointer is over no box (#1220)", by Nico Burns.

## `git diff --cached --stat`
```
 packages/blitz-dom/src/document.rs | 20 +++++++++++++++++---
 1 file changed, 17 insertions(+), 3 deletions(-)
```
`git diff --cached --name-only` lists `packages/blitz-dom/src/document.rs` alone.

Hunks against HEAD (`-U0`): `@@ -2264,2 +2264,3 @@` and `@@ -2268 +2269,4 @@` in `get_cursor`, and
`@@ -3127,0 +3132,10 @@` in `mod hover_state_tests` (the new `hovering_below_content_reports_default_cursor`). The line map
is in `evidence/readings.md`.

## The entries that prove it (trail `.andromeda/runs/2026-10-18T06-16-11-implement/gate-2026-10-18-upstream-sync-determinism-and-traces.json`)
```
  1 probe       green · exit 0 · 0.01s · 0 B → 1.log · git show fb8d6c15e9ce062b3b41e71cd48988f0093506cc -- packa… (110 chars)
  2 probe       green · exit 0 · 0.01s · 0 B → 2.log · diff <(git diff fb842671137fc4f5e48059f269e796b57cf25017 -… (232 chars)
  3 probe       green · exit 0 · 0.01s · 0 B → 3.log · git diff --quiet fb842671137fc4f5e48059f269e796b57cf25017 … (189 chars)
```
- Entry 1 (patch present): upstream's patch reverse-applies to the work tree.
- Entry 2 (exact take): no output. The chunk's `document.rs` delta is upstream's lines and nothing else.
- Entry 3 (preservation guard): nothing outside `document.rs` moved.

Nothing of ours in `document.rs` changed. The merge base stays `23b9b26a9`. The four Taffy-main commits (`1a11761bc`,
`4a5ab9cc9`, `510b592ab`, `1e5479ae6`) are not taken and wait for the next sync, on F1 B (delegate the operator,
2026-10-18, PROVISIONAL; inputs#I5).
