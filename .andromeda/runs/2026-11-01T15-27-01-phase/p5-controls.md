# P5 controls — 2026-11-01-upstream-sync-ui-graph

Each fired as the entry, `gate.py run --chunk 2026-11-01-upstream-sync-ui-graph --docs-root {root} --entry {n} --project-root {clone}`, no `--run-dir`, over a `git clone --shared` of HEAD `65938a21e` at `$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/8d21f6c1-9112-4e1a-8853-11189b113e90/scratchpad/ctl-clone`. Fence rev 3. The clone's inputs lived in the clone alone.

| Entry | Input | Reading |
|---|---|---|
| 14, manifests | unmodified | green · exit 0 · 0 B, as on the tree |
| 14 | a comment line in the root `Cargo.toml` | red · `no output ✗ (11 B)` |
| 14 | a comment line in `README.md` | green · exit 0 · 0 B |
| 15, time reads | unmodified | green · exit 1 · last line 0, as on the tree |
| 15 | a minted `control_time.rs` with `web_time::Instant::now()` | red · `exit 1 ✗ (exit 0)` |
| 17, silence | unmodified, none of the two new files there | green · exit 1 · last line 0, as on the tree |
| 17 | the new check minted, silent | green · exit 1 · last line 0 |
| 17 | the new check minted with a `println!` | red · `last line '0' ✗ (last: '1')` |

Entry 7's control is implement's red-before-green run: the new target does not exist yet.

Entry 28, the job-log tally, control fired on CI#41235503227, the log read to a scratch file first, 2026-11-01T16:50Z: Test [default features] 217 · 1236 · 0 · 12; Test (macos) 198 · 1233 · 0 · 12; Coverage report 198 · 1233 · 0 · 12; Test (windows) 198 · 1185 · 0 · 9; Accessibility (a11y) tests 14 · 114 · 0 · 0; Stand checks repeated 228 · 1155 · 0 · 21. Equal to the recorded rows.
