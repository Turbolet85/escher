@row_set n=1 row=decision/circle-drawer-circles-painted-on-canvas cell=rule seen=2 detector=none
value: Circle Drawer's circles are painted by a canvas widget from one data-circles attribute (x,y,d per circle, semicolon-joined, malformed text read as no circle), never as DOM nodes, so they reach no snapshot, tree, graph or lint and cross as shot pixels and canvas draw commands alone.
basis: fct-x7cb4p36, fct-hbtyqmii, fct-orfcp2mx
because: The canvas verb now reads Circle Drawer's draw commands on every carrier, so the circles no longer cross as shot pixels alone.
@row_set n=2 row=contract/escher-session-command/form cell=promise seen=2 detector=none
value: Every command is escher-session <verb> [arguments] --session <dir> over the sixteen verbs of the driver's verb table, --session exactly once on every command but help.
basis: fct-snllpxfi, fct-hecr3z5l
because: The verb table moved from fifteen to sixteen verbs with canvas, so the command form's verb count must follow.
@row_set n=3 row=contract/escher-session-mcp/tools cell=promise seen=2 detector=none
value: Lists the sixteen verbs of the verb table as tools in order, layout after actions, canvas after layout and help last, each description the verb's help then ". Returns: " and its result fields.
basis: fct-snllpxfi, fct-fzvmboq3
because: The MCP tool listing moved from fifteen to sixteen tools with canvas after layout, so the listing's promise must follow.
@row_set n=4 row=contract/escher-session-refusals/fonts-unavailable cell=promise seen=1 detector=none
value: screenshot, lint or canvas on an app with no font it can paint text with is refused fonts-unavailable, exit 1.
basis: fct-x7cb4p36, fct-gekogvs7
because: The canvas read is refused fonts-unavailable where the app cannot paint text, so the refusal's list of verbs must name it.
@row_add n=1 row=contract/escher-session-canvas/answer area=surface-cli fact=F1 detector=none
boundary: escher-session canvas
item: answer
promise: One required --id; without --full one line summarizing counts per command kind (layer, clip, mask, pop, fill, stroke, glyphs, shadow) with 64-bit digests, with --full a dump of one line per draw command.
basis: fct-hecr3z5l, fct-x7cb4p36
because: canvas is a new driver verb at the instance level after layout, and its answer is a contract on the command line no row states.
@row_add n=2 row=decision/canvas-answer-holds-no-text area=surface-cli fact=F2 detector=none
name: Canvas answer holds no text or glyph id
rule: A canvas answer holds closed words, numbers and 64-bit digests only, never text, a glyph id, a glyph position, font bytes or a font family name; glyph ids cross as a digest.
guard: the_fixtures_answers_hold_no_glyph_id_and_no_text
basis: fct-f4jjdetj, fct-ol62onmg
because: The chunk chose what a canvas answer may carry across every carrier, a ban future work on the canvas must respect.
@row_add n=3 row=contract/escher-session-canvas/part area=surface-cli fact=F3 detector=none
boundary: escher-session canvas
item: --part
promise: Optional whole number 1 to 1000, part 0 or 1001 refused malformed; a full dump over the 10,000-byte answer budget is cut into parts of whole lines, the fonts listed in part 1 alone, the parts joined equal to the whole, and a part past the last empty.
basis: fct-gr2bwf56, fct-hecr3z5l, fct-ol62onmg
because: The canvas dump is paged by a new argument whose cut is a contract callers read part by part.
@row_none n=1 fact=F4 type=9
what: A stated limit pending virtual nodes and an owed incubator line: the route and its deferred entry hold it, not a row.
@row_add n=4 row=decision/canvas-read-changes-nothing area=surface-desktop-native fact=F5 detector=none
name: Canvas read leaves the app unmoved
rule: A canvas read changes nothing in the app: the snapshot, both focus readings, the diff and the virtual clock stay unmoved, and pending Timer ticks stay unapplied.
guard: a_canvas_read_changes_nothing_with_timer_ticks_pending
basis: fct-gekogvs7
because: The new canvas read is promised to be side-effect free, a behaviour the other readings and the clock rely on.
@row_add n=5 row=decision/circle-drawer-canvas-reads-deterministically area=surface-desktop-native fact=F6 detector=none
name: Circle Drawer canvas reads deterministically
rule: Circle Drawer's canvas reads deterministically in both layout modes: two reads give the same bytes, one committed table holds across linux, windows and macOS, a click's line diff is all added, and advancing the clock gives an empty diff.
guard: stand_canvas_commands
basis: fct-hbtyqmii, fct-xpdkzexi
because: The chunk makes Circle Drawer's canvas reading a deterministic, cross-OS pinned surface that future canvas work must keep.
@row_add n=6 row=contract/escher-session-mcp/canvas-tool area=surface-cli fact=F7 detector=none
boundary: escher-session MCP server
item: canvas tool
promise: inputSchema id (required), full (boolean) and part (integer); answers the same line as the command line, and its tool_call span parents one command span.
basis: fct-x7cb4p36, fct-fzvmboq3
because: The MCP server gains a canvas tool whose input schema and answer are a contract to MCP clients.
