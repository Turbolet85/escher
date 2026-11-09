@leaf_group group=architecture+test-plan
@leaf_read leaf=.claude/docs/commands.md lines=82
last: - Fonts missing in headless tests → `system-fonts` feature /
@leaf_read leaf=.claude/docs/services/blitz-test-harness.md lines=37
last: - `.claude/rules/verification-harness.md` · `.claude/docs/te
@leaf_edit n=1 leaf=.claude/docs/commands.md line=58 class=omission
leaf-text: `<dir>` must be short
source: fct-movjyb23 «The command line's `coverage` role: its reader takes the first word `coverage` after `mcp` and before the verb table, in one shape, `coverage <app> <check>...`»
replace-with: the `coverage` role — `coverage <app> <check>...`, no `--session` and no `--record` among its words, any other shape a usage error, status `2` — replays each named kept check on a fresh session through `handler_coverage` and prints the coverage line of the handlers they left unexercised, status `0`; a file that cannot be read, one past `MAX_CHECK_BYTES`, one that is no check or a check of another app refused `malformed`, status `1`; a failed boot status `3`; `<dir>` must be short
@leaf_edit n=2 leaf=.claude/docs/commands.md line=60 class=stale
leaf-text: 5 tests, unix only, the fifth opening and choosing in a select's list through the host at `trace` with 0 option needles, the fourth serving `cells`
source: fct-k5y3m6cu «`tests/host_log.rs` (6 tests, unix only, the fifth and the open list's below, the fourth a `coverage crud` run over a CRUD session kept through the binary with a typed sentinel and a row click»
replace-with: 6 tests, unix only, one opening and choosing in a select's list through the host at `trace` with 0 option needles, the fourth a `coverage crud` run over a CRUD session kept through the binary with a typed sentinel and a row click, reading 0 id, name, coverage, sentinel, path and recorded needles, the fifth serving `cells`
@leaf_edit n=3 leaf=.claude/docs/commands.md line=61 class=omission
leaf-text: --test stand_graph --test stand_actions`
source: fct-glytb3w3 «`run stand`'s `run.end` reads passed 262 · failed 0 · ignored 7 over 54 files, `stand_handler_coverage` a file its `stand_` prefix picks up with no script change»
replace-with: --test stand_graph --test stand_actions --test stand_handler_coverage`
@leaf_edit n=4 leaf=.claude/docs/commands.md line=61 class=omission
leaf-text: the whole Cells answer within the answer bound; each picked up by `run stand`
source: fct-ldnjdynp «`stand_handler_coverage` (8) holds `escher_driver::handler_coverage` over the six served tasks in both layout modes — a snapshot check listing exactly `HANDLERS_AT_BOOT`»
replace-with: the whole Cells answer within the answer bound; since 2026-11-08-handler-coverage `stand_handler_coverage`, 8 tests: `escher_driver::handler_coverage` over the six served tasks, a snapshot check listing exactly `HANDLERS_AT_BOOT`, each `GRAPH_DRIVES` task's drives leaving its `UNEXERCISED_AFTER_DRIVES` row, a refused step exercising nothing and a keyboard activation its click, a handler whose step removes its element reading exercised, a typed sentinel never reaching the coverage line, and a check of another app contributing nothing; each picked up by `run stand`
@leaf_edit n=5 leaf=.claude/docs/commands.md line=3 class=tree
leaf-text: _From `.andromeda/architecture.md`, the
source: fct-scmruv42 «`scripts/agent-run.sh` and `scripts/cold-agent.sh` the agent-run test and cold-agent pipe contracts, in full in test-plan §3.»
replace-with: _From `doc:architecture`, the
@leaf_edit n=6 leaf=.claude/docs/services/blitz-test-harness.md line=3 class=tree
leaf-text: `.andromeda/test-plan.md` §3 and `.andromeda/architecture.md` §Standard Contracts
source: fct-gkkvjkmi «**Test harness:** blitz-test-harness's `Harness`, generic over the document, with constructors, inspection and input helpers that pump and never settle»
replace-with: `doc:test-plan` §3 and `doc:architecture` §Standard Contracts
