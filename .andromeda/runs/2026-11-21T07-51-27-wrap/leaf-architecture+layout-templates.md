@leaf_group group=architecture+layout-templates
@leaf_read leaf=.claude/docs/services/seven_guis.md lines=44
last: - `.claude/rules/a11y.md` (the stand is in its paths) · `.cl
@leaf_edit n=1 leaf=.claude/docs/services/seven_guis.md line=11 class=stale
leaf-text: The ninth instance verb, `graph --session <dir>`, after `lint`, takes no argument (one named refused `malformed`)
source: ctr-s3qxxjb3 «`graph` takes one optional `part`, a whole number from 1 to 1000; a part of 0, of 1001 or of text is refused `malformed` with nothing run.»
replace-with: The ninth instance verb, `graph [--part <part>] --session <dir>`, after `lint`, takes one optional `--part`, a whole number from 1 to 1000 (a part of 0, of 1001 or of text refused `malformed` with nothing run)
@leaf_edit n=2 leaf=.claude/docs/services/seven_guis.md line=11 class=omission
leaf-text: answers one uncoloured line `{"nodes":[…],"states":[…]}` on stdout, exit 0, carrying ids, component names, handler event names, registered state names, closed flag words and booleans only, never a value; over the `mcp` role an accepted `graph` answers its JSON line `{"nodes","states"}` as one text block.
source: ctr-glvimj7c «One uncoloured line {"nodes","states"} for a whole graph, {"nodes","states","part","parts"} for a part, part 1 alone carrying the states and a later part's empty states meaning read part 1; ids, component names»
replace-with: answers one uncoloured line `{"nodes":[…],"states":[…]}` on stdout, exit 0, whole when no part is asked and the answer is within the 10,000-byte snapshot text budget, else in parts of whole nodes adding `part` and `parts` — part 1 alone carrying the states, a later part's empty states meaning read part 1, a part past the last empty nodes and states — carrying ids, component names, handler event names, registered state names, closed flag words, booleans and the part counts only, never a value; over the `mcp` role an accepted `graph` answers its JSON line `{"nodes","states"}` (a part's adding `part` and `parts`) as one text block.
@leaf_edit n=3 leaf=.claude/docs/services/seven_guis.md line=38 class=stale
leaf-text: and the whole Cells answer (293,869 bytes) within the 1,048,576-byte answer bound.
source: ctr-rzq3uzv4 «graph answers whole when no part is asked and the answer is within the 10,000-byte snapshot text budget, else in parts of whole nodes within it; the states ride in part 1 alone, a later part's empty»
replace-with: and the Cells graph (293,869 bytes whole), past the 10,000-byte snapshot text budget, answered in parts of whole nodes within it, the states in part 1 alone.
