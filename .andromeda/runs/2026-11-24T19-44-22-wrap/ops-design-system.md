@row_add n=1 row=contract/escher-session-commands/canvas-arguments area=surface fact=F1 detector=none
boundary: escher-session commands
item: canvas arguments
promise: canvas takes one required --id, an optional --full and an optional --part from 1 to 1000; a missing id, an id past the id byte bound, a part of 0 or 1001, an argument it does not take or a second --full is refused malformed.
basis: fct-hecr3z5l
because: The chunk adds the canvas verb with its own argument shape, which no standing row of the surface states.
@row_add n=2 row=contract/escher-session-commands/canvas-answer area=surface fact=F2 detector=none
boundary: escher-session commands
item: canvas answer
promise: canvas answers by default a summary counting its commands by eight kinds (Layer, Clip, Mask, Pop, Fill, Stroke, Glyphs, Shadow) with a digest, and with --full one line per command; two reads of one state are the same bytes.
basis: fct-x7cb4p36
because: The chunk adds the canvas answer's two levels and its determinism, a contract no standing row holds.
@row_add n=3 row=contract/escher-session-commands/canvas-parts area=surface fact=F3 detector=none
boundary: escher-session commands
item: canvas parts
promise: A canvas full dump over the 10,000-byte answer budget is cut into parts of whole lines; the fonts are listed in part 1 alone, the parts joined equal the whole, and a part past the last is empty.
basis: fct-gr2bwf56
because: The chunk pages the canvas dump with its own rules, which the snapshot and graph part rows do not state.
@row_add n=4 row=contract/canvas-line/content area=surface fact=F4 detector=none
boundary: The canvas line
item: content
promise: A canvas answer, MCP result or kept check holds no glyph id, glyph position, text, font bytes or family name; glyph ids cross only as a digest.
basis: fct-f4jjdetj
because: The chunk adds a new answer whose crossing content is promised, as the actions and graph lines' content rows are.
@row_add n=5 row=decision/canvas-line-form-conventions area=surface fact=F5 detector=none
name: Canvas line form conventions
rule: The canvas line writes colours as rgba hex and a Custom paint as #00000000, blend= as mix,compose, cap= as one word where both caps agree, dash= as the lengths, @, then the offset, and a glyph run with no glyph as at=none.
why: Settled from neighbouring code and stated in the text field's help.
basis: fct-m6fmcplu
because: The chunk settles the canvas line's open form choices, a convention future readers and writers of the line must keep.
@row_add n=6 row=absence/canvas-command-identity area=surface fact=F6 detector=none
capability: An identity for a canvas draw command, so a changed command reads as one line removed and one added
basis: fct-yvyptbkp
because: The chunk states as a limit that canvas commands have no identity, a whole capability the project does not have.
@row_add n=7 row=contract/canvas-line/layer-filter area=surface fact=F6 detector=none
boundary: The canvas line
item: layer filter
promise: A layer's filter is written as a flag only.
basis: fct-yvyptbkp
because: The chunk states as a limit of the canvas read that a layer's filter crosses as a flag alone.
@row_add n=8 row=contract/driver-verbs-and-virtual-clock/canvas area=motion-calibrated-to-expression-level-unrecorded fact=F7 detector=none
boundary: driver verbs and the virtual clock
item: canvas
promise: The driver's canvas read changes nothing, neither snapshot, focus, diff nor VirtualClock, and applies no pending Timer tick.
basis: fct-gekogvs7
because: The chunk adds a verb whose relation to the virtual clock is promised beside advance, click, press and type.
@row_add n=9 row=contract/escher-session-mcp/canvas-input-schema area=surface fact=F8 detector=none
boundary: escher-session mcp role
item: canvas tool inputSchema
promise: The canvas tool's inputSchema has id required, full a boolean and part an integer.
basis: fct-x7cb4p36
because: The chunk adds the canvas tool to the MCP role, whose input shape is a contract no standing row holds; its result is already the other-results row's.
