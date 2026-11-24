@row_set n=1 row=contract/driver-refusal-causes/fonts-unavailable cell=promise seen=1 detector=none
value: A screenshot, lint or canvas on an app with no usable font is refused fonts-unavailable with nothing changed.
basis: fct-x7cb4p36 · fct-gekogvs7
because: Session::canvas is refused fonts-unavailable where the app cannot paint text, so the row's list of verbs so refused must name canvas.
@row_set n=2 row=contract/mcp-server/tools-list cell=promise seen=2 detector=none
value: tools/list offers the sixteen verbs in order, `layout` after `actions`, `canvas` after `layout` and `help` last.
basis: fct-snllpxfi
because: The MCP tool listing moved from fifteen to sixteen with `canvas` after `layout`, so the stated count and order are false.
@row_set n=3 row=contract/cold-agent-check/step cell=promise seen=2 detector=none
value: A step holds `tool` among the sixteen driver verbs, `arguments` an object and `expect` a list of strings; a check holds at most 400 steps.
basis: fct-snllpxfi · fct-rrwh4urw
because: The cold-agent pipe's verb list takes canvas and moved from fifteen to sixteen, so a step's tool is one of sixteen verbs.
@row_set n=4 row=contract/cold-agent-verdict/isolated cell=promise seen=2 detector=none
value: True when the init tools are exactly the sixteen `mcp__escher__{verb}` tools, one MCP server `escher` is connected, skills and slash commands are empty, the API key source is none and no `cwd` or `memory_paths` names the repository root's basename.
basis: fct-snllpxfi · fct-bhmucdym
because: The MCP server now lists sixteen tools and the pipe reads sixteen, so the isolation check's tool count of fifteen is false.
@row_set n=5 row=decision/socket-carries-commands cell=rule seen=3 detector=none
value: The session socket carries a call's element id and typed text in, and ids, accessible names, control values (a password's or file input's as the fixed mask), the snapshot's text, the diff, a step's caused with its error text, lint findings, layout answers and canvas answers out.
basis: fct-fzvmboq3 · fct-x7cb4p36
because: The canvas verb is served over the session socket's call, so canvas answers now cross it outward and the list of what crosses is incomplete.
@row_set n=6 row=contract/driver-session/reading-verbs cell=promise seen=2 detector=none
value: `snapshot`, `screenshot`, `lint`, `graph`, `layout` and `canvas` run no step and wait on nothing.
basis: fct-orfcp2mx · fct-gekogvs7
because: The canvas verb is a new reading verb that runs no step and waits on no load, so the closed list of reading verbs must name it.
@row_set n=7 row=contract/escher-session-command-line/command-form cell=promise seen=3 detector=none
value: `escher-session <verb> … --session <dir>` is the whole command line, sixteen verbs; the instance-level verbs take their arguments as flags spelt as the schema spells them (`--id`, `--text`, `--key`, `--ms` and `--part` each with one value, `--shift`, `--ctrl` and `--full` bare).
basis: fct-x7cb4p36 · fct-snllpxfi
because: The command line gains the canvas command with a bare `--full` flag and the verb count moved from fifteen to sixteen.
@row_set n=8 row=contract/driver-verbs/verb-table cell=promise seen=2 detector=none
value: Sixteen verbs in a stated order at three levels — `instance`, `session`, `tool` — each with help, arguments and at least one result field.
basis: fct-uhfo5wt4 · fct-snllpxfi
because: The verb table moved from fifteen to sixteen verbs with canvas after layout, so the stated count is false.
@row_set n=9 row=decision/typed-password-never-returned cell=rule seen=3 detector=none
value: A password typed through the driver and a file input's path occur nowhere in what a call returns, graph, actions, layout and canvas answers included, nor in a kept or an explored path's check, in both layout modes; the value reads the mask, and a typed password never leaves as pixels.
basis: fct-jjgzq2qg · fct-xmjwtkdr
because: The masks check now reads the new canvas answer for the typed sentinel and the file input, so the answers the rule covers include canvas.
@row_set n=10 row=contract/mcp-tools-list/order cell=promise seen=2 detector=none
value: `tools/list` offers the sixteen verbs in order, `graph` after `lint`, `actions` after `graph`, `layout` after `actions`, `canvas` after `layout` and `help` last.
basis: fct-snllpxfi · fct-fzvmboq3
because: The MCP tool listing moved from fifteen to sixteen with `canvas` after `layout`, so the stated count and order are false.
@row_add n=1 row=decision/canvas-commands-table-across-hosts area=test-strategy fact=F1 detector=D-tests-coverage
name: Canvas commands held to one table across hosts
rule: Circle Drawer's canvas draw commands are read in process in both layout modes into one committed table, at boot, after a click and at 12 and 100 circles, as summary, full dump and parts, each answer held equal by size and digest on linux x86_64, windows x86_64 and macOS aarch64.
guard: stand_canvas_commands
basis: fct-hbtyqmii · fct-xpdkzexi
because: The chunk adds a cross-host committed table for canvas command reads, a test strategy no standing row states, the shot rows covering pixels alone.
@row_add n=2 row=contract/driver-session/canvas area=session-lifecycle fact=F2 detector=none
boundary: Driver session
item: canvas
promise: Answers a summary by default (command counts per kind, size, digest), the whole dump with `full`, or one part with `part` 1 to 1000; an element holding no canvas reads `canvas` false; an unread or deleted id is refused `not-found` or `stale`, changing nothing.
basis: fct-hecr3z5l · fct-gekogvs7
because: The canvas verb is a new reading of the driver session whose answer levels and refusals no row states.
@row_add n=3 row=decision/canvas-answer-holds-no-content area=integration-test-strategy fact=F3 detector=none
name: Canvas answer holds no content
rule: A canvas answer holds closed words, numbers and 64-bit digests only, never a glyph id, a glyph position, text, font bytes or a family name, on every carrier and in kept checks.
guard: the_fixtures_answers_hold_no_glyph_id_and_no_text
basis: fct-f4jjdetj · fct-ol62onmg
because: The canvas answer is a new surface crossing socket, command line, MCP and kept checks, and what it may hold is a boundary decision no row states.
@row_add n=4 row=contract/session-run/canvas-repeatable area=integration-test-strategy fact=F4 detector=none
boundary: escher_driver Session
item: canvas read
promise: A canvas read is deterministic: two reads give the same bytes, a click's line diff is all added and none removed, and an advance with nothing drawn reads an empty diff.
basis: fct-hbtyqmii
because: The canvas read is promised diffable and repeatable, a contract of the new verb no row states.
@row_add n=5 row=contract/driver-session/canvas-parts area=session-lifecycle fact=F5 detector=none
boundary: Driver session
item: canvas parts
promise: A canvas dump over the text budget is cut into parts of whole lines, each within 10,000 bytes, fonts and images in part 1 alone; the parts joined equal the whole, and a part past the last is empty.
basis: fct-gr2bwf56 · fct-x7cb4p36
because: The canvas dump is paged by its own cut, a contract beside the snapshot's and graph's parts that no row states.
@row_add n=6 row=contract/canvas-answer/changed-command area=integration-test-strategy fact=F6 detector=none
boundary: Canvas answer
item: changed command
promise: A canvas command has no identity: a changed command reads as one line removed and one added, until virtual nodes give commands an identity.
basis: fct-re2xbmyu · fct-yvyptbkp
because: The chunk ships a stated limit of the canvas diff that checks and future work must read as a promise of the answer.
@row_add n=7 row=contract/canvas-answer/layer-filter area=integration-test-strategy fact=F6 detector=none
boundary: Canvas answer
item: layer filter
promise: A layer's filter is written as a flag only.
basis: fct-yvyptbkp
because: The chunk ships a stated limit of the canvas line form, what a layer's filter reads as, which no row states.
@row_add n=8 row=contract/escher-session-command-line/canvas area=session-lifecycle detector=none
boundary: escher-session command line
item: canvas
promise: `canvas --id <id> [--full] [--part <part>]` takes `--part` as a whole number 1 to 1000 and `--full` once, anything else refused `malformed`, status `1`; an accepted answer is one line, status `0`.
basis: fct-hecr3z5l · fct-x7cb4p36
because: The usage line gains the canvas command, which would pass the usage row's bound, so the command's form stands as a row of its own.
@row_add n=9 row=decision/control-leaves-earlier-target-green area=unit-test-strategy fact=F7 detector=none
name: Control edit leaves earlier test targets green
rule: A control whose case lies in the later of two test targets named by one gate entry with no fail-fast override needs an edit that leaves the earlier target green.
why: Cargo runs the targets in name order and stops at the first red one, so the named case would go unrun.
basis: fct-hqpvmd4z · fct-kqodhpk2
because: The chunk measured that a control reddening an earlier target leaves its own case unrun, a rule for building controls no row states.
