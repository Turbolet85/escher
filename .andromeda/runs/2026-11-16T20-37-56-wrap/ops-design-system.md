@row_add n=1 row=contract/acting-verb-answer/caused area=surface fact=F1 detector=none
boundary: acting verb answers
item: caused
promise: An acting verb's answer carries caused, one item per panic, error or log record: kind, level, source (app, framework or engine), target, message, location, the boundary that took it (root or own), an error's cause chain, a log's kept fields, and the handler's id, owner and event.
basis: fct-z2b5kqi7
because: The closed-words caused shape of counts was retired (fct-wed52rrj) and the answer now carries each record's text, a new promise on the agent-read surface that no surface row states.
@row_add n=2 row=contract/acting-verb-answer/caused-past area=surface fact=F2 detector=none
boundary: acting verb answers
item: caused_past
promise: Each text in caused is cut at 1024 bytes and says how much was cut; panics come first, then errors, then records in order; the first 16 items are kept and the rest are counted in caused_past, one entry per kind and source, written only when non-empty.
basis: fct-z2b5kqi7
because: The five acting verbs gained the caused_past field and caused a measured bound (fct-xz256kkb), a promise of the answer's shape that no surface row states.
@row_add n=3 row=contract/acting-verb-answer/caused-redaction area=surface fact=F3 detector=none
boundary: acting verb answers
item: caused field redaction
promise: In caused, a field whose name is in the content-field set, password among them, reads [redacted] on every source; the message and the other fields stay as their author wrote them, and a masked control's value stays masked.
basis: fct-xz256kkb
because: The capture now scrubs every source's fields by name before they cross in caused, a content promise of the answer that no surface row states.
@row_none n=1 fact=F4 type=4
relied: A caught panic always answers a panic item in caused, its message <payload unread> where the hook saw no payload, so the exploration's panic count never loses an item.
@row_none n=2 fact=F5 type=0
what: The carriers already promise the one answer line on the command line, the MCP role and the socket (the standing stdout and MCP result rows); caused rides that line, and the kept check's replay is the session-recording contract held outside design-system.
