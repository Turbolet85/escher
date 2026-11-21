@leaf_group group=security-plan-1
@leaf_read leaf=.claude/docs/security-summary.md lines=56
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=13 class=stale
leaf-text: a key press with focus on a control whose anonymous block was dropped does not panic, for `hidden`, `style="display:none"` and CSS alone, `BaseDocument::deallocate_anonymous_block` setting `layout_parent` to `None` on each child still in the slab naming the block; focus stays and the key still acts
source: dec-je76blg7 «Dropping an anonymous block clears the layout parent of every child still naming it, so a key press on a re-focused control whose block was dropped does not panic and still acts.»
replace-with: a key press on a re-focused control whose anonymous block was dropped does not panic, for `hidden`, `style="display:none"` and CSS alone, `BaseDocument::deallocate_anonymous_block` setting `layout_parent` to `None` on each child still in the slab naming the block; the key still acts (a focused control that no longer shows gives focus up after layout)
@leaf_edit n=2 leaf=.claude/docs/security-summary.md line=13 class=omission
leaf-text: the clear firing no DOM `blur` or `focusout`, while any other programmatic focus change fires `blur`, `focusout`, `focus` and `focusin`
source: dec-hz7r3imf «A focused node gives focus up with no DOM `blur` or `focusout` when it comes under `inert` and, after each layout, when it no longer shows; a `display: contents` control and one beside a hidden sibling keep focus.»
replace-with: the clear firing no DOM `blur` or `focusout`; after each layout a focused node that no longer shows gives focus up the same way, firing none (a `display: contents` control and one beside a hidden sibling keep focus), while any other programmatic focus change fires `blur`, `focusout`, `focus` and `focusin`
@leaf_edit n=3 leaf=.claude/docs/security-summary.md line=14 class=omission
leaf-text: the browser's JS is behind the non-default `javascript` feature.
source: dec-jihszp6m «The seeded source of randomness takes its seed only from in-process code: a root context the virtual DOM's builder gives else seed 0, or a script document's building code, whose page Math.random it replaces;»
replace-with: a script document's page `Math.random` is replaced by the seeded source, seeded only by the document's in-process building code, reading no env var, file or carrier; the browser's JS is behind the non-default `javascript` feature.
