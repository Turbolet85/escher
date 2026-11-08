@leaf_group group=architecture+test-plan
@leaf_read leaf=.claude/docs/commands.md lines=82
last: - Fonts missing in headless tests → `system-fonts` feature /
@leaf_read leaf=.claude/docs/services/blitz-test-harness.md lines=37
last: - `.claude/rules/verification-harness.md` · `.claude/docs/te
@leaf_edit n=1 leaf=.claude/docs/commands.md line=40 class=stale
leaf-text: given only the escher server's thirteen tools
source: fct-jelmr2gx «with the fixed prompt `STAND_PROMPT` and only the escher server's fourteen tools»
replace-with: given only the escher server's fourteen tools
@leaf_edit n=2 leaf=.claude/docs/commands.md line=58 class=omission
leaf-text: status `0`) · `status` each run one command against it
source: fct-c5e4c7ao «`screenshot [--id <id>]`, `lint`, `graph`, `actions`, `start <task> [--record <dir>]`, `status`, `stop`»
replace-with: status `0`) · `actions` (the tenth instance verb, no argument, an argument refused `malformed`, status `1`; its accepted answer one line `{"nodes":[{"id":…,"owner":…,"keyed":…,"verbs":[{"verb":…,"cause":…}]}]}`, the whole screen unpaged, status `0`) · `status` each run one command against it
@leaf_edit n=3 leaf=.claude/docs/commands.md line=59 class=stale
leaf-text: its tools the thirteen driver verbs
source: fct-t7zq5feb «`tools/list` answers the fourteen verbs of `VERBS` in order (`screenshot` after `scroll`, then `lint`, then `graph`, then `actions`), `help` last»
replace-with: its tools the fourteen driver verbs
@leaf_edit n=4 leaf=.claude/docs/commands.md line=59 class=omission
leaf-text: then `graph`, answering its line of the UI graph)
source: fct-wubzr4jw «`mcp_server`'s `actions` tool call answers its line of the actionable nodes, and an argument to `actions` reads `isError`»
replace-with: then `graph`, answering its line of the UI graph, then `actions`, answering its line of the actionable nodes, an argument to it answered `isError`)
@leaf_edit n=5 leaf=.claude/docs/commands.md line=61 class=omission
leaf-text: --test stand_load_page --test stand_graph`
source: fct-z2ynupl3 «`stand_actions` 8, declaring `mod common;`, `mod fixtures;` and `mod session_common;`, through `Session::actions` and `Session::run` on the six served screens in both layout modes»
replace-with: --test stand_load_page --test stand_graph --test stand_actions`
@leaf_edit n=6 leaf=.claude/docs/commands.md line=61 class=omission
leaf-text: answer bound; each picked up by `run stand`
source: fct-krn4eywd «`stand_actions` (8) holds, over the six served screens in both layout modes, each screen's listed actionable nodes with the table's verbs and their counts»
replace-with: answer bound; since 2026-11-08-what-can-i-do-here `stand_actions`, 8 tests: each served screen's actionable nodes listed with the table's verbs and counts, every listed cause on the four lean tasks equal to what `Session::run` returns, the flight booker's two options `covered` under `flight-type` until the list opens, `crud-person-14` `off-screen` until a `scroll`, the fixtures reading `covered`, unkeyed and `fonts-unavailable`, a read changing nothing and holding no typed text or mask, and the whole Cells answer within the answer bound; each picked up by `run stand`
