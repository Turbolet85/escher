# P5 controls — 2026-11-15-taffy-from-upstream-git

The six probe entries (1–6) were fired as the entries with `gate.py run --chunk 2026-11-15-taffy-from-upstream-git
--docs-root . --entry {n} --project-root $TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/0e7ab680-cb3d-469a-a3b1-2348a2b4cf5d/scratchpad/ctl-clone`
over a `git clone --shared` scratch clone at HEAD `0dba5035b`. The script was
`$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/0e7ab680-cb3d-469a-a3b1-2348a2b4cf5d/scratchpad/controls.sh`.
The pin input is the P3 trial's staged diff
(`$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/0e7ab680-cb3d-469a-a3b1-2348a2b4cf5d/scratchpad/trial-pin.patch`:
`Cargo.toml`, `Cargo.lock`, `packages/blitz-dom/src/layout/inline/layout.rs`, +4 −4), applied uncommitted.
Every input lived in the clone.

| input (clone HEAD 0dba5035b) | entry | reading |
|---|---|---|
| unmodified | 1–6 | red · exit 1 · count 0, each — the same as on the tree |
| the pin taken | 1–6 | green · exit 0, each (last lines 2 · 1 · 1 · 5 · 1 · 3) — the must-pass |
| the pin + `thin-vec = "0.2.1"` in `Cargo.toml` | 1 | red · last line 4 (expects 2) |
| the pin + `slotmap` moved to a git source in `Cargo.lock` | 2 | red · last line 2 (expects 1) |
| the pin + `slotmap` moved to a git source in `Cargo.lock` | 4 | red · last line 7 (expects 5) |
| the pin + taffy at another rev (`1009534e…`) in `Cargo.lock` | 3 | red · exit 1 · count 0 |
| the pin + a fourth file changed (`packages/blitz-dom/src/lib.rs`) | 6 | red · last line 4 (expects 3) |

Entry 5's must-fail input is the untouched tree itself: the line is absent, count 0.
