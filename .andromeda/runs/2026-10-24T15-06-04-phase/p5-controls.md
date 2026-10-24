# P5 controls — 2026-10-24-stand-requirement-sweep

Fired as the entries (`gate.py run --plan … --entry {n} --project-root {clone}`, no run dir) over the scratch clone
`$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/bd7c515f-a97a-4c48-b797-fc46dacba1bb/scratchpad/p5clone`
(`git clone --shared` at HEAD `4b4804ba5d420bff40b752cc13a3ba645073edcb`, the chunk's plan, research and scope copied
in), by the script `$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/bd7c515f-a97a-4c48-b797-fc46dacba1bb/scratchpad/p5-controls.sh`,
2026-10-24 about 16:28Z. Each input lived in the clone alone.

| entry | clone input | reading |
|---|---|---|
| 8 (silence census) | unmodified | green · exit 1 (last line 0), as on the real tree |
| 8 | must-pass: a silent comment appended to `stand_lint.rs` and `text_extent.rs` | green · exit 1 |
| 8 | must-fail: `fn _p() { println!("x"); }` appended to `stand_lint.rs` | red · exit 1 ✗ (exit 0) |
| 10 (manifest/source guard) | unmodified | green · last line 0 |
| 10 | must-pass: the two silent comments above (files in the write set) | green · last line 0 |
| 10 | must-fail: a comment appended to `packages/escher-driver/src/lint.rs` | red · last line '0' ✗ (last: '1') |
| 11 (gate.py guard) | unmodified | green · guard: clean |
| 11 | must-pass: the two allowed files edited | green · guard: clean (allowed 2) |
| 11 | must-fail: `packages/escher-driver/src/lint.rs` edited | red · last: 'guard: outside 1 files' |
| 11 | must-fail: lint.rs restored, an untracked `tests/blitz-tests/tests/stray_extra.rs` minted | red · last: 'guard: outside 1 files' |

Entry 9 (the stand graph's adapter census), the literal `^accesskit_(xplat|winit) ` given its must-pass reading by an
operand-only control typed as the tool types an entry, `/usr/bin/bash -o pipefail -c '…'`, on the real tree:
`cargo tree --locked -p blitz-shell -e normal --prefix none | grep -c -E '^accesskit_(xplat|winit) '` → `1`, exit 0
(blitz-shell takes accesskit_xplat), against entry 9's own `0`, exit 1 on seven_guis.
