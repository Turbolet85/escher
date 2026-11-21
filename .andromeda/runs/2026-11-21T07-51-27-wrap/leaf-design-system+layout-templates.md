@leaf_group group=design-system+layout-templates
@leaf_read leaf=.claude/docs/design-summary.md lines=43
last: **Full plans:** `.andromeda/design-system.md` + `.andromeda/l
@leaf_edit n=1 leaf=.claude/docs/design-summary.md line=28 class=stale
leaf-text: takes no argument (one named refused `malformed`, nothing run)
source: ctr-i7o7dvjr «graph takes one optional --part, 1 to 1000; a part of 0, over 1000 or not a number, or any other argument, is refused malformed with nothing run.»
replace-with: takes one optional `--part`, 1 to 1000 (a part of 0, over 1000 or not a number, or any other argument, refused `malformed`, nothing run)
@leaf_edit n=2 leaf=.claude/docs/design-summary.md line=28 class=stale
leaf-text: answers the screen's whole UI graph as one uncoloured line `{"nodes":[…],"states":[…]}` on stdout
source: ctr-glvimj7c «One uncoloured line {"nodes","states"} for a whole graph, {"nodes","states","part","parts"} for a part, part 1 alone carrying the states and a later part's empty states meaning read part 1;»
replace-with: answers one uncoloured line on stdout, `{"nodes":[…],"states":[…]}` for a whole graph and `{"nodes":[…],"states":[…],"part":n,"parts":m}` for a graph read in parts, part 1 alone carrying the states and a later part's empty states meaning read part 1
@leaf_edit n=3 leaf=.claude/docs/design-summary.md line=28 class=stale
leaf-text: the closed flag words `undeclared` · `unconfirmed` and booleans only, never a value or typed text
source: ctr-i6rjnwml «The line carries ids, owner ids, component names, registered state names, handler event names, the closed flag words undeclared and unconfirmed, booleans, and for a graph read in parts its part and parts numbers»
replace-with: the closed flag words `undeclared` · `unconfirmed`, booleans, and for a graph read in parts its `part` and `parts` numbers only, never a value or typed text
@leaf_edit n=4 leaf=.claude/docs/design-summary.md line=28 class=stale
leaf-text: that answer unpaged, the whole screen in one line, Cells at boot 293,869 bytes, the stated limit «An unpaged graph» (residual `res-7cc5bi4k`, target next)
source: ctr-i6xa4xai «graph answers one uncoloured line of JSON on stdout, status 0, the same on the session socket's call and in a kept check, over the mcp role the text block of that line: {"nodes":[…],"states":[…]} for a whole graph»
replace-with: that answer a whole graph in one line or, with `--part`, a graph read in parts, each part its own text block
@leaf_edit n=5 leaf=.claude/docs/design-summary.md line=28 class=stale
leaf-text: its three stated limits: handlers inside a canvas not counted (owner 0.4.0), a handler on no screen a replay reached not listed (no owner), a bubbling dispatch counting every ancestor listener it passed, a `stopPropagation` unread (a residual row, target next).
source: ctr-rparzix5 «A bubbling dispatch that still propagates counts every listener it passed; a dispatch a listener stops, or a handler's panic unwinds, counts its innermost listener alone.»
replace-with: its two stated limits: handlers inside a canvas not counted (owner 0.4.0) and a handler on no screen a replay reached not listed (no owner); a bubbling dispatch that still propagates counts every listener it passed, and a dispatch a listener stops, or a handler's panic unwinds, counts its innermost listener alone.
