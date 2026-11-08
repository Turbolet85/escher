@disposition n=1
row: stretched fct-k5y3m6cu examples/seven_guis/tests/cli_commands.rs:135-692 → 136-716 — changed inside
disposition: claim false — raised at Validate (cli_commands.rs now asserts `VERBS.len()` 14 and holds a new test, `actions_answers_one_line_of_the_actionable_nodes`, so the stated test count and the list of commands it runs no longer match the file)
@disposition n=2
row: stretched fct-k5y3m6cu examples/seven_guis/tests/mcp_server.rs:101-261 → 102-263 — changed inside
disposition: claim false — raised at Validate (`tools/list` now offers fourteen verbs, with `actions` right after `graph`, not thirteen)
@disposition n=3
row: stretched fct-k5y3m6cu examples/seven_guis/tests/mcp_server.rs:263-405 → 265-433 — changed inside
disposition: claim false — raised at Validate (mcp_server.rs now holds 8 tests, not 7: `an_actions_tool_call_answers_its_line_of_the_actionable_nodes` was added and the fact does not state it)
@disposition n=4
row: stretched fct-k5y3m6cu examples/seven_guis/tests/mcp_log.rs:55-335 → 57-352 — changed inside
disposition: holds (still 3 tests; the flow adds an `actions` call and reads 0 actions needles on stderr, which tightens the stated zero-needle reading)
@disposition n=5
row: stretched fct-k5y3m6cu examples/seven_guis/tests/common/mod.rs:1-573 → 1-597 — changed inside
disposition: holds (every helper the fact names still stands; the new `actions_needles`, called by `host_log` and `mcp_log` alone, is not listed)
@disposition n=6
row: stretched fct-nks3g3e7 packages/escher-driver/src/schema_tests.rs:1-434 → 1-454 — changed inside
disposition: claim false — raised at Validate (the verb table now has fourteen verbs, ten of them instance-level with `actions` after `graph`, not thirteen with nine)
@disposition n=7
row: stretched fct-nks3g3e7 packages/escher-driver/src/command.rs:382-911 → 386-917 — changed inside
disposition: holds (still 7 tests; `actions` was added to the admitted, malformed and reached rows, and the stated behaviours are unchanged)
@disposition n=8
row: stretched fct-nks3g3e7 packages/escher-driver/src/json.rs:524-1143 → 552-1219 — changed inside
disposition: holds (still 7 tests; each outcome is still written with its verb's fields in order, the actions outcome among them)
@disposition n=9
row: stretched fct-nks3g3e7 packages/escher-driver/src/cli_tests.rs:1-581 → 1-582 — changed inside
disposition: claim false — raised at Validate (a line is now read by 102 rows, `actions` among them, not 101)
@disposition n=10
row: stretched fct-nks3g3e7 packages/escher-driver/src/mcp_tests.rs:1-807 → 1-809 — changed inside
disposition: claim false — raised at Validate (the listing is now the fourteen verbs of the table, with `actions` right after `graph`, not thirteen)
@disposition n=11
row: stretched fct-nks3g3e7 packages/escher-driver/src/describe.rs:97-234 → 97-234 — changed inside
disposition: claim false — raised at Validate (describe's count row now reads `[14, 14, 12, 3, 8, 4, 15]`, not `[13, 14, 12, 3, 8, 4, 15]`)
@disposition n=12
row: stretched fct-i5a6vv3t@c4160 packages/dioxus-native-dom/src/actionable.rs:124-270 → 192-456 — changed inside
disposition: claim false — raised at Validate (actionable.rs's test module now holds eight tests, not four: four new tests read the actions listing, covering a removed listener, a select's options owned by the select, a positional id read as unkeyed, and an inert or hidden element left out)
@disposition n=13
row: stretched fct-z2ynupl3 tests/blitz-tests/tests/stand_act_spans.rs:1-24 → 1-24 — changed inside
disposition: holds (the module doc still states one driver line per command; it now names ten verbs, `actions` the new one, and the fact's ninth-verb reading still holds)
@disposition n=14
row: stretched fct-z2ynupl3@c40748 tests/blitz-tests/tests/stand_mcp_tools.rs:771-843 → 772-867 — changed inside
disposition: holds (the graph tool call still answers the reader's graph in both modes and parents one command recording `verb` alone, now through a shared helper; the range also holds the new actions test, which takes the file to 8 tests where the fact counts 7)
@disposition n=15
row: stretched fct-z2ynupl3 tests/blitz-tests/tests/stand_requirement_sweep.rs:542-837 → 542-837 — changed inside
disposition: holds (the nine 0.2.0 rows and their witnesses stand; only one witness's test name was renamed, to `the_fourteen_verbs_are_named_in_order_each_at_its_level`)
@disposition n=16
row: stretched fct-klxgwgcy packages/escher-driver/src/recording_tests.rs:1-559 → 1-589 — changed inside
disposition: holds (the read-back now keeps an `actions` answer whole too; every stated pin still stands)
@disposition n=17
row: stretched fct-klxgwgcy packages/escher-driver/src/describe.rs:97-234 → 97-234 — changed inside
disposition: holds (the document still holds `help`'s seven fields in order and every text in it is its tables'; this fact states no count row)
@disposition n=18
row: stretched fct-klxgwgcy packages/escher-driver/src/schema_tests.rs:1-434 → 1-454 — changed inside
disposition: claim false — raised at Validate (the table now has fourteen verbs, ten instance-level with `actions` after `graph`, and thirteen field kinds with `ActionNodes`, not thirteen verbs, nine instance-level and twelve field kinds)
@disposition n=19
row: stretched fct-klxgwgcy packages/escher-driver/src/command.rs:382-911 → 386-917 — changed inside
disposition: holds (level checks, admitted and malformed rows, the first-failure order, and the table agreeing with `Command` all still hold, now with `actions`)
@disposition n=20
row: changed fct-fghufsse packages/escher-driver/src/schema_tests.rs:137 → ? — rewritten «assert_eq!(amended.len(), 11);»
disposition: claim false — raised at Validate (`VERBS` now stands at 14, not 13, describe's count row reads `[14, 14, 12, 3, 8, 4, 15]`, and the amended-help rows now number 12, with `actions` added, asserted at line 149)
