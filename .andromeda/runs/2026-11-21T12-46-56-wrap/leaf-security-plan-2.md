@leaf_group group=security-plan-2
@leaf_read leaf=.claude/rules/security.md lines=32
last: - 2026-10-15: A listener or env read that only a test makes 
@leaf_edit n=1 leaf=.claude/rules/security.md line=9 class=omission
leaf-text: (a root context the virtual DOM's builder gives, else the default seed 0; outside any run, the host's per-process random keys)
source: dec-jihszp6m «The seeded source of randomness takes its seed only from in-process code: a root context the virtual DOM's builder gives else seed 0, or a script document's building code, whose page Math.random it replaces»
replace-with: (a root context the virtual DOM's builder gives, else the default seed 0, or a script document's building code, whose page `Math.random` it replaces; outside any run, the host's random keys)
@leaf_edit n=2 leaf=.claude/rules/security.md line=12 class=stale
leaf-text: A key press with focus on a control whose anonymous block was dropped does not panic
source: dec-je76blg7 «Dropping an anonymous block clears the layout parent of every child still naming it, so a key press on a re-focused control whose block was dropped does not panic and still acts.»
replace-with: A key press on a re-focused control whose anonymous block was dropped does not panic
@leaf_edit n=3 leaf=.claude/rules/security.md line=12 class=stale
leaf-text: focus stays and the key still acts
source: dec-hz7r3imf «A focused node gives focus up with no DOM `blur` or `focusout` when it comes under `inert` and, after each layout, when it no longer shows; a `display: contents` control and one beside a hidden sibling keep focus.»
replace-with: a focused control that no longer shows gives focus up after layout, and on one re-focused the key still acts
@leaf_edit n=4 leaf=.claude/rules/security.md line=17 class=omission
leaf-text: the clear firing no DOM `blur` or `focusout`,
source: dec-hz7r3imf «A focused node gives focus up with no DOM `blur` or `focusout` when it comes under `inert` and, after each layout, when it no longer shows; a `display: contents` control and one beside a hidden sibling keep focus.»
replace-with: the clear firing no DOM `blur` or `focusout`, and after each layout a focused node that no longer shows gives focus up the same way, firing none (a `display: contents` control and one beside a hidden sibling keep focus),
