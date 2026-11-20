@row_set n=1 row=contract/snapshot-node-keys/expanded cell=promise seen=1 detector=D-arch-resources
value: The snapshot carries no expanded key and NodeState no field for a list; NODE_FIELDS keeps fifteen keys.
basis: fct-odzydliy
because: The node-key table grew from fourteen to fifteen keys with highlighted, so the count the row holds is now false.
@row_set n=2 row=contract/snapshot-diff/reading cell=promise seen=1 detector=D-arch-resources
value: Thirteen variants: Parent, Role, Name, Enabled, Checked, Value, Selected, Pressed, Invalid, Focused, Clickable, Highlighted, Bounds, with Reading::ALL.
basis: fct-voh3czmv
because: Reading gained the Highlighted variant before Bounds and Reading::ALL moved from twelve to thirteen, so the listed variants are now incomplete.
@row_set n=3 row=contract/snapshot/node-state cell=promise seen=1 detector=D-arch-resources
value: NodeState carries nine fields in the order enabled, checked, value, selected, pressed, invalid, focused, clickable, highlighted.
basis: fct-odzydliy
because: NodeState gained the public highlighted field, moving from eight to nine fields, so the row's count and order are false.
@row_set n=4 row=contract/snapshot-text/state-tokens cell=promise seen=1 detector=D-arch-resources
value: In fixed order enabled/disabled, checked/unchecked, value=, selected/unselected, pressed/unpressed, invalid, focused, clickable, highlighted; absent where the model holds nothing, invalid, clickable and highlighted written only when true.
basis: fct-voh3czmv
because: The snapshot text gained the highlighted token after clickable, written only when true, so the closed token list is incomplete.
@row_set n=5 row=contract/answer-json/node cell=promise seen=1 detector=D-arch-resources
value: An object keyed by the node keys in order, parent, enabled, checked, value, selected and pressed left out when absent, highlighted written only when true; id, role, name, invalid, focused, clickable and bounds always written.
basis: fct-s4gho2nl
because: The JSON node now writes highlighted true only when true, a key the row's written-or-left-out rule does not cover.
@row_set n=6 row=decision/select-list-crossing cell=rule seen=2 detector=D-arch-decisions
value: The select list crosses the carriers only as a row's bounds and closed bool highlighted in the snapshot and diff, differs naming either, the words select and select-list in a layout answer, and pixels in a screenshot; no new carrier, exit, cause, session error, log field or span.
basis: fct-s4gho2nl
because: The highlight now crosses as the closed bool node key highlighted inside the ratified snapshot and diff carriers, so the rule's ban on a new node key is false.
@row_set n=7 row=decision/select-list-changed-set cell=rule seen=1 fact=F6 detector=D-arch-decisions
value: Where a select list stands, a page scroll or a move that closes it marks its select at the next resolve, a wheel elsewhere marks it on close, and a highlight move to another row, by key or pointer, marks it; a move to the same row and a wheel move inside the list mark nothing.
basis: fct-voh3czmv
because: A highlight move to another row now inserts the list's select into the changed set, so the row's claim that a highlight move marks nothing is false.
@row_set n=8 row=decision/changed-set-write-sites cell=rule seen=2 detector=D-arch-decisions
value: The changed set is written only by mutations whose condition holds, focus or checked changes, typed text, range moves, value-changing select choices and a select list opening, closing or moving its highlight to a new row; never by creation, hover, active, scroll, resize, load, animation or layout.
basis: fct-voh3czmv
because: A highlight move to another row of an open select list is a new write site of the changed set, so the row's closed list of write sites is incomplete.
@row_set n=9 row=contract/lint-verb/contrast cell=promise seen=1 fact=F4 detector=none
value: A node with direct non-whitespace text or a text control, reported below the required ratio, exempt when disabled; an open list's highlighted row is read at its painted pair, the engine's highlight colours; a node past an element with a `mix-blend-mode` other than `normal` is read as not measured.
basis: fct-voh3czmv
because: The lint's contrast check now reads an open list's highlighted row at its painted pair before the composited reading, which the row's promise does not say.
@row_add n=1 row=contract/test-harness/highlight-contrast area=standard-contracts fact=F1 detector=D-arch-resources
boundary: blitz-test-harness
item: highlight_contrast
promise: A public read-only reader that answers only for an open select list's highlighted row, giving the engine's highlight text on highlight background through the one contrast_ratio with the option's computed font size and weight; it answers none elsewhere and changes nothing.
basis: fct-voh3czmv
because: The harness gained a new public reader, highlight_contrast, which no row of the harness's contracts registers.
@row_add n=2 row=contract/node-keys/highlighted area=standard-contracts fact=F2 detector=D-arch-resources
boundary: node keys
item: highlighted
promise: A closed bool, true on an open single select list's highlighted option alone, read from the accessibility tree's active descendant; written only when true, as a bare text token after clickable and as JSON true.
basis: fct-s4gho2nl
because: The node-key schema gained highlighted as its fifteenth key, which no node-keys row registers.
@row_add n=3 row=contract/accessibility-tree/active-descendant area=standard-contracts fact=F3 detector=D-arch-resources
boundary: Accessibility tree
item: active descendant
promise: While a single select's list stands open with a current option, the tree names that option as the select's active descendant, and none while closed; the tree's focus stays on the select and a page's aria-activedescendant is never read.
basis: fct-voh3czmv
because: The accessibility builder now writes an open single select's active descendant, a tree promise no accessibility-tree row states.
@row_add n=4 row=decision/select-list-rows-one-height area=established-decisions fact=F5 detector=none
name: Open select list rows share one height
rule: Every row of an open select list takes one height: the tallest row's own layout height, with the select's one-line height as the floor, plus row padding.
why: so no row's glyphs are clipped; ratified as a direct engine change
basis: fct-voh3czmv
because: The list's row height is now the tallest row's height rather than the select's, a ratified engine decision no row of architecture holds.
