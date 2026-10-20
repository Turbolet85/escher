# Implement outcome — 2026-10-20-large-screen-in-parts

**Outcome: green.** Smoke: covered by P2 gates; no separate run (see Smoke). Run dir `.andromeda/runs/2026-10-20T10-25-09-implement/`.

## Gates
- First block run: `entries 21 · green 17 · red 0 · recorded 0 · timeout 0 · not-run 4`.
- Re-run after the test extension (see the second deviation): `entries 21 · green 17 · red 0 · recorded 0 · timeout 0 · not-run 4`.
  - The 4 not run are the operator entries 18–21.
- Trail: `.andromeda/runs/2026-10-20T10-25-09-implement/gate-2026-10-20-large-screen-in-parts.json`.
- No deferral. Every changed-surface gate ran.
- One red came before the gates, in my own `cargo test -p escher-driver` run: `execute::tests::the_span_has_eight_fields_and_none_is_an_argument_name` pins the table's argument count. I moved it 9 → 10 (the new `part`) in `execute.rs`, a listed file.

### Counts the notes asked for (as measured)
| Reading | Before | Now |
|---|---|---|
| dioxus-native-dom lib tests | 75 | 81 (+6 `text_parts` units) |
| escher-driver unit tests | 99 | 100 (+1, the snapshot tool listing pin) |
| `stand_snapshot_parts` | — | 3 passed, 0 failed |
| `stand_act_spans` calls per mode | 34 | 35, asserted |
| seven_guis `cli_commands` | 13 | 14 |
| `host_log` command-span lines | 6 | 7 (one per call, asserted) |
| `mcp_log` `tool_call` lines | 9 | 10 |
| `mcp_log` `command` lines | 7 | 8 |
| `run stand` | 47 files · 192 passed · 7 ignored | 48 files · 195 passed · 0 failed · 7 ignored |
| a11y leg | 6 · 6 · 4 · 8 · 5 · 14 = 43 | unmoved |
| workspace tally | lines 204 · passed 1087 · ignored 12 | lines 205 · passed 1098 · failed 0 · ignored 12 |

The workspace tally gained 11 passes: +6 dom, +1 driver, +3 the new target, +1 cli_commands.

- `info_span!` sites: 2.
- `snapshot_text.rs` census: 6 lines.
- Manifest / stand / shared-module diff: 0 lines.

### Guard (entry 11), its pass lines copied whole
```
guard: allowed 19 — examples/seven_guis/tests/cli_commands.rs, examples/seven_guis/tests/host_log.rs, examples/seven_guis/tests/mcp_log.rs, packages/dioxus-native-dom/src/lib.rs, packages/dioxus-native-dom/src/snapshot_text.rs, packages/escher-driver/src/cli.rs, packages/escher-driver/src/cli_tests.rs, packages/escher-driver/src/command.rs, packages/escher-driver/src/execute.rs, packages/escher-driver/src/json.rs, packages/escher-driver/src/lib.rs, packages/escher-driver/src/mcp/listing.rs, packages/escher-driver/src/mcp_tests.rs, packages/escher-driver/src/schema.rs, packages/escher-driver/src/schema/spec.rs, packages/escher-driver/src/schema_tests.rs, tests/blitz-tests/tests/session_common/mod.rs, tests/blitz-tests/tests/stand_act_spans.rs, tests/blitz-tests/tests/stand_snapshot_parts.rs · companions excluded 0 · changed 19 · excluded 0
guard: clean
```
No file is excluded and no companion is excused.

## Files changed (18 modified · 1 new, all inside research's lists)
- **dioxus-native-dom**
  - `src/snapshot_text.rs`: `TextPart` and `Snapshot::text_parts(budget)`; a private `each_line` walk shared with `to_text`; the budget doc; 6 unit tests.
  - `src/lib.rs`: re-exports `TextPart`; the feature doc names `text_parts`.
