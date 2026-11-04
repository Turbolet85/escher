@carried n=1 label=security-plan/threat-model-summary fact=fct-ljkdert4
why: The probe must leave the session socket's carried set exactly as ratified: no component name, handler, signal value or `NodeId` it reads may reach a call or an answer.
quote: The wire carries the lifecycle (`hello`, `stop`) and, since 2026-10-10-driver-cli, calls and their answers: a call brings an element id and typed text in, an answer takes ids, accessible names, control values (a password's or a file input's as the fixed mask), the snapshot's text and the diff out
@carried n=2 label=security-plan/threat-model-summary fact=fct-ljkdert4
why: The probe must add no listener or port; the one local IPC stays the owning user's Unix-domain socket.
quote: no TCP or UDP port; unix only
@carried n=3 label=security-plan/threat-model-summary fact=fct-ljkdert4
why: Any widening of the socket's crossing is the founder's call, so a probe link whose verdict would need a new crossing must stop at the verdict and go to the founder as a fork, not be built.
quote: A boundary widening, the founder's choice (the founder, 2026-10-07, relayed verbatim)
@carried n=4 label=security-plan/threat-model-summary fact=fct-ljkdert4
why: The probe must add no node key or snapshot field, so the node-field count and the absence of new keys stay as measured.
quote: `NodeState` holds no field of it, `NODE_FIELDS` is thirteen and no `expanded` node key stands
@carried n=5 label=security-plan/threat-model-summary fact=fct-gphdtjgv
why: The MCP stdio server must stay listener-free and its result set unchanged, because anything put in a result can reach the model provider through the client.
quote: it reads JSON-RPC 2.0 from stdin and writes its replies to stdout — no listener, port or socket — and holds the session it boots in its own process
@carried n=6 label=security-plan/threat-model-summary fact=fct-gphdtjgv
why: The probe's readings must not enter an MCP result, since what a client does with a result is out of escher's hands.
quote: A result carries ids, accessible names, control values (a password's or a file input's as the mask), the snapshot text and an acting call's diff to the client
@carried n=7 label=security-plan/threat-model-summary fact=fct-r5teidpw
why: Since the probe adds no carrier and no carried item, the committed statement in `docs/capabilities.md` must remain true without an edit.
quote: a statement of the ratified crossing that adds no carrier, its pin reading none of it
@carried n=8 label=security-plan fact=fct-z5ttw3ol
why: The body's broad line for the socket vector, which the probe must leave unmoved: its carried set and its "no `NodeId`" rule.
quote: it carries the lifecycle and calls with their answers — ids, names, control values (a password's or file input's masked), snapshot text, diffs, screenshots, a lint's findings — and no `NodeId`.
@binding n=1
**Session socket and MCP carriers ↔ the scope's «Nothing crosses» boundary**: the scope bars any new verb, argument, node key, result field, carrier, listener or port, and the ratified carried sets this file states are the measure the probe must leave unchanged (security-plan/threat-model-summary fct-ljkdert4) (security-plan/threat-model-summary fct-gphdtjgv) (security-plan fct-z5ttw3ol).
@binding n=2
**Socket crossing ↔ Input Validation's widening rule**: a field that holds authored text or an open value set (a component name, a handler name, a signal value) would be a boundary widening, and only the founder can ratify one. So such a need must appear in the founder's reading as a fork and must not be built (security-plan/threat-model-summary fct-ljkdert4) (security-plan fct-n4q5kc73).
@binding n=3
**MCP crossing ↔ `docs/capabilities.md`**: the committed statement of what an MCP client carries onward must stay accurate unedited, which holds only while the probe adds nothing to a result (security-plan/threat-model-summary fct-r5teidpw) (security-plan/threat-model-summary fct-gphdtjgv).
