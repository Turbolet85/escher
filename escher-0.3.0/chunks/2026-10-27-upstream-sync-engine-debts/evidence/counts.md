# Counts — 2026-10-27-upstream-sync-engine-debts

Measured by the gate run of `.andromeda/runs/2026-10-27T09-58-03-implement/` (trail
`gate-2026-10-27-upstream-sync-engine-debts.json`), against `before.md`.

| count | before | on the take | moved by |
|---|---|---|---|
| workspace tally (entry 16, `target/ci-logs/test.log`) | 208 · 1117 · 0 · 12 | **209 · 1125 · 0 · 12** | +1 line: `pseudo_element_flow_classification` (`2963b65a2`), upstream's new target. +8 passed: 3 in `pseudo_element_flow_classification.rs` (`2963b65a2`); 3 in blitz-vibey-script `tests/dom.rs` (2 `cee6b8731`, 1 `efdc5472a`); 1 in dioxus-native-dom `dioxus_document.rs`, `input_focused_on_mount_enables_ime` (`7fc9ee3af`); 1 in `apps/browser/src/tab.rs`, `tab::tests::tab_lens_survives_webview_props_replacement` (`d63ed2c8c`) |
| `pseudo_element_flow_classification` (entry 10) | absent | 3 passed | `2963b65a2` |
| blitz-vibey-script `tests/dom.rs` (entry 11) | 27 | 30 | `cee6b8731` +2, `efdc5472a` +1 |
| dioxus-native-dom lib (entry 12) | 81 | 82 | `7fc9ee3af` +1 |
| blitz-dom lib (entry 13) | 106 | 106 | — |
| integration readings (entry 14) | 23 targets · 125 passed · 0 failed | 23 · 125 · 0 | — |
| `tests/blitz-tests/tests/*.rs` (entry 18) | 147 | 148 | `2963b65a2` |
| every blitz-tests file but `all.rs` ran as its own target (entry 17) | 0 absent | 0 absent | — |
| CI scripts (entry 19, recorded) | Ran 108 tests · OK | Ran 108 tests · OK | — |
| a11y leg (entry 23) | 14 · 85 · 0 | 14 · 85 · 0 | — |
| `run stand` run.end (entry 25) | 205 · 0 · 7 | 205 · 0 · 7 | — |
| `tests/blitz-tests/tests/stand_*.rs` (entry 26) | 50 | 50 | — |
| `run all` run.end (entry 27) | 630 · 0 · 12 | **633 · 0 · 12** | +3: `pseudo_element_flow_classification`'s 3, picked up with no script change |
| seven_guis per-package (entry 28) | 12 lines · 41 passed | 12 · 41 · 0 failed | — |

No stand figure moved: `run stand` passed every check over 50 files at the same 205 · 0 · 7, so `stand_keyboard`'s Tab
tables, `stand_contrast`'s rows (the accent pairs 5.47), `stand_lint`'s boot readings and the `crud-list` bounds,
the snapshot sizes, the shots and the committed Counter checks (`stand_replay`, seven_guis `recording`) read as
recorded. Every predicted value matched the measurement.
