@row_set n=1 row=contract/escher-session-command/form cell=promise seen=1 detector=none
value: Every command is escher-session <verb> [arguments] --session <dir> over the fifteen verbs of the driver's verb table, --session exactly once on every command but help.
basis: fct-w5rznwmn
because: The verb table grew from fourteen to fifteen verbs with layout, so the command form's verb count must follow.
@row_set n=2 row=contract/escher-session-mcp/tools cell=promise seen=1 detector=none
value: Lists the fifteen verbs of the verb table as tools in order, layout after actions and help last, each description the verb's help then ". Returns: " and its result fields.
basis: fct-w5rznwmn
because: The MCP tools listed moved from fourteen to fifteen with layout after actions, so the listing's promise must follow.
@row_set n=3 row=decision/no-caller-input-echoed cell=except seen=1 detector=none
value: The layout answer's id field names the element the call's id named.
basis: fct-xa2wojve
because: The layout answer always writes the id it was asked for as its first field, so the ban on writing back caller input now has this named exception.
@row_add n=1 row=contract/escher-session-layout/answer area=surface-cli fact=F1 detector=D-layout-surface
boundary: escher-session layout
item: answer
promise: One required --id; one uncoloured line of id, bounds and laid_out (x, y, width, height), placed (by, container, canvas, at, relative), shifts (by, holder, dx, dy), width and height (px, by, size, min, max, flex with basis, grow, shrink); ids, closed words and numbers only.
basis: fct-xa2wojve
because: The chunk adds the fifteenth verb layout with this answer form, a new contract at the command line no row states.
@row_add n=2 row=contract/escher-session-layout/placed-by area=surface-cli fact=F2 detector=none
boundary: escher-session layout
item: placed.by
promise: One of the closed words block, flex, grid, table, inline, float, absolute, fixed, root, select, select-list, unexplained or app.
basis: fct-xa2wojve
because: The layout answer's placing words are a new closed list at the command line, one row an item.
@row_add n=3 row=contract/escher-session-layout/shifts-by area=surface-cli fact=F2 detector=none
boundary: escher-session layout
item: shifts.by
promise: One of the closed words scroll or transform.
basis: fct-xa2wojve
because: The layout answer's shift words are a new closed list at the command line, one row an item.
@row_add n=4 row=contract/escher-session-layout/size-by area=surface-cli fact=F3 detector=none
boundary: escher-session layout
item: width.by and height.by
promise: One of the closed size causes style, min, max, flex-grow, flex-shrink, stretch, grid, table, replaced, content or layout.
basis: fct-xa2wojve
because: The layout answer's size causes are a new closed list at the command line, one row an item.
@row_add n=5 row=contract/escher-session-layout/length area=surface-cli fact=F3 detector=none
boundary: escher-session layout
item: length
promise: A length is one of auto, none, min-content, max-content, fit-content or stretch, or px, percent or calc with a number.
basis: fct-xa2wojve
because: The layout answer's length forms are a new closed list at the command line, one row an item.
@row_add n=6 row=decision/layout-words-fall-back area=surface-cli fact=F4 detector=none
name: Layout answer words fall back
rule: An engine value later than this build writes a fallback word in the layout answer: unexplained for a placer, transform for a shift, layout for a cause, auto for a length; the writer never panics.
why: The engine's sets are non-exhaustive; a fallback word leaves the writer no panic path.
basis: fct-pmdi26bm
because: The chunk chose a fallback word for each non-exhaustive engine set, a convention the layout answer's readers rely on.
@row_add n=7 row=contract/escher-session-layout/unexplained area=surface-cli fact=F5 detector=none
boundary: escher-session layout
item: unexplained
promise: An element the snapshot lists but the layout engine gives no box answers placed by unexplained at its snapshot bounds.
basis: fct-ighszuki
because: The chunk adds the unexplained placer for an element with no box, a promise of the layout answer no row states.
@row_add n=8 row=contract/escher-session-layout/changes-nothing area=surface-cli fact=F6 detector=none
boundary: escher-session layout
item: changes-nothing
promise: The layout verb changes nothing in the session and leaves delivered Timer ticks unapplied; an unread id is refused not-found and a deleted row's id stale, nothing changed.
basis: fct-h7vqoq7u
because: The chunk's layout verb promises a read that changes nothing, with its refusals, a contract no row states.
@row_add n=9 row=contract/escher-session-layout/carriers area=surface-cli fact=F7 detector=none
boundary: escher-session layout
item: carriers
promise: The layout MCP tool's text equals the command line's one line for the same id; every carrier writes the same answer line.
basis: fct-gd5fq3ek
because: The chunk puts layout on every carrier with one written line, a promise across the command line and MCP no row states.
@row_add n=10 row=decision/explorer-verbs-by-field-kind area=surface-cli fact=F8 detector=none
name: Explorer verbs chosen by field kind
rule: The explorer tries only click, type and scroll; a verb whose result holds an image or a layout reading is left out by its field kind, not its name.
basis: fct-yx7iahh5
because: The chunk keeps layout out of the explorer by its result's field kinds, a rule future verbs must respect.
@row_add n=11 row=decision/cold-agent-pipe-verbs-fourteen area=surface-cli fact=F9 detector=none
name: Cold-agent pipe verb list at fourteen
rule: The cold-agent pipe's verb list stays at fourteen without layout while the MCP server lists fifteen, so a live run reads not-isolated until the pipe widens or reads the listing less layout.
why: The conservative option of an open fork, applied ahead of its ratification.
state: provisional
basis: fct-tnufsys6
because: The pipe was held at fourteen as a provisional choice while the server lists fifteen, a decision the cold-agent rows do not state.
