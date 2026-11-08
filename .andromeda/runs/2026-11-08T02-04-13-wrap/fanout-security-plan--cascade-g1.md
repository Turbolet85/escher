@proposal n=1 detector=D-new-detail severity=escalate file=threat-model-summary anchor=fct-ljkdert4 fact=F2 basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-qyqo7z6u
section: ## Threat Model Summary
change: add after the entity's last line: «An answer's `graph` also takes the screen's UI graph out on the socket, the command line, MCP and a kept check — ids, owner ids, component names, registered state names, handler event names (without `on`), the closed flag words `undeclared` and `unconfirmed` and booleans, never a control value or typed text — a boundary widening ratified by the founder's P4 fork 1 («Новый глагол graph (Рекомендую)», inputs#I4), halted on one Validate card with one confirm before apply (as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md).»
sidecar: threat-model-summary boundary-session-socket — the graph answer's crossing on every ratified carrier added, a ratified widening (2026-11-07-ui-graph)
rationale: the report's Expected amendments carry the graph's names crossing the socket, command line, MCP and kept checks as a ratified widening (inputs#I4), and its security outcome (fct-dbzqvx2q) states the carried set: ids, component, event and state names, closed flag words and booleans only
@proposal n=2 detector=D-cascade severity=warning file=threat-model-summary anchor=fct-ljkdert4 basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-yr4375o3
section: ## Threat Model Summary
change: in its select-list line, «`NodeState` holds no field of it, `NODE_FIELDS` is thirteen and no `expanded` node key stands» reads «`NodeState` holds no field of it, `NODE_FIELDS` is fourteen (`clickable`, a closed bool, joined after `focused`) and no `expanded` node key stands»
sidecar: threat-model-summary boundary-session-socket — NODE_FIELDS thirteen to fourteen, `clickable` (2026-11-07-ui-graph)
rationale: the report's Schema / config moves NODE_FIELDS 13 to 14 and its Counts / qualifiers mark every node-keys row stale, `clickable` joining after `focused`
@proposal n=3 detector=D-cascade severity=warning file=threat-model-summary anchor=fct-gphdtjgv dependent_of=D-new-detail basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-zklbw7ne
section: ## Threat Model Summary
change: add after the entity's last line: «A `graph` result carries the screen's UI graph to the client as the text block of its JSON line — ids, owner ids, component names, registered state names, handler event names, closed flag words and booleans, no control value or typed text — the crossing the boundary-session-socket group states; what a client does with it, a model provider included, is the client's (as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md).»
sidecar: threat-model-summary boundary-mcp-stdio — the graph result's carriage to the MCP client added (2026-11-07-ui-graph)
rationale: the report's Symbols / APIs state the MCP listing and result route `graph` unchanged, its answer the text block of its JSON line, and Coverage of new surfaces names the MCP tool among the graph's carriers
@proposal n=4 detector=D-cascade severity=warning file=threat-model-summary anchor=fct-r5teidpw dependent_of=D-new-detail basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-ypaagpkk
section: ## Threat Model Summary
change: «the carried set is ids, names, masked values, the snapshot text and diffs, a shot as an `image` block painted masked and a lint's findings, no description» reads «the carried set is ids, names, masked values, the snapshot text and diffs, a shot as an `image` block painted masked, a lint's findings and a UI graph (ids, component names, registered state names, handler event names, closed flag words and booleans), no description»
sidecar: threat-model-summary boundary-mcp-stdio — the capability statement's carried-onward set gains the UI graph (2026-11-07-ui-graph)
rationale: the report's New text, by line lists docs/capabilities.md 350-351 «- a UI graph: ids, component names, registered state names, handler event names, closed flag words and bool…» added to the carried set
@proposal n=5 detector=D-cascade severity=warning file=body anchor=fct-z5ttw3ol dependent_of=D-new-detail basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-dbzqvx2q
section: ## Threat Model Summary
change: «snapshot text, diffs, screenshots, a lint's findings — and no `NodeId`» reads «snapshot text, diffs, screenshots, a lint's findings, a UI graph's ids, component, state and event names, flag words and booleans — and no `NodeId`»
sidecar: security-plan body Threat Model Summary socket line — the graph answer joins the carried set (2026-11-07-ui-graph)
rationale: the broad line the boundary-session-socket group details lists the carried set, which the ratified graph answer joins (report security outcome fct-dbzqvx2q)
@proposal n=6 detector=D-cascade severity=warning file=body anchor=fct-dgkl4a64 dependent_of=D-new-detail basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-dbzqvx2q
section: ## Threat Model Summary
change: «snapshot text, diffs, screenshots and a lint's findings to the client» reads «snapshot text, diffs, screenshots, a lint's findings and a UI graph's ids, component, state and event names, flag words and booleans to the client»
sidecar: security-plan body Threat Model Summary MCP line — the graph result joins the carried set (2026-11-07-ui-graph)
rationale: the broad line the boundary-mcp-stdio group details lists what a result carries, which the graph tool's result now joins (report Coverage of new surfaces, fct-qxguy4c4)
@remark
F1 F3 F4 F5 F6 F9 (input-validation), F7 (api-security) and F8 (logging-monitoring) are other groups'; clickable (F6) is a closed bool inside the ratified carrier, no widening; detectors D-security-input, D-security-auth and D-security-deps read clean (graph argument refused malformed, no auth change, no dependency added)
