@leaf_group group=security-plan-1
@leaf_read leaf=.claude/docs/security-summary.md lines=56
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=17 class=stale
leaf-text: and `snapshot`, `screenshot`, `lint`, `graph` and `actions` wait on none
source: dec-k3eandm3 «Time on a held instance moves only when a caller asks for it with advance, 1 to 60000 ms a call; no driver verb has an argument or result field of a wait, and snapshot, screenshot, lint, graph, actions and canvas wait on no load.»
replace-with: and `snapshot`, `screenshot`, `lint`, `graph`, `actions` and `canvas` wait on none
@leaf_edit n=2 leaf=.claude/docs/security-summary.md line=19 class=omission
leaf-text: and a live run also any layout, graph and actions answers of ids, closed words, booleans and numbers
source: dec-ncc47n6d «The cold-agent pipe's call allowlist takes the canvas verb, so a live run carries canvas answers through the claude client to the model provider.»
replace-with: and a live run also any layout, graph and actions answers of ids, closed words, booleans and numbers and any canvas answers of closed words, numbers and 64-bit digests (the pipe's call allowlist taking the canvas verb)
@leaf_edit n=3 leaf=.claude/docs/security-summary.md line=19 class=stale
leaf-text: `tool` one of the fifteen verbs
source: ctr-epkgn7ep «A step holds `tool` among the sixteen driver verbs, `arguments` an object and `expect` a list of strings; a check holds at most 400 steps.»
replace-with: `tool` one of the sixteen verbs
@leaf_edit n=4 leaf=.claude/docs/security-summary.md line=20 class=omission
leaf-text: and a `layout` answer's readings
source: dec-j54ckoth «An answer carries what the screen reads: ids, accessible names, control values, password and file input masked, snapshot text, diff, lint findings, UI graph, layout and canvas readings and a step's caused text»
replace-with: and a `canvas` answer's readings (closed words, numbers and 64-bit digests, fonts and images as indexes with digests, never a glyph id, glyph position, text, font bytes or family name; a boundary widening, ratified) and a `layout` answer's readings
@leaf_edit n=5 leaf=.claude/docs/security-summary.md line=20 class=omission
leaf-text: every listed node carrying `layout` with no cause: ids
source: ctr-jqgym43w «A list of entries each holding verb, a word of the verb table, and cause, a closed cause word left out when absent; every listed node carries layout with no cause, and canvas with no cause on an app that can paint text.»
replace-with: every listed node carrying `layout` with no cause, and `canvas` with no cause on an app that can paint text: ids
@leaf_edit n=6 leaf=.claude/docs/security-summary.md line=21 class=omission
leaf-text: `--ctrl`, bare and taking no value
source: ctr-52g3lj6v «contract/cli-flags/full · escher-session flags · --full — Bare, taking no value.»
replace-with: `--full`, bare and taking no value, is the `canvas` verb's full flag; `--ctrl`, bare and taking no value
@leaf_edit n=7 leaf=.claude/docs/security-summary.md line=22 class=omission
leaf-text: every listed node carrying `layout` with no cause)
source: ctr-jqgym43w «A list of entries each holding verb, a word of the verb table, and cause, a closed cause word left out when absent; every listed node carries layout with no cause, and canvas with no cause on an app that can paint text.»
replace-with: every listed node carrying `layout` with no cause, and `canvas` with no cause on an app that can paint text)
@leaf_edit n=8 leaf=.claude/docs/security-summary.md line=22 class=omission
leaf-text: and a `layout` result (ids, closed words and numbers, never a `NodeId`, text or value) by the crossing
source: dec-y7fvqq6a «A canvas's app-painted content crosses as pixels in a shot and, on the socket, the command line, MCP and a kept check, as a canvas answer of closed words, numbers and 64-bit digests, fonts and images as indexes»
replace-with: and a `layout` result (ids, closed words and numbers, never a `NodeId`, text or value) and a `canvas` result (closed words, numbers and 64-bit digests, fonts and images as indexes with digests, never a glyph id, glyph position, text, font bytes or family name; a boundary widening, ratified) by the crossing
@leaf_edit n=9 leaf=.claude/docs/security-summary.md line=22 class=stale
leaf-text: `tool` one of the eleven instance verbs
source: ctr-kwpi7ayu «One of the twelve instance verbs.»
replace-with: `tool` one of the twelve instance verbs
@leaf_edit n=10 leaf=.claude/docs/security-summary.md line=23 class=stale
leaf-text: a closed table of fifteen verbs in three levels — eleven asked of the held instance
source: ctr-s4mfww2m «The verb table holds sixteen verbs, `graph` listed after `lint`, `actions` after `graph`, `layout` after `actions` and `canvas` after `layout`.»
replace-with: a closed table of sixteen verbs in three levels — twelve asked of the held instance
@leaf_edit n=11 leaf=.claude/docs/security-summary.md line=23 class=omission
leaf-text: and three asked of the session
source: ctr-tdbagoml «Takes a required id within the id bound, an optional full flag and an optional part from 1 to 1000; any other argument, full twice, or part 0 or 1001 is refused malformed with nothing run, and the verb adds»
replace-with: and `canvas` the twelfth since 2026-11-24-canvas-draw-commands (one required `id` within the id bound, an optional `full` flag and an optional `part` from 1 to 1000; any other argument, `full` twice or `part` 0 or 1001 refused `malformed` with nothing run; adding no cause, argument kind, session error or exit status), and three asked of the session
@leaf_edit n=12 leaf=.claude/docs/security-summary.md line=23 class=stale
leaf-text: `VERBS` holding fifteen
source: ctr-s4mfww2m «The verb table holds sixteen verbs, `graph` listed after `lint`, `actions` after `graph`, `layout` after `actions` and `canvas` after `layout`.»
replace-with: `VERBS` holding sixteen
@leaf_edit n=13 leaf=.claude/docs/security-summary.md line=23 class=omission
leaf-text: a `layout` records `verb` alone and, refused, `cause`; a layout answer
source: ctr-jrtcmpib «Records the instance-level verb; a graph call, a graph part's included, an actions, layout or canvas call records the verb alone, plus cause when refused, never the part asked or the part count.»
replace-with: a `layout` or a `canvas` records `verb` alone and, refused, `cause`; a canvas answer, a layout answer
@leaf_edit n=14 leaf=.claude/docs/security-summary.md line=32 class=omission
leaf-text: and a `layout` step's whole answer
source: dec-37vb5ccc «A kept check holds a canvas step and its whole answer of closed words, numbers and 64-bit digests, never a glyph id, text or NodeId, and its replay stops naming digest when a canvas digest differs.»
replace-with: and a `canvas` step's whole answer (closed words, numbers and 64-bit digests, never a glyph id, text or `NodeId`; its replay stopping naming `digest` when a canvas digest differs) and a `layout` step's whole answer
@leaf_edit n=15 leaf=.claude/docs/security-summary.md line=32 class=retired
leaf-text: crosses only as pixels in a screenshot on every ratified carrier, the model provider on a cold-agent live run included, with no new carrier, field, cause, session error, exit, log field or span, and a kept check keeps no pixels
source: dec-y7fvqq6a «A canvas's app-painted content crosses as pixels in a shot and, on the socket, the command line, MCP and a kept check, as a canvas answer of closed words, numbers and 64-bit digests, fonts and images as indexes»
replace-with: crosses as pixels in a screenshot and, since 2026-11-24-canvas-draw-commands, on the socket, the command line, MCP and a kept check as a canvas answer of closed words, numbers and 64-bit digests, fonts and images as indexes with digests, never a glyph id, glyph position, text, font bytes or family name, the model provider on a cold-agent live run included, with no new carrier, cause, session error, exit, log field or span, and a kept check keeps no pixels
