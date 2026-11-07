@carried n=1 label=security-plan/threat-model-summary fact=fct-ljkdert4
why: the buffered render must leave the socket's answers holding exactly this ratified set, so the buffer's ids, tag and attribute names and text never reach an answer and the snapshot and diff the stand answers with stay identical.
quote: an answer takes ids, accessible names, control values (a password's or a file input's as the fixed mask), the snapshot's text and the diff out
@carried n=1 run=2 label=security-plan fact=fct-z5ttw3ol
quote: it carries the lifecycle and calls with their answers — ids, names, control values (a password's or file input's masked), snapshot text, diffs, screenshots, a lint's findings — and no `NodeId`.
@carried n=2 label=security-plan/threat-model-summary fact=fct-gphdtjgv
why: every Dioxus document now renders through the buffer, so an MCP result must keep carrying only this ratified set, masked values still masked, with no mutation content and no new result field.
quote: A result carries ids, accessible names, control values (a password's or a file input's as the mask), the snapshot text and an acting call's diff to the client
@carried n=2 run=2 label=security-plan fact=fct-dgkl4a64
quote: a result carries ids, names, control values (a password's or file input's masked), snapshot text, diffs, screenshots and a lint's findings to the client
@binding n=1
**threat-model-summary ↔ input-validation**: the carried set on the socket and MCP carriers stays as ratified (security-plan/threat-model-summary fct-ljkdert4) (security-plan/threat-model-summary fct-gphdtjgv) only while the snapshot still reads a password's or file input's value as `MASKED_VALUE` and the stable id stays the HTML id or a path with no `NodeId` crossing (security-plan fct-767qdy6s) (security-plan fct-4jl4t3ye); the buffered replay must build the same document so both readers yield the same values.
@binding n=2
**threat-model-summary ↔ logging-monitoring**: the buffer is held in process only, so no log line or span on the render path may carry its mutation content onto any carrier (security-plan/threat-model-summary fct-ljkdert4) (security-plan fct-ggtnnptr).
