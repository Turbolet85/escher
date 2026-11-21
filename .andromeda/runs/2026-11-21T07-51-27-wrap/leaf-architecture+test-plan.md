@leaf_group group=architecture+test-plan
@leaf_read leaf=.claude/docs/commands.md lines=82
last: - Fonts missing in headless tests → `system-fonts` feature /
@leaf_read leaf=.claude/docs/services/blitz-test-harness.md lines=37
last: - `.claude/rules/verification-harness.md` · `.claude/docs/te
@leaf_edit n=1 leaf=.claude/docs/commands.md line=58 class=stale
leaf-text: `press --key <key> [--shift]`
source: ctr-lshli5ex «contract/escher-session/press-key-shift-ctrl · escher-session command line · press --key [--shift] [--ctrl] — The verb press takes --key and the optional bare flags --shift and --ctrl.»
replace-with: `press --key <key> [--shift] [--ctrl]` (`--ctrl` bare, held as the host's action key, Meta on macOS and Control elsewhere)
@leaf_edit n=2 leaf=.claude/docs/commands.md line=58 class=stale
leaf-text: the ninth instance verb, no argument, an argument refused `malformed`; its accepted answer one line `{"nodes":[…],"states":[…]}`, status `0`)
source: ctr-amy33am2 «contract/escher-session-command-line/graph · escher-session command line · graph — `graph` takes `--part` with one value, a whole number 1 to 1000, anything else refused `malformed`;»
replace-with: the ninth instance verb, `graph [--part <part>]`, `--part` a whole number 1 to 1000, anything else refused `malformed`; a graph within the snapshot text budget with no part asked answers whole, one line `{"nodes":[…],"states":[…]}`, else a part one line `{"nodes":[…],"states":[…],"part":n,"parts":m}` with `states` in part 1 alone, status `0`)
@leaf_edit n=3 leaf=.claude/docs/commands.md line=59 class=omission
leaf-text: answering its line of the UI graph
source: dec-rt3wufhv «decision/large-screen-in-parts · Screen over the snapshot budget crosses in parts — A screen's snapshot or UI graph over the snapshot budget crosses in parts, one within the budget answering whole,»
replace-with: taking an optional integer `part` from 1 to 1000 and answering its line of the UI graph, a graph over the snapshot budget in parts as its JSON line with `part` and `parts`
