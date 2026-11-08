@apply_row n=1 ref_file=fanout-security-plan.md ref_n=1 v=applied edit=E1
@apply_row n=2 ref_file=fanout-security-plan--cascade-g2.md ref_n=15 v=folded_into to_file=fanout-security-plan.md to_n=1
@apply_row n=3 ref_file=fanout-security-plan.md ref_n=2 v=applied edit=E2
@apply_row n=4 ref_file=fanout-security-plan--cascade-g2.md ref_n=16 v=folded_into to_file=fanout-security-plan.md to_n=2
@apply_row n=5 ref_file=fanout-security-plan.md ref_n=3 v=applied edit=E3
@apply_row n=6 ref_file=fanout-security-plan--cascade-g1.md ref_n=4 v=folded_into to_file=fanout-security-plan.md to_n=3
@apply_row n=7 ref_file=fanout-security-plan.md ref_n=4 v=applied edit=E4
@apply_row n=8 ref_file=fanout-security-plan--cascade-g1.md ref_n=5 v=folded_into to_file=fanout-security-plan.md to_n=4
@apply_row n=9 ref_file=fanout-security-plan--cascade-g1.md ref_n=1 v=applied edit=E5
@apply_row n=10 ref_file=fanout-security-plan--cascade-g1.md ref_n=2 v=applied edit=E6
@apply_row n=11 ref_file=fanout-security-plan--cascade-g1.md ref_n=3 v=applied edit=E7
@fact id=fct-cvyxuuhn expect_rev=2
old: a closed `'static` table of thirteen verbs
new: a closed `'static` table of fourteen verbs

@fact id=fct-6hwcr6nz expect_rev=2
old: handler event names, closed flag words and booleans, the state readings
new: handler event names, closed flag words and booleans, an actions answer's ids, owner ids, the table's verb words, closed cause words and `keyed` booleans, the state readings

@fact id=fct-z5ttw3ol expect_rev=2
old: closed flag words and booleans — and no `NodeId`
new: closed flag words and booleans, an actions answer's ids, owner ids, the table's verb words, closed cause words and `keyed` booleans — and no `NodeId`

@fact id=fct-dgkl4a64 expect_rev=2
old: a lint's findings and a UI graph's ids
new: a lint's findings, a UI graph's ids
old: closed flag words and booleans to the client
new: closed flag words and booleans, and an actions answer's ids, owner ids, the table's verb words, closed cause words and `keyed` booleans to the client

@fact after=fct-fcig2nza file=threat-model-summary gid=new:actions-answer-socket kind=values subject=boundary-session-socket
- An answer's `actions` also takes what each actionable node can take now out on the socket, the command line, MCP and a kept check — `{"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}`: ids, owner ids, `keyed` booleans, the verb table's words and closed cause words, never a control value, typed text or `NodeId` (as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md).

@fact after=fct-algpllm7 file=threat-model-summary gid=new:actions-answer-mcp kind=values subject=boundary-mcp-stdio
- An `actions` result carries the actions answer to the client within the socket's content limits — ids, owner ids, `keyed` booleans, the verb table's words and closed cause words, no control value, typed text or `NodeId` — its crossing onto every ratified carrier a boundary widening, ratified, the `keyed` field provisional; what a client does with it, a model provider included, is the client's (as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md).

@fact id=fct-r5teidpw expect_rev=2
old: a lint's findings and a UI graph (ids
new: a lint's findings, a UI graph (ids
old: closed flag words and booleans), no description
new: closed flag words and booleans) and the actions answer (ids, owner ids, `keyed` booleans, verb words and closed cause words, no value), no description
@pattern id=sec1-01 ignore_case=true
fixed: table of thirteen verbs
