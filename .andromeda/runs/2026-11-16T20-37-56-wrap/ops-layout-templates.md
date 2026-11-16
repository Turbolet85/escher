@row_add n=1 row=contract/escher-session-acting-verbs/caused area=surface-cli fact=F1 detector=none
boundary: escher-session acting verbs
item: caused
promise: Every acting verb's answer carries caused, one item per panic, error and log record, each with its message text, source (app, framework or engine), target, location, boundary and cause chain or kept fields; a content-named field reads [redacted].
basis: fct-z2b5kqi7, fct-xz256kkb
because: The acting verbs' answer now carries each record's text, source and location in caused instead of closed-word counts, a promise every caller of the CLI and MCP surface reads.
@row_add n=2 row=contract/escher-session-acting-verbs/caused-past area=surface-cli fact=F2 detector=none
boundary: escher-session acting verbs
item: caused_past
promise: caused keeps 16 items, 8 chain links and 8 fields an item, each text cut at 1024 bytes saying how many were cut; caused_past counts the rest by kind and source on click, type, press, advance and scroll, written only when non-empty.
basis: fct-z2b5kqi7, fct-pi3u54rh
because: The answer gains the caused_past field and fixed bounds on caused, a new promise of the acting verbs' answer shape.
@row_add n=3 row=decision/mcp-result-text-decoded-whole area=surface-cli fact=F3 detector=none
name: MCP result text decoded whole in comparisons
rule: A check comparing an MCP tool result against the in-process answer decodes the result text whole, as a JSON string, before comparing.
why: Un-escaping quotes alone leaves a Windows path's backslashes doubled, so the same answer reads unmatched.
basis: fct-xxjvdom5, fct-zbtw6u7h
because: The operator fix ruled how a check reads the MCP carrier against the in-process answer, a convention future MCP checks must follow.
