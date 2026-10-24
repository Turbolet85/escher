# CI read — CI#40114977936 on `18b50a2a20f3` (plan entry 23, 2026-10-24)

The libtest result lines in each job's log, summed per job (lines · passed · failed · ignored), from entry 23 fired with
`--id 40114977936` (`recorded`, `history: unmoved`). The control is CI#40105170961, the green run on `4b4804ba5`, read at P4.

| Job | CI#40105170961 (base) | CI#40114977936 (this chunk) | Moved, by name |
|---|---|---|---|
| Test [default features] | 206 · 1099 · 0 · 12 | 206 · 1105 · 0 · 12 | +6 passed: `stand_lint` +1, `text_extent` +3, `host_log` +1, `stand_requirement_sweep` +1; no new target |
| Test (macos) | 187 · 1096 · 0 · 12 | 187 · 1102 · 0 · 12 | +6 passed, the same six (`host_log`'s new `cfg(unix)` test runs on macOS) |
| Coverage report | 187 · 1096 · 0 · 12 | 187 · 1102 · 0 · 12 | +6 passed, the same six |
| Test (windows) | 187 · 1052 · 0 · 9 | 187 · 1057 · 0 · 9 | +5 passed: the six bar `host_log`'s `cfg(unix)` test, compiled out on windows |
| Accessibility (a11y) tests | 6 · 43 · 0 · 0 | 6 · 43 · 0 · 0 | unmoved: the a11y leg's six files |
| Stand checks repeated | 168 · 684 · 0 · 21 | 198 · 837 · 0 · 21 | +30 lines: the ten named witnesses × 3 rounds (predicted +30); +153 passed = 3 × 51: the ten's 48 tests plus `stand_lint` +1, `stand_requirement_sweep` +1 and `host_log` +1 |

Every job: 0 failed. The vertical-text check ran green under the workspace build on linux (Test [default features]),
macOS and windows. The CI counts equal the local readings (`evidence/counts.md`: workspace 206 · 1105 · 0 · 12;
stand-repeat 198 · 837 · 0 · 21).
