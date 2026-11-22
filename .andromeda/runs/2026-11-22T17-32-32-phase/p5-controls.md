# P5 controls — revision 1 of 2026-11-22-epoch-2-corrective

Each entry fired AS ITSELF through `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py run --chunk
2026-11-22-epoch-2-corrective --docs-root . --entry {n} --project-root $TMPDIR/{scratchpad}/ctl1`, the plan read from
the store at rev 5, over a `git clone --shared` of the project. Clone HEAD `40ec4749e74a0a3fb3b355f79aec0323ec86fbf6`.

| entry | input | reading |
|---|---|---|
| 4 guard | unmodified clone | red · last line '' — as on the tree |
| 5 lock move | unmodified clone | red · last line '' — as on the tree |
| 6 audit leg | unmodified clone | red · exit 1 — as on the tree (RUSTSEC-2026-0592) |
| 4 guard | must-pass: `cargo update -p url --precise 2.5.9`, a `// control` line appended to the four modified touchpoints, `command_tests.rs` minted | green |
| 5 lock move | must-pass: the same input | green — the last-line literal re-read from the entry's own output |
| 6 audit leg | must-pass: the same input | green · `advisories ok, licenses ok` |
| 4 guard | must-fail: the must-pass input + a `# control` line appended to the root `Cargo.toml` | red · last line 'Cargo.lock Cargo.toml packages/…' |
| 4 guard | must-fail: the must-pass input + a `# control` line appended to `deny.toml` (root `Cargo.toml` restored) | red · last line 'Cargo.lock deny.toml packages/…' |
| 5 lock move | must-fail: the must-pass input + bindgen's `version = "0.73.2"` turned `0.73.3` in `Cargo.lock` | red · last line '-version = "0.73.2" +version = "0.73.3" …' |

The audit leg's must-fail is its unmodified red (the advisory present). The take-up's controls of entries 3, 5
(now 7), 13 (now 15) and 14 (now 16) stand in `.andromeda/runs/2026-11-22T16-25-24-phase/p5-controls.md`; their `run`
is unchanged.
