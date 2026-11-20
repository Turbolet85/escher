@row_set n=1 row=decision/chosen-select-row-colours cell=rule seen=1 detector=none
value: A chosen select row is painted with the engine default #000000 on #c8c8c8, and an open list's highlighted row shares that pair.
basis: fct-73upmmhj
because: The report pins the highlight to the default sheet's chosen row read as #000000 on #c8c8c8, so the row's pair and its reach must follow.
@row_set n=2 row=decision/contrast-reader-reads-only cell=rule seen=1 detector=none
value: The harness's contrast readers, of an element's text and of an open list's highlighted row, read and change nothing: no mutation, input, pump or settle.
basis: fct-voh3czmv
because: A second public read-only contrast reader was added to the harness, so the row naming one reader is incomplete.
@row_set n=3 row=decision/count-and-rename-sweeps cell=rule seen=2 fact=F4 detector=none
value: A change of the driver's verb or node-key count sweeps the tests and examples for pins of the table's length and of its indices, the help description's count list and the stand diff check's per-reading list among them, and a test rename sweeps the old name held as a string literal.
basis: fct-biulgp6m
because: The node-key table's length is pinned outside its name in the help description's test and the stand diff check, which a sweep on the table names missed.
@row_set n=4 row=decision/one-contrast-formula cell=rule seen=1 fact=F2 detector=none
value: Session lint's contrast reads the harness's highlighted-row reading first and its text contrast where that answers none, so an open list's highlighted row is judged at its painted pair, with no second formula.
basis: fct-voh3czmv
because: The lint's contrast now takes the highlight reader before the text contrast, so a row saying it reads as text contrast alone is false.
@row_set n=5 row=decision/expected-value-pinned-as-literal cell=except seen=1 fact=F3 detector=none
value: A pixel-row pin that font-embolden moves is pinned as a stated band with its reason in a comment where the test crate cannot see the feature, its wholly-inside assertions kept exact.
basis: fct-fafhq3ea
because: The operator ruled a stated band for an embolden-moved pixel row where the feature is unreadable, an exception to pinning the measured literal.
@row_set n=6 row=contract/session-run/changed-node-states cell=promise seen=1 detector=none
value: An acting call returns the node it changed with its `selected`, `invalid` or `highlighted` reading as it reads after the step.
basis: fct-s4gho2nl
because: The diff's changed nodes can now name `highlighted`, as a press in an open list does, so the promise listing two readings is incomplete.
@row_add n=1 row=contract/harness-contrast/highlight-contrast area=3-test-harness-contract fact=F1 detector=none
boundary: Harness contrast
item: highlight_contrast
promise: highlight_contrast(NodeId) -> Option<TextContrast> is a public read-only reader answering only for the highlighted row of an open select list, with the list's highlight text and background through the one contrast formula and the option's own font size and weight, and None for any other node.
basis: fct-voh3czmv
because: The harness's public API gained a contrast reader, a contract at the harness boundary no row holds.
@row_add n=2 row=contract/snapshot-node/highlighted area=unit-test-strategy fact=F5 detector=none
boundary: Snapshot node
item: highlighted
promise: `highlighted` is a closed bool written as a bare token after `clickable` and as JSON true only when true, read from the tree's active descendant of an open single select.
basis: fct-s4gho2nl
because: The snapshot node gained a fifteenth key, a promise at the snapshot boundary beside `clickable` that no row holds.
