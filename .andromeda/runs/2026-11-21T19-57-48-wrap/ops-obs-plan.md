@row_set n=1 row=decision/select-size-attribute-read cell=rule seen=1 detector=none
value: The size attribute on a select, multiple or single, is read as a whole number capped at 65534, carrying no text out and moving no redaction shape, for every document.
basis: fct-prneohlv
because: A single select's size is now read in every document as its display size through the same capped read, so the row's multiple-select-only reach is incomplete.
@row_set n=2 row=decision/select-chosen-set-never-logged cell=rule seen=1 fact=F1 detector=none
value: A select's chosen state never leaves as a log line, span or field: a multiple select's chosen set leaves only as form entries, the snapshot's value and the options' tree nodes, and a single list box's chosen row and row labels only in the snapshot, the diff and the actions answer.
basis: fct-3lv5erx6
because: A single select of display size above 1 is now a list box whose chosen row and row labels cross in the snapshot, the diff and the actions answer with no log site, span or field added, which the row stated for multiple selects alone.
@row_add n=1 row=contract/snapshot/single-list-box-value area=pii-scrubbing-compliance fact=F2 detector=none
boundary: the snapshot
item: value of a single list box
promise: No value is read while nothing is chosen; a single list box may stand with nothing chosen, where a drop-down left with nothing chosen settles to its first enabled option.
basis: fct-s4plbaso
because: A single list box may now stand with nothing chosen and the snapshot then reads no value for it, a promise at the snapshot boundary beside the multiple select's value row.
