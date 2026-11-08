@leaf_group group=architecture+test-plan
@leaf_read leaf=.claude/docs/commands.md lines=82
last: - Fonts missing in headless tests → `system-fonts` feature /
@leaf_read leaf=.claude/docs/services/blitz-test-harness.md lines=37
last: - `.claude/rules/verification-harness.md` · `.claude/docs/te
@leaf_edit n=1 leaf=.claude/docs/commands.md line=40 class=stale
leaf-text: given only the escher server's twelve tools
source: fct-jelmr2gx «with the fixed prompt `STAND_PROMPT` and only the escher server's thirteen tools»
replace-with: given only the escher server's thirteen tools
@leaf_edit n=2 leaf=.claude/docs/commands.md line=58 class=omission
leaf-text: `lint` (the eighth instance verb, no argument; its accepted answer one line `{"findings":[…]}`, status `0`)
source: fct-76fzwvj7 «`graph` the ninth, no argument (an argument refused `malformed`), its accepted answer one line `{"nodes":[…],"states":[…]}`, status `0`»
replace-with: `lint` (the eighth instance verb, no argument; its accepted answer one line `{"findings":[…]}`, status `0`) · `graph` (the ninth instance verb, no argument, an argument refused `malformed`; its accepted answer one line `{"nodes":[…],"states":[…]}`, status `0`)
@leaf_edit n=3 leaf=.claude/docs/commands.md line=59 class=stale
leaf-text: its tools the twelve driver verbs
source: fct-t7zq5feb «`tools/list` answers the thirteen verbs of `VERBS` in order (`screenshot` after `scroll`, then `lint`, then `graph`), `help` last»
replace-with: its tools the thirteen driver verbs
@leaf_edit n=4 leaf=.claude/docs/commands.md line=59 class=omission
leaf-text: then `lint`, answering its line of findings)
source: fct-gtzxj2kr «a `graph` tool call answering its line of the UI graph»
replace-with: then `lint`, answering its line of findings, then `graph`, answering its line of the UI graph)
@leaf_edit n=5 leaf=.claude/docs/commands.md line=61 class=stale
leaf-text: 47,089 bytes in 740 lines and 5 parts
source: fct-tz35iqmq «a screen of 53,859 bytes in 740 lines, 6 parts of `SNAPSHOT_TEXT_BUDGET`, as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md»
replace-with: 53,859 bytes in 740 lines and 6 parts
@leaf_edit n=6 leaf=.claude/docs/commands.md line=61 class=omission
leaf-text: --test stand_cells_served --test stand_load_page`
source: fct-egnkfdyv «`stand_graph` 6 (`Session::run` in both layout modes, the `graph` verb's answer equal to `DioxusDocument::graph` on every read)»
replace-with: --test stand_cells_served --test stand_load_page --test stand_graph`
@leaf_edit n=7 leaf=.claude/docs/commands.md line=61 class=omission
leaf-text: and the page keyed with no lint finding at boot; each picked up by `run stand`
source: fct-q7ubtgz3 «`stand_graph`, in both layout modes, holds the `graph` verb's answer equal to `DioxusDocument::graph`, every graph node a snapshot node, no served id holding the reserved `///` form»
replace-with: and the page keyed with no lint finding at boot; since 2026-11-07-ui-graph `stand_graph`, 6 tests: the `graph` verb's answer equal to `DioxusDocument::graph` on every served screen, every graph node a snapshot node, a graph read changing nothing and holding no typed text, and the whole Cells answer within the answer bound; each picked up by `run stand`
