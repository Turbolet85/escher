# Row-guard controls — 2026-10-26-codebase-health (plan step 4)

Taken on the refactored tree after the gate block read green (implement run
`.andromeda/runs/2026-10-27T01-14-55-implement/`, 2026-10-27). One control at a time, by a scratchpad script
(`$TMPDIR/{scratchpad}/row_controls.py`). The script saves the file's bytes and applies one
mutation, asserting it found its site. It runs the named test alone with `cargo test … -- --exact {test}` (stderr
merged into stdout), then writes the saved bytes back and compares sha256 before the next control. A control reads
red when the run exits 101, prints `FAILED` at the test line and has zero `error[` / `could not compile` lines.

| control | file mutated | mutation | test run alone | exit | panicked at | message | `error[` | restored |
|---|---|---|---|---|---|---|---|---|
| (a) | `.github/scripts/ci-leg.sh` | ` --test select_interaction` removed — both sites (the `a11y` leg at :39 and `stand-repeat` at :57; removing the stand-repeat one alone would leave the `contains` read true) | `stand_requirement_sweep::every_0_2_0_requirement_names_its_witness_inside_stand_repeat` | 101 | `stand_requirement_sweep.rs:1039` | `named target 8: the leg runs it` | 0 | sha256 equal |
| (b) | `tests/blitz-tests/tests/stand_requirement_sweep.rs` | in `WITNESSES_020`, v020-01's third witness `each_line_is_read_by_its_rules` renamed to a name no file holds | the same | 101 | `stand_requirement_sweep.rs:1046` (the `assert_0_2_0_row` call) | `row 0, witness 2: the test exists in its file` | 0 | sha256 equal |
| (c) | `packages/escher-driver/src/mcp/listing.rs` | `description` writes its result fields `.skip(1)`, leaving the first one out | `mcp::tests::the_listing_is_the_twelve_verbs_of_the_table` | 101 | `mcp_tests.rs:415` (the `assert_described` call) | `row 0` | 0 | sha256 equal |
| (d) | `packages/escher-driver/src/mcp/listing.rs` | the `Part` argument's `property.count("maximum", …)` line dropped | the same | 101 | `mcp_tests.rs:416` (the `assert_schema` call, through `assert_property`) | `row 0` | 0 | sha256 equal |
| (e) | `packages/escher-driver/src/lint.rs` | `.chain(colours)` dropped from `readings`, so no contrast finding is pushed | `stand_lint::the_planted_defects_are_named_each_by_its_id_and_reason` | 101 | `stand_lint.rs:192` | `incremental=false: the findings are the seven planted, in document order` | 0 | sha256 equal |

Each red run's result line: `test result: FAILED. 0 passed; 1 failed; …`. (c) reads red even though the expectation
`description(verb)` moves with the mutation: the equality still holds, and the per-field `contains` assertion that
`assert_described` keeps is the guard that fires.

## Green on the restored tree
The same three tests, run alone after the last restore:
- the sweep test: exit 0, `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out`
- the listing test: exit 0, `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 99 filtered out`
- the planted-lint test: exit 0, `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out`

## Byte-equality
sha256 of the five files the controls read or mutated, before the first control and after the last restore,
identical:

```
b2bbe4ffc2e26b742b6a84eb33efb237d7dbf16f5903f752ecfdc3ed88426e22  .github/scripts/ci-leg.sh
99d5cbdf9bc2661fc2f13f5b4b6caa5cebc430fa4d0f20ca8705fc8962295feb  tests/blitz-tests/tests/stand_requirement_sweep.rs
3a8800149f5399435a2a09e89898540d4048bbb66a5e112d0668dcf450692f3b  packages/escher-driver/src/mcp/listing.rs
2e6a750cd3e94b82358a52851ece61ee5a8bbb9c782bee684eb3c771e54b36e6  packages/escher-driver/src/lint.rs
3ead8d7503c9af3d6c96ab46f119ad18a14acebf77878ccff732635e6cf59e55  packages/escher-driver/src/mcp_tests.rs
```

`git status --short` after the controls lists only the three chunk files as modified under source, so `ci-leg.sh` and
`listing.rs` are unchanged against HEAD.
