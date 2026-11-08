@apply_row n=1 ref_file=fanout-security-plan--cascade-g1.md ref_n=5 v=applied edit=E1
@apply_row n=2 ref_file=fanout-security-plan--cascade-g1.md ref_n=6 v=applied edit=E2
@apply_row n=3 ref_file=fanout-security-plan--cascade-g2.md ref_n=14 v=applied edit=E3
@apply_row n=4 ref_file=fanout-security-plan--cascade-g3.md ref_n=4 v=folded_into to_file=fanout-security-plan--cascade-g2.md to_n=14
@apply_row n=5 ref_file=fanout-security-plan--cascade-g2.md ref_n=15 v=applied edit=E4
@apply_row n=6 ref_file=fanout-security-plan--cascade-g1.md ref_n=1 v=applied edit=E5
@apply_row n=7 ref_file=fanout-security-plan--cascade-g1.md ref_n=2 v=applied edit=E6
@apply_row n=8 ref_file=fanout-security-plan--cascade-g1.md ref_n=3 v=applied edit=E7
@apply_row n=9 ref_file=fanout-security-plan--cascade-g1.md ref_n=4 v=applied edit=E8
@fact id=fct-z5ttw3ol expect_rev=1
old: a lint's findings — and no `NodeId`
new: a lint's findings, a UI graph's ids, component, state and handler event names, closed flag words and booleans — and no `NodeId`

@fact id=fct-dgkl4a64 expect_rev=1
old: screenshots and a lint's findings to the client
new: screenshots, a lint's findings and a UI graph's ids, component, state and handler event names, closed flag words and booleans to the client

@fact id=fct-cvyxuuhn expect_rev=1
old: table of twelve verbs
new: table of thirteen verbs

@fact id=fct-6hwcr6nz expect_rev=1
old: screenshots and a lint's findings, the state readings
new: screenshots, a lint's findings and a UI graph's ids, component, state and handler event names, closed flag words and booleans, the state readings

@fact after=fct-ljkdert4 file=threat-model-summary gid=new:socket-graph kind=values subject=boundary-session-socket
- An answer's `graph` also takes the screen's UI graph out on the socket, the command line, MCP and a kept check — ids, owner ids, component names, registered state names, handler event names (without `on`), the closed flag words `undeclared` and `unconfirmed` and booleans, never a control value, typed text or `NodeId` — a boundary widening, ratified (as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md).

@fact id=fct-ljkdert4 expect_rev=1
old: `NODE_FIELDS` is thirteen and
new: `NODE_FIELDS` is fourteen (`clickable`, a closed bool, after `focused`; as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md) and

@fact after=fct-gphdtjgv file=threat-model-summary gid=new:mcp-graph kind=values subject=boundary-mcp-stdio
- A `graph` result carries the screen's UI graph to the client as the text block of its JSON line — ids, owner ids, component names, registered state names, handler event names, closed flag words and booleans, no control value or typed text — the crossing the boundary-session-socket group states; what a client does with it, a model provider included, is the client's (as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md).

@fact id=fct-r5teidpw expect_rev=1
old: painted masked and a lint's findings, no description
new: painted masked, a lint's findings and a UI graph (ids, component names, registered state names, handler event names, closed flag words and booleans), no description

@pattern id=sec1-01 ignore_case=true
fixed: table of twelve verbs
@pattern id=sec1-02
fixed: `NODE_FIELDS` is thirteen
