@row_add n=1 row=contract/escher-session-commands/layout-answer area=surface fact=F1 detector=none
boundary: escher-session commands
item: layout answer
promise: layout --id answers one uncoloured line of JSON {id, bounds, laid_out, placed, shifts, width, height} on stdout, status 0, the same on the session socket's call, over the mcp role and in a kept check; bounds and laid_out read x, y, width, height, an optional key left out when absent.
basis: fct-xa2wojve, fct-gd5fq3ek
because: The chunk adds the layout command on every carrier with its one-line answer, and no row of escher-session commands states its shape.
@row_add n=2 row=contract/layout-line/content area=surface fact=F2 detector=none
boundary: The layout line
item: content
promise: The line carries ids, closed kebab-case words for placers, shifts, size causes and length keywords, and numbers only.
basis: fct-xa2wojve
because: The new layout answer is held to ids, closed words and numbers only, as the actions, coverage and UI graph lines each state of their own content.
@row_add n=3 row=contract/escher-session-commands/layout-arguments area=surface fact=F4 detector=none
boundary: escher-session commands
item: layout arguments
promise: layout takes one required --id; a missing id, an id past the id bound or a second argument is refused malformed, status 1.
basis: fct-jgbhcxfs, fct-suhuk4an
because: The new layout command's argument contract stands beside those of actions and graph, and a missing id is refused malformed, not a usage ending.
@row_add n=4 row=contract/layout-line/no-box area=surface fact=F5 detector=none
boundary: The layout line
item: an element with no layout box
promise: An element the snapshot lists that has no layout box answers placed by unexplained at its snapshot bounds, its size causes and sizes read as for any other element.
basis: fct-ighszuki, fct-yx7iahh5
because: The layout answer gains the unexplained placer for a listed element with no box, a promise a caller reading the answer relies on.
@row_add n=5 row=contract/driver-verbs-and-virtual-clock/layout area=motion-calibrated-to-expression-level-unrecorded fact=F8 detector=none
boundary: driver verbs and the virtual clock
item: layout
promise: The driver's layout changes nothing and leaves the VirtualClock where it was; delivered Timer ticks stay unapplied across it.
basis: fct-h7vqoq7u
because: The new layout verb joins the driver verbs whose effect on the virtual clock this boundary states one verb a row.
@row_add n=6 row=decision/cold-agent-pipe-leaves-out-layout area=surface-cli fact=F9 detector=none
name: Cold-agent pipe leaves layout out
rule: The cold-agent pipe offers the driver verbs other than layout, fourteen, while the command line and the mcp role serve all fifteen.
why: The conservative option of an open fork, held until the fork is decided.
state: provisional
basis: fct-w5rznwmn, fct-tnufsys6
because: The chunk keeps the cold-agent pipe at fourteen verbs as a provisional choice, a decision future work on the pipe must respect.
@row_none n=1 fact=F3 type=4
what: The layout writer's fallback words for an engine set's later member, unreachable on this build; the code holds it.
@row_none n=2 fact=F6 type=5
what: The MCP tool listing's order and count; the code and its tests hold it, and the escher-session-mcp other-results row already states the tool result equals the command's line.
@row_none n=3 fact=F7 type=5
what: Layout's position among each actions node's verbs and the explored verb list; the actions-line nodes row already lists every id-taking instance verb, and the code holds the order.
