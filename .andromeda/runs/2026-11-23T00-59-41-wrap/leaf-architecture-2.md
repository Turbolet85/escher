@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/blitz-dom.md lines=45
last: - `doc:architecture` (the records store) · `.claude/docs/stac
@leaf_read leaf=.claude/docs/services/blitz-paint.md lines=36
last: - `doc:architecture` (the records store) · `.claude/docs/serv
@leaf_read leaf=.claude/docs/services/blitz-shell.md lines=35
last: - `doc:architecture` (the records store) · `.claude/docs/serv
@leaf_read leaf=.claude/docs/services/blitz-test-harness.md lines=38
last: - `.claude/rules/verification-harness.md` · `.claude/docs/test
@leaf_read leaf=.claude/docs/services/blitz-traits.md lines=34
last: - `doc:architecture` (the records store) · `.claude/docs/serv
@leaf_edit n=1 leaf=.claude/docs/services/blitz-dom.md line=21 class=omission
leaf-text: `pseudo_element_flow_classification`, each of whose tests runs both layout modes)
source: dec-bb2kr7mh «decision/block-containment-through-display-contents · Block containment looks through display contents — Block containment looks through display contents, so an element whose display contents child hol»
replace-with: `pseudo_element_flow_classification`, each of whose tests runs both layout modes; block containment looks through `display: contents`, so an element whose `display: contents` child holds a block counts as containing a block and a `display: contents` element holding only inlines does not break its parent's inline flow, an upstream behaviour change taken)
@leaf_edit n=2 leaf=.claude/docs/services/blitz-dom.md line=21 class=omission
leaf-text: `create_text_editor`: two upstream engine changes for every document, not counted among escher's direct ones.
source: dec-xexhbk5x «decision/textarea-text-laid-out-once-width-known · Textarea text laid out once its width is known — A textarea's text is laid out only once the textarea's width is known, never at a provisional width, in every document.»
replace-with: `create_text_editor`: two upstream engine changes for every document, not counted among escher's direct ones; so, taken from upstream, a textarea's text is laid out only once the textarea's width is known, never at a provisional width.
@leaf_edit n=3 leaf=.claude/docs/services/blitz-dom.md line=21 class=omission
leaf-text: its own font loads — a direct engine change for every document, flagged upstreamable.
source: dec-dc2mrix6 «decision/logical-float-clear-by-direction · Logical float and clear sides resolved by direction — Logical float and clear values, inline-start and inline-end, resolve to left or right by the box's direction»
replace-with: its own font loads — a direct engine change for every document, flagged upstreamable. Logical `float` and `clear` values, `inline-start` and `inline-end`, resolve to left or right by the box's direction (an upstream behaviour change taken; stylo_taffy's float and clear converters take an `ltr` flag).
@leaf_stands n=1 leaf=.claude/docs/services/blitz-dom.md line=15
because: Its range pointer words match dec-szcsd6ey, unchanged in this pass: a main-button pointerdown sets the value from the hit's fraction of the content box and a click focuses it and changes no value.