- **escher-driver**
  - `schema/spec.rs`: `ArgKind::Part`.
  - `schema.rs`: `MAX_PART = 1000`; `snapshot`'s optional `part` argument; the `part` and `parts` count fields (`always: false`); the help texts.
  - `schema_tests.rs`, `command.rs` (`Command::Snapshot { part }`, its admission and tests) and `lib.rs`.
  - `execute.rs`: `Outcome::Part` and the part read; the span arm; the argument-count pin.
  - `json.rs`: `{"text","part","parts"}` and its tests.
  - `mcp/listing.rs`: an integer with minimum 1 and maximum 1000.
  - `mcp_tests.rs`, `cli.rs` (a `Part` flag is read as a number) and `cli_tests.rs`.
- **seven_guis tests:** `cli_commands.rs` (a new test), `host_log.rs` (+1 part read) and `mcp_log.rs` (+1 part read).
- **blitz-tests:** `session_common/mod.rs` (`snapshot_part`, `part`, the `act` arm), `stand_act_spans.rs` (`Expect::Part`, one part read per mode) and the new `stand_snapshot_parts.rs`.

## Deviations from the plan, with why
- **Part of the walk was moved out of a test closure.** In step 12, assertions 4–6 (end, addressable, masked) run in the one walk test. The MCP walk is a second test and the Counter check a third. Assertion messages carry a mode and an index only.
- **The MCP test walks everything.** Step 12.7 asked for two MCP calls: no argument, then part 2. `verification-matrix.json#v020-09`'s acceptance reads "the same walk through serve_mcp returns the same JSON lines". So `over_mcp_the_walk_answers_the_json_lines_of_the_same_calls` walks the whole thing: no argument, parts 1 to `parts`, then `parts + 1`. Each answer is compared with `Session::run`'s JSON line, and the whole block was re-run green.
- **Small differences from the plan's wording:**
  - The plan names `part 1000` as admitted. It is admitted at `validate`, not on a screen.
  - A flag value is also refused `WrongKind`, beside a text value.
  - The plan's touchpoint row "the record fed from the part" disagrees with step 7. I followed step 7: the record takes the whole screen's ids, as `snapshot` and `lint` do.
- **The fixture uses author `id`s.** It uses `key` plus `id`, an `output` for the click count, and `display: block` buttons, so `item-400` lies below the viewport. Its whole text is over the budget, asserted, not measured as a count.
- **No scope-record lines.** None was needed: `gate.py scope` reads `clean — changed 19 · listed 19 · recorded 0`.

## Matrix
- v020-09: `planned → implemented`.
- Its ref is `stand_snapshot_parts.rs::a_screen_over_the_budget_is_walked_in_parts` · `::over_mcp_the_walk_answers_the_json_lines_of_the_same_calls`.

## Smoke
- The plan lists no smoke entry.
- The one binary touchpoint, `escher-driver/src/cli.rs` (the argv reader), was driven as the `escher-session` process by entry 5:
  - `start` → `snapshot --part 1|2|0|1001|x` → `stop`;
  - `host_log`'s serve host and `mcp_log`'s mcp child.
- Entries 12 and 13 booted the stand. No separate smoke run.

## Surfaced / stuck
None.

## Next
The operator pass, entries 18 to 21, in order:
1. 18 — `gate.py hygiene`.
2. The pre-CI commit, subject `chore(2026-10-20-large-screen-in-parts): operator pre-CI commit, for the run this chunk's verdict reads`.
3. 19 — fast, then the push.
4. 20 — `ci.py conclusion`.
5. 21 — the per-OS tallies.

Then `/andromeda-wrap-session`.

## Process hygiene
| Process | Started by | Final state |
|---|---|---|
| `escher-session` hosts and mcp children, started by the cli_commands, host_log and mcp_log checks | a P2 entry (5, and 15's workspace tests) | terminated |
| the agent-run harness's test runs | P2 entries 12 and 13 | terminated |

`gate.py census --comm escher-session --comm seven_guis_native --args escher-session --args agent-run` reads `census: none running`.

## Decisions & corrections
- **Guard refusal.** bash-guard refused `cd packages/escher-driver/src && …`: "cd … moves the session cwd". I re-issued it once as a subshell `( cd … && … )`.
- **A matrix acceptance wider than its plan step.** v020-09 says "the same walk through serve_mcp", while plan step 12.7 named two calls. I followed the acceptance; it costs one more full block run.
