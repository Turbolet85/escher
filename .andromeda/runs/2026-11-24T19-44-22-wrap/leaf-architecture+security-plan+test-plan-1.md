@leaf_group group=architecture+security-plan+test-plan-1
@leaf_read leaf=.claude/docs/services/escher-driver.md lines=63
last: - The command line, through the real `escher-session` binary
@leaf_edit n=1 leaf=.claude/docs/services/escher-driver.md line=6 class=stale
leaf-text: the **MCP server** that serves the fifteen verbs as MCP tools
source: ctr-stdygm64 «The sixteen verbs in table order, help last, each with its help and its result fields as description and an inputSchema with additionalProperties false.»
replace-with: the **MCP server** that serves the sixteen verbs as MCP tools
@leaf_edit n=2 leaf=.claude/docs/services/escher-driver.md line=6 class=omission
leaf-text: a `layout` call read inside its call's `command` span.
source: ctr-elrfiwmi «A read answering a CanvasReading of one canvas in process, resolving its id as Session::layout does, refusing fonts-unavailable where the document cannot paint text, changing nothing and opening no span of its own.»
replace-with: a `layout` call read inside its call's `command` span. Since 2026-11-24-canvas-draw-commands the **canvas read** (`Session::canvas` in process and the `canvas` verb, the twelfth instance verb, after `layout`, on every carrier — the session socket's `call`, the command line and MCP — and recorded, its answer kept whole in a kept check) answers a `CanvasReading` of one canvas — a summary of per-kind command counts and digests, with `full` a line-per-command dump, in parts past the snapshot text budget — resolving its id as `Session::layout` does, refusing `fonts-unavailable` where the document cannot paint text, changing nothing and opening no span of its own.
@leaf_edit n=3 leaf=.claude/docs/services/escher-driver.md line=16 class=stale
leaf-text: a verb outside the eleven instance verbs, a `lint`, a `graph`, an `actions` or a `layout` step kept like every other with its answer whole
source: ctr-sfd4mvbb «Only calls of the twelve instance verbs are kept, `lint`, `graph`, `actions`, `layout` and `canvas` among them with their answers whole; any other call is not kept.»
replace-with: a verb outside the twelve instance verbs, a `lint`, a `graph`, an `actions`, a `layout` or a `canvas` step kept like every other with its answer whole
@leaf_edit n=4 leaf=.claude/docs/services/escher-driver.md line=17 class=stale
leaf-text: `VERBS` holds fifteen, `graph` listed after `lint`, `actions` after `graph` and `layout` after `actions`.
source: ctr-s4mfww2m «The verb table holds sixteen verbs, `graph` listed after `lint`, `actions` after `graph`, `layout` after `actions` and `canvas` after `layout`.»
replace-with: `VERBS` holds sixteen, `graph` listed after `lint`, `actions` after `graph`, `layout` after `actions` and `canvas` after `layout`.
@leaf_edit n=5 leaf=.claude/docs/services/escher-driver.md line=18 class=stale
leaf-text: today `click` · `type` · `scroll` · `screenshot` · `layout`, `layout` read with no cause)
source: ctr-rzjqi3nn «For each node listed, the read covers each instance verb that takes an id argument, in table order, today `click` · `type` · `scroll` · `screenshot` · `layout` · `canvas`, `layout` with no cause and `canvas` with none on a document that can paint text.»
replace-with: today `click` · `type` · `scroll` · `screenshot` · `layout` · `canvas`, `layout` read with no cause and `canvas` with none on a document that can paint text)
@leaf_edit n=6 leaf=.claude/docs/services/escher-driver.md line=18 class=omission
leaf-text: while `snapshot`, `screenshot`, `lint`, `graph`, `actions` and `layout`, which run no step, wait on nothing
source: ctr-s46mb5gp «The five acting verbs (`click` · `type` · `press` · `advance` · `scroll`) wait on every carrier; `snapshot`, `screenshot`, `lint`, `graph`, `actions` and `canvas` wait on nothing.»
replace-with: while `snapshot`, `screenshot`, `lint`, `graph`, `actions`, `layout` and `canvas`, which run no step, wait on nothing
@leaf_edit n=7 leaf=.claude/docs/services/escher-driver.md line=24 class=stale
leaf-text: `VERBS` — the verb table, fifteen verbs in three levels
source: ctr-s4mfww2m «The verb table holds sixteen verbs, `graph` listed after `lint`, `actions` after `graph`, `layout` after `actions` and `canvas` after `layout`.»
replace-with: `VERBS` — the verb table, sixteen verbs in three levels
@leaf_edit n=8 leaf=.claude/docs/services/escher-driver.md line=24 class=omission
leaf-text: `layout` (the eleventh, since 2026-11-17-layout-on-the-carriers: one required `id`; result `id`, `bounds`, `laid_out`, `placed`, `shifts`, `width` and `height`, each always present), then `start` (`app`)
source: ctr-y7bshpk5 «Instance level, after layout; takes id, optional full and part (1 to 1000); answers a summary of per-kind command counts and digests, with full a line-per-command dump, in parts past the snapshot text budget»
replace-with: `layout` (the eleventh, since 2026-11-17-layout-on-the-carriers: one required `id`; result `id`, `bounds`, `laid_out`, `placed`, `shifts`, `width` and `height`, each always present), `canvas` (the twelfth, since 2026-11-24-canvas-draw-commands: one required `id`, an optional bare `full` and an optional `part` (1 to 1000); a summary of per-kind command counts and digests, with `full` a line-per-command dump, in parts past the snapshot text budget; refused `not-found`, `stale`, `fonts-unavailable` or `malformed`; changes nothing), then `start` (`app`)
@leaf_edit n=9 leaf=.claude/docs/services/escher-driver.md line=24 class=stale
leaf-text: `snapshot`'s and `graph`'s, the seventh kind
source: ctr-tsfek65q «A whole number from 1 to 1000, the snapshot, graph or canvas dump part asked for.»
replace-with: `snapshot`'s, `graph`'s and `canvas`'s, the seventh kind
@leaf_edit n=10 leaf=.claude/docs/services/escher-driver.md line=24 class=stale
leaf-text: — 15 verbs, 15 node keys
source: ctr-yimmfwol «The command and refusal schema is stated once as 'static data: sixteen verbs, eight closed causes.»
replace-with: — 16 verbs, 15 node keys
@leaf_edit n=11 leaf=.claude/docs/services/escher-driver.md line=26 class=stale
leaf-text: `fonts-unavailable` from `screenshot` and `lint` among the verbs
source: ctr-uhg3mn2p «The app has no font it can paint text with; screenshot, lint and canvas refuse with nothing painted or read; remedy: boot the app with a font it can decode.»
replace-with: `fonts-unavailable` from `screenshot`, `lint` and `canvas` among the verbs
@leaf_edit n=12 leaf=.claude/docs/services/escher-driver.md line=27 class=stale
leaf-text: `tools/list` → the fifteen verbs of `VERBS` in order (`screenshot` after `scroll`, then `lint`, then `graph`, then `actions`, then `layout`)
source: ctr-stdygm64 «The sixteen verbs in table order, help last, each with its help and its result fields as description and an inputSchema with additionalProperties false.»
replace-with: `tools/list` → the sixteen verbs of `VERBS` in order (`screenshot` after `scroll`, then `lint`, then `graph`, then `actions`, then `layout`, then `canvas`)
@leaf_edit n=13 leaf=.claude/docs/services/escher-driver.md line=28 class=omission
leaf-text: read as a number, `--shift` and `--ctrl` bare
source: ctr-suwkkat5 «--id, --text, --key, --ms and --part each take one value, --shift, --ctrl and --full are bare, and --session <dir> exactly once; the call is validated before the session is asked for.»
replace-with: read as a number, `--shift`, `--ctrl` and `--full` bare
@leaf_edit n=14 leaf=.claude/docs/services/escher-driver.md line=32 class=retired
leaf-text: Since 2026-11-23-canvas-in-a-headless-shot a canvas's app-painted content, a widget painted from Rust, crosses only as pixels in a `screenshot` on every ratified carrier, the model provider on a cold-agent live run included, with no new carrier, verb, field, exit, refusal cause, session error, log field or span, and a kept check keeps no pixels: a ratified boundary widening.
source: dec-y7fvqq6a «A canvas's app-painted content crosses as pixels in a shot and, on the socket, the command line, MCP and a kept check, as a canvas answer of closed words, numbers and 64-bit digests, fonts and images as indexes»
replace-with: Since 2026-11-24-canvas-draw-commands a canvas's app-painted content, a widget painted from Rust, crosses as pixels in a `screenshot` and, on the session socket, the command line, MCP and a kept check, as the `canvas` verb's answer of closed words, numbers and 64-bit digests, fonts and images as indexes with digests, never a glyph id, glyph position, text, font bytes or family name, the model provider on a cold-agent live run included (the pipe's allowlist takes the `canvas` verb), with no new carrier, exit, cause, session error or span field: a ratified boundary widening.
@leaf_edit n=15 leaf=.claude/docs/services/escher-driver.md line=38 class=omission
leaf-text: and `snapshot`, `screenshot`, `lint`, `graph`, `actions` and `layout` wait on none
source: ctr-s46mb5gp «The five acting verbs (`click` · `type` · `press` · `advance` · `scroll`) wait on every carrier; `snapshot`, `screenshot`, `lint`, `graph`, `actions` and `canvas` wait on nothing.»
replace-with: and `snapshot`, `screenshot`, `lint`, `graph`, `actions`, `layout` and `canvas` wait on none
@leaf_edit n=16 leaf=.claude/docs/services/escher-driver.md line=49 class=omission
leaf-text: an accepted `actions` and an accepted `layout`; a refused call `verb` and `cause` only
source: ctr-jrtcmpib «Records the instance-level verb; a graph call, a graph part's included, an actions, layout or canvas call records the verb alone, plus cause when refused, never the part asked or the part count.»
replace-with: an accepted `actions`, an accepted `layout` and an accepted `canvas`; a refused call `verb` and `cause` only
@leaf_edit n=17 leaf=.claude/docs/services/escher-driver.md line=49 class=omission
leaf-text: an actions answer, its ids or its verb readings, the label
source: ctr-sia4gcg2 «No screen text, image, findings, graph, actions answer, canvas answer, label, id record, argument, call, outcome or refusal value is ever a field.»
replace-with: an actions answer, its ids or its verb readings, a canvas answer, the label
