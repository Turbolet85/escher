# P5 controls — 2026-11-18-capability-statement

Each control was fired as its entry, `gate.py run --chunk 2026-11-18-capability-statement --docs-root {root} --entry {n}
--project-root {clone}`, over a scratch clone. The clone is `git clone --shared` of the project at HEAD
`6ebf42e65b5c95930fde09baace1ac532ae5e2ec`, at `$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/67503691-ec1b-4471-9d72-9dce6cf12f5a/scratchpad/p5clone`.
The scripts are `…/scratchpad/p5-controls.sh` and `…/scratchpad/p5-control-guard.sh`. Every input was reverted in the
clone after its reading, and `git status --short` read empty after each script.

| entry | clone input | reading |
|---|---|---|
| 3 | unmodified | red · exit 1 ✗ (exit 0), 8 lines, as on the tree |
| 3 | every stale token replaced (seventeen, eighteen, 0.3.0, the layout-carrier line) | green · exit 1, 0 B (must-pass) |
| 4 | unmodified | red · exit 1 ✗ (exit 0), 6 lines, as on the tree |
| 4 | `.andromeda/residuals.md` re-pointed in `deny.toml` and the page | green · exit 1, 0 B (must-pass) |
| 5 | unmodified | green · exit 1, 0 B, as on the tree |
| 5 | `planted /home/<user>/path` appended to the page | red · exit 1 ✗ (exit 0), 52 B (known positive) |
| 6 | unmodified | green · guard: clean, as on the tree |
| 6 | `docs/capabilities.md` edited | green · guard: allowed 1, clean (must-pass) |
| 6 | `docs/capabilities.md` and `.claude/docs/tests-summary.md` edited | green · changed 2 · excluded 1 — the guard excludes `.claude/` by its own rule, so it does not witness the leaves |
| 6 | `docs/capabilities.md` and `.github/scripts/ci-leg.sh` edited | red · `guard: outside 1 files` (must-fail) |

After the review fold (inputs#I2), the guard is entry 7 and the new leaf entry is entry 6. Its control script is
`…/scratchpad/p5-control-leaves.sh`, on the same clone and HEAD.

| entry | clone input | reading |
|---|---|---|
| 6 (leaves) | unmodified | red · exit 1 ✗ (exit 0), hitting the 9 lines step 10 edits: `a11y.md:19`, `:35`, `:42`; `a11y-summary.md:8`, `:14`, `:35`; `tests-summary.md:21`, `:29`; `seven_guis.md:38` |
| 6 (leaves) | every stale count removed by `sed`, the correct «fourteen» left | green · exit 1, 0 B (must-pass). `git grep -c fourteen` afterwards reads 1 · 1 · 2 · 1 on the four pages: `NODE_FIELDS` ×2, the node keys, «fourteenth», `fourteen-people`. The control's `sed` wrote «eighteen since» at `a11y-summary.md:35`; step 10 removes that count instead. |
