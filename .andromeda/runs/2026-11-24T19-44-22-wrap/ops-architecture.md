@row_set n=1 row=decision/no-node-id-across-session cell=rule seen=3 detector=none
value: Only ids, names, masked values, snapshot text, diffs, shots, lint findings, a step's caused text, layout readings, canvas readings, closed words, booleans, counts and numbers cross a driver session, never a `NodeId`.
basis: fct-x7cb4p36
because: The canvas verb now carries a canvas reading across every carrier with no NodeId held by CanvasReading, so the closed list of what crosses must name it.
@row_set n=2 row=decision/kept-check-never-logged cell=rule seen=1 detector=none
value: A kept check's screens, ids, names, values, snapshot text, diffs, findings, graph, actions and canvas answers rest at rest only, never logged and never named by a tool call.
basis: fct-ez6ffs55
because: A kept check now holds canvas answers whole, and the canvas answer is never logged, so the list of kept answers that rest unlogged must name it.
@row_set n=3 row=contract/actions-verb/target-verbs cell=promise seen=2 detector=none
value: For each node listed, the read covers each instance verb that takes an id argument, in table order, today `click` · `type` · `scroll` · `screenshot` · `layout` · `canvas`, `layout` with no cause and `canvas` with none on a document that can paint text.
basis: fct-snllpxfi
because: Target verbs per listed node moved from five to six, canvas last with no cause on an app that can paint text.
@row_set n=4 row=contract/argument-kinds/part cell=promise seen=2 detector=none
value: A whole number from 1 to 1000, the snapshot, graph or canvas dump part asked for.
basis: fct-x7cb4p36
because: The canvas verb takes the part argument for its paged dump, so the argument kind now serves three answers.
@row_set n=5 row=contract/command-line/verb-flags cell=promise seen=2 detector=D-arch-resources
value: --id, --text, --key, --ms and --part each take one value, --shift, --ctrl and --full are bare, and --session <dir> exactly once; the call is validated before the session is asked for.
basis: fct-x7cb4p36
because: The command line gains the canvas verb's bare --full flag, a new flag the contract must register.
@row_set n=6 row=contract/command-span/never-fielded cell=promise seen=1 fact=F7 detector=none
value: No screen text, image, findings, graph, actions answer, canvas answer, label, id record, argument, call, outcome or refusal value is ever a field.
basis: fct-ez6ffs55
because: The canvas answer is never logged, its command span recording verb alone and cause when refused, so the never-fielded list must name it.
@row_set n=7 row=contract/driver-explore/explored-verbs cell=promise seen=2 fact=F8 detector=none
value: The verbs explored are the actions table's target verbs less any whose result holds an image, a layout reading or a canvas reading field (png, rect, placement, shifts, size, commands or command-counts kind), selected by the field's kind and never its name; today click, type and scroll.
basis: fct-x7cb4p36
because: The explorer's filter by field kind now also leaves out the commands and command-counts kinds, the explored verbs unchanged.
@row_set n=8 row=contract/driver-session/command-schema cell=promise seen=2 detector=none
value: The command and refusal schema is stated once as 'static data: sixteen verbs, eight closed causes.
basis: fct-snllpxfi
because: The verb table moved from fifteen to sixteen verbs, a count this cell holds.
@row_set n=9 row=contract/kept-check-reader/strict-read cell=promise seen=2 detector=none
value: A kept check is read strictly: at most 1,048,576 bytes, a JSON array of at most 400 records, closed keys, the twelve instance verbs, schema argument names, `masked` on `type` alone, `digest` 16 lowercase hex, `answer` a required JSON object; key order is not enforced.
basis: fct-snllpxfi
because: Instance verbs moved from eleven to twelve and the reader accepts a canvas record, a count this cell holds.
@row_set n=10 row=contract/kept-check/kept-verbs cell=promise seen=2 detector=none
value: Only calls of the twelve instance verbs are kept, `lint`, `graph`, `actions`, `layout` and `canvas` among them with their answers whole; any other call is not kept.
basis: fct-snllpxfi
because: Instance verbs moved from eleven to twelve and a canvas call is kept with its answer whole.
@row_set n=11 row=contract/load-wait/verbs cell=promise seen=1 detector=none
value: The five acting verbs (`click` · `type` · `press` · `advance` · `scroll`) wait on every carrier; `snapshot`, `screenshot`, `lint`, `graph`, `actions` and `canvas` wait on nothing.
basis: fct-orfcp2mx
because: The canvas read runs no step and waits on nothing, so the list of verbs that wait on nothing must name it.
@row_set n=12 row=contract/mcp-server/tools-list cell=promise seen=2 detector=none
value: The sixteen verbs in table order, help last, each with its help and its result fields as description and an inputSchema with additionalProperties false.
basis: fct-snllpxfi
because: The MCP tool listing moved from fifteen to sixteen, canvas after layout, a count this cell holds.
@row_set n=13 row=contract/refusal-causes/fonts-unavailable cell=promise seen=1 detector=none
value: The app has no font it can paint text with; screenshot, lint and canvas refuse with nothing painted or read; remedy: boot the app with a font it can decode.
basis: fct-x7cb4p36
because: Session::canvas and the canvas target-check arm refuse fonts-unavailable where the document cannot paint text.
@row_set n=14 row=contract/verb-table/order cell=promise seen=2 detector=none
value: The verb table holds sixteen verbs, `graph` listed after `lint`, `actions` after `graph`, `layout` after `actions` and `canvas` after `layout`.
basis: fct-x7cb4p36
because: The verb row CANVAS joins VERBS after LAYOUT, moving the count from fifteen to sixteen.
@row_retire n=1 row=decision/canvas-crosses-as-pixels seen=1 heir=decision/canvas-crossing detector=D-arch-decisions
why: A canvas's content no longer crosses only as pixels: its draw commands now cross every carrier as the canvas verb's answer.
basis: fct-yvyptbkp
because: The founder's own answers re-state the pixels-alone decision, so the row is false in its name and rule and its heir states the new crossing.
@row_add n=1 row=decision/canvas-crossing area=established-decisions fact=F5 detector=D-arch-decisions
name: Canvas content crosses as draw commands and pixels
rule: A canvas's app-painted content crosses the driver's carriers, the session socket, the command line, MCP and a kept check, as the canvas verb's answer of draw commands beside a screenshot's pixels, with no new carrier, exit, cause, session error or span field.
why: A boundary widening, ratified.
basis: fct-fzvmboq3
because: The canvas verb crosses socket, command line, MCP and kept check, ratified on the founder's own answers, replacing the pixels-alone decision.
@row_add n=2 row=decision/canvas-reading-diffable area=established-decisions fact=F4 detector=none
name: Canvas reading deterministic and diffable by line
rule: A canvas reading is deterministic and diffable by line: two reads give the same bytes, a changed command reads as one line removed and one added, and a layer's filter is written as a flag only.
basis: fct-hbtyqmii
because: The canvas answer is read as a line-per-command dump whose two reads give the same bytes, a stated limit future work on command identity must respect.
@row_add n=3 row=decision/canvas-dump-in-parts area=established-decisions detector=none
name: Canvas dump over the snapshot budget crosses in parts
rule: A canvas's full dump over the snapshot text budget crosses in parts of whole command lines, each within the budget, the fonts in part 1 alone, one within the budget answering whole, with no new verb, carrier, exit, cause, session error or span field.
state: provisional
basis: fct-gr2bwf56
because: The canvas dump now pages within the snapshot text budget on a delegate's answer, so it stands provisional beside the ratified snapshot and graph paging.
@row_add n=4 row=contract/driver-verbs/canvas area=standard-contracts fact=F1 detector=D-arch-resources
boundary: driver verbs
item: canvas
promise: Instance level, after layout; takes id, optional full and part (1 to 1000); answers a summary of per-kind command counts and digests, with full a line-per-command dump, in parts past the snapshot text budget; refused not-found, stale, fonts-unavailable or malformed; changes nothing.
basis: fct-hecr3z5l
because: The chunk lands canvas as the sixteenth driver verb, a new verb the verb contracts must register.
@row_add n=5 row=contract/canvas-verb/content area=standard-contracts fact=F2 detector=D-arch-resources
boundary: The canvas verb
item: content
promise: The answer holds closed words, numbers and 64-bit FNV-1a digests on every carrier and in a kept check, fonts and images indexed by digest; never a glyph id, glyph position, text, font bytes or family name.
basis: fct-f4jjdetj
because: The canvas answer is a new answer crossing every carrier, and what it may hold is a promise at that boundary.
@row_add n=6 row=contract/canvas-verb/command-kinds area=standard-contracts detector=D-arch-resources
boundary: The canvas verb
item: command kinds
promise: The command kind set is closed at eight, named Layer · Clip · Mask · Pop · Fill · Stroke · Glyphs · Shadow.
basis: fct-x7cb4p36
because: The canvas answer's command kinds are a new closed set written on every carrier.
@row_add n=7 row=contract/canvas-verb/brush-forms area=standard-contracts detector=D-arch-resources
boundary: The canvas verb
item: brush forms
promise: A brush is one of five closed forms: solid, gradient, image, resource or pattern.
basis: fct-x7cb4p36
because: The canvas answer's brush forms are a new closed set written on every carrier.
@row_add n=8 row=contract/driver-session/canvas area=standard-contracts fact=F3 detector=D-arch-resources
boundary: escher-driver session
item: Session::canvas
promise: A read answering a CanvasReading of one canvas in process, resolving its id as Session::layout does, refusing fonts-unavailable where the document cannot paint text, changing nothing and opening no span of its own.
basis: fct-x7cb4p36
because: Session::canvas is a new public in-process read of the driver session.
@row_add n=9 row=contract/escher-session/canvas area=standard-contracts detector=D-arch-resources
boundary: escher-session command line
item: canvas --id [--full] [--part]
promise: The verb canvas takes --id and the optional bare --full and --part.
basis: fct-x7cb4p36
because: The command line gains the canvas verb, a new command form beside each other verb's row.
@row_add n=10 row=contract/target-check/canvas area=standard-contracts detector=D-arch-resources
boundary: Per-verb target check
item: canvas
promise: `canvas` meets `fonts-unavailable` alone.
basis: fct-x7cb4p36
because: The target check gains a canvas arm refusing fonts-unavailable and clearing every other target.
@row_add n=11 row=contract/paint/custom-widget-scene area=standard-contracts fact=F6 detector=D-arch-resources
boundary: blitz-paint
item: custom_widget_scene
promise: custom_widget_scene returns one custom widget's scene outside a raster at a given scale, behind the custom-widget feature, beside build_custom_widget_scenes; an escher addition flagged upstreamable.
basis: fct-hlztzwhc
because: blitz-paint gains a new public entry point reading one widget's scene, which the driver's canvas read relies on.
@row_add n=12 row=contract/test-harness/canvas-scene area=standard-contracts detector=D-arch-resources
boundary: blitz-test-harness
item: canvas_scene
promise: Harness::canvas_scene reads one custom widget's scene at the viewport's scale, with no resolve, tick or pump.
basis: fct-x7cb4p36
because: The test harness gains a new public reader of one widget's scene.
@row_none n=1 fact=F9 type=5
what: The canvas dump's measured byte sizes are figures stated in the tool's help and the capability statement; the decision that measured limits are stated where an agent reads the tool already stands.
