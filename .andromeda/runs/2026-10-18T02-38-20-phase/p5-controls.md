# P5 controls — 2026-10-18-engine-debts (validation.md check 4 (9))

Every control fired AS THE ENTRY: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py run --plan
escher-0.2.0/chunks/2026-10-18-engine-debts/plan.md --entry {n} --project-root $TMPDIR/{scratchpad}/ctl`, no `--run-dir`,
over a `git clone --shared` of the project at HEAD `34bcf71c678c6b40960ee629e19af6c27b6986cb`
(`$TMPDIR/{scratchpad}/ctl`, the session scratchpad). Each entry fired first unmodified, then with each input; the
input lived in the clone only, reset by `git checkout -- .` between controls. Logs:
`$TMPDIR/andromeda-gate/2026-10-18-engine-debts/run-20261018T0304*Z` and `…T0305*Z`.

| entry | clone input | reading | expected |
|---|---|---|---|
| 17 census | unmodified | green · exit 1 · last line 0 | as on the real tree |
| 17 census | must-fail: `    println!("x");` appended to packages/blitz-dom/src/node/node.rs | red · exit 0 | red |
| 17 census | must-pass: `    let kept = 1;` appended to the same file | green · exit 1 · last line 0 | green |
| 19 bridge identity | unmodified | green · exit 0 | as on the real tree |
| 19 bridge identity | must-fail: `// x` appended to packages/dioxus-native-dom/src/mutation_writer.rs | red · exit 1 | red |
| 20 manifests | unmodified | green · exit 0 | as on the real tree |
| 20 manifests | must-fail: a newline appended to Cargo.lock | red · exit 1 | red |
| 21 scope | unmodified | green · exit 1 · last line 0 | as on the real tree |
| 21 scope | must-fail: `// x` appended to examples/seven_guis/src/app.rs | red · exit 0 | red |
| 21 scope | must-pass: `// x` appended to packages/blitz-dom/src/node/node.rs | green · exit 1 · last line 0 | green |
| 15 controls reader | must-pass: an evidence/controls.md of 8 rows in the step-10 form | green · last line 8 | green |
| 15 controls reader | must-fail: the same, its last row removed | red · last line 7 | red |
| 16 merge-surface reader | must-pass: an evidence/merge-surface.md of 7 fixed rows (F1, F2, F4–F8) plus an F3 row | green · last line 7 (F3 not counted) | green |
| 16 merge-surface reader | must-fail: the same, its F8 row removed | red · last line 6 | red |
| 18 silence | must-pass: the five new check files present, print-free | green · exit 1 · last line 0 | green |
| 18 silence | must-fail: `fn noisy() { println!("x"); }` appended to the clone's inert_subtree.rs | red · exit 0 | red |

| 26 gated founder's word | absent (fired with `--void '26:P5 control in the scratch clone, the block standing'`) | red · exit 2 | red |
| 26 gated founder's word | must-pass: an evidence/founder-word.md of 9 ratified rows (F1–F9) | green · last line 9 | green |
| 26 gated founder's word | must-fail: the same, its F9 row removed | red · last line 8 | red |

Every reading as expected. Entry 26 is gated (`defer = "gated: …"`): it was fired through `--void` in the clone alone,
added after the operator's review of p5-card-1.md (inputs#I3).
