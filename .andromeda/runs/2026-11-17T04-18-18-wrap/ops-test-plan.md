@row_set n=1 row=contract/mcp-server/tools-list cell=promise seen=1 detector=none
value: tools/list offers the fifteen verbs in order, `layout` after `actions` and `help` last.
basis: fct-w5rznwmn
because: The MCP tools listed moved from fourteen to fifteen with `layout` after `actions`, so the row's fourteen is false.
@row_set n=2 row=contract/escher-session-command-line/command-form cell=promise seen=1 detector=none
value: `escher-session <verb> … --session <dir>` is the whole command line, fifteen verbs; the instance-level verbs take their arguments as flags spelt as the schema spells them (`--id`, `--text`, `--key`, `--ms` and `--part` each with one value, `--shift` bare).
basis: fct-w5rznwmn
because: The driver verbs moved from fourteen to fifteen with `layout`, so the command line's verb count in the row is false.
@row_set n=3 row=contract/escher-session-command-line/usage-line cell=promise seen=1 detector=none
value: Usage reads `snapshot [--part <part>]`, `start <app> [--record <dir>]`, `host role: serve <app> --session <dir> [--record <dir>]`, `mcp role: mcp [--record <dir>]` and `coverage role: coverage <app> <check>...`, with `graph` after `lint`, `layout --id <id>` after `actions` and `help` after `stop`.
basis: fct-w5rznwmn
because: The usage line gained `layout --id <id>` after `actions`, so the row's listing of the line is incomplete.
@row_set n=4 row=contract/driver-session/reading-verbs cell=promise seen=1 detector=none
value: `snapshot`, `screenshot`, `lint`, `graph` and `layout` run no step and wait on nothing.
basis: fct-stryxpwu
because: The reading verbs gained `layout`, a verb that runs no step and changes nothing, so the row's set of reading verbs is incomplete.
@row_set n=5 row=contract/driver-verbs/verb-table cell=promise seen=1 detector=none
value: Fifteen verbs in a stated order at three levels — `instance`, `session`, `tool` — each with help, arguments and at least one result field.
basis: fct-yx7iahh5
because: The verb table now holds fifteen verbs with `layout` after `actions`, so the row's fourteen is false.
@row_set n=6 row=contract/session-layout/changes-nothing cell=promise seen=1 detector=none
value: A layout read, in process or through the `layout` verb, changes nothing: no change is pending, the snapshot, focus, diff and clock are unchanged, an open list stays open and delivered Timer ticks stay unapplied, in both layout modes.
basis: fct-h7vqoq7u
because: The layout read is now reached by the `layout` verb through the call path, and its no-change promise holds over the verb too, so the row must name it.
@row_set n=7 row=contract/mcp-tools-list/order cell=promise seen=1 detector=none
value: `tools/list` offers the fifteen verbs in order, `graph` after `lint`, `actions` after `graph`, `layout` after `actions` and `help` last.
basis: fct-w5rznwmn
because: The MCP tools listed moved from fourteen to fifteen with `layout` after `actions`, so the row's fourteen is false.
@row_set n=8 row=decision/socket-carries-commands cell=rule seen=2 detector=none
value: The session socket carries a call's element id and typed text in, and ids, accessible names, control values (a password's or file input's as the fixed mask), the snapshot's text, the diff, a step's caused with its error text, lint findings and layout answers out.
basis: fct-stryxpwu
because: The layout answer now crosses the socket as the `layout` verb's answer, so the row's list of what the socket carries out is incomplete.
@row_add n=1 row=contract/session-run/layout-answer area=integration-test-strategy fact=F1 detector=D-tests-coverage
boundary: escher_driver Session
item: layout verb answer
promise: The `layout` verb takes one required id and answers one line of id, bounds, laid_out, placed, shifts, width and height, holding ids, closed kebab-case words and numbers only, equal to the in-process layout read on every node of the six served screens in both layout modes.
basis: fct-h7vqoq7u
because: The chunk adds the `layout` verb read through the session's call path, a contract at the driver's boundary no row states.
@row_add n=2 row=contract/session-run/layout-unexplained area=integration-test-strategy fact=F2 detector=D-tests-coverage
boundary: escher_driver Session
item: layout of an element with no box
promise: An element the snapshot lists that has no box answers the `layout` verb placed by `unexplained` at its snapshot bounds, with its layout causes and sizes read.
basis: fct-ighszuki
because: The chunk adds the `unexplained` placer for a listed element with no box, a promise of the layout answer no row states.
@row_add n=3 row=contract/cli-layout/output area=e2e-test-strategy fact=F3 detector=none
boundary: escher-session CLI layout verb
item: output
promise: `layout --id <id> --session <dir>` answers one line and exits 0; without an id it is refused `malformed`, status 1.
basis: fct-gd5fq3ek
because: The command line gains the `layout` command, a contract of its output and refusal that no row states.
@row_add n=4 row=contract/mcp-layout/parity area=e2e-test-strategy fact=F3 detector=none
boundary: escher-session MCP role
item: layout
promise: The `layout` tool call's text equals the command line's `layout` line.
basis: fct-gd5fq3ek
because: The MCP role gains the `layout` tool, held equal to the command line's answer, a parity no row states.
@row_add n=5 row=decision/cold-agent-verbs-without-layout area=3-test-harness-contract fact=F4 detector=none
name: Cold-agent pipe's verb list without layout
rule: The cold-agent pipe's verb list holds the fourteen driver verbs other than `layout`.
why: The conservative option of an open fork, applied ahead of its ratification.
state: provisional
basis: fct-w5rznwmn
because: The driver gained a fifteenth verb while the cold-agent pipe stays at fourteen by a provisional choice no row states.
@row_add n=6 row=decision/count-and-rename-sweeps area=test-strategy fact=F6 detector=none
name: Verb-count and test-rename sweeps
rule: A change of the driver's verb count sweeps the tests and examples for pins of the verb table's length and of its indices, and a test rename sweeps the old name held as a string literal, such as a requirement's witness.
why: A word sweep misses a numeric pin and a test name held as a string.
basis: fct-lko5l3en
because: The chunk met a missed numeric pin and a missed renamed witness, giving a convention for future count changes and renames no row states.
@row_none n=1 fact=F5 type=5
what: a test of the layout step under the replay's stop, which contract/check-replay/stop and decision/replay-through-the-call-path already state for every step and result field; stand_replay holds it
