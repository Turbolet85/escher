# Counts — step 23

From gate entry 14 (`bash .github/scripts/ci-leg.sh fast`, exit 0, 131.95 s) of implement's first block pass,
2026-10-15: `target/ci-logs/test.log` (written 05:36:54Z, merged streams) and `target/ci-logs/ci-scripts.log`
(05:37:00Z). Tallied per test binary by the parser of `evidence/per-os-tallies.md`; HEAD's per-binary figures are
CI#38649787488's linux job (`Test [default features]`, the same `cargo test --workspace --locked`), whose totals
equal the local HEAD baseline.

## Workspace — `cargo test --workspace --locked`

| | result lines | passed | failed | ignored |
|---|---|---|---|---|
| HEAD (`eb5ba90c`) | 185 | 951 | 0 | 12 |
| this tree | 186 | 955 | 0 | 12 |
| delta | +1 | +4 | 0 | 0 |

Every delta, by target:

| target | HEAD | this tree | cause |
|---|---|---|---|
| `font_face_sniffed_source` (new) | — | 1 line · 2 passed | step 19, CARRY 9: the hint-less source's two checks |
| `escher_driver` (lib) | 72 passed | 73 passed | step 17: `client::imp::tests::an_answer_read_past_its_bound_is_a_timeout` |
| seven_guis `cli_commands` | 11 passed | 12 passed | step 18: `a_closed_stdout_drops_the_answer_and_keeps_the_ending` |

No other target's figures move. The two `hover_state_tests` (step 15) were counted as passed at HEAD too — they
passed by skipping — and now pass by asserting; `stand_act_spans` (step 16) keeps 2 passed · 2 ignored, its new
call inside the existing capture child.

## CI scripts — `python3 -m unittest discover -s .github/scripts`

`Ran 94 tests`, `OK`, against 79 at HEAD (+15):

| file | added |
|---|---|
| `test_coverage_floor.py` (new) | 7 — above, at and under a floor; two crates read apart; two holding; an absent crate (exit 2); five malformed calls (exit 2) |
| `test_ci_workflows.py` · `LegScriptTest` | 5 — the coverage leg reaching its floor, and failing under it; the clippy leg's two lines; the stand-repeat leg's three rounds, and its first red ending it |
| `test_ci_workflows.py` · `InstallScriptTest` | 3 — a healthy install runs no repair; an install failed by its bound is repaired before the next attempt; a failed repair fails its attempt in phase `repair` |

`CiWorkflowTest` gains no case: its tables gain the new job (`LINUX_JOB_LEGS`, `RESTORE_ONLY_JOBS`,
`INSTALL_JOBS`), so cases b, c, d, e, j, k, l and o now hold the `stand-repeat` job's shape.
