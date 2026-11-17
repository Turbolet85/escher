# Outcome — 2026-11-17-layout-on-the-carriers

**Outcome:** green. Next → the OPERATOR PASS (entries 9 hygiene, 10 push, 11 CI read), then `/andromeda-wrap-session`.

## Files changed (32)
- In research's lists (30): packages/escher-driver/src/{layout,layout_tests,schema,schema_tests,command,execute,json,explore,explore_tests,actions,describe,mcp,mcp_tests,cli_tests,recording_tests,lib}.rs, packages/escher-driver/src/schema/spec.rs, tests/blitz-tests/tests/{stand_layout,stand_actions,stand_cross_os_readings,stand_act_spans,stand_mcp_tools,stand_replay}.rs, tests/blitz-tests/tests/session_common/mod.rs, examples/seven_guis/tests/{cli_commands,mcp_server,mcp_log,host_log}.rs, docs/capabilities.md, README.md.
- Recorded companions (2): tests/blitz-tests/tests/stand_act_scroll.rs, tests/blitz-tests/tests/stand_requirement_sweep.rs.

## Gates (final firing, run 3 of the block)
1 `cargo test -p escher-driver --locked --lib` green (123 passed) · 2 `… --test stand_layout` green (11 passed) · 3 companions green · 4 seven_guis four process checks green · 5 `python3 .github/scripts/test_capability_statement.py` green · 6 `bash .github/scripts/ci-leg.sh fast` green · 7 `bash .github/scripts/ci-leg.sh doc` green · 8 `bash .github/scripts/ci-leg.sh stand-repeat` green (`stand-repeat: 3 of 3 rounds green`) · 9 · 10 · 11 `not run — leg operator`.
Reds recorded before their re-fires (each after an edit): firing 1 — `fast` red on rustfmt diffs of scripted edits (cli_commands.rs, explore_tests.rs); `stand-repeat` red at `stand_act_scroll::a_scroll_needs_a_target_on_the_screen_and_nothing_else_of_it` (`VERBS.len()` pinned 14). Firing 2 — both red at `stand_requirement_sweep::every_0_2_0_requirement_names_its_witness_inside_stand_repeat` (row 0 witness 1: the renamed schema test).
Controls (`gate.py control`, evidence/control-1.txt): unexplained · verb-pumps · explore-by-kind — 3 holds.
Matrix: v030-11 planned → implemented.

## Deviations
1. Plan step 10 says a missing `--id` on the command line is a usage error; the CLI refuses it `malformed` (status 1), as the plan's acceptance states («`malformed` without one»). `cli_commands` asserts the refusal.
2. Plan step 10 says the span check's calls-per-mode count moves by one; one accepted and one refused `layout` call move it 38 → 40.
3. `stand_actions`' compared-pair counts are pinned per node (×4 → ×5, five sites), and the Cells answer's bound test now pins the measured 118,292 bytes beside the bound.
4. JSON fallback words for a member a `#[non_exhaustive]` engine set gains later: `PlacedBy` → `unexplained`, `ShiftBy` → `transform`, `SizeCause` → `layout`, `Length`/`SizeKeyword` → `auto`. Unreachable on this build.
5. `placed` writes `canvas` after `container` for the reserved `Placer::App`.
6. Renamed tests: schema_tests `the_fourteen_verbs_…` → `the_fifteen_verbs_are_named_in_order_each_at_its_level`; mcp_tests `the_listing_is_the_fourteen_verbs_of_the_table` → `…_fifteen_…`; explore_tests' two tests renamed for the layout kinds. Masters or leaves citing the old names are the wrap's.
Scope record: `tests/blitz-tests/tests/stand_act_scroll.rs` · companion · serves `packages/escher-driver/src/schema.rs` · self; `tests/blitz-tests/tests/stand_requirement_sweep.rs` · companion · serves `packages/escher-driver/src/schema_tests.rs` · self. `gate.py scope`: clean — changed 32 · listed 30 · recorded 2.

## Surfaced / stuck
none

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo test binaries, escher-session hosts and mcp children | a leg (gate entries 1–8) and the control clone's firings | terminated — `gate.py census --comm escher-session --comm seven_guis --args escher-session`: none running |
| control clone ~/.cache/andromeda-control/b840ffaa | this run (`gate.py control`) | removed (`clone removed`) |

## Decisions & corrections
- Guard refusal: the bash guard refused a `cd` into the skill's references and a `cat` heredoc to a file; each re-issued once (Read; a scratchpad script run by path).
- Sweep hazard: a pin of the verb table outside the driver crate (`stand_act_scroll`'s `VERBS.len()`) and a test name held as data (`stand_requirement_sweep`'s witness strings) were missed by research's `fourteen|ten instance` sweep; a test rename needs a sweep for the old name as a string.
- Recurrence of a standing learning: testing.md 2026-10-10 («search tests/ for the table's name before calling a check unaffected») met at stand_act_scroll.
- Scripted edits fire no format hook; `fast` read red on rustfmt until `cargo fmt --all` ran.
