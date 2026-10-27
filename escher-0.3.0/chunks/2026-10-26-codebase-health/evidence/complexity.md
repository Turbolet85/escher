# Complexity reading — 2026-10-26-codebase-health (plan step 5)

The audit's own reading: rust-code-analysis 0.0.25 (`~/.cargo/bin/rust-code-analysis-cli`), every space of kind
`function` (closures included), `cognitive.sum > 15`, over the 429 files of
`.andromeda/runs/2026-10-25T21-43-01-code-audit/srcfiles.txt` (the recipe at that run's `record.json:685`).

## Before — base `19f9ff91ca`
Read at P3 and baselined at P5 (plan entry 1's `baseline`; research.md §Sweeps):
- `packages/escher-driver/src/lint.rs findings 16.0`
- `packages/escher-driver/src/mcp_tests.rs the_listing_is_the_twelve_verbs_of_the_table 18.0`
- `tests/blitz-tests/tests/stand_requirement_sweep.rs every_0_2_0_requirement_names_its_witness_inside_stand_repeat 18.0`
- `packages/blitz-dom/src/document.rs clear_interaction_state_for_removed_node 17.0`
- `files read 429` · `over 15: 88`

## After — this chunk's tree (plan entry 1, implement run `.andromeda/runs/2026-10-27T01-14-55-implement/`)
The entry's whole output (`gate.py show --n 1`, log `$TMPDIR/andromeda-gate/2026-10-26-codebase-health/implement-2026-10-27T01-14-55/1.log`, sha8 `0509365a`):

```
packages/blitz-dom/src/document.rs clear_interaction_state_for_removed_node 17.0
packages/blitz-dom/src/document.rs get_text_selection_ranges 32.0
files read 429
over 15: 85
```

Every atom held: exit 0, the engine row `17.0` present, none of the three rows listed, `files read 429`, last line
`over 15: 85`. `get_text_selection_ranges 32.0` is `document.rs`'s other row over the ceiling, which the entry prints
because `document.rs` is a named file. It was over the ceiling at the base as well, is not touched, and is counted
in both 88 and 85. The population fell by exactly the three, with no new entrant: predicted 88 → 85, measured 85.

## Each function's own reading after the factoring
Same tool and reading, over the three touched files alone (`rust-code-analysis-cli -m -O json -p …`, each space of kind
`function`, its `cognitive.sum`):

| file | function | cognitive.sum | was |
|---|---|---|---|
| `packages/escher-driver/src/lint.rs` | `findings` | 7.0 | 16.0 |
| `packages/escher-driver/src/lint.rs` | `readings` (new) | 1.0 | — |
| `packages/escher-driver/src/lint.rs` | `placement` (new) | 1.0 | — |
| `packages/escher-driver/src/lint.rs` | `ordered` | 0.0 | 0.0 |
| `packages/escher-driver/src/lint.rs` | `overlaps` | 13.0 | 13.0 |
| `packages/escher-driver/src/mcp_tests.rs` | `the_listing_is_the_twelve_verbs_of_the_table` | 3.0 | 18.0 |
| `packages/escher-driver/src/mcp_tests.rs` | `assert_described` (new) | 2.0 | — |
| `packages/escher-driver/src/mcp_tests.rs` | `assert_schema` (new) | 2.0 | — |
| `packages/escher-driver/src/mcp_tests.rs` | `assert_property` (new) | 3.0 | — |
| `packages/escher-driver/src/mcp_tests.rs` | `listed` | 2.0 | 2.0 |
| `tests/blitz-tests/tests/stand_requirement_sweep.rs` | `every_0_2_0_requirement_names_its_witness_inside_stand_repeat` | 3.0 | 18.0 |
| `tests/blitz-tests/tests/stand_requirement_sweep.rs` | `assert_0_2_0_row` (new) | 3.0 | — |
| `tests/blitz-tests/tests/stand_requirement_sweep.rs` | `assert_0_2_0_witness` (new) | 2.0 | — |
| `tests/blitz-tests/tests/stand_requirement_sweep.rs` | `inside_stand_repeat` (new) | 4.0 | — |
| `tests/blitz-tests/tests/stand_requirement_sweep.rs` | `every_requirement_names_its_witness` | 4.0 | 4.0 |

Every function the chunk adds reads at most 4, and none reaches 15. `overlaps` is unmoved at 13, and nothing was moved
into it. The two tests keep their names and stay in the counted population.
